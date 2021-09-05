[![Build Status](https://travis-ci.com/levkk/rust-chess.svg?branch=master)](https://travis-ci.com/levkk/rust-chess)

# Rust Chess

Implementation of chess in Rust.

## Requirements

### Linux & macOS
This program is using native OpenGL and GLFW. So, you'll need pretty much everything that comes up from

```
$ pkg-config --libs --static gl glfw3
```

#### Ubuntu 18.04+
Just installing GLFW is enough:

```
apt-get install -y libglfw3-dev cmake libssl-dev libxinerama-dev libxcursor-dev libxi-dev libxxf86vm-dev
```

### Windows
Same, you'll need your graphics card OpenGL driver and GLFW installed. Never tried to compile a Rust program with OpenGL
bindings on Windows, so let me know if it works. :)


## Build & Run
```
$ cargo run
```

## Features
- Validation of moves using basic chess rules.
- Gameplay using chess notation (e.g. e2e4 moves any piece from e2 to e4)
- Saving/loading to/from JSON.
- Multiplayer using direct connection TCP.
- Super basic and kind of unplayable GUI in OpenGL.

## Missing Features
- Validation of piece ownership: a player can move the pieces of the other player.
- Collision checking for validation of moves.
- HTTP client/server for NATed players.
