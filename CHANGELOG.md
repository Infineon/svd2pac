# Changelog

## [0.5.0]

### Changed

- previously the `usage` tag was ignored in `enumeratedValues` tag and all label were merged. In this release different types are generated based on `usage` value. The enumerate value type is postfixed with `_Read` and `_Write` if `usage` is `read` or `write`. No postfix if `usage` is absent or `read-write`
- panic if an `enumeratedValue` has attribute `derivedFrom`

## [0.4.0]

### Added
- `write_raw` function to register type to allow writing directly to register using primitive type without having to create a RegValue type.

### Changed
- Refactored how register, register array, cluster and cluster array are represented. Now reference to Zero Size Type (ZST) is used. For further details refer to issue #48 

### Fixed
- None

## [0.3.1]