# OBS Hotkeys
This tool allows you to integrate your OBS setup into a bigger architecture via CLI commands


## Installation
You have to install a Rust toolchain (with rustup for example) and use cargo to build the tool:
```
git clone https://github.com/nab-os/obs_hotkeys
cd obs_hotkeys
cargo build
cargo install --path .
```


## Usage
To display the full usage:
```
obs_hotkeys --help
```

To connect to the OBS instance you need to setup the address, the port and the websocket password.

You can do this with a config file or with command parameters.

The default config file is looked at `~/.config/obs_hotkeys/config.yaml` but you can specify a path with the `--config-file <path>` parameter.

Here is an example config file:
```
address: "localhost"
port: 4455
password: "password"
```

Or you can use the `--address <address>` `--port <port>` and `--password <password>` parameters.


To list hotkeys available in your OBS Instance:
```
obs_hotkeys list 
```


To trigger a hotkey:
```
obs_hotkeys trigger <hotkey name>
```

To trigger a key sequence:
```
obs_hotkeys sequence <key_id> [--shift] [--control] [--alt] [--command]
```

*key_id* is in the form OBS_KEY_<KEY>, ex: OBS_KEY_A for 'a'

