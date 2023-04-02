//
// Copyright (c) 2021 chiya.dev
//
// Use of this source code is governed by the MIT License
// which can be found in the LICENSE file and at:
//
//   https://opensource.org/licenses/MIT
//
//! Contains code required by `laby` at both compile time and runtime.
//!
//! This crate mainly provides the [`Render`] trait that forms the basis of all HTML rendering
//! operations, the [`escape`](internal::escape) function that escapes strings for HTML inclusion,
//! and the [`Buffer`](internal::Buffer) type that provides the rendering output buffer.
//!
//! This crate is re-exported by crate `laby`. If you are using laby, you should not depend on this
//! crate directly.
#![no_std]
#![deny(missing_docs)]
extern crate alloc;

pub mod internal;
mod render;

pub use render::*;
