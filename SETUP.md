<!--
SPDX-FileCopyrightText: 2015–2022 Felix A. Crux <felixc@felixcrux.com> and CONTRIBUTORS
SPDX-License-Identifier: GPL-3.0-or-later
-->

Setup & Dependencies
====================

Dependencies
------------

rexiv2 requires Rust 1.4 or newer.

Given that it links to gexiv2, and transitively to Exiv2, rexiv2 obviously
depends on them (and on their dependencies). Having these libraries installed on
your system is a prerequisite to using rexiv2, or any software built on it.

gexiv2 is supported from version 0.10 onwards, and Exiv2 from version 0.23.

Platform-specific instructions for how to accomplish this are below:

### GNU/Linux

On a GNU/Linux system, you can typically install these dependencies through your
package manager, but be aware that they may be older versions without all the
features you’d like to use.

On Debian and its derivatives (like Ubuntu), run `sudo apt-get install
libgexiv2-dev`. On Arch, the package `libgexiv2` may have all you need. On
Fedora-like distros, try `libgexiv2-devel`. All of these should come with all
their dependencies as well.

If you need a newer version of a library than the one provided by your current
distribution version (e.g. because you are on Debian Stable, or Ubuntu LTS), you
may be able to build your own “backport” of the packages provided by later
distro releases.

For example, on Debian, add “`deb-src http://httpredir.debian.org/debian
unstable main`” to your `/etc/apt/sources.list` file, and run:

```shell
mkdir /tmp/gexiv2 && cd /tmp/gexiv2
apt-get update
sudo apt-get build-dep libgexiv2-dev libgexiv2-2
sudo apt-get -b source libgexiv2-dev libgexiv2-2
sudo dpkg -i *gexiv2*.deb
```

If you really need the latest upstream versions, you can always download them
directly from their project download pages and install them manually:
[Exiv2][exiv2-dl]; [gexiv2][gexiv2-dl].

### Mac OS X

The simplest known way of installing the required dependencies on Mac OS X is
with the Homebrew package manager, using the [gexiv2][gexiv2-brew] formula:

```shell
brew update
brew install gexiv2 pkg-config
```

For the build to succeed, you will have to tell `pkg-config` where Homebrew
installed some relevant libraries, using:

```shell
export PKG_CONFIG_PATH="/usr/local/opt/libffi/lib/pkgconfig"
```

It may also be possible to install dependencies via MacPorts, using the
[gexiv2][gexiv2-port] port, but I have not tested this. If you have more
information, please consider contributing your knowledge to this document.

Otherwise, you will likely have to download the dependencies directly from their
project download pages: [Exiv2][exiv2-dl]; [gexiv2][gexiv2-dl].

### Windows

I currently do not know what steps are needed to install these dependencies on
Windows. If you figure it out, please consider contributing your knowledge to
this document (see our sister-project gexiv2-sys’s
[GitHub Issue #11](https://github.com/felixc/gexiv2-sys/issues/11)).

You will likely have to download the dependencies directly from their project
download pages: [Exiv2][exiv2-dl]; [gexiv2][gexiv2-dl].

[exiv2-dl]: http://www.exiv2.org/download.html
[gexiv2-dl]: https://wiki.gnome.org/Projects/gexiv2/BuildingAndInstalling
[gexiv2-brew]: http://brewformulas.org/Gexiv2
[gexiv2-port]: https://trac.macports.org/browser/trunk/dports/gnome/gexiv2/Portfile


Using rexiv2 In Your Code
-------------------------

The best way to download and use rexiv2 in your own code is by depending on it
via Cargo, and fetching it from [crates.io][crates-rexiv2]. You can do this by
adding a dependency on rexiv2 in your crate’s `Cargo.toml` file:

```toml
[dependencies]
rexiv2 = "0.10"
```

Alternatively, if you’d like to work off of the bleeding edge (note that this is
not recommended unless you’re actively developing on rexiv2 itself), you can
depend directly on the Git repository using the line

```toml
rexiv2 = { git = "https://github.com/felixc/rexiv2" }
```

or on a local copy, using the `path` option:

```toml
rexiv2 = { path = "../rexiv2" }  # Or wherever your copy is located
```

Now you can import and use the functions defined in rexiv2 like this:

```rust
extern crate rexiv2;

fn main() {
    let meta = rexiv2::Metadata::new_from_path("example.jpg").unwrap();
    println!("{:?}", meta.get_media_type());
}
```

[crates-rexiv2]: https://crates.io/crates/rexiv2
