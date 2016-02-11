# Amethyst Tools

[![Build Status][s1]][tc] [![Crates.io][s2]][ci] [![GPL3 License][s3]][gl] [![Join the chat][s4]][gc]

[s1]: https://api.travis-ci.org/ebkalderon/amethyst_tools.svg
[s2]: https://img.shields.io/badge/crates.io-0.2.4-orange.svg
[s3]: https://img.shields.io/badge/license-GPL%20v3-blue.svg
[s4]: https://badges.gitter.im/ebkalderon/amethyst.svg

[tc]: https://travis-ci.org/ebkalderon/amethyst_tools/
[ci]: https://crates.io/crates/amethyst_tools/
[gl]: https://github.com/ebkalderon/amethyst_tools/blob/master/COPYING
[gc]: https://gitter.im/ebkalderon/amethyst?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge

A suite of game development tools written in [Rust][rl], intended for use with
the [Amethyst][am] engine. This project is a *work in progress* and is very
incomplete; pardon the dust!

[rl]: https://www.rust-lang.org/
[am]: https://github.com/ebkalderon/amethyst

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

## Toolchain

At the moment, there is only one tool available in this distribution. Please
suggest ideas for more tools on [our issue tracker][it].

[it]: https://github.com/ebkalderon/amethyst_tools/issues

* [Amethyst CLI][ac] - Command-line interface for creating and deploying game
  projects, intentionally very similar to [Cargo][ca].

[ac]: https://github.com/ebkalderon/amethyst_tools/tree/master/src/cli
[ca]: https://github.com/rust-lang/cargo

## Contributing

We are a community project that welcomes contribution from anyone. If you're
interested in helping out, please read the [CONTRIBUTING.md][cm] file before
getting started. Don't know what to hack on? See the [Development Roadmap][dr]
on our wiki, or search though [our issue tracker][it].

[cm]: https://github.com/ebkalderon/amethyst/blob/master/CONTRIBUTING.md
[dr]: https://github.com/ebkalderon/amethyst/wiki/Roadmap
