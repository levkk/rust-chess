# Rust Chess

Implementation of chess in Rust.

## Build & Run
```
$ cargo run
```

## Features
- Validation of moves using basic chess rules.
- Gameplay using chess notation (e.g. e2e4 moves any piece from e2 to e4)
- Saving/loading to/from JSON.
- Multiplayer using direct connection TCP.

## Missing Features
- Validation of piece ownership: a player can move the pieces of the other player.
- Collision checking for validation of moves.
- GUI (might write an OpenGL GUI in Kiss3d)
- HTTP client/server for NATed players.
