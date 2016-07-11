# Toolchain Usage

* [Build Tool](#Build Tool)

# Build Tool

Command-line tool for managing game projects. Intentionally very similar to
[Cargo][ca].

[ab]: https://github.com/amethyst/tools/tree/master/src/bin/amethyst.rs
[ca]: https://github.com/rust-lang/cargo

#### add

Grabs Amethyst engine modules (e.g. rendering, scripting, physics, etc.) called
"shards" from either crates.io or GitHub, configures your `Cargo.toml` and
modifies the `resources` directory accordingly. Once a new shard is installed
and configured, just drop your assets into the appropriate folders and you can
start writing your game logic.

### new

Generates an empty game project, much like Cargo. The default directory
structure is laid out like this:

```
project/
├── Cargo.toml
├── resources
│   ├── config.yml
│   ├── entities/
│   ├── input.yml
│   └── prefabs/
└── src
    └── main.rs
```

### build

Compiles the current project and not much else. More features are in the
works, like [Lua][lu] or [mruby][mr] script precompilation and offline GPU
shader compilation for APIs that support it.

[lu]: http://www.lua.org/
[mr]: http://mruby.org/

#### clean

Removes the `target` directory. Will have switches whether or not to clear out
said precompiled scripts or cached shader program bytecode.

#### deploy

Performs a clean rebuild of the game and engine, runs any unit and integration
tests if there are any, zips up the `resources` directory, and places it and
the game binary in a directory called `deployed`.

#### run

Runs the main binary of the project, exactly like `cargo run`. Proposed extra
features include real-time profiling and debugging, and manual skipping of
levels when playtesting.

#### test

Compiles and runs all integration and unit tests you may have written in your
game project.
