//
// Copyright (c) 2021 chiya.dev
//
// Use of this source code is governed by the MIT License
// which can be found in the LICENSE file and at:
//
//   https://opensource.org/licenses/MIT
//
use alloc::string::ToString;
use core::fmt::Display;
use laby_common::{internal::Buffer, Render};

/// Indicates that an attribute is an HTML boolean attribute.
///
/// If a _normal_ attribute value evaluates to a boolean value `true` or `false`,
/// the string representation `"true"` or `"false"` is rendered. This macro indicates
/// that the attribute is an HTML _boolean_ attribute.
///
/// If a _boolean_ attribute value evalutes to `false`, it is not rendered at all. Conversely,
/// if it evaluates to `true`, only the attribute's name is rendered without a value component
/// (i.e. `="..."`).
///
/// This is a special macro which is recognized internally by the markup macro.
/// If called outside a markup macro, or called in an invalid position, compilation will fail.
///
/// This macro is necessary because it is not possible for laby to determine the type of
/// attribute value at compile time. This is a fundamental limitation of procedural macros that
/// must operate on syntax tokens before the compiler performs any type inference.
///
/// # Example
///
/// ```
/// # use laby::*;
/// let enabled = false;
/// let n = button!(disabled = bool!(!enabled), "laby");
///
/// assert_eq!(render!(n), "<button disabled>laby</button>");
/// ```
#[macro_export]
macro_rules! bool {
    ($($x:tt)*) => {
        panic!("invalid use of boolean macro")
    };
}

#[allow(unused_imports)] // for docs
use alloc::string::String;

/// Renders the given value and returns the result as a [`String`].
///
/// This is a convenience macro that simply constructs a new [`Buffer`], renders the given
/// expression into it, and returns the buffer as a [`String`].
///
/// The value must implement the [`Render`] trait.
///
/// When multiple values are given, they are wrapped using the [`frag!`][crate::frag] macro and
/// rendered sequentially without delimiters.
///
/// # Expansion
///
/// ```ignore
/// // render!($expr)
/// {
///     let mut buffer = Buffer::new();
///     $expr.render(&mut buffer);
///     buffer.into_string()
/// }
///
/// // render!($expr*)
/// {
///     render!(frag!($expr*))
/// }
/// ```
///
/// # Examples
///
/// You can render a simple node as a string.
///
/// ```
/// # use laby::*;
/// let s = render!(
///     div!("content")
/// );
///
/// assert_eq!(s, "<div>content</div>");
/// ```
///
/// This example constructs a more complex tree of nodes and renders it as a string.
///
/// ```
/// # use laby::*;
/// let s = render!(
///     html!(
///         head!(title!("laby")),
///         body!(p!("paragraph"))
///     )
/// );
///
/// assert_eq!(s, "<html><head><title>laby</title></head><body><p>paragraph</p></body></html>");
/// ```
///
/// The [`render!`] macro can render any value that implements the [`Render`] trait,
/// which is not limited only to nodes constructed by markup macros.  See the
/// [list of foreign impls](Render#foreign-impls) on the [`Render`] trait to see which
/// types are supported.
///
/// ```
/// # use laby::*;
/// let v: u32 = 100;
/// assert_eq!(render!(v), "100");
/// ```
///
/// All strings are escaped by default when rendered. This behavior can be opted out of by
/// using the [`raw!`] macro.
///
/// ```
/// # use laby::*;
/// let escaped = render!("a < b");
/// let raw = render!(raw!("a < b"));
///
/// assert_eq!(escaped, "a &lt; b");
/// assert_eq!(raw, "a < b");
/// ```
///
/// This example renders multiple nodes using the [`render!`] macro. It is equivalent to
/// passing a single argument using the [`frag!`][crate::frag] macro that wraps the values together.
///
/// ```
/// # use laby::*;
/// let n = render!(div!(), span!());
/// let m = render!(frag!(div!(), span!()));
///
/// assert_eq!(n, "<div></div><span></span>");
/// assert_eq!(n, m);
/// ```
#[macro_export]
macro_rules! render {
    ($expr:expr) => {{
        let mut buffer = $crate::internal::Buffer::with_capacity(16384);
        $crate::Render::render($expr, &mut buffer);
        buffer.into_string()
    }};

    ($($expr:expr),* $(,)?) => {{
        let frag = $crate::frag!($($expr),*);
        $crate::render!(frag)
    }};
}

