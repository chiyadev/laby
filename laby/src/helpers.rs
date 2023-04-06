//
// Copyright (c) 2021 chiya.dev
//
// Use of this source code is governed by the MIT License
// which can be found in the LICENSE file and at:
//
//   https://opensource.org/licenses/MIT
//
use core::fmt::Display;
use laby_common::{internal::Buffer, Render};

/// Indicates that an attribute is an HTML boolean attribute.
///
/// If a _regular_ attribute value evaluates to a boolean value `true` or `false`, the
/// corresponding string representation `"true"` or `"false"` is rendered. This macro indicates
/// that the attribute is an HTML _boolean_ attribute.
///
/// If a _boolean_ attribute value evalutes to `false`, it is not rendered at all. Conversely, if
/// it evaluates to `true`, only the attribute's name is rendered, without a value component (i.e.
/// without `="true"`).
///
/// This is a special macro which is recognized internally by laby when used as an attribute value
/// in expression position. If called outside a markup macro, or called in an invalid position, or
/// the argument to this macro is not actually a boolean, compilation will fail.
///
/// This macro is necessary because it is not possible for laby to determine the type of an
/// attribute's value at compile time. This is a fundamental limitation of procedural macros in
/// Rust that must operate on syntax tokens before the compiler performs any kind of type
/// resolution.
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
        ::core::compile_error!("invalid use of boolean macro")
    };
}

#[allow(unused_imports)] // for docs
use alloc::string::String;

/// Renders the given value and returns the result as a [`String`].
///
/// This is a convenience macro that constructs a new [`Buffer`], renders the given expression into
/// it, and returns the buffer as a [`String`].
///
/// The value must implement the [`Render`] trait.
///
/// If multiple values are given, they are wrapped using the [`frag!`][crate::frag] macro and
/// rendered sequentially without delimiters.
///
/// # Expansion
///
/// ```ignore
/// // render!($expr)
/// {
///     let mut buf = Buffer::new();
///     $expr.render(&mut buf);
///     buf.into_string()
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
/// Render a simple node as a string.
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
/// The [`render!`](crate::render) macro can render any value that implements the [`Render`] trait,
/// which is not limited only to nodes constructed by markup macros.  See the [list of foreign
/// impls](Render#foreign-impls) on the [`Render`] trait to see which types are supported.
///
/// ```
/// # use laby::*;
/// let v: u32 = 100;
/// assert_eq!(render!(v), "100");
/// ```
///
/// All strings are escaped by default when rendered. This behavior can be opted out of by using
/// the [`raw!`](crate::raw) macro.
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
/// This example renders multiple nodes using the [`render!`](crate::render) macro. It is
/// equivalent to passing a single argument using the [`frag!`][crate::frag] macro that wraps the
/// values together.
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
        let mut buf = $crate::internal::Buffer::with_capacity(16384);
        $crate::Render::render($expr, &mut buf);
        buf.into_string()
    }};

    ($($expr:expr),* $(,)?) => {{
        let frag = $crate::frag!($($expr),*);
        $crate::render!(frag)
    }};
}

/// Wraps an [`Iterator`], making it implement [`Render`].
///
/// This is a workaround for Rust's lack of impl specialization. When Rust gets specialization,
/// this type may no longer be necessary.
///
/// All items yielded by the iterator must implement [`Render`], which will be rendered
/// sequentially, in the order they are yielded, without delimiters.
///
/// This renderer is lazy; the iterator wrapped by this type will only be iterated when
/// [`render`][Render::render] is called on the value, not when the value is constructed.
///
/// Consider using the [`iter!`](crate::iter) macro instead of constructing this type manually.
pub struct RenderIterator<I>(
    /// The iterator from which items are rendered.
    pub I,
)
where
    I: IntoIterator,
    I::Item: Render;

impl<I> From<I> for RenderIterator<I>
where
    I: IntoIterator,
    I::Item: Render,
{
    #[inline]
    fn from(iter: I) -> Self {
        Self(iter)
    }
}

impl<I> Render for RenderIterator<I>
where
    I: IntoIterator,
    I::Item: Render,
{
    #[inline]
    fn render(self, buf: &mut Buffer) {
        for item in self.0 {
            item.render(buf);
        }
    }
}

/// Wraps an [`Iterator`], making it implement [`Render`], with a delimiter between items.
///
/// All items yielded by the iterator must implement [`Render`], which will be rendered
/// sequentially, in the order they are yielded, with a string delimiter in-between. The delimiter
/// is **not escaped**.
///
/// This renderer is lazy; the iterator wrapped by this type will only be iterated when
/// [`render`][Render::render] is called on the value, not when the value is constructed.
///
/// Consider using the [`iter!`](crate::iter) macro instead of constructing this type manually.
pub struct RenderIteratorDelimited<I, S>(
    /// The iterator from which items are rendered.
    pub I,
    /// The delimiter to render between the items.
    pub S,
)
where
    I: IntoIterator,
    I::Item: Render,
    S: AsRef<str>;

