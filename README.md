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
To list hotkeys available in your OBS Instance:
```
obs_hotkeys list 
```


To trigger a hotkey:
```
obs_hotkeys trigger <hotkey name>
```


## To-do
* Add a way to configure address, port & password
* Add an action to send a key combo directly to OBS via [trigger-by-sequence](https://docs.rs/obws/latest/obws/client/struct.Hotkeys.html#method.trigger_by_sequence) (because of the hotkeys naming scheme some hotkeys have the same name, 'SelectScene' for example)
