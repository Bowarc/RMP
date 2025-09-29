### Rust Music Player

A music player with interchangeable clients

## Goal

Not relying on big platforms or even an internet connection to store, organise and listen to music.

And being able to control the music server with whatever client im more confortable using in the moment.  
TUI or GUI most of the time, but cli is nice to have for scripting or occasional commands

## State
Works well, need playlists

- Server  
    Rodio player is working well  
    Basic downloader done, need to clean it up a bit
- Cli  
  Closely follows what the server is capable of
- Tui  
  Not yet started
- Gui  
  Player is working, most of the features are implemented but the interface is still very ugly  
  Downloader technically works, but is still very debug-y looking  

Check the [Roadmap](./roadmap.md) for precise information

## Notes
The current implementation of the downloader uses a Unix specific implementation of non blocking pipes, so the project doesn't compile on W$ for now (see [#7](https://github.com/bowarc/rmp/issues/7)).  
I currently use ytdlp as the backend for the downloader, but it's not too tightly integrated, so change is still possible

## Project structure
[server](/server/README.md) (bin) - The server, runs in the background and receive commands from clients.  
[cli](/cli/README.md) (bin) - The CLI client  
[tui](/tui/README.md) (bin) - The TUI client  
[gui](/gui/README.md) (bin) - The GUI client  
[client](./client) (lib) - Helps creating new clients  
[shared](/shared/README.md) (lib) - Holds a lot of shared definition  
[models](./models) (lib) - Stores models of json objects (helps with compilation times)  

## Installation

Still a big todo, I'll probably make a simple installer like the one I did for [Lumin](https://github.com/bowarc/lumin)


