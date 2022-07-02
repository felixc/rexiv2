rexiv2
======

[![build-badge][]][build] &nbsp;
[![downloads-badge][]][crates-io] &nbsp;
[![version-badge][]][crates-io] &nbsp;
[![license-badge][]][license] &nbsp;

[build]: https://travis-ci.org/felixc/rexiv2
[build-badge]: https://img.shields.io/travis/felixc/rexiv2.svg
[crates-io]: https://crates.io/crates/rexiv2
[downloads-badge]: https://img.shields.io/crates/d/rexiv2.svg
[version-badge]: https://img.shields.io/crates/v/rexiv2.svg
[license]: https://github.com/felixc/rexiv2/blob/master/LICENSE
[license-badge]: https://img.shields.io/crates/l/rexiv2.svg


Rust library for working with media file metadata
-------------------------------------------------

This library provides a Rust wrapper around the [gexiv2][gexiv2] library,
which is a GObject-based wrapper around the [Exiv2][exiv2] library, which
provides read and write access to the Exif, XMP, and IPTC metadata in media
files (typically photos) in various formats.

[gexiv2]: https://wiki.gnome.org/Projects/gexiv2
[exiv2]:  http://www.exiv2.org/


Documentation
-------------

API documentation is [available online][rexiv2-doc].

Exiv2’s homepage has documentation on available [namespaces and tags][tags-doc].

[gexiv2’s APIs][gexiv2-api] may also be a useful reference, along with [Exiv2’s
API docs][exiv2-api].

During development and testing, the [Exiv2 command-line utility][exiv2-cli] may
come in handy.

[rexiv2-doc]: https://felixcrux.com/files/doc/rexiv2/
[tags-doc]:   http://exiv2.org/metadata.html
[gexiv2-api]: https://git.gnome.org/browse/gexiv2/tree/gexiv2/gexiv2-metadata.h
[exiv2-api]:  http://exiv2.org/doc/index.html
[exiv2-cli]:  http://exiv2.org/manpage.html


Setup & Dependencies
--------------------

rexiv2 requires Rust 1.31 or newer, and uses the 2018 edition of the language.

Being a wrapper for gexiv2 and Exiv2, rexiv2 obviously depends on them. These
libraries are not bundled with rexiv2: you will need to install them separately.

gexiv2 is supported from version 0.10 onwards, and Exiv2 from version 0.23.

For full instructions on how to get started with rexiv2, including how to
install the prerequisite dependencies, refer to the [`SETUP`](SETUP.md) file.

Note that if you want BMFF support (e.g. HEIC, HEIF, AVIF, CR3, JXL/bmff files)
you will need an up to date version of the underlying libraries (gexiv2 v0.13.0 and Exiv2 v0.27.4).
You will also need to ensure that your version of Exiv2 has BMFF support enabled.
This is generally enabled by default, but may be switched off in certain distributions
due to licensing issues.

Versioning & History
--------------------

rexiv2 is currently available as a pre-1.0 development version.

Version numbers follow the principles of [Semantic Versioning][semver].

No further breaking API changes are planned, but they are possible as a result
of feedback on the API as more users try it out. Such feedback is welcome, and
having the API tried out in real applications is part of ensuring it’s ready for
a 1.0 release.

See the [`CHANGELOG`](CHANGELOG.md) file for a history of released versions.

[semver]: http://semver.org/spec/v2.0.0.html


Optional Features
-----------------

**raw-tag-access**: If you need access to the raw byte values of tags, you can
enable this feature and gain the `get_tag_raw` function.

This feature is disabled by default because it introduces a new dependency on
[`glib-sys`][glib-sys], and consequently on the GLib system library.

[glib-sys]: https://crates.io/crates/glib-sys/


Contributions & Bug Reports
---------------------------

Contributions are gladly accepted, either through GitHub pull requests or by
mailing patches to `felixc@felixcrux.com` (PGP key [8569B6311EE485F8][pgp-key]).

**By contributing, you are agreeing to make your contribution available under
the same license terms as the rest of the project.**

Bug reports and feature requests can also be sent through GitHub Issues or by
email, and are very welcome and appreciated.

For more information, see the [`CONTRIBUTING`](CONTRIBUTING.md) file.

[pgp-key]: http://hkps.pool.sks-keyservers.net/pks/lookup?op=vindex&search=0x8569B6311EE485F8


Copyright & License
-------------------

The Exiv2 and gexiv2 libraries are both released under the terms of the GNU
General Public License (GPL), and since rexiv2 is linked to them, it too is
made available under the terms of the GPL. Specifically:

This program is free software: you can redistribute it and/or modify it
under the terms of the GNU General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option)
any later version.

This program is distributed in the hope that it will be useful, but WITHOUT
ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program. If not, see <http://www.gnu.org/licenses/>.

Please refer to the [`LICENSE`](LICENSE) file for a full copy of the license.
