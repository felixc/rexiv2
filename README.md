rexiv2
======

[![Build Status](https://img.shields.io/travis/felixc/rexiv2.svg)](https://travis-ci.org/felixc/rexiv2)&nbsp;
[![Crates.io Downloads](https://img.shields.io/crates/d/rexiv2.svg)](https://crates.io/crates/rexiv2)&nbsp;
[![Crates.io Version](https://img.shields.io/crates/v/rexiv2.svg)](https://crates.io/crates/rexiv2)&nbsp;
[![License](https://img.shields.io/crates/l/rexiv2.svg)](https://github.com/felixc/rexiv2/blob/master/LICENSE)&nbsp;


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


Dependencies
------------

Being a wrapper for Exiv2, and gexiv2, rexiv2 obviously depends on them. Only
the library (e.g. `.so` or `.dll`) files are needed; not the headers or source
code. You can download these dependencies from their download pages:
[Exiv2][exiv2-dl]; [gexiv2][gexiv2-dl].

On a Linux system, you can typically install these dependencies through your
package manager (look for packages with names like “libgexiv2-dev”). Mac OS X
users may also have this option through unofficial package management systems.
Note that to build rexiv2 from source you may need not just the library
packages, but the “dev” versions of them as well.

[exiv2-dl]:  http://www.exiv2.org/download.html
[gexiv2-dl]: https://wiki.gnome.org/Projects/gexiv2/BuildingAndInstalling


Versioning & History
--------------------

rexiv2 is currently only available in an unstable development version.

Version numbers follow the principles of [Semantic Versioning][semver]. In
particular, this means that once development reaches the 1.0.0 version, the
API will be considered stable, and any changes will be made gradually and
gracefully across multiple versions, with reasonable deprecation timelines.
But, until then, don’t rely on the API working the same way from minor version
to minor version.

See the [`CHANGELOG`](CHANGELOG) file for a history of released versions.

[semver]: http://semver.org/spec/v2.0.0.html


Contributing
------------

Contributions are gladly accepted, either through GitHub pull requests or by
mailing patches to `felixc@felixcrux.com` (PGP key [8569B6311EE485F8][pgp-key]).

It is strongly recommended that you indicate your intent to contribute by
opening an issue on the tracker (or commenting on an existing one) describing
the bug you are fixing or the improvement you intend to make. This helps prevent
duplication of efforts and allows for discussion of e.g. API specifics before
too much time is spent on implementation.

By contributing, you are agreeing to make your contribution available under the
same license terms as the rest of the project.

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
