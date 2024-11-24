use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use clap::Parser;
use indoc::formatdoc;
use linode_api::{
    apis::{
        configuration::Configuration,
        linode_instances_api::{
            add_linode_config, boot_linode_instance, create_linode_instance,
            delete_linode_instance, get_linode_instances,
        },
        linode_types_api::get_linode_type,
        volumes_api::get_volumes,
    },
    models::{
        AddLinodeConfigRequest, BootLinodeInstanceRequest, CreateLinodeInstanceRequest,
        GetLinodeConfigs200ResponseDataInnerDevices,
        GetLinodeConfigs200ResponseDataInnerDevicesSda, GetLinodeInstances200ResponseDataInner,
    },
};
use std::{fs::File, io::Write};

/// A command line utility to manage remote development environments on Linode.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

/// Start a development environment.
#[derive(Parser, Debug)]
struct StartCommand {
    #[clap(short, long, default_value = "4", value_parser = ["4", "16"])]
    cpu: String,
}

/// Stop a development environment.
#[derive(Parser, Debug)]
struct StopCommand {}

/// Show statistics (uptime, cost) for the development environment.
#[derive(Parser, Debug)]
struct StatsCommand {}

#[derive(clap::Subcommand, Debug)]
enum Command {
    #[command(about)]
    Start(StartCommand),

    #[command(about)]
    Stop(StopCommand),

