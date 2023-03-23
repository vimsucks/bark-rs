use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "bark")]
#[command(author = "Vimsucks <dev@vimsucks.com>")]
#[command(version = "1.0")]
#[command(about = "A simple tool for https://github.com/Finb/Bark", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Push(PushArgs),
    Device(DeviceCli),
}

#[derive(Parser, Debug)]
#[command(about = "manage device", long_about = None)]
pub struct DeviceCli {
    #[command(subcommand)]
    pub command: DeviceCommands,
}

#[derive(Subcommand, Debug)]
pub enum DeviceCommands {
    List(ListDeviceArgs),
    Add(AddDeviceArgs),
    SetDefault(SetDefaultDeviceArgs),
}

#[derive(Args, Debug)]
#[command(about = "list device", long_about = None)]
pub struct ListDeviceArgs {}

#[derive(Args, Debug)]
#[command(about = "add device", long_about = None)]
pub struct AddDeviceArgs {
    #[arg(short = 'n', long = "name")]
    pub name: String,
    #[arg(short = 'u', long = "server-url")]
    pub server_url: String,
    #[arg(short = 'k', long = "device-key")]
    pub device_key: String,
    #[arg(short, long)]
    pub default: bool,
}

#[derive(Args, Debug)]
#[command(about = "set default device", long_about = None)]
pub struct SetDefaultDeviceArgs {}

#[derive(ValueEnum, Clone, Debug)]
pub enum PushLevel {
    Active,
    TimeSensitive,
    Passive,
}

/// visit https://bark.day.app/#/tutorial for bark tutorial
#[derive(Args, Debug)]
#[command(about = "push notification to device", long_about = None)]
pub struct PushArgs {
    #[arg(short = 't', long = "title")]
    pub title: Option<String>,

    #[arg(short = 'b', long = "body")]
    pub body: Option<String>,

    #[arg(value_enum, short = 'l', long = "level")]
    pub level: Option<PushLevel>,

    #[arg(short = None, long = "badge")]
    pub badge: Option<u32>,

    #[arg(short=None, long = "auto-copy")]
    pub auto_copy: bool,

    #[arg(short = 'c', long = "copy")]
    pub copy: Option<String>,

    #[arg(short = 's', long = "sound")]
    pub sound: Option<String>,

    #[arg(short = 'i', long = "icon")]
    pub icon: Option<String>,

    #[arg(short = 'g', long = "group")]
    pub group: Option<String>,

    #[arg(short=None, long = "archive")]
    pub archive: bool,

    #[arg(short = 'u', long = "url")]
    pub url: Option<String>,

    #[arg(short = 'd', long = "device", help = "push to which device")]
    pub device: Option<String>,
}
