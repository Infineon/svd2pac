# Changelog

## [Unreleased]

### Added

- New `--workspace-generation` experimental mode to generate a Cargo workspace with a shared `common` crate and one crate per peripheral.

### Changed

- Cluster accessor methods now take `&self` instead of consuming `self`, allowing chaining without moving the peripheral instance (e.g. `TIMER.cluster1().hssl().ch()[0].reg().read()`).
- Updated generated PAC register abstractions and type paths.
- Refactored register abstractions from phantom-type structs to trait-based APIs (`Reg`, `DataTypeT`) and added `ReadCore`/`WriteCore` support for Aurix CSFR access.
- Register value types are now generated within peripheral modules/crates instead of the shared `common` module.
- Refactored workspace generation internals into a dedicated module.
- Updated `anyhow` to address a soundness issue and refreshed regression test references.
- For generated Aurix PACs, the minimum supported Rust version is now 1.94 and the unstable `feature(stdsimd)` usage was removed.

### Breaking

- Register initialization: `SomeReg::default()` no longer works; use `peripheral.register().reset_value()` instead.
- Register value type paths changed: code importing value types from `common` must be updated to the corresponding peripheral module paths.

## [0.7.0]

### Improvements

- Register and Clusters implement trait AsPtr. This trait allow easy conversion from/to pointers

### Changed

- Upgraded Cargo.lock to fix GitHub dependabot about rand package
- Updated edition to 2024 and rust minimum version to 1.85

## [0.6.1]

### Fixed

- Fixed bug related to Aurix code (--target aurix). Usage of features tracing in code even if tracing is not enabled in code generation
- Fixed warning deteced by rust 1.88 version.
- Upgraded tera to avoid unmaintained unicode dependencies.

## [0.6.0]

### Fixed

- Template bug that block generation of all PAC documentation

### Changed

- Internal implementation: instead of panic and assertion return errors from function. Kept assertion that identify implementation bugs
- Upgraded thiserror to 2.0
- Generate IRQ handler extern ref in a module to avoid collisions with peripheral instances. Only applicable when generating for cortex-m devices.

## [0.5.0]

### Fixed

- Fixed all warning in generated crates

### Changed

- previously the `usage` tag was ignored in `enumeratedValues` tag and all label were merged. In this release different types are generated based on `usage` value. The enumerate value type is postfixed with `_Read` and `_Write` if `usage` is `read` or `write`. No postfix if `usage` is absent or `read-write`
- panic if an `enumeratedValue` has attribute `derivedFrom`
- generated pac has rust minimal required version 1.70 instead of 1.64

### Added

- Added tests that verify there are no clippy warnings in generated pac

## [0.4.0]

### Added

- `write_raw` function to register type to allow writing directly to register using primitive type without having to create a RegValue type.

### Changed

- Refactored how register, register array, cluster and cluster array are represented. Now reference to Zero Size Type (ZST) is used. For further details refer to issue #48

### Fixed

- None

## [0.3.1]
