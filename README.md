# DEPRECATED

This project has been deprecated. For more details please see [this forum post].

[this forum post]: https://community.amethyst.rs/t/end-of-life-for-amethyst-tools-the-cli/1656

# Amethyst Tools

[![Build Status][s1]][tc] [![Crates.io][s2]][ci] [![MIT/Apache License][s3]][li]
[![Join us on Discord][s4]][di] [![Code coverage][s5]][cc]

[s1]: https://travis-ci.org/amethyst/tools.svg?branch=master
[s2]: https://img.shields.io/crates/v/amethyst_tools.svg
[s3]: https://img.shields.io/badge/license-MIT%2FApache-blue.svg
[s4]: https://img.shields.io/discord/425678876929163284.svg?logo=discord
[s5]: https://img.shields.io/codecov/c/github/amethyst/tools.svg

[tc]: https://travis-ci.org/amethyst/tools/
[ci]: https://crates.io/crates/amethyst_tools/
[li]: https://github.com/amethyst/tools/blob/master/COPYING
[di]: https://discord.gg/GnP5Whs
[cc]: https://codecov.io/gh/amethyst/tools


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

**Note:** Any interaction with the Amethyst project is subject to our [Code of Conduct][coc].

Amethyst is a community-based project that welcomes contributions from anyone. If you're interested in helping out, please read the [contribution guidelines][cm] before getting started.

We have a [good first issue][gfi] category that groups all issues or feature requests that can be made without having an extensive knowledge of Rust or Amethyst. Working on those issues is a good, if not the best, way to learn.

If you think you are not ready to code yet, you can still contribute by reviewing code written by other members of the community. Code reviews ensure that code merged into Amethyst is of the highest quality as possible. Pull requests that are available for reviews can be found [here][pr].

If for some reason we don't have any open PRs in need of a review nor any good first issues (that would be a good thing), feel free to consult our [issue tracker][it].

[coc]: https://github.com/amethyst/amethyst/blob/master/CODE_OF_CONDUCT.md
[cm]: https://github.com/amethyst/amethyst/blob/master/docs/CONTRIBUTING.md
[gfi]: https://github.com/amethyst/tools/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22
[pr]: https://github.com/amethyst/tools/pulls
[it]: https://github.com/amethyst/tools/issues

## License

Amethyst is free and open source software distributed under the terms of both the [MIT License][lm] and the [Apache License 2.0][la].

[lm]: LICENSE-MIT
[la]: LICENSE-APACHE

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
