# OBS CLI Helper
This tool allows you to integrate your OBS setup into a bigger architecture via CLI commands


## Installation
You have to install a Rust toolchain (with rustup for example) and use cargo to build the tool:
```
cargo build
```


## Usage
To list hotkeys available in your OBS Instance:
```
obs_cli_helper list 
```


To trigger a hotkey:
```
obs_cli_helper trigger <hotkey name>
```