    #[command(about)]
    Stats(StatsCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let ssh_config_path = dirs::home_dir()
        .with_context(|| "failed to infer user home dir".to_string())?
        .join(".ssh/config_nowdev");
    let token_path = dirs::home_dir()
        .with_context(|| "failed to infer user config dir".to_string())?
        .join(".config/nowdev/token");
    let access_token = std::fs::read_to_string(&token_path)
        .with_context(|| format!("failed to read access token from: {:?}", token_path))?
        .trim()
        .to_string();

    let linode_cfg = Configuration {
        bearer_access_token: Some(access_token),
        ..Configuration::default()
    };

    match args.command {
        Command::Start(start_command) => {
            let r#type = match start_command.cpu.parse::<u32>().unwrap() {
                4 => "g6-standard-4",
                16 => "g6-standard-16",
                _ => panic!("Invalid CPU count: {}", start_command.cpu),
            };

            // Create the instance.
            let instance = create_linode_instance(
                &linode_cfg,
                CreateLinodeInstanceRequest {
                    booted: Some(false),
                    r#type: r#type.to_string(),
                    region: "eu-west".to_string(),
                    label: Some("nowdev-dev".to_string()),
                    tags: Some(vec!["nowdev".to_string()]),
                    ..CreateLinodeInstanceRequest::default()
                },
            )
            .await
            .with_context(|| "failed to create instance".to_string())?;

            let volumes = get_volumes(&linode_cfg, None, None)
                .await
                .with_context(|| "failed to get volumes".to_string())?
                .data
                .unwrap();

            let dev_volume = volumes
                .iter()
                .find(|v| v.label.as_ref().map(|l| l == "nowdev-dev").unwrap_or(false))
                .expect("failed to find nowdev-dev volume");

            let config = add_linode_config(
                &linode_cfg,
                instance.id.unwrap(),
                AddLinodeConfigRequest {
                    label: "boot configuration".to_string(),
                    devices: Box::new(GetLinodeConfigs200ResponseDataInnerDevices {
                        sda: Some(Box::new(GetLinodeConfigs200ResponseDataInnerDevicesSda {
                            disk_id: None,
                            volume_id: Some(dev_volume.id.unwrap()),
                        })),
                        ..GetLinodeConfigs200ResponseDataInnerDevices::default()
                    }),
                    ..AddLinodeConfigRequest::default()
                },
            )
            .await
            .with_context(|| "failed to create instance config".to_string())?;

            println!("Added config");

            boot_linode_instance(
                &linode_cfg,
                instance.id.unwrap(),
                Some(BootLinodeInstanceRequest {
                    config_id: Some(config.id.unwrap()),
                }),
            )
            .await
            .with_context(|| "failed to boot instance".to_string())?;

            let ipv4 = instance.ipv4.unwrap()[0].clone();
            println!("https://cloud.linode.com/linodes/{}", instance.id.unwrap());
            println!("IP: {}", ipv4.as_str());

            let mut ssh_config = File::create(ssh_config_path)
                .with_context(|| "failed to open ssh config file".to_string())?;
            ssh_config.write_all(
                formatdoc! {"
            Host nowdev-dev
                HostName {0}
                StrictHostKeyChecking no

            Host nowdev-dev-tmux
                HostName {0}
                StrictHostKeyChecking no
                RequestTTY yes
                RemoteCommand tmux -CC new-session -A -s main
            ", ipv4.as_str()}
                .as_bytes(),
            )?;
            println!("Writing ssh configuration to ~/.ssh/config_nowdev");

            loop {
                let instance = find_instance(&linode_cfg, "nowdev-dev").await?;
                match instance {
                    Some(instance) => {
                        println!("Instance status: {:?}", instance.status.as_ref().unwrap());
                        if *instance.status.as_ref().unwrap() == linode_api::models::get_linode_instances_200_response_data_inner::Status::Running {
                            return Ok(())
                        } else {
                            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                        }
                    }
                    None => {
                        return Err(anyhow!("Instance not found"));
                    }
                }
            }
        }
        Command::Stop(_) => {
            if ssh_config_path.exists() {
                std::fs::remove_file(ssh_config_path)
                    .with_context(|| "failed to remove ssh config file".to_string())?;
            }

            let instance = find_instance(&linode_cfg, "nowdev-dev").await?;
            if instance.is_none() {
                println!("Instance not found");
                return Ok(());
            }

            print_instance_stats(&linode_cfg, instance.as_ref().unwrap()).await?;

            delete_linode_instance(&linode_cfg, instance.as_ref().unwrap().id.unwrap())
                .await
                .with_context(|| {
                    format!(
                        "failed to delete instance: {}",
                        instance.unwrap().id.unwrap()
                    )
                })?;
            println!("Instance deleted");
        }
        Command::Stats(_) => {
            let instance = find_instance(&linode_cfg, "nowdev-dev").await?;
            if instance.is_none() {
                println!("Instance not found");
                return Ok(());
            }

            print_instance_stats(&linode_cfg, instance.as_ref().unwrap()).await?;
        }
    }

    Ok(())
}

async fn find_instance(
    linode_cfg: &Configuration,
    name: &str,
) -> Result<Option<GetLinodeInstances200ResponseDataInner>> {
    let instances = get_linode_instances(linode_cfg, None, None)
        .await
        .with_context(|| "failed to get instances".to_string())?
        .data
        .unwrap();
    let instance = instances
        .iter()
        .find(|instance| instance.label.as_ref().unwrap() == name);
    Ok(instance.cloned())
}

async fn print_instance_stats(
    linode_cfg: &Configuration,
    instance: &GetLinodeInstances200ResponseDataInner,
) -> Result<()> {
    let created =
        DateTime::parse_from_rfc3339(&format!("{}Z", instance.created.as_ref().unwrap())).unwrap();
    let uptime = Utc::now().signed_duration_since(created);

    println!(
        "Uptime: {} hours and {} minutes",
        &uptime.num_hours(),
        &uptime.num_minutes() % 60
    );
    println!("Instance type: {}", instance.r#type.as_ref().unwrap());

    let hourly_price = get_linode_type(linode_cfg, instance.r#type.as_ref().unwrap())
        .await?
        .price
        .unwrap()
        .hourly
        .unwrap();

    let cost = (uptime.num_seconds() as f64 / 3600.0).ceil() * hourly_price as f64;

    println!("Cost: ${:.2} (per hour: ${})", cost, hourly_price);

    Ok(())
}
