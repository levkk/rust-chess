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
- Super basic and kind of unplayable GUI in OpenGL.

## Missing Features
- Validation of piece ownership: a player can move the pieces of the other player.
- Collision checking for validation of moves.
- HTTP client/server for NATed players.
