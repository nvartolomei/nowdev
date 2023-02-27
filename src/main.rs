use anyhow::{Context, Result};
use clap::Parser;
use indoc::formatdoc;
use linode_api::{
    apis::{
        configuration::Configuration,
        linode_instances_api::{
            add_linode_config, boot_linode_instance, create_linode_instance,
            delete_linode_instance, get_linode_instances,
        },
    },
    models::{
        AddLinodeConfigRequest, BootLinodeInstanceRequest, CreateLinodeInstanceRequest,
        GetLinodeConfigs200ResponseDataInnerDevices,
        GetLinodeConfigs200ResponseDataInnerDevicesSda,
    },
};
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
struct StartCommand {
    #[clap(short, long, default_value = "4", value_parser = ["4", "16"])]
    cpu: String,
}

#[derive(Parser, Debug)]
struct StopCommand {}

#[derive(clap::Subcommand, Debug)]
enum Command {
    #[command(about = "start a development environment")]
    Start(StartCommand),

    #[command(about = "stop a development environment")]
    Stop(StopCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

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
                    label: Some("dev".to_string()),
                    tags: Some(vec!["nowdev".to_string()]),
                    ..CreateLinodeInstanceRequest::default()
                },
            )
            .await
            .with_context(|| "failed to create instance".to_string())?;

            let config = add_linode_config(
                &linode_cfg,
                instance.id.unwrap(),
                AddLinodeConfigRequest {
                    label: "boot configuration".to_string(),
                    devices: Box::new(GetLinodeConfigs200ResponseDataInnerDevices {
                        sda: Some(Box::new(GetLinodeConfigs200ResponseDataInnerDevicesSda {
                            disk_id: None,
                            volume_id: Some(898271),
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

            let mut ssh_config = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(
                    dirs::home_dir()
                        .with_context(|| "failed to infer user home dir".to_string())?
                        .join(".ssh/config_nowdev"),
                )
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
        }
        Command::Stop(_) => {
            let instances = get_linode_instances(&linode_cfg, None, None)
                .await
                .with_context(|| "failed to get instances".to_string())?
                .data
                .unwrap();
            let instance = instances
                .iter()
                .find(|instance| instance.label == Some("dev".to_string()));
            if instance.is_none() {
                println!("No instance found");
                return Ok(());
            }

            delete_linode_instance(&linode_cfg, instance.unwrap().id.unwrap())
                .await
                .with_context(|| {
                    format!(
                        "failed to delete instance: {}",
                        instance.unwrap().id.unwrap()
                    )
                })?;
            println!("Instance deleted");
        }
    }

    Ok(())
}
