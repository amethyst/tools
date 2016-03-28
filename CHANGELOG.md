# Change Log

All notable per-release changes will be documented in this file. This project
adheres to [Semantic Versioning][sv].

[sv]: http://semver.org/

## 0.4.0 (2016-03-XX)

### Added
* Amethyst CLI
  * Add support for `--release` flag (issue #19)

### Fixed
* Amethyst CLI
  * Fix for bad file descriptor error for new command (pull request [#18])

[#18]: https://github.com/amethyst/tools/issues/18
[#19]: https://github.com/amethyst/tools/issues/19

## 0.3.0 (2016-02-23)

### Added
* Amethyst CLI
  * New integration test for detecting failed builds

### Changed
* Amethyst CLI
  * `check_new` integration test checks for exit status (pull request [#14])
  * Reduce verbosity with `try!` (pull request [#16])

[#14]: https://github.com/amethyst/tools/issues/14
[#16]: https://github.com/amethyst/tools/issues/16

### Fixed
* General
  * Recent changes to crates.io breaks `amethyst_tools` installation for some
    users (issue [#17])
* Amethyst CLI
  * Expose Cargo exit status if non-zero (issue [#13])

[#13]: https://github.com/amethyst/tools/issues/13
[#17]: https://github.com/amethyst/tools/issues/17

## 0.2.4 (2016-02-11)

### Fixed
* Amethyst CLI
  * Properly print errors and warnings from Cargo (issue [#7])
  * Overhaul `new` command and stomp out bugs (issue [#8], [#9], [#10])

[#7]: https://github.com/amethyst/tools/issues/7
[#8]: https://github.com/amethyst/tools/issues/8
[#9]: https://github.com/amethyst/tools/issues/9
[#10]: https://github.com/amethyst/tools/issues/10

## 0.2.0 (2016-01-27)

### Added
* General
  * New repo-wide README.md
* Amethyst CLI
  * New project template, updated to use 0.2.0 engine API

### Changed
* Renamed repository to `amethyst_tools`, general restructuring (issue [#4])
* New change log format (issue [#5])

[#4]: https://github.com/amethyst/tools/issues/4
[#5]: https://github.com/amethyst/tools/issues/5

## 0.1.4 (2016-01-13)

### Fixed
* Amethyst CLI
  * Display Cargo stdout in real-time, not when the process exits (issue [#1])

[#1]: https://github.com/amethyst/tools/issues/1

## 0.1.3 (2016-01-12)

### Changed
* Amethyst CLI
  * Do not print to stdout when extracting the template project files
  * Eliminate all unused variable warnings

## 0.1.2 (2016-01-11)

### Changed
* Amethyst CLI
  * Update template project's `main.rs` to eliminate unused variable warning

## 0.1.1 (2016-01-10)

### Changed
* Amethyst CLI
  * Update included `main.rs` in the template project to API version 0.1.3
  * Remove unused zip-rs dependency "bzip2"

## 0.1.0 (2016-01-07)

* Initial release
