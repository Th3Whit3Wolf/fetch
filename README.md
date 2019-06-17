<h1 align=center>fetch</h1>
<h3 align=center>A command-line system information tool written in Rust[WIP]</h3>

[![Build Status](https://travis-ci.org/Th3Whit3Wolf/fetch.svg?branch=master)](https://travis-ci.org/Th3Whit3Wolf/fetch)


[![codecov](https://codecov.io/gh/Th3Whit3Wolf/fetch/branch/master/graph/badge.svg)](https://codecov.io/gh/Th3Whit3Wolf/fetch)


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
```bash
cargo install --git https://github.com/Th3Whit3Wolf/fetch.git
```

## Philosophy
- Be unnecessarily fast
- Use the fewest(ideally none) externall processes
- Run[eventually] on all Linux Distros, BSD, Mac, Windows, and Haiku
