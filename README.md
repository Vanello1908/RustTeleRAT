# RustTeleRAT
## Description
This is a simple RAT written on Rust. It is based on Telegram Bot and uses workspace with modules. 

It has two modes:
- single use mode
- RAT mode

In single use mode program uses all modules and send data to Telegram Bot with specified chat id.

In RAT mode you can control program and run modules directly by Telegram Bot commands.

## Usage
You have to edit bot token and chat id in [config lib](config/src/lib.rs). After that run command `cargo build --release` 

Binaries:
- one-time, it is single use mode
- long, it is RAT mode
- main, you shouldn't use this binary, it is specififed for different experiments

## TODO
### Camera
- [x] Photo
- [ ] Video without audio
- [ ] Video with audio
### Info
- [x] Ip
- [ ] Geolocation
- [x] OS information
- [ ] Hardware information
- [ ] Hard drive usage
### Scrapper
- [x] Telegram session
- [ ] Discord session
- [ ] Steam session
- [ ] Browser data(passwords, cookies, etc.)
### System
- [x] Take screenshot
- [ ] Execute shell commands
- [ ] Download files
- [ ] Upload files
- [ ] Record audio



