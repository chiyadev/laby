//
// Copyright (c) 2021 chiya.dev
//
// Use of this source code is governed by the MIT License
// which can be found in the LICENSE file and at:
//
//   https://opensource.org/licenses/MIT
//
// This file contains a custom string buffer implementation from sailfish. References to std were
// replaced with alloc and core. Additional documentation were added. The original source code can
// be found at:
//
//   https://github.com/rust-sailfish/sailfish/blob/master/sailfish/src/runtime/buffer.rs
//
// ===============================================================================
//
// The MIT License (MIT)
// Copyright (c) 2020 Ryohei Machida
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE
// OR OTHER DEALINGS IN THE SOFTWARE.
//
use alloc::{
    alloc::{alloc, dealloc, handle_alloc_error, realloc, Layout},
    boxed::Box,
    string::String,
};
use core::{
    fmt,
    mem::{align_of, ManuallyDrop},
    ops::{Add, AddAssign},
    ptr,
};

/// Buffer for rendered contents.
///
/// This is a port of [sailfish][1]'s [`Buffer`][2] struct.
///
/// [1]: https://docs.rs/sailfish/
/// [2]: https://docs.rs/sailfish/latest/sailfish/runtime/struct.Buffer.html
///
/// # Example
///
/// ```
/// # use laby_common::internal::*;
/// let mut buffer = Buffer::new();
///
/// buffer.push_str("hello,");
/// buffer.push_str(" world!");
///
/// assert_eq!(buffer.into_string(), "hello, world!");
/// ```
pub struct Buffer {
    data: *mut u8,
    len: usize,
    capacity: usize,
}

impl Buffer {
    /// Create an empty buffer.
    #[inline]
    pub const fn new() -> Buffer {
        Self {
            data: align_of::<u8>() as *mut u8, // dangling pointer
            len: 0,
            capacity: 0,
        }
    }

    /// Create an empty buffer with the given capacity.
    #[inline]
    pub fn with_capacity(n: usize) -> Buffer {
        if n == 0 {
            Self::new()
        } else {
            Self {
                data: safe_alloc(n),
                len: 0,
                capacity: n,
            }
        }
    }

    /// Extracts a string slice containing the contents of the buffer.
    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe {
            let bytes = core::slice::from_raw_parts(self.data, self.len);
            core::str::from_utf8_unchecked(bytes)
        }
    }

    /// Returns an unsafe mutable pointer to the inner data.
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.data
    }

    /// Returns the length of this buffer in bytes.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the capacity of this buffer in bytes.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    #[inline]
    #[doc(hidden)]
    pub unsafe fn _set_len(&mut self, new_len: usize) {
        debug_assert!(new_len <= self.capacity);
        self.len = new_len;
    }

    /// Increase the length of buffer by `additional` bytes.
    ///
    /// # Safety
    ///
    /// - `additional` must be less than or equal to `capacity() - len()`.
    /// - The elements at `old_len..old_len + additional` must be initialized.
    #[inline]
    pub unsafe fn advance(&mut self, additional: usize) {
        self.len += additional;
    }

    /// Returns `true` if this buffer has a length of zero, and `false` otherwise.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Ensures that the capacity of this buffer is at least `additional` bytes larger than its
    /// length.
    ///
    /// # Panics
    ///
    /// Panics if `size` overflows `isize::MAX`.
    #[inline]
    pub fn reserve(&mut self, size: usize) {
        if size <= self.capacity - self.len {
            return;
        }

        self.reserve_internal(size);
    }

    /// Same as [`reserve`](Self::reserve) but does not guard against `size` overflowing
    /// `isize::MAX`.
    ///
    /// # Safety
    ///
    /// Undefined behavior may result if `size` overflows `isize::MAX`.
    #[inline]
    pub unsafe fn reserve_small(&mut self, size: usize) {
        debug_assert!(size <= core::isize::MAX as usize);
        if self.len + size <= self.capacity {
            return;
        }
        self.reserve_internal(size);
    }

    /// Truncates this buffer, removing all contents.
    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Converts this buffer into [`String`].
    ///
    /// This is a cheap operation that does not involve any copying or allocation.
    #[inline]
    pub fn into_string(self) -> String {
        debug_assert!(self.len <= self.capacity);
        let buf = ManuallyDrop::new(self);

        // SAFETY: This operations satisfy all requirements specified in
        // https://doc.rust-lang.org/std/string/struct.String.html#safety
        unsafe { String::from_raw_parts(buf.data, buf.len, buf.capacity) }
    }

    /// Appends the given string slice to the end of this buffer.
    #[inline]
    pub fn push_str(&mut self, data: &str) {
        let size = data.len();

        unsafe {
            // SAFETY: this operation won't overflow because slice cannot exceeds
            // isize::MAX bytes.
            // https://doc.rust-lang.org/reference/behavior-considered-undefined.html
            self.reserve_small(size);

            let p = self.data.add(self.len);
            core::ptr::copy_nonoverlapping(data.as_ptr(), p, size);
            self.len += size;
        }
        debug_assert!(self.len <= self.capacity);
    }

    /// Appends the given `char` to the end of this buffer.
    #[inline]
    pub fn push(&mut self, data: char) {
        // Question: Is it safe to pass uninitialized memory to `encode_utf8` function?
        unsafe {
            self.reserve_small(4);
            let bp = self.data.add(self.len) as *mut [u8; 4];
            let result = data.encode_utf8(&mut *bp);
            self.len += result.len();
        }
    }

    #[inline]
    fn reserve_internal(&mut self, size: usize) {
        debug_assert!(size <= core::isize::MAX as usize);

        let new_capacity = core::cmp::max(self.capacity * 2, self.capacity + size);
        debug_assert!(new_capacity > self.capacity);
        self.data = unsafe { safe_realloc(self.data, self.capacity, new_capacity) };
        self.capacity = new_capacity;

        debug_assert!(!self.data.is_null());
        debug_assert!(self.len <= self.capacity);
    }
}

