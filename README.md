# stable-alloc-shim

A simple crate that provides stable copies of the allocator APIs found in
`alloc`, for the purpose of implementing collections targetting stable Rust.

```toml
[dependencies]
stable-alloc-shim = "0.57"
```

The minimum required Rust version of this crate is Rust 1.50.

This crate does not do its own versioning and instead follows the standard
library. As an example, v0.57.x of this crate will have the definitions and
implementations of the allocator API as they were in Rust version 1.57. The
x will be reserved for fixing errors, and applying the
[semver trick](https://github.com/dtolnay/semver-trick) for future
compatibility.

When nightly features get changed in future releases, this crate will update
their definitions in a new version. If a feature gets stabilized, it is
similarly changed to a re-export from the standard library, if a
sufficiently high rustc version is detected (as to not unnecessarily bump
the minimum required Rust version). Either way the semver trick is used for
unchanged definitions in the older version to keep versions as compatible as
possible.
