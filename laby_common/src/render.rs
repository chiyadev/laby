//
// Copyright (c) 2021 chiya.dev
//
// Use of this source code is governed by the MIT License
// which can be found in the LICENSE file and at:
//
//   https://opensource.org/licenses/MIT
//
use crate::internal::{escape, Buffer};
use alloc::{
    borrow::{Cow, ToOwned},
    string::String,
};
use core::{
    fmt::{Arguments, Write},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
};

/// Formats a value into an HTML representation.
///
/// # Example
///
/// The following example shows how this trait can be implemented. This is a low-level trait, so
/// any data written to the output buffer is [not escaped](escape) by default.
///
/// ```
/// # use laby_common as laby;
/// use laby::Render;
/// use laby::internal::{Buffer, escape};
///
/// struct Hello<'a> {
///     name: &'a str
/// }
///
/// impl Render for Hello<'_> {
///     #[inline]
///     fn render(self, buffer: &mut Buffer) {
///         buffer.push_str("<div>");
///         buffer.push_str("hello, ");
///         escape(self.name, buffer); // ensure string is escaped
///         buffer.push_str("</div>");
///     }
/// }
///
/// let x = Hello {
///     name: "laby"
/// };
///
/// let mut buffer = Buffer::new();
/// x.render(&mut buffer);
/// assert_eq!(buffer.into_string(), "<div>hello, laby</div>");
/// ```
pub trait Render {
    /// Formats this value into the given output buffer, consuming itself.
    fn render(self, buffer: &mut Buffer);
}

impl Render for () {
    #[inline]
    fn render(self, _: &mut Buffer) {}
}

impl Render for char {
    #[inline]
    fn render(self, buffer: &mut Buffer) {
        match self {
            '"' => buffer.push_str("&quot;"),
            '&' => buffer.push_str("&amp;"),
            '\'' => buffer.push_str("&#39;"),
            '<' => buffer.push_str("&lt;"),
            '>' => buffer.push_str("&gt;"),
            v => buffer.push(v),
        }
    }
}

impl Render for bool {
    #[inline]
    fn render(self, buffer: &mut Buffer) {
        buffer.push_str(if self { "true" } else { "false" });
    }
}

macro_rules! impl_str {
    ($type:ty) => {
        impl Render for $type {
            #[inline]
            fn render(self, buffer: &mut Buffer) {
                escape(self.as_ref(), buffer);
            }
        }
    };
}

impl_str!(&str);
impl_str!(String);

macro_rules! impl_int {
    ($type:ty) => {
        impl Render for $type {
            // fast integer rendering function from sailfish, using itoap.
            // https://github.com/Kogia-sima/sailfish/blob/6ea0ae2fad1d961b9495b3d50d1d0e1b0b30a219/sailfish/src/runtime/render.rs#L195
            #[inline]
            fn render(self, buffer: &mut Buffer) {
                use itoap::Integer;

                // SAFETY: `MAX_LEN < 40` and then does not overflows `isize::MAX`.
                // Also `b.len()` should be always less than or equal to `isize::MAX`.
                unsafe {
                    buffer.reserve_small(Self::MAX_LEN);
                    let ptr = buffer.as_mut_ptr().add(buffer.len());

                    // SAFETY: `MAX_LEN` is always greater than zero, so
                    // `b.as_mut_ptr()` always point to valid block of memory
                    let l = itoap::write_to_ptr(ptr, self);
                    buffer.advance(l);
                }
            }
        }
    };
}

impl_int!(u8);
impl_int!(u16);
impl_int!(u32);
impl_int!(u64);
impl_int!(u128);
impl_int!(usize);

impl_int!(i8);
impl_int!(i16);
impl_int!(i32);
impl_int!(i64);
impl_int!(i128);
impl_int!(isize);

macro_rules! impl_nonzero_int {
    ($type:ty) => {
        impl Render for $type {
            #[inline]
            fn render(self, buffer: &mut Buffer) {
                self.get().render(buffer);
            }
        }
    };
}

impl_nonzero_int!(NonZeroU8);
impl_nonzero_int!(NonZeroU16);
impl_nonzero_int!(NonZeroU32);
impl_nonzero_int!(NonZeroU64);
impl_nonzero_int!(NonZeroU128);
impl_nonzero_int!(NonZeroUsize);

impl_nonzero_int!(NonZeroI8);
impl_nonzero_int!(NonZeroI16);
impl_nonzero_int!(NonZeroI32);
impl_nonzero_int!(NonZeroI64);
impl_nonzero_int!(NonZeroI128);
impl_nonzero_int!(NonZeroIsize);

macro_rules! impl_float {
    ($type:ty, $min:literal, $fn:ident) => {
        impl Render for $type {
            // fast float rendering function from sailfish, using ryu.
            // https://github.com/Kogia-sima/sailfish/blob/6ea0ae2fad1d961b9495b3d50d1d0e1b0b30a219/sailfish/src/runtime/render.rs#L230
            #[inline]
            fn render(self, buffer: &mut Buffer) {
                use ryu::raw::$fn;

                if self.is_finite() {
                    unsafe {
                        buffer.reserve_small($min);
                        let ptr = buffer.as_mut_ptr().add(buffer.len());
                        let l = $fn(self, ptr);
                        buffer.advance(l);
                    }
                } else if self.is_nan() {
                    buffer.push_str("NaN");
                } else if self > 0.0 {
                    buffer.push_str("inf");
                } else {
                    buffer.push_str("-inf");
                }
            }
        }
    };
}

impl_float!(f32, 16, format32);
impl_float!(f64, 24, format64);

impl<R> Render for Option<R>
where
    R: Render,
{
    #[inline]
    fn render(self, buffer: &mut Buffer) {
        if let Some(value) = self {
            value.render(buffer);
        }
    }
}

impl<'a, R> Render for Cow<'a, R>
where
    R: ToOwned,
    &'a R: Render,
    R::Owned: Render,
{
    #[inline]
    fn render(self, buffer: &mut Buffer) {
        match self {
            Cow::Owned(value) => value.render(buffer),
            Cow::Borrowed(value) => value.render(buffer),
        }
    }
}

impl<'a> Render for Arguments<'a> {
    fn render(self, buffer: &mut Buffer) {
        struct EscapingBufferWriter<'a>(&'a mut Buffer);

        impl<'a> Write for EscapingBufferWriter<'a> {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                Ok(s.render(self.0))
            }

            fn write_char(&mut self, c: char) -> core::fmt::Result {
                Ok(c.render(self.0))
            }
        }

        core::fmt::write(&mut EscapingBufferWriter(buffer), self).unwrap();
    }
}
