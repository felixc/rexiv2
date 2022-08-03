<!--
SPDX-FileCopyrightText: 2015–2022 Felix A. Crux <felixc@felixcrux.com> and CONTRIBUTORS
SPDX-License-Identifier: GPL-3.0-or-later
-->

Contributing
============

Contributions are gladly accepted, either through GitHub pull requests or by
mailing patches to `felixc@felixcrux.com` (PGP key [8569B6311EE485F8][pgp-key]).

**By contributing, you are agreeing to make your contribution available under
the same license terms as the rest of the project.**

Contributions of feature code, test code, documentation, bug reports, etc. are
all appreciated, and I’d be happy to help guide you through the process if you
have any questions.

[pgp-key]: http://hkps.pool.sks-keyservers.net/pks/lookup?op=vindex&search=0x8569B6311EE485F8


Bug Reports & Feature Requests
------------------------------

Outstanding work for the project is tracked with GitHub’s “Issues” feature. You
can see the current list of [open issues here][issues]. This covers all of bugs,
feature requests, documentation improvements, and any other kind of enhancement.

If you encounter any problems with the software, or would like it to be improved
in some way, please feel free to file a new GitHub Issue or send a report by
email to `felixc@felixcrux.com`. The more detail you can provide, the better.

If you’d like to contribute to the project, the [issue tracker][issues] is also
the place to start. Please indicate your interest in addressing the issue by
leaving a comment, potentially describing at a high level what you intend to do.
This helps avoid duplication of work when two people silently work on the same
problem, and also needless churn that can arise if the implementation you submit
is surprising to the parties interested in the issue and hasn’t been explained
in advance. **If there is no issue already open for the work you’d like to do,
please create one.**

### Issue Labels
Issues in the GitHub tracker are categorized with “labels”. These mostly
describe what kind of work the issue covers (e.g. there are labels for
“infrastructure”, “documentation”, “feature”, “bug”…).

Of particular note, however, is the “[good-first-bug][g-f-b]” label. Issues
tagged in this way are believed to be especially suitable for new contributors
(whether to the project, or to Rust code, or to Free Software/Open Source in
general). Anyone is welcome to work on any issue, but if you’re unsure about how
to get going, that may be the place to start.

[issues]: https://github.com/felixc/rexiv2/issues
[g-f-b]: https://github.com/felixc/rexiv2/issues?q=is%3Aissue+is%3Aopen+label%3Agood-first-bug


Setting Up Your Development Environment
---------------------------------------

To get started on making a code contribution, you’ll first need to set up your
development environment. For instructions, consult the [`SETUP`](SETUP.md) file.

### Advanced: Non-Stable Rust
The code for this project is primarily meant to be used with the current Stable
version of Rust. However, we do want to try to keep it functional under Beta,
Nightly, and some older releases — on a best-effort basis.

If you encounter issues with one of these versions, or would like to test to
ensure your changes are compatible with them, you’ll need a way to easily switch
between Rust releases. The best tool for this is [rustup][rustup].

After installation, you can switch versions by running commands like `rustup
update beta && rustup default beta` (substituting the version you want in place
of `beta`). Remember to switch back after you’re done!

[rustup]: https://rustup.rs/



Copyright & Licensing of Contributions
--------------------------------------

The copyright to any contribution to this software project is retained by the
original author of the contribution. However, by contributing, you are agreeing
to make your contribution available as part of this project under the terms of
the GNU General Public License (version 3 of the License, or any later version).

Please refer to the [`LICENSE`](LICENSE) file for a full copy of the license.
