use std::fs;
use std::{default::Default, path::Path};

use anyhow::Result;
use obws::{requests::hotkeys::KeyModifiers, Client};

use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

//===== Config =====//
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ConfigFile {
    #[serde(default)]
    address: Option<String>,

    #[serde(default)]
    port: Option<u16>,

    #[serde(default)]
    password: Option<String>,
}

#[derive(Debug)]
struct Config {
    address: String,
    port: u16,
    password: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            address: String::from("localhost"),
            port: 4455,
            password: String::from("password"),
        }
    }
}

impl From<ConfigFile> for Config {
    fn from(config: ConfigFile) -> Self {
        let default = Config::default();
        Config {
            address: config.address.unwrap_or(default.address),
            port: config.port.unwrap_or(default.port),
            password: config.password.unwrap_or(default.password),
        }
    }
}

//===== Arg Parser =====//

#[derive(Parser)]
#[command(
    author = "Nabos",
    version = "1.0",
    about = "OBS Hotkeys",
    long_about = "This tool allows you to integrate your OBS setup into a bigger architecture via CLI commands"
)]
struct Cli {
    #[clap(subcommand)]
    action: Action,

    ///YAML Configuration file to load the parameters from
    #[arg(short, long, default_value = "~/.config/obs_hotkeys/config.yaml")]
    config_file: String,

    ///OBS host address to connect to
    #[arg(short, long)]
    address: Option<String>,

    ///OBS host port to connect to
    #[arg(short, long)]
    port: Option<u16>,

    ///OBS host password to connect with
    #[arg(long)]
    password: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Action {
    ///List hotkeys registered in OBS
    List,

    ///Asks OBS to trigger the specified hotkey
    Trigger(TriggerAction),

    ///Sends a key sequence to OBS
    Sequence(SequenceAction),
}

#[derive(Args, Debug)]
struct TriggerAction {
    ///OBS hotkey name to trigger
    hotkey_name: String,
}

#[derive(Args, Debug)]
struct SequenceAction {
    ///OBS Key ID to send (in the form OBS_KEY_<KEY>, ex: OBS_KEY_A for 'a')
    key_id: String,

    ///Shift modifier
    #[arg(long)]
    shift: bool,

    ///Control modifier
    #[arg(long)]
    control: bool,

    ///Alt modifier
    #[arg(long)]
    alt: bool,

    ///Command (super) modifier
    #[arg(long)]
    command: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    //Default configuration
    let config_path = shellexpand::tilde(&args.config_file);
    let mut config: Config = if Path::new(&config_path.to_string()).exists() {
        let config_txt = fs::read_to_string(&config_path.to_string())
            .expect("Unable to read configuration file");
        let config_file: ConfigFile =
            serde_yaml::from_str(&config_txt).expect("Could not parse configuration file");
        config_file.into()
    } else {
        Config::default()
    };

    //Configuration overrides
    if let Some(address) = args.address {
        config.address = address;
    }
    if let Some(port) = args.port {
        config.port = port;
    }
    if let Some(password) = args.password {
        config.password = password;
    }

    let client = Client::connect(config.address, config.port, Some(config.password)).await?;

    match args.action {
        Action::List => {
            let hotkeys = client.hotkeys().list().await?;
            println!("{:#?}", hotkeys);
        }
        Action::Trigger(action) => {
            client
                .hotkeys()
                .trigger_by_name(&action.hotkey_name)
                .await?;
        }
        Action::Sequence(action) => {
            client
                .hotkeys()
                .trigger_by_sequence(
                    &action.key_id,
                    KeyModifiers {
                        shift: action.shift,
                        control: action.shift,
                        alt: action.alt,
                        command: action.command,
                    },
                )
                .await?;
        }
    }

    Ok(())
}
