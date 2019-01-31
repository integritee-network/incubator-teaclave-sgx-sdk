// Copyright (C) 2017-2018 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! # The Rust SGX SDK Standard Library
//!
//! The Rust SGX standard library (previously named as `sgx_tstdc`) is
//! the foundation of portable Rust SGX SDK, a
//! set of minimal and battle-tested shared abstractions for the Rust SGX
//! ecosystem. Similar to Rust's libstd, it offers core types, like [`Vec<T>`] and
//! [`Option<T>`], library-defined [operations on language
//! primitives](#primitives), [standard macros](#macros), [I/O] and
//! [multithreading], among [many other things][other].
//!
//! `std` is available to all Rust crates by default, just as if each one
//! contained an `extern crate sgx_tstd as std;` import at the [crate root]. Therefore the
//! standard library can be accessed in [`use`] statements through the path
//! `std`, as in [`use std::env`], or in expressions through the absolute path
//! `::std`, as in [`::std::env::args`].

#![no_std]
#![needs_panic_runtime]
#![allow(non_camel_case_types)]
#![allow(unused_must_use)]
#![allow(unused_features)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(deprecated)]

#![feature(alloc)]
#![feature(alloc_error_handler)]
#![feature(allocator_api)]
#![feature(allocator_internals)]
#![feature(alloc_layout_extra)]
#![feature(allow_internal_unsafe)]
#![feature(allow_internal_unstable)]
#![feature(align_offset)]
#![feature(arbitrary_self_types)]
#![feature(array_error_internals)]
#![feature(box_syntax)]
#![feature(c_variadic)]
#![feature(cfg_target_has_atomic)]
#![feature(char_error_internals)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(const_int_ops)]
#![feature(const_ip)]
#![feature(const_raw_ptr_deref)]
#![feature(const_cstr_unchecked)]
#![feature(core_intrinsics)]
#![feature(dropck_eyepatch)]
#![feature(duration_as_u128)]
#![feature(fixed_size_array)]
#![feature(fn_traits)]
#![feature(fnbox)]
#![feature(futures_api)]
#![feature(generator_trait)]
#![feature(hashmap_internals)]
#![feature(int_error_internals)]
#![feature(integer_atomics)]
#![feature(lang_items)]
#![feature(needs_panic_runtime)]
#![feature(never_type)]
#![feature(nll)]
#![feature(optin_builtin_traits)]
#![feature(pin)]
#![feature(prelude_import)]
#![feature(ptr_internals)]
#![feature(raw)]
#![feature(rustc_attrs)]
#![feature(rustc_const_unstable)]
#![feature(shrink_to)]
#![feature(slice_concat_ext)]
#![feature(str_internals)]
#![feature(thread_local)]
#![feature(toowned_clone_into)]
#![feature(try_from)]
#![feature(try_reserve)]
#![feature(unboxed_closures)]
#![feature(untagged_unions)]
#![feature(unwind_attributes)]
#![feature(slice_patterns)]
#![feature(panic_internals)]
#![cfg_attr(feature = "backtrace", feature(panic_unwind))]
#![feature(libc)]
#![feature(std_internals)]
#![feature(panic_info_message)]
#![feature(unicode_internals)]
#![feature(non_exhaustive)]
#![default_lib_allocator]

#[global_allocator]
static ALLOC: sgx_alloc::System = sgx_alloc::System;

// Explicitly import the prelude. The compiler uses this same unstable attribute
// to import the prelude implicitly when building crates that depend on std.
#[prelude_import]
#[allow(unused)]
use prelude::v1::*;

// Re-export a few macros from core
pub use core::{assert_eq, assert_ne, debug_assert, debug_assert_eq, debug_assert_ne};
pub use core::{unreachable, unimplemented, write, writeln, try};

#[allow(unused_imports)] // macros from `alloc` are not used on all platforms
#[macro_use]
extern crate alloc as alloc_crate;

// We always need an unwinder currently for backtraces
#[cfg(feature = "backtrace")]
extern crate sgx_unwind;

extern crate sgx_alloc;
#[macro_use]
extern crate sgx_types;
pub use sgx_types::{cfg_if, __cfg_if_items, __cfg_if_apply};

#[macro_use]
extern crate sgx_trts;
pub use sgx_trts::{global_ctors_object, global_dtors_object};

extern crate sgx_tprotected_fs;

// The standard macros that are not built-in to the compiler.
#[macro_use]
mod macros;

// The Rust prelude
pub mod prelude;

// Public module declarations and reexports

pub use core::any;
pub use core::cell;
pub use core::clone;
pub use core::cmp;
pub use core::convert;
pub use core::default;
pub use core::hash;
pub use core::intrinsics;
pub use core::iter;
pub use core::marker;
pub use core::mem;
pub use core::ops;
pub use core::ptr;
pub use core::raw;
pub use core::result;
pub use core::option;
pub use core::isize;
pub use core::i8;
pub use core::i16;
pub use core::i32;
pub use core::i64;
pub use core::i128;
pub use core::usize;
pub use core::u8;
pub use core::u16;
pub use core::u32;
pub use core::u64;
pub use core::u128;
pub use core::char;
pub use core::hint;
pub use core::pin;
pub use alloc_crate::boxed;
pub use alloc_crate::rc;
pub use alloc_crate::borrow;
pub use alloc_crate::fmt;
pub use alloc_crate::format;
pub use alloc_crate::slice;
pub use alloc_crate::str;
pub use alloc_crate::string;
pub use alloc_crate::vec;

pub mod f32;
pub mod f64;

#[macro_use]
pub mod thread;
pub mod ascii;
pub mod collections;
pub mod env;
pub mod error;
pub mod ffi;
pub mod sgxfs;
#[cfg(feature = "untrusted_fs")]
pub mod fs;
pub mod io;
pub mod net;
pub mod num;
pub mod os;
pub mod panic;
pub mod path;
pub mod sync;
pub mod time;
pub mod enclave;
pub mod untrusted;

pub mod task {
    //! Types and Traits for working with asynchronous tasks.
    #[doc(inline)]
    pub use core::task::*;
    #[doc(inline)]
    pub use alloc_crate::task::*;
}

pub mod future;

// Platform-abstraction modules
#[macro_use]
mod sys_common;
mod sys;

pub mod alloc;

// Private support modules
mod panicking;
mod cpuid;
mod memchr;
#[cfg(not(feature = "untrusted_fs"))]
mod fs;

// The runtime entry point and a few unstable public functions used by the
// compiler
pub mod rt;

#[cfg(feature = "backtrace")]
pub mod backtrace;

pub use cpuid::*;
pub use self::thread::{rsgx_thread_self, rsgx_thread_equal};

#[cfg(debug_assertions)]
pub mod debug;


