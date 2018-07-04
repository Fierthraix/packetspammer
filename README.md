# packetspammer

- [Installation](#installation)
    - [Linux](#linux)
    - [Rust Users](#rust-users)
        - [Building](#building)
- [Usage](#usage)
    - [TCP](#tcp)
    - [UDP](#udp)
- [Useful Examples](#useful-examples)
- [Contributing](#contributing)
- [Licence](#licence)

## Installation

### Linux

#### Arch Linux

You can install `packetspammer` on Arch Linux from the AUR package `packetspammer`

### Rust Users

    $ cargo install packetspammer

#### Building

    $ git clone 'https://github.com/Fierthraix/packetspammer'
    $ cargo build --release
    $ sudo ./target/release/packetspammer

## Usage

With no arguments packetspammer will attempt to spam packets on the default device indefinitely

### TCP

Use the `--tcp` flag to simulate a tcp conversation

### UDP

Use the `--udp` flag to send udp packets


