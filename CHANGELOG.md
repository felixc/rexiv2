<!--
SPDX-FileCopyrightText: 2015–2022 Felix A. Crux <felixc@felixcrux.com> and CONTRIBUTORS
SPDX-License-Identifier: CC0-1.0
-->

## [NEXT] - Unreleased
  * Require Rust 1.56 as the minimum supported language version.
  * Adopt 2021 edition of the language.
  * Dependency upgrades.

## [v0.9.1] - 2020-07-12
  * Fixed build failure on arm32 due to invalid assumptions about int size.
  * Fixed null pointer crash when using `get_tag_raw()`.

## [v0.9.0] - 2019-11-23
  * Added functionality to get and set log levels, thanks to GitHub user t1ra.

## [v0.8.0] - 2019-09-02
  * Added operations on metadata thumbnail images.
  * Added way to get raw byte values of metadata.
  * Added methods on preview images. All of these thanks to Jean-Baptiste Daval!
  * Require Rust 1.31 as the minimum supported version (and use 2018 edition).

## [v0.7.0] - 2018-11-25
  * Added `initialize()` method for safe multi-threaded use.
  * Dependency upgrades.

## [v0.6.0] - 2018-02-19
  * Require Rust 1.20 as the minimum supported version to match dependencies.
  * Fixed segfault bug in `get_tag_multiple_strings` when there are no results.
  * Updated gexiv2-sys internal dependency.

## [v0.5.0] - 2017-06-21
  * Require Rust 1.8 as the minimum supported version to match dependencies.

## [v0.4.3] - 2017-06-21
  * Pin version of num-traits to unbreak builds on older rustc versions.

## [v0.4.2] - 2017-04-11
  * Upgraded gexiv2-sys dependency to 0.7.
  * Replaced num dependency with num-rational for faster builds.

## [v0.4.1] - 2016-10-04
  * Fix for potential crash due to dereferencing null pointer.

## [v0.4.0] - 2016-07-21
  * Path operations now accept anything that implements AsRef<ffi::OsStr>,
    which enables support for path::Paths in addition to strs.
  * Breaking change: Image media types are now represented by an enum instead of
    magic strings. It is easy to convert between the two forms using ::from().
  * Breaking change: get/set_tag_long() are renamed get/set_tag_numeric().
  * Breaking change: get/set_exif_tag_rational() renamed get/set_tag_rational().
  * Breaking change: get/set_tag_long() now operate on i32 values, not i64.
  * Breaking change: Errors are now wrapped in a new rexiv2::Rexiv2Error type.
  * Breaking change: Results are now using a library-specific alias that fixes
    all Err() instances as Rexiv2Error.

## [v0.3.3] - 2016-03-30
  * Dependency cleanup: removed rustc-serialize & bumped gexiv2-sys.
  * Types implement more common useful traits.
  * Documentation improvements, including bundling setup instructions.

## [v0.3.2] - 2015-09-11
  * Dependency version bump (gexiv2-sys to 0.5 and libc to 0.2).

## [v0.3.1] - 2015-09-20
  * Fixed memory leak of some values returned over FFI boundary.

## [v0.3.0] - 2015-09-13
  * All instances of success/failure boolean return values are now Results.
  * Fixed critical bug that caused dangling pointers and mysterious errors.
  * Updated to use latest gexiv2-sys FFI declarations.

## [v0.2.3] - 2015-04-30
  * Library now builds with regular stable rustc.

## [v0.2.2] - 2015-04-03
  * Updated to work with 1.0.0-nightly (d17d6e7f1 2015-04-02) (Note: not Beta!).
  * More permissive and up-to-date dependency version requirements.

## [v0.2.1] - 2015-03-02
  * Added support for loading metadata from byte-array data buffers.
  * Split gexiv2 FFI declarations off into separate gexiv2-sys crate dependency.

## [v0.2.0] - 2015-03-01
  * The "get_tag_type" function now returns an item from an enum of data types.
  * Some methods that used to return magic numbers on error now return Options.
  * The "get_mime_type" method is renamed "get_media_type" for correctness.
  * Custom Rational type replaced by common num::rational::Ratio.

## [v0.1.0] - 2015-02-25
  * First development release.
  * Added ability to set multiple string values for a tag.
  * Fixed array terminator bug when getting list of Exif tags.

## [v0.1.0-pre] - 2015-02-21
  * First preview release to solicit code review and feedback.


[v0.7.0]: https://github.com/felixc/rexiv2/compare/v0.6.0...v0.7.0
[v0.6.0]: https://github.com/felixc/rexiv2/compare/v0.5.0...v0.6.0
[v0.5.0]: https://github.com/felixc/rexiv2/compare/v0.4.3...v0.5.0
[v0.4.3]: https://github.com/felixc/rexiv2/compare/v0.4.2...v0.4.3
[v0.4.2]: https://github.com/felixc/rexiv2/compare/v0.4.1...v0.4.2
[v0.4.1]: https://github.com/felixc/rexiv2/compare/v0.4.0...v0.4.1
[v0.4.0]: https://github.com/felixc/rexiv2/compare/v0.3.3...v0.4.0
[v0.3.3]: https://github.com/felixc/rexiv2/compare/v0.3.2...v0.3.3
[v0.3.2]: https://github.com/felixc/rexiv2/compare/v0.3.1...v0.3.2
[v0.3.1]: https://github.com/felixc/rexiv2/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/felixc/rexiv2/compare/v0.2.3...v0.3.0
[v0.2.3]: https://github.com/felixc/rexiv2/compare/v0.2.2...v0.2.3
[v0.2.2]: https://github.com/felixc/rexiv2/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/felixc/rexiv2/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/felixc/rexiv2/compare/v0.1.0...v0.2.0
[v0.1.0]: https://github.com/felixc/rexiv2/compare/25c31ad...v0.1.0
[v0.1.0-pre]: https://github.com/felixc/rexiv2/commit/25c31ad5a0bdbc51ce95e416f1931771fdfd950d
