//
// Copyright (c) 2021 chiya.dev
//
// Use of this source code is governed by the MIT License
// which can be found in the LICENSE file and at:
//
//   https://opensource.org/licenses/MIT
//
//! Provides types used by laby internally.
//!
//! Types provided in this module may be useful when implementing the
//! [`Render`](crate::render::Render) trait for a custom object.
mod buffer;
mod escape;

pub use buffer::*;
pub use escape::*;
