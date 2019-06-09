# Amethyst Tools

[![Build Status][s1]][tc] [![Crates.io][s2]][ci] [![MIT/Apache License][s3]][li]
[![Join us on Discord][s4]][di]

[s1]: https://travis-ci.org/amethyst/tools.svg?branch=master
[s2]: https://img.shields.io/crates/v/amethyst_tools.svg
[s3]: https://img.shields.io/badge/license-MIT%2FApache-blue.svg
[s4]: https://img.shields.io/discord/425678876929163284.svg?logo=discord

[tc]: https://travis-ci.org/amethyst/tools/
[ci]: https://crates.io/crates/amethyst_tools/
[li]: https://github.com/amethyst/tools/blob/master/COPYING
[di]: https://discord.gg/GnP5Whs

Command-line interface for the [Amethyst][am] engine to create and deploy game
projects. This project is a *work in progress* and is very incomplete; pardon
the dust!

[am]: https://github.com/amethyst/amethyst

## Vision

One of the goals of [Amethyst][am] is to split up the traditional "mega-editor"
seen in many other game engines into several small but well-integrated tools,
adhering to the [Unix philosophy][up]. This approach allows for nifty things
like:

[up]: https://en.wikipedia.org/wiki/Unix_philosophy

* Piping and streaming data between tools like regular Unix commands.
* Network transparency (e.g. mirroring gameplay from your development machine
  onto a testbed computer or smartphone).
* Customizing your workflow to your liking with plain ol' shell scripts.
* Stripping out tools you don't want or need, or easily supplanting them with
  third-party utilities.
* Serving as backends for various "mega-editors" provided by third parties or
  written in-house.

## Installing

### Requirements
* Ensure that you have the OpenSSL development headers installed (check for
openssl-devel or something similar)

By executing

```sh
cargo install amethyst_tools
```

a binary called `amethyst` will be placed in your `~/cargo/bin` folder.

## Usage

### Creating a new project

```sh
amethyst new <project_name>
```

## Contributing

We are a community project that welcomes contribution from anyone. If you're
interested in helping out, please read the [CONTRIBUTING.md][cm] file before
getting started. Don't know what to hack on? See the [Development Roadmap][dr]
on our wiki, or search though [our issue tracker][it].

All contributions are assumed do be dual-licensed under MIT/Apache-2.0.

[cm]: https://github.com/amethyst/amethyst/blob/master/CONTRIBUTING.md
[dr]: https://github.com/amethyst/amethyst/wiki/Roadmap
