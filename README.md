<h1 align=center>fetch</h1>
        <h3 align=center>A command-line system information tool written in Rust</h3>
                <h3 align=center>WIP</h3>

## Features
- Fast

### Installation
#### Arch Linux
```bash
cargo install --git https://github.com/FriedPandaFries/fetch.com --features arch
```
#### Debian, Linux Mint, Ubuntu
```bash
sudo apt install libarchive-dev pkg-config
cargo install --git https://github.com/FriedPandaFries/fetch.com --features deb
```
#### Everyone else
``` bash
cargo install --git https://github.com/FriedPandaFries/fetch.com
```
## Goals
- Be unnecessarily fast
- Use the fewest(ideally none) externall processes
- Run on all Linux Distros and eventually BSD, Mac, and Windows

## ToDos
- [ ] Display WM, WM Theme, Theme, Terminal fonts
- [ ] Support More shells
- [ ] Finish Package Manager Finder function
- [ ] Implement a new way to parser Ascii Art
- [ ] Achieve Feature parity with Neofetch
- [ ] Rework uptime implementation to something more easily read
