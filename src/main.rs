use clap::Parser;
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let access_token = std::fs::read_to_string("/Users/nv/.config/nowdev/token")?
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
            .unwrap();

            println!("Created instance");

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
            .await?;

            println!("Added config");

            boot_linode_instance(
                &linode_cfg,
                instance.id.unwrap(),
                Some(BootLinodeInstanceRequest {
                    config_id: Some(config.id.unwrap()),
                }),
            )
            .await?;

            println!("https://cloud.linode.com/linodes/{}", instance.id.unwrap());
            println!("IP: {}", instance.ipv4.unwrap()[0]);
        }
        Command::Stop(_) => {
            let instances = get_linode_instances(&linode_cfg, None, None)
                .await
                .unwrap()
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
                .unwrap();
            println!("Instance deleted");
        }
    }

    Ok(())
}
