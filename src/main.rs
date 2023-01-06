use anyhow::Result;
use obws::{requests::hotkeys::KeyModifiers, Client};

use clap::{Args, Parser, Subcommand};

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
    let client = Client::connect("localhost", 4455, Some("tTCuXSshGJzfZQxg")).await?;

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
