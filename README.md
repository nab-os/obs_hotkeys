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
