mod cli;
mod conf;
mod device;
mod push;
use clap::Parser;
use cli::{Cli, Commands, DeviceCommands};

#[tokio::main]
async fn main() {
    // let bark_app = bark_client::App::new("https://api.day.app", "LMHCuHorEjyqiGwzNkuTZQ");
    // match bark_app
    //     .send(bark_client::new_push().with_title("fuck you").with_body("mother fucker"))
    //     .await
    // {
    //     Ok(result) => println!("{:?}", result),
    //     Err(e) => println!("{}", e),
    // }
    let cli = Cli::parse();

    match &cli.command {
        Commands::Device(device_cmd) => match &device_cmd.command {
            DeviceCommands::List(_) => device::list(),
            DeviceCommands::Add(add_device_args) => device::add(add_device_args),
            DeviceCommands::SetDefault(set_default_device_args) => {
                println!("{:?}", set_default_device_args)
            }
        },
        Commands::Push(push_args) => {
            push::push(push_args).await;
        }
    }
}
