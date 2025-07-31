### Rust Music Player

A music player with interchangeable clients

## Goal

Not relying on big platforms or even an internet connection to listen to store, organise and play music.

And being able to control the music server with whatever client im more confortable using in the moment.  
TUI or GUI most of the time, but cli is nice to have for scripting or occasional commands

## State
Still in very early phase of dev.

- Server
    Rodio player is working well
    Basic downloader done, need to clean it up a bit
- Cli
  Closely follows what the server is capable of
- Tui
  Not yet started
- Gui
  Not yet started

Check the [Roadmap](./roadmap.md) for precise information

## Notes
The current implementation of the downloader uses a Unix specific implementation of non blocking pipes, so the project doesn't compile on W$ for now (see #7).  
I currently use ytdlp as the backend for the downloader, but it's not too tightly integrated, so change is possible

## Installation

Still a big todo, I'll probably make a simple installer like the one I did for [Lumin](https://github.com/bowarc/lumin)



