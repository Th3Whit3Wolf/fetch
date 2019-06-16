<h1 align=center>fetch</h1>
        <h3 align=center>A command-line system information tool written in Rust[WIP]</h3>

## Detection
- Username
- Hostname
- OS
- Host
- Uptime
- Destop Environment
- Shell
- Memory
- Packages

#### In Progress
- GPU(s)
- CPU (Frequency, Logical Cores, Physical Cores, Temperature)

## TODO
- [ ] Better Shell Support
- [ ] More package detection (limited to pacman at the moment)
- [ ] Window Manager Detection
- [ ] IP Address
- [ ] OS Installation Date
- [ ] Themes
- [ ] Icons
- [ ] Ascii Art
- [ ] Better Cross Platform Support

## Goals
- Multithreaded
- Async
- Hook into system libraries

### Installation
#### Arch Linux
```bash
cargo install --git https://github.com/Th3Whit3Wolf/fetch.git--features arch
```

## Philosophy
- Be unnecessarily fast
- Use the fewest(ideally none) externall processes
- Run[eventually] on all Linux Distros, BSD, Mac, Windows, and Haiku
