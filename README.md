# choose-your-own-adventure
A CYOA game engine in Rust. Main goal for now is learning Rust by choosing a project that really benefits 
from a robust type system and is easy to start with a simple command line interface and scale from there to anything from a web UI for playing
to story creation GUIs. For now you just specify a TOML file in an undocumented format and play the game in the terminal with no saves.

You can parse and play the short included example game by running this command in the project root:

```
cargo run examples/basic-choices.toml
```

I'm no writer, so don't expect thrilling adventures or good prose.