#[inline(never)]
fn safe_alloc(capacity: usize) -> *mut u8 {
    assert!(capacity > 0);
    assert!(
        capacity <= core::isize::MAX as usize,
        "capacity is too large"
    );

    // SAFETY: capacity is non-zero, and always multiple of alignment (1).
    unsafe {
        let layout = Layout::from_size_align_unchecked(capacity, 1);
        let data = alloc(layout);
        if data.is_null() {
            handle_alloc_error(layout);
        }

        data
    }
}

/// # Safety
///
/// - if `capacity > 0`, `capacity` is the same value that was used to allocate the block of memory
/// pointed by `ptr`.
#[cold]
#[inline(never)]
unsafe fn safe_realloc(ptr: *mut u8, capacity: usize, new_capacity: usize) -> *mut u8 {
    assert!(new_capacity > 0);
    assert!(
        new_capacity <= core::isize::MAX as usize,
        "capacity is too large"
    );

    let data = if capacity == 0 {
        let new_layout = Layout::from_size_align_unchecked(new_capacity, 1);
        alloc(new_layout)
    } else {
        let old_layout = Layout::from_size_align_unchecked(capacity, 1);
        realloc(ptr, old_layout, new_capacity)
    };

    if data.is_null() {
        handle_alloc_error(Layout::from_size_align_unchecked(new_capacity, 1));
    }

    data
}

impl Clone for Buffer {
    fn clone(&self) -> Self {
        unsafe {
            if self.is_empty() {
                Self::new()
            } else {
                let buf = Self {
                    data: safe_alloc(self.len),
                    len: self.len,
                    capacity: self.len,
                };

                ptr::copy_nonoverlapping(self.data, buf.data, self.len);
                buf
            }
        }
    }
}

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        if self.capacity != 0 {
            // SAFETY: when `self.capacity > 0`, `self.capacity` is the same value used for
            // allocate the block of memory pointed by `self.data`.
            unsafe {
                let layout = Layout::from_size_align_unchecked(self.capacity, 1);
                dealloc(self.data, layout);
            }
        }
    }
}

impl fmt::Write for Buffer {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        Buffer::push_str(self, s);
        Ok(())
    }
}

impl From<String> for Buffer {
    /// Shrink the data and pass raw pointer directory to buffer
    ///
    /// This operation is `O(1)`
    #[inline]
    fn from(other: String) -> Buffer {
        let bs = other.into_boxed_str();
        let data = Box::leak(bs);
        Buffer {
            data: data.as_mut_ptr(),
            len: data.len(),
            capacity: data.len(),
        }
    }
}

impl From<&str> for Buffer {
    #[inline]
    fn from(other: &str) -> Buffer {
        let mut buf = Buffer::with_capacity(other.len());

        if !other.is_empty() {
            // SAFETY: `Buffer.capacity()` should be same as `other.len()`, so if `other` is not
            // empty, `buf.as_mut_ptr()` is supporsed to point to valid memory.
            unsafe {
                ptr::copy_nonoverlapping(other.as_ptr(), buf.as_mut_ptr(), other.len());
                buf.advance(other.len());
            }
        }

        buf
    }
}

impl Add<&str> for Buffer {
    type Output = Buffer;

    #[inline]
    fn add(mut self, other: &str) -> Buffer {
        self.push_str(other);
        self
    }
}

impl AddAssign<&str> for Buffer {
    #[inline]
    fn add_assign(&mut self, other: &str) {
        self.push_str(other)
    }
}

impl Default for Buffer {
    #[inline]
    fn default() -> Buffer {
        Buffer::new()
    }
}

unsafe impl Send for Buffer {}
unsafe impl Sync for Buffer {}