impl<I, S> Render for RenderIteratorDelimited<I, S>
where
    I: IntoIterator,
    I::Item: Render,
    S: AsRef<str>,
{
    #[inline]
    fn render(self, buf: &mut Buffer) {
        let mut iter = self.0.into_iter();
        let del = self.1.as_ref();

        if let Some(item) = iter.next() {
            item.render(buf);
        }

        for item in iter {
            buf.push_str(del);
            item.render(buf);
        }
    }
}

/// Wraps an [`Iterator`] in [`RenderIterator`], making it implement [`Render`].
///
/// This is a convenience macro that wraps the given expression in [`RenderIterator`] or
/// [`RenderIteratorDelimited`], depending on the number of arguments.
///
/// When *one* argument is given, the expression is wrapped in [`RenderIterator`] which does not
/// insert any delimiter between the items.
///
/// When *two* arguments are given, the *first* argument specifies the string delimiter to insert
/// between items, and the *second* argument specifies the iterator expression. The arguments are
/// wrapped in [`RenderIteratorDelimited`].
///
/// If you are rendering an iterator with the newline `"\n"` string as the delimiter, consider
/// using the [`iter_lines!`](crate::iter_lines) macro instead.
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
/// Anything that implements [`Iterator<Item = T>`](Iterator) where `T` implements [`Render`] can
/// be rendered.
///
/// The following example renders three consecutive `span` elements, by mapping a range of numbers
/// to a function that returns a node created by the [`span!`](laby_macros::span) macro. No
/// delimiters are specified.
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
/// Another example renders three consecutive `span` elements, this time with the string `", "` as
/// the delimiter.
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
        $crate::RenderIterator($expr)
    };

    ($del:expr, $expr:expr) => {
        $crate::RenderIteratorDelimited($expr, $del)
    };
}

/// Convenience macro for [`iter!`](crate::iter) with the newline delimiter.
///
/// This macro is equivalent to the [`iter!`](crate::iter) macro with the newline `"\n"` delimiter.
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

/// Wraps an [`AsRef<str>`], rendering it without escaping.
///
/// When rendered, the value will be written to the output buffer directly without being escaped.
///
/// Consider using the [`raw!`](crate::raw) macro instead of constructing this type manually.
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
///     fn render(self, buf: &mut Buffer) {
///         // escape(self.0, buf);
///         buf.push_str(self.0); // not escaped
///     }
/// }
///
/// let s = render!(Raw("\""));
///
/// assert_eq!(s, "\"");
/// assert_ne!(s, "&quot;");
/// ```
pub struct RenderRaw<S>(
    /// The value to write without escaping.
    pub S,
)
where
    S: AsRef<str>;

impl<S> From<S> for RenderRaw<S>
where
    S: AsRef<str>,
{
    #[inline]
    fn from(s: S) -> Self {
        Self(s)
    }
}

impl<S> Render for RenderRaw<S>
where
    S: AsRef<str>,
{
    #[inline]
    fn render(self, buf: &mut Buffer) {
        buf.push_str(self.0.as_ref());
    }
}

/// Wraps an [`AsRef<str>`] in [`RenderRaw`], rendering it without escaping.
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
/// When rendering a value wrapped in [`RenderRaw`], it is your responsibility to protect yourself
/// from code injection attacks such as [XSS][1]. laby will never perform automatic escaping for
/// raw values.
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
/// // vulnerable!
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
/// Consider using the [`disp!`](crate::disp) macro instead of constructing this type manually.
pub struct RenderDisplay<D>(
    /// The value to render.
    pub D,
)
where
    D: Display;

impl<D> From<D> for RenderDisplay<D>
where
    D: Display,
{
    #[inline]
    fn from(dsp: D) -> Self {
        Self(dsp)
    }
}

impl<D> Render for RenderDisplay<D>
where
    D: Display,
{
    #[inline]
    fn render(self, buf: &mut Buffer) {
        format_args!("{}", self.0).render(buf)
    }
}

/// Wraps a [`Display`] in [`RenderDisplay`], making it implement [`Render`].
///
/// This is a convenience macro that wraps the given expression in [`RenderDisplay`].
///
/// Beginning with laby `0.4`, [`Arguments`](core::fmt::Arguments) struct implements [`Render`]
/// directly so [`disp!`] and [`RenderDisplay`] are deprecated. You should write
/// `format_args!("{}", expr)` instead of `disp!(expr)`.
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
/// This example renders a value that implements [`Display`] by wrapping it using
/// [`disp!`](crate::disp).
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
#[deprecated(since = "0.4.0", note = "use `format_args!(\"{}\", ...)` instead")]
macro_rules! disp {
    ($expr:expr) => {
        $crate::RenderDisplay($expr)
    };
}
