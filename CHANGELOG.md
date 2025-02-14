# Changelog

## [0.4.0]

### Added
- `write_raw` function to register type to allow writing directly to register using primitive type without having to create a RegValue type.

### Changed
- Refactored how register, register array, cluster and cluster array are represented. Now reference to Zero Size Type (ZST) is used. For further details refer to issue #48 

### Fixed
- None

## [0.3.1]