/// Wraps an [`Iterator`], making it implement [`Render`].
///
/// This is a workaround for Rust's lack of impl specialization.
/// When Rust gets specialization, this type may no longer be necessary.
///
/// All items yielded by the iterator must implement [`Render`],
/// which will be rendered sequentially without delimiters.
///
/// This renderer is lazy; the iterator wrapped by this object will only be iterated when
/// [`render`][Render::render] is called on this object, not when this object is constructed.
///
/// Consider using the [`iter!`] macro instead of constructing this type manually.
pub struct RenderIterator<R: Render, I: Iterator<Item = R>>(
    /// The iterator from which items are rendered.
    pub I,
);

impl<R: Render, I: Iterator<Item = R>> From<I> for RenderIterator<R, I> {
    #[inline]
    fn from(iter: I) -> Self {
        Self(iter)
    }
}

impl<R: Render, I: Iterator<Item = R>> Render for RenderIterator<R, I> {
    #[inline]
    fn render(self, buffer: &mut Buffer) {
        for item in self.0 {
            item.render(buffer);
        }
    }
}

/// Wraps an [`Iterator`], making it implement [`Render`], with a delimiter between items.
///
/// All items yielded by the iterator must implement [`Render`],
/// which will be rendered sequentially with a string delimiter in between.
/// The delimiter is **not escaped**.
///
/// This renderer is lazy; the iterator wrapped by this object will only be iterated when
/// [`render`][Render::render] is called on this object, not when this object is constructed.
///
/// Consider using the [`iter!`] macro instead of constructing this type manually.
pub struct RenderIteratorDelimited<R: Render, I: Iterator<Item = R>, S: AsRef<str>>(
    /// The iterator from which items are rendered.
    pub I,
    /// The delimiter to render between the items.
    pub S,
);

impl<R: Render, I: Iterator<Item = R>, S: AsRef<str>> Render for RenderIteratorDelimited<R, I, S> {
    #[inline]
    fn render(self, buffer: &mut Buffer) {
        let mut first = true;

        for item in self.0 {
            if first {
                first = false;
            } else {
                buffer.push_str(self.1.as_ref());
            }

            item.render(buffer);
        }
    }
}

/// Wraps an [`Iterator`] in [`RenderIterator`], making it implement [`Render`].
///
/// This is a convenience macro that wraps the given expression in [`RenderIterator`] or
/// [`RenderIteratorDelimited`] depending on the number of arguments.
///
/// When *one* argument is given, the expression is wrapped in [`RenderIterator`] which does not
/// insert any delimiter between the items.
///
/// When *two* arguments are given, the *first* argument specifies the string delimiter to insert
/// between items, and the *second* argument specifies the iterator expression. Both arguments
/// are wrapped in [`RenderIteratorDelimited`].
///
/// If you are rendering an iterator with the newline `"\n"` string as the delimiter,
/// consider using the [`iter_lines!`] macro instead.
///
/// # Expansion
///
/// ```ignore
/// // iter!($expr)
/// {
///     RenderIterator($expr.into_iter())
/// }
///
/// // iter!($del, $expr)
/// {
///     RenderIteratorDelimited($expr.into_iter(), $del)
/// }
/// ```
///
/// # Example
///
/// Anything that implements [`Iterator`] and yields items that implement [`Render`]
/// can be rendered.
///
/// The following example renders three consecutive `span` elements, by mapping a range of numbers
/// to a function that returns a node created by the [`span!`](laby_macros::span) macro.
/// No delimiters are specified.
///
/// ```
/// # use laby::*;
/// let s = iter!(
///     (1..=3).into_iter().map(|i: u32| span!(i))
/// );
///
/// assert_eq!(render!(s), "<span>1</span><span>2</span><span>3</span>");
/// ```
///
/// Another example renders three consecutive `span` elements, this time with the string `", "`
/// as the delimiter.
///
/// ```
/// # use laby::*;
/// let s = iter!(
///     ", ",
///     (1..=3).into_iter().map(|i: u32| span!(i))
/// );
///
/// assert_eq!(render!(s), "<span>1</span>, <span>2</span>, <span>3</span>");
/// ```
///
/// Note that whitespace in HTML can have [different meanings][1] depending on the context.
///
/// [1]: https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model/Whitespace
#[macro_export]
macro_rules! iter {
    ($expr:expr) => {
        $crate::RenderIterator(($expr).into_iter())
    };

    ($del:expr, $expr:expr) => {
        $crate::RenderIteratorDelimited(($expr).into_iter(), $del)
    };
}

/// Convenience macro for [`iter!`] with the newline delimiter.
///
/// This macro is equivalent to the [`iter!`] macro with the newline `"\n"` delimiter.
///
/// # Expansion
///
/// ```ignore
/// // iter_lines!($expr)
/// {
///     iter!("\n", $expr)
/// }
/// ```
///
/// # Example
///
/// ```
/// # use laby::*;
/// let s = iter_lines!(
///     (1..=2).into_iter().map(|i: u32| span!(i))
/// );
///
/// assert_eq!(render!(s), "<span>1</span>\n<span>2</span>");
/// ```
#[macro_export]
macro_rules! iter_lines {
    ($expr:expr) => {
        $crate::iter!("\n", $expr)
    };
}

