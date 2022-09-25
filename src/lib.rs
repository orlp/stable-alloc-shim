#![no_std]
#![allow(unused_unsafe)]

//! A simple crate that provides stable copies of the unstable allocator APIs
//! found in `alloc`, for the purpose of implementing collections targetting
//! stable Rust.
//!
//! ```toml
//! [dependencies]
//! stable-alloc-shim = "0.57"
//! ```
//!
//! The minimum required Rust version for this crate is Rust 1.50.
//!
//! This crate does not do its own versioning and instead follows the standard
//! library. As an example, v0.57.x of this crate will have the definitions and
//! implementations of the allocator API as they were in Rust version 1.57. The
//! x will be reserved for fixing errors, and applying the
//! [semver trick](https://github.com/dtolnay/semver-trick) for future
//! compatibility.
//!
//! When nightly features get changed in future releases, this crate will update
//! their definitions in a new version. If a feature gets stabilized, it is
//! similarly changed to a re-export from the standard library, if a
//! sufficiently high rustc version is detected (as to not unnecessarily bump
//! the minimum required Rust version). Either way the semver trick is used for
//! unchanged definitions in the older version to keep versions as compatible as
//! possible.

extern crate alloc as std_alloc;

mod alloc_alloc; // alloc/alloc.rs
mod core_alloc; // core/alloc/mod.rs

pub mod alloc {
    pub use crate::alloc_alloc::Global;
    pub use crate::core_alloc::{AllocError, Allocator};
}

pub mod collections;

use core::hint::unreachable_unchecked;
use core::mem::MaybeUninit;
use core::ptr::NonNull;

use std_alloc::alloc::Layout;

// Private stable shims.
fn nonnull_as_mut_ptr<T>(ptr: NonNull<[T]>) -> *mut T {
    ptr.as_ptr() as *mut T
}

unsafe fn nonnull_len<T>(ptr: NonNull<[T]>) -> usize {
    let uninit_ptr = core::mem::transmute::<NonNull<[T]>, NonNull<[MaybeUninit<T>]>>(ptr);
    (*uninit_ptr.as_ptr()).len()
}

fn nonnull_slice_from_raw_parts<T>(data: NonNull<T>, len: usize) -> NonNull<[T]> {
    // SAFETY: `data` is a `NonNull` pointer which is necessarily non-null
    unsafe { NonNull::new_unchecked(core::ptr::slice_from_raw_parts_mut(data.as_ptr(), len)) }
}

fn invalid_mut<T>(addr: usize) -> *mut T {
    unsafe { core::mem::transmute(addr) }
}

fn layout_dangling(slf: &Layout) -> NonNull<u8> {
    unsafe { NonNull::new_unchecked(invalid_mut(slf.align())) }
}

unsafe fn assume(b: bool) {
    if !b {
        unreachable_unchecked()
    }
}
