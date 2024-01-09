# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Fixed

- Fix `--cluster-name` not taking effect. The local test clusters always used the default cluster name ([#181]).

[#181]: https://github.com/stackabletech/stackable-cockpit/pull/181

## [23.11.3] - 2024-01-03

### Fixed

- Fix `stackablectl release uninstall` command. It now deletes the operators included in the selected release correctly
  ([#174]).

[#174]: https://github.com/stackabletech/stackable-cockpit/pull/174

### CI

- Fix GitHub workflow syntax ([#175]).

[#175]: https://github.com/stackabletech/stackable-cockpit/pull/175

## [23.11.2] - 2024-01-02

### Changed

- Bump Rust version from `1.74.0` to `1.75.0` ([#172]).
- Bump Rust and Go dependencies ([#135], [#162], [#167], [#168], [#170]).
- Rename old output style `plain` to `table`. The new output option `plain` will output a reduced view (which removes
  borders from tables for example) ([#142], [#163]).

[#135]: https://github.com/stackabletech/stackable-cockpit/pull/135
[#142]: https://github.com/stackabletech/stackable-cockpit/issues/142
[#162]: https://github.com/stackabletech/stackable-cockpit/pull/162
[#163]: https://github.com/stackabletech/stackable-cockpit/pull/163
[#167]: https://github.com/stackabletech/stackable-cockpit/pull/167
[#168]: https://github.com/stackabletech/stackable-cockpit/pull/168
[#170]: https://github.com/stackabletech/stackable-cockpit/pull/170
[#172]: https://github.com/stackabletech/stackable-cockpit/pull/172

## [23.11.1] - 2023-12-06

### Fixed

- Fix Helm repo selection mechanism ([#156]).

[#156]: https://github.com/stackabletech/stackable-cockpit/pull/156

## [23.11.0] - 2023-11-30

First official release of the `stackablectl` rewrite.

### Changed

- Bump Rust version from `1.73.0` to `1.74.0` ([#151]).

[#151]: https://github.com/stackabletech/stackable-cockpit/pull/151