/// Wraps an [`AsRef<str>`] without escaping.
///
/// When rendered, the value will be written to the output buffer directly without being escaped.
///
/// Consider using the [`raw!`] macro instead of constructing this type manually.
///
/// # Example
///
/// Using this struct is equivalent to writing to the output buffer directly using
/// [`push_str`](Buffer::push_str) instead of escaping using
/// [`escape`](laby_common::internal::escape).
///
/// ```
/// # use laby::{*, internal::*};
/// struct Raw<'a>(&'a str);
///
/// impl Render for Raw<'_> {
///     #[inline]
///     fn render(self, buffer: &mut Buffer) {
///         // escape(self.0, buffer);
///         buffer.push_str(self.0); // not escaped
///     }
/// }
///
/// let s = render!(Raw("\""));
///
/// assert_eq!(s, "\"");
/// assert_ne!(s, "&quot;");
/// ```
pub struct RenderRaw<S: AsRef<str>>(
    /// The value to write without escaping.
    pub S,
);

impl<S: AsRef<str>> From<S> for RenderRaw<S> {
    #[inline]
    fn from(s: S) -> Self {
        Self(s)
    }
}

impl<S: AsRef<str>> Render for RenderRaw<S> {
    #[inline]
    fn render(self, buffer: &mut Buffer) {
        buffer.push_str(self.0.as_ref());
    }
}

/// Wraps an [`AsRef<str>`] in [`RenderRaw`] without escaping.
///
/// This is a convenience macro that wraps the given expression in [`RenderRaw`].
///
/// # Expansion
///
/// ```ignore
/// // raw!($expr)
/// {
///     RenderRaw($expr)
/// }
/// ```
///
/// # Example
///
/// The following example renders a malicious input string without escaping.
///
/// When rendering a value wrapped in [`RenderRaw`], it is your responsibility to protect
/// yourself from [XSS attacks][1]. laby will never perform automatic escaping for raw values.
///
/// [1]: https://en.wikipedia.org/wiki/Cross-site_scripting
///
/// ```
/// # use laby::*;
/// let input = "<script>maliciousFunc()</script>";
///
/// let escaped = body!(input);
/// let raw = body!(raw!(input));
///
/// // safe
/// assert_eq!(render!(escaped), "<body>&lt;script&gt;maliciousFunc()&lt;/script&gt;</body>");
///
/// // vulnerable to xss
/// assert_eq!(render!(raw), "<body><script>maliciousFunc()</script></body>");
/// ```
#[macro_export]
macro_rules! raw {
    ($expr:expr) => {
        $crate::RenderRaw($expr)
    };
}

/// Wraps a [`Display`], making it implement [`Render`].
///
/// This wrapper exists for convenience but should be avoided when writing code where
/// performance matters.
///
/// The wrapped value will be formatted using an intermediary buffer, which will then be escaped
/// and written to the output buffer. As rendering this wrapper involves an extra heap allocation,
/// implementing the [`Render`] trait directly on the wrapped type should be preferred
/// over using this wrapper.
///
/// Consider using the [`disp!`] macro instead of constructing this type manually.
pub struct RenderDisplay<D: Display>(
    /// The value to render.
    pub D,
);

impl<D: Display> From<D> for RenderDisplay<D> {
    #[inline]
    fn from(dsp: D) -> Self {
        Self(dsp)
    }
}

impl<D: Display> Render for RenderDisplay<D> {
    #[inline]
    fn render(self, buffer: &mut Buffer) {
        self.0.to_string().render(buffer);
    }
}

/// Wraps a [`Display`] in [`RenderDisplay`], making it implement [`Render`].
///
/// This is a convenience macro that wraps the given expression in [`RenderDisplay`].
///
/// # Expansion
///
/// ```ignore
/// // disp!($expr)
/// {
///     RenderDisplay($expr)
/// }
/// ```
///
/// # Example
///
/// This example renders a value that implements [`Display`] by wrapping it using [`disp!`].
///
/// ```
/// # use laby::*;
/// # use std::fmt::*;
/// struct Name<'a> {
///     s: &'a str,
/// }
///
/// impl Display for Name<'_> {
///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
///         write!(f, "{}", self.s)
///     }
/// }
///
/// let s = render!(disp!(Name {
///     s: "laby"
/// }));
///
/// assert_eq!(s, "laby");
/// ```
#[macro_export]
macro_rules! disp {
    ($expr:expr) => {
        $crate::RenderDisplay($expr)
    };
}
