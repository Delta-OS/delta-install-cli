
# Delta-Install CLI

Delta-Install CLI is our command-line tool to install Delta OS GNU/Linux and to customize it with our choices or even your choices !


## Installation

Build Delta-Install CLI from source with `cargo`:
```bash
git clone https://github.com/Delta-OS/delta-install-cli.git
cd delta-install-cli
cargo build
```

Or download the binary [release](https://github.com/Delta-OS/delta-install-cli/releases) file and execute it.


## Documentation

Welcome to the delta-install-cli wiki!  
This wiki is going to teach you the good way to use our install tool to install Delta OS GNU/Linux correctly or your custom of **Delta OS** with **your choices** !
## Configuration

So, primarily, delta-install is based on a JSON configuration file that you are fully permitted to custom as you want.  
This is a config.json file template to use delta-install:
```json
{ 
    "device": "desktop",
    "arch": "amd64",
    "packages": ["git","neofetch"],
    "sources": "/etc/apt/sources.list",
    "hostname": "deltaos",
    "osrel": "/etc/os-release"
}
```
Let's explain the config file.  
`Device` entry of the JSON file is written to know what is your device.  
It will be more useful in the next releases but for now, just learn that there are three possible values of this entry: `desktop`, `server` and `pi` (pi is used to install Delta OS GNU/Linux on a Raspberry-Pi).  
This entry determines what is the type of your machine and in what way you are going to use it.  
`desktop` is prefered for **daily use** while `server` is prefered for a **server use or eventually a minimal installation**.

`Arch` entry is the **architecture of your machine**.  
Actually, there are two different values for this entry: `amd64` for **x86_64 CPUs** and `arm64` for **ARM64 CPUs** like the Pi 4 CPU.

`Packages` entry is the array of **additional custom packages** which will be added during your installation of Delta OS GNU/Linux.  
For example, here, this installation will install Delta OS GNU/Linux with git and neofetch as additional packages.

`Sources` entry is the path to the `sources.list` file which will be copied into your new Delta OS GNU/Linux system.  
It is **very important** to write a valid path without a `/` at the end of the path.

`osrel` entry is the path to the `os-release` file which will be copied into your new Delta OS GNU/Linux system and which will contain most of important informations of your OS.  
It is **very important** to write a valid path without a `/` at the end of the path.

`hostname` entry is the hostname of your system, just write it and it will be copied into the good file in your future system.


## Usage

```bash
delta-install --config=<config file> <DESTINATION PATH without a / at the end>
```