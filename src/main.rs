use std::fs;
use std::{default::Default, path::Path};

use anyhow::Result;
use obws::{requests::hotkeys::KeyModifiers, Client};

use argh::FromArgs;
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

#[derive(FromArgs, PartialEq, Debug)]
#[argh(
    description = "This tool allows you to integrate your OBS setup into a bigger architecture via CLI commands"
)]
struct Cli {
    #[argh(subcommand)]
    action: Action,

    ///YAML Configuration file to load the parameters from
    #[argh(
        option,
        short = 'c',
        default = "String::from(\"~/.config/obs_hotkeys/config.yaml\")"
    )]
    config_file: String,

    ///OBS host address to connect to
    #[argh(option, short = 'a')]
    address: Option<String>,

    ///OBS host port to connect to
    #[argh(option, short = 'p')]
    port: Option<u16>,

    ///OBS host password to connect with
    #[argh(option)]
    password: Option<String>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Action {
    ///List hotkeys registered in OBS
    List(ListAction),

    ///Asks OBS to trigger the specified hotkey
    Trigger(TriggerAction),

    ///Sends a key sequence to OBS
    Sequence(SequenceAction),
}

#[derive(FromArgs, PartialEq, Debug)]
///List OBS Events
#[argh(subcommand, name = "list")]
struct ListAction {}

#[derive(FromArgs, PartialEq, Debug)]
///Trigger an OBS Event
#[argh(subcommand, name = "trigger")]
struct TriggerAction {
    ///OBS hotkey name to trigger
    #[argh(positional)]
    hotkey_name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
///Send an OBS Key
#[argh(subcommand, name = "sequence")]
struct SequenceAction {
    ///OBS Key ID to send (in the form OBS_KEY_<KEY>, ex: OBS_KEY_A for 'a')
    #[argh(positional)]
    key_id: String,

    ///shift modifier
    #[argh(switch)]
    shift: bool,

    ///control modifier
    #[argh(switch)]
    control: bool,

    ///alt modifier
    #[argh(switch)]
    alt: bool,

    ///command (super) modifier
    #[argh(switch)]
    command: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Cli = argh::from_env();

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
        Action::List(_) => {
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
