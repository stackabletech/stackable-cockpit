# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [24.7.0] - 2024-07-23

### Changed

- helm-sys: Bump Go dependencies ([#307]).
- Bump Rust dependencies ([#307]).

### Fixed

- helm-sys: Double the helm timeout to 20m ([#306]).

[#306]: https://github.com/stackabletech/stackable-cockpit/pull/306
[#307]: https://github.com/stackabletech/stackable-cockpit/pull/307

## [24.3.6] - 2024-06-24

### Fixed

- Remove error message trimming in error report ([#303]).

[#303]: https://github.com/stackabletech/stackable-cockpit/pull/303

## [24.3.5] - 2024-06-17

### Fixed

- Remove colons from error messages, because the snafu report removes all
  content after the colon which results in loss of detail ([#300]).

[#300]: https://github.com/stackabletech/stackable-cockpit/pull/300

## [24.3.4] - 2024-05-28

### Fixed

- Avoid unnecessary `k8s::Client` creations ([#295]).
- Re-run GVK discovery after resolution failure ([#294]).

[#294]: https://github.com/stackabletech/stackable-cockpit/pull/294
[#295]: https://github.com/stackabletech/stackable-cockpit/pull/295

## [24.3.3] - 2024-05-13

- Bump Rust, Go and Node dependencies ([#238]).

[#238]: https://github.com/stackabletech/stackable-cockpit/pull/238

## [24.3.2] - 2024-04-25

### Added

- Add pre-built binary for `aarch64-unknown-linux-gnu` ([#232]).

### Changed

- Bump Rust dependencies ([#233]).

[#232]: https://github.com/stackabletech/stackable-cockpit/pull/232
[#233]: https://github.com/stackabletech/stackable-cockpit/pull/233

## [24.3.1] - 2024-03-21

### Added

- Added experimental `debug` command ([#204]).

[#204]: https://github.com/stackabletech/stackable-cockpit/pull/204

## [24.3.0] - 2024-03-20

### Added

- Support listing endpoints of Listeners in `stackablectl stacklet list` command.
  Currently only HDFS is using listener-op, so we can only test that so far ([#213], [#219]).

### Changed

- Operators are now installed in parallel when installing a release ([#202]).

### Fixed

- Fix `--cluster-name` not taking effect. The local test clusters always used the default cluster name ([#181]).

[#181]: https://github.com/stackabletech/stackable-cockpit/pull/181
[#202]: https://github.com/stackabletech/stackable-cockpit/pull/202
[#213]: https://github.com/stackabletech/stackable-cockpit/pull/213

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
