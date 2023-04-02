//
// Copyright (c) 2021 chiya.dev
//
// Use of this source code is governed by the MIT License
// which can be found in the LICENSE file and at:
//
//   https://opensource.org/licenses/MIT
//
//! laby is a small *macro* library for writing fast HTML templates in Rust. It focuses on three
//! things:
//!
//! - **Simplicity**: laby has minimal dependencies, works out of the box without any
//! configuration, and can be easily extended to add extra functionality where necessary.
//! - **Performance**: laby generates specialized code that generate HTML. It generally requires no
//! heap allocation at runtime other than the buffer that the resulting HTML gets rendered to. Any
//! operation that involves extra heap allocations is opt-in. All rendering code is statically type
//! checked at compile time and inlined for performance.
//! - **Familiarity**: laby provides macros that can accept any valid Rust code and expand to
//! regular Rust code; learning a new [DSL][1] for HTML templating is not necessary. Macros can be
//! nested, composed, formatted by [rustfmt][2], separated into components and returned by
//! functions just like regular Rust code.
//!
//! Much of laby's high-performance code was inherited from [sailfish][3], an extremely fast HTML
//! templating engine for Rust. However, whereas sailfish processes HTML template files *with
//! special syntax*, laby provides macros that are embedded *into your code directly*. Which
//! library to adopt is up to your coding style and personal preference.
//!
//! laby targets *Rust stable* and supports embedded environments with [no_std][4]. No
//! configuration is required.
//!
//! # Installation
//!
//! In your project, add the following line to your `Cargo.toml` in the `[dependencies]` section:
//!
//! ```toml
//! [dependencies]
//! laby = "0.3"
//! ```
//!
//! Additionally, you may want to import laby into your code like this:
//!
//! ```
//! use laby::*;
//! ```
//!
//! This is purely for convenience because laby exports a large amount of macros, each of which
//! represent an [HTML tag][5]. Of course, it is possible to import only the macros you use
//! individually. The rest of this guide assumes that you have imported the necessary macros
//! already.
//!
//! laby does not provide integration support for popular web frameworks. It returns a plain old
//! [`String`][7] as the rendered result, so you are encouraged to write your own macro that writes
//! that [`String`][7] to the response stream. Most web frameworks can do this out of the box.
//!
//! # Basics
//!
//! laby provides procedural macros that generate specialized Rust code at compile time, which in
//! turn generate HTML code when rendered at runtime. In order to use laby effectively,
//! understanding how it transforms your code is necessary. Consider the following example.
//!
//! ```
//! # use laby::*;
//! // construct a tree of html nodes
//! let n = html!(
//!     head!(
//!         title!("laby"),
//!     ),
//!     body!(
//!         class = "dark",
//!         p!("hello, world"),
//!     ),
//! );
//!
//! // convert the tree into a string
//! let s = render!(n);
//!
//! // check the result
//! assert_eq!(s, "\
//!     <html>\
//!         <head>\
//!             <title>laby</title>\
//!         </head>\
//!         <body class=\"dark\">\
//!             <p>hello, world</p>\
//!         </body>\
//!     </html>\
//! ");
//! ```
//!
//! The above code uses the macros [`html!`], [`head!`], [`title!`], [`body!`] and [`p!`] to
//! construct a basic HTML structure. Then, the [`render!`] macro is used to convert the tree into
//! a [`String`][7] representation. The result is compared to another string which is spread over
//! multiple lines for readability.
//!
//! Notice how the children of a node are passed as regular positional arguments, while the
//! attributes of a node are specified as assignment expressions. This is a perfectly valid Rust
//! syntax, which means it can be formatted using [rustfmt][2].
//!
//! Under the hood, laby transforms the above code into code that looks something like this:
//!
//! ```
//! # use laby::*;
//! let n = {
//!     struct _html {}
//!     impl Render for _html {
//!         #[inline]
//!         fn render(self, buffer: &mut laby::internal::Buffer) {
//!             buffer.push_str("<html><head><title>laby</title></head><body class=\"dark\"><p>hello, world</p></body></html>");
//!         }
//!     }
//!     _html {}
//! };
//!
//! let s = render!(n);
//! // assert_eq!(s, ...);
//! ```
//!
//! This is, in essence, all that laby macros do; they simply declare a new specialized struct for
//! a tree of nodes, implement the [`Render`] trait for that struct, construct that struct, and
//! return the constructed value.
//!
//! When this code is compiled for release, all that wrapper code is stripped away and the
//! rendering code is inlined, leaving something like this for execution:
//!
//! ```
//! # use laby::*;
//! let mut buffer = laby::internal::Buffer::new();
//! buffer.push_str("<html><head><title>laby</title></head><body class=\"dark\"><p>hello, world</p></body></html>");
//!
//! let s = buffer.into_string();
//! // assert_eq!(s, ...);
//! ```
//!
//! # Templating
//!
//! laby accepts any valid expression in place of attribute names and values and child nodes, and
//! can access variables in the local scope just like regular code. It is not limited to only
//! string literals.
//!
//! The only requirement is for the expression to evaluate to a value that implements the
//! [`Render`] trait. Refer to the [list of foreign impls](Render#foreign-impls) to see which types
//! implement this trait out of the box. The evaluated value is stored in the specialized struct
//! and rendered when the [`render!`] macro is called. Consider the following example.
//!
//! ```
//! # use laby::*;
//! // retrieve an article from a database
//! let title = "laby";
//! let content = "hello, 'world'";
//! let date = "2030-01-01";
//!
//! // construct a tree of nodes, with templated expressions
//! let n = article!(
//!     class = format!("date-{}", date),
//!     h1!({
//!         let mut title = title.to_owned();
//!         title.truncate(30);
//!         title
//!     }),
//!     p!(content),
//! );
//!
//! // convert the tree into a string
//! let s = render!(n);
//!
//! // check the result
//! assert_eq!(s, "\
//!     <article class=\"date-2030-01-01\">\
//!         <h1>laby</h1>\
//!         <p>hello, &#39;world&#39;</p>\
//!     </article>\
//! ");
//! ```
//!
//! The above code constructs a basic HTML structure for an article with the title, content and
//! class attribute templated.
//!
//! - `class` attribute: a `format!` macro expression is expanded and evaluated.
//! - `<h1>` node: an expression that truncates the title to at most thirty characters is
//! evaluated.
//! - `<p>` node: a simple local variable expression is evaluated.
//!
//! Note, that these expressions are evaluated where the node is *constructed* (i.e. `let n =
//! ...`), not where the [`render!`] macro is called.
//!
//! Additionally, the apostrophes in the article contents are escaped with the HTML entity `&#39;`.
//! laby escapes all templated expressions by default unless the [`raw!`] macro is used.
//!
//! Under the hood, laby transforms the above code into code that looks something like this:
//!
//! ```
//! # use laby::*;
//! let title = "laby";
//! let content = "hello, 'world'";
//! let date = "2030-01-01";
//!
//! let n = {
//!     struct _article<T1, T2, T3> { t1: T1, t2: T2, t3: T3 }
//!     impl<T1, T2, T3> Render for _article<T1, T2, T3>
//!         where T1: Render, T2: Render, T3: Render {
//!         #[inline]
//!         fn render(self, buffer: &mut laby::internal::Buffer) {
//!             buffer.push_str("<article class=\"");
//!             self.t1.render(buffer); // date
//!             buffer.push_str("\"><h1>");
//!             self.t2.render(buffer); // title
//!             buffer.push_str("</h1><p>");
//!             self.t3.render(buffer); // content
//!             buffer.push_str("</p></article>");
//!         }
//!     }
//!     _article {
//!         t1: format!("date-{}", date),
//!         t2: {
//!             let mut title = title.to_owned();
//!             title.truncate(30);
//!             title
//!         },
//!         t3: content
//!     }
//! };
//!
//! let s = render!(n);
//! // assert_eq!(s, ...);
//! ```
//!
//! Notice how the fields of the generated specialized struct are generic over the templated
//! expressions. When that struct is constructed (i.e. `_article { ... }`), the compiler is able to
//! infer the generic type arguments from the field assignments and monomorphize the struct. Iff
//! all field expressions evaluate to a value that implements the [`Render`] trait, then that trait
//! will also be implemented for the generated struct, allowing for it to be rendered by
//! [`render!`].
//!
//! # Componentization
//!
//! Writing a large template for rendering an entire HTML document quickly becomes unwieldy and
//! unmaintainable, so it is often necessary to break up the document into several smaller
//! components. There are two popular techniques around this problem: *include* and *inherit*. laby
//! supports both patterns, using the language features provided by Rust.
//!
//! In practice, these patterns are often mixed and matched together to form a complete and
//! coherent document. Examples of both approaches are explored below.
//!
//! #### Template inheritance
//!
//! This is a [top-down approach][6] that breaks down a large document into small components. This
//! leads to a consistent but rigid structure that is difficult to extend or change easily.
//!
//! ```
//! # use laby::*;
//! // a large template that takes small components
//! fn page(title: impl Render, header: impl Render, body: impl Render) -> impl Render {
//!     html!(
//!         head!(
//!             title!(title),
//!         ),
//!         body!(
//!             header!(header),
//!             main!(body),
//!         ),
//!     )
//! }
//!
//! // a component that *inherits* a large template
//! fn home() -> impl Render {
//!     page(
//!         "Home",
//!         h1!("About laby"),
//!         p!("laby is an HTML macro library for Rust."),
//!     )
//! }
//!
//! assert_eq!(render!(home()), "\
//!     <html>\
//!         <head>\
//!             <title>Home</title>\
//!         </head>\
//!         <body>\
//!             <header>\
//!                 <h1>About laby</h1>\
//!             </header>\
//!             <main>\
//!                 <p>laby is an HTML macro library for Rust.</p>\
//!             </main>\
//!         </body>\
//!     </html>\
//! ");
//! ```
//!
//! #### Template inclusion
//!
//! This is a [bottom-up approach][6] that consolidates small components to form a large document.
//! This leads to a flexible but possibly inconsistent structure that may also result in more
//! boilerplate code.
//!
//! ```
//! # use laby::*;
//! // small individual components
//! fn title() -> impl Render {
//!     "Home"
//! }
//!
//! fn header() -> impl Render {
//!     h1!("About laby")
//! }
//!
//! fn body() -> impl Render {
//!     p!("laby is an HTML macro library for Rust.")
//! }
//!
//! // a large component that *includes* the small components
//! fn home() -> impl Render {
//!     html!(
//!         head!(
//!             title!(title()),
//!         ),
//!         body!(
//!             header!(header()),
//!             main!(body()),
//!         ),
//!     )
//! }
//!
//! assert_eq!(render!(home()), "\
//!     <html>\
//!         <head>\
//!             <title>Home</title>\
//!         </head>\
//!         <body>\
//!             <header>\
//!                 <h1>About laby</h1>\
//!             </header>\
//!             <main>\
//!                 <p>laby is an HTML macro library for Rust.</p>\
//!             </main>\
//!         </body>\
//!     </html>\
//! ");
//! ```
//!
//! ## Naming arguments
//!
//! Sometimes components can get big and accept a long list of positional arguments that hurts
//! readability. laby provides an attribute macro called [`#[laby]`][13] which lets you call
//! arbitrary functions with explicitly named arguments and optional values, similar to HTML
//! macros.
//!
//! To enable support, simply prepend the attribute before the component function and call it using
//! the generated macro.
//!
//! ```
//! # use laby::*;
//! #[laby]
//! fn page(title: impl Render, header: impl Render, body: impl Render) -> impl Render {
//!     html!(
//!         head!(
//!             title!(title),
//!         ),
//!         body!(
//!             header!(header),
//!             main!(body),
//!         ),
//!     )
//! }
//!
//! #[laby]
//! fn home() -> impl Render {
//!     // `page` function called using the generated `page!` macro
//!     page!(
//!         title = "Home",
//!         header = h1!("About laby"),
//!         body = p!("laby is an HTML macro library for Rust."),
//!     )
//! }
//! ```
//!
//! # Extensions
//!
//! laby can be extended by simply implementing the [`Render`] trait, which is a low-level trait
//! that represents the smallest unit of a rendering operation. If what laby provides out of the
//! box is too limiting for your specific use case, or if laby does not provide a [`Render`]
//! implementation for a type you need, implementing this trait yourself may be a viable solution.
//!
//! The general pattern for creating an extension is like this:
//!
//! 1. Write a struct that stores all necessary data for your rendering operation.
//! 2. Implement the [`Render`] trait for that struct.
//! 3. Provide a simple, short macro that constructs that struct conveniently.
//!
//! In fact, the macros [`iter!`], [`raw!`] and [`disp!`] are implemented in this way. They are not
//! magic; they are simply extensions of laby's core rendering system. You can even ignore laby's
//! HTML macros and write your own transformations to implement the [`Render`] trait.
//!
//! # License
//!
//! laby is written by [chiya.dev][0], licensed under the [MIT License][9]. Portions of code were
//! taken from [sailfish][3] which is written by [Ryohei Machida][10], also licensed under the [MIT
//! License][8]. Documentation for HTML tags were taken from [MDN][11], licensed under [CC-BY-SA
//! 2.5][12].
//!
//! [0]: https://chiya.dev/
//! [1]: https://en.wikipedia.org/wiki/Domain-specific_language
//! [2]: https://github.com/rust-lang/rustfmt
//! [3]: https://docs.rs/sailfish/
//! [4]: https://docs.rust-embedded.org/book/intro/no-std.html
//! [5]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element
//! [6]: https://en.wikipedia.org/wiki/Top-down_and_bottom-up_design
//! [7]: alloc::string::String
//! [8]: https://github.com/Kogia-sima/sailfish/blob/master/LICENSE
//! [9]: https://fossil.chiya.dev/laby/file?name=LICENSE
//! [10]: https://github.com/Kogia-sima
//! [11]: https://developer.mozilla.org/
//! [12]: https://github.com/mdn/content/blob/main/LICENSE.md
//! [13]: laby
#![no_std]
#![deny(missing_docs)]
extern crate alloc;

mod doctype;
mod helpers;

pub use doctype::*;
pub use helpers::*;
pub use laby_common::*;
pub use laby_macros::{
    __laby_internal_call_fn_named, __laby_internal_set_hygiene_call_site, a, abbr, address, area,
    article, aside, audio, b, base, bdi, bdo, blockquote, body, br, button, canvas, caption, cite,
    code, col, colgroup, data, datalist, dd, del, details, dfn, dialog, div, dl, dt, em, embed,
    fieldset, figcaption, figure, footer, form, h1, h2, h3, h4, h5, h6, head, header, hgroup, hr,
    html, i, iframe, img, input, ins, kbd, label, legend, li, link, main, map, mark, menu,
    menuitem, meta, meter, nav, noscript, object, ol, optgroup, option, output, p, param, picture,
    pre, progress, q, rb, rp, rt, rtc, ruby, s, samp, script, section, select, slot, small, source,
    span, strong, style, sub, summary, sup, table, tbody, td, template, textarea, tfoot, th, thead,
    time, title, tr, track, u, ul, var, video, wbr,
};

/// Generates a macro that calls a function with named arguments.
///
/// Named arguments can be useful when a function accepts several arguments, because explicitly
/// stating the arguments with parameter names can improve readability.
///
/// This attribute macro generates a *function-like macro*, with the same visibility and path as
/// the target function, which allows callers to call that function with the arguments specified in
/// any arbitrary order using *assignment-like expressions* (`name = value`).
///
/// Although this attribute is provided for use in laby components, its implementation is not
/// specific to laby. It may be applied to any function, albeit with some caveats documented below.
/// Refer to the crate-level documentation for more usage examples.
///
/// # `#[default]` arguments
///
/// By default, all arguments must be specified explicitly, even [`Option<T>`] types. Omittable
/// arguments are opt-in. To mark a parameter as omittable, prepend the `#[default]` attribute to
/// the parameter.
///
/// ```
/// # use laby::*;
/// #[laby]
/// fn foo(x: Option<&str>) {
///     assert!(x.is_none());
/// }
///
/// #[laby]
/// fn bar(#[default] x: Option<&str>) {
///     assert!(x.is_none());
/// }
///
/// foo!(x = None); // required
/// bar!(x = None); // omittable
/// bar!(); // omitted; equivalent to the above line
/// ```
///
/// This attribute by default defaults to [`Default::default()`]. This behavior can be customized
/// by passing a default expression as the attribute argument. The expression is evaluated in the
/// macro expansion context.
///
/// ```
/// # use laby::*;
/// #[laby]
/// fn test(left: &str, #[default("b")] right: &str) {
///     assert_eq!(left, right);
/// }
///
/// test!(left = "a", right = "a");
/// test!(left = "b", right = "b");
/// test!(left = "b"); // omitted; equivalent to the above line
/// ```
///
/// It is not possible to apply `#[default]` on generic parameters like `impl Render` because the
/// compiler cannot infer which default implementation of [`Render`] should be used. This can be
/// circumvented by using the unit type `()` implementation of [`Render`] as the default
/// expression, which simply renders nothing.
///
/// ```
/// # use laby::*;
/// #[laby]
/// fn component(#[default(())] title: impl Render) -> impl Render {
///     article!(
///         h1!(title),
///     )
/// }
///
/// assert_eq!(render!(component!()), "<article><h1></h1></article>");
/// assert_eq!(render!(component!(title = a!("title"))), "<article><h1><a>title</a></h1></article>");
/// ```
///
/// # `#[rest]` arguments
///
/// By default, all arguments must be specified with their respective parameter name. A function
/// may declare at most one parameter with this attribute, which binds all arguments without a
/// specified name to that parameter, wrapped together using [`frag!`]. This behavior is similar to
/// [React children][2].
///
/// ```
/// # use laby::*;
/// #[laby]
/// fn component(#[default(())] title: impl Render, #[rest] children: impl Render) -> impl Render {
///     article!(
///         h1!(title),
///         main!(children),
///     )
/// }
///
/// assert_eq!(render!(component!()), "<article><h1></h1><main></main></article>");
/// assert_eq!(render!(component!("para")), "<article><h1></h1><main>para</main></article>");
/// assert_eq!(render!(component!(p!("para1"), p!("para2"))), "<article><h1></h1><main><p>para1</p><p>para2</p></main></article>");
/// assert_eq!(render!(component!(title = "laby", p!("para1"), p!("para2"))), "<article><h1>laby</h1><main><p>para1</p><p>para2</p></main></article>");
/// ```
///
/// # Caveats
///
/// ## Function must be free-standing
///
/// The target function with this attribute must be free-standing; it must be declared at the
/// module-level, not within a `trait` or `impl` block. This is because Rust simply does not
/// support macro declarations in such places.
///
/// ```compile_fail
/// struct Foo;
///
/// #[laby]
/// fn good(x: &Foo) {}
///
/// impl Foo {
///     // this will not compile:
///     #[laby]
///     fn bad(&self) {}
/// }
/// ```
///
/// ## Function should not be named after an HTML tag
///
/// When a markup macro named after an HTML tag is invoked within another markup macro, laby
/// recognizes this pattern and inlines that nested HTML macro into the parent macro as an
/// optimization, regardless of whether that HTML macro is indeed an HTML macro or another macro
/// with a conflicting name that actually does something completely different. As a workaround, you
/// may alias the function with a different name.
///
/// ```compile_fail
/// # use laby::*;
/// #[laby]
/// fn article() -> impl Render {
///     "foo"
/// }
///
/// fn good() {
///     use article as foo;
///
///     let s = render!(div!(foo!()));
///     assert_eq!(s, "<div>foo</div>");
/// }
///
/// fn bad() {
///     // refers to `laby::article`, not the `article` macro declared above!
///     let s = render!(div!(article!()));
///     assert_eq!(s, "<div><article></article></div>");
/// }
/// # good(); bad();
/// ```
///
/// ## Macro must be imported into scope
///
/// When calling the target function using the generated macro, both that function and the macro
/// must be imported directly into the current scope. It cannot be called by relative or fully
/// qualified paths. This is due to hygiene limitations of `macro_rules!` which prevent functions
/// from being referenced within macros unambiguously.
///
/// This caveat can be circumvented by enabling the `decl_macro` feature.
///
/// ```compile_fail
/// # use laby::*;
/// mod foo {
///     #[laby]
///     pub fn bar() {}
/// }
///
/// fn good() {
///     use foo::bar;
///     bar!();
/// }
///
/// fn bad() {
///     foo::bar!(); // no function named `bar` in scope
/// }
/// # good(); bad();
/// ```
///
/// ## Macro is not exported outside the crate
///
/// The generated macro is defined using `macro_rules!` which prevents macros from being exported
/// in modules other than the crate root. Due to this limitation, the maximum visibility of the
/// generated macro is restricted to `pub(crate)` even if the target function is `pub`.
///
/// This caveat can be circumvented by enabling the `decl_macro` feature.
///
/// ```compile_fail
/// # use laby::*;
/// // crate_a
/// #[laby]
/// pub fn foo() {}
///
/// fn good() {
///     foo!();
/// }
///
/// // crate_b
/// fn bad() {
///     use crate_a::foo; // macro `foo` is private
///     foo!();
/// }
/// # good(); bad();
/// ```
///
/// # Macros 2.0 support
///
/// laby comes with support for the experimental [Declarative Macros 2.0][1] compiler feature which
/// can be enabled using the feature flag `decl_macro`. This requires a nightly toolchain.
///
/// To enable this feature, add laby's feature flag in your `Cargo.toml`,
///
/// ```toml
/// [dependencies]
/// laby = { version = "...", features = ["decl_macro"] }
/// ```
///
/// and enable the compiler's feature flag in your crate root.
///
/// ```
/// #![feature(decl_macro)]
/// ```
///
/// The generated macros will now use the new `macro foo { ... }` syntax instead of `macro_rules!
/// foo { ... }`.
///
/// [1]: https://rust-lang.github.io/rfcs/1584-macros.html
/// [2]: https://reactjs.org/docs/composition-vs-inheritance.html
pub use laby_macros::laby;

/// Wraps multiple values implementing [`Render`][2] into one.
///
/// This macro is similar to [React fragments][1] which wrap multiple nodes into one. It is useful
/// when passing multiple values to a function that accepts only one value, or when returning
/// multiple values as one return value.
///
/// All wrapped values will be rendered sequentially in the order of arguments without delimiters.
///
/// [1]: https://reactjs.org/docs/fragments.html
/// [2]: laby_common::Render
///
/// # Example
///
/// The following example passes multiple nodes to a function that accepts only one node, by
/// wrapping the arguments in [`frag!`]. By using fragments, intermediary container elements like
/// [`div`](div!) or [`span`](span!), which changes the semantics of the markup, can be avoided.
///
/// This example passes multiple nodes to a function which takes only one value.
///
/// ```
/// # use laby::*;
/// fn component(node: impl Render) -> impl Render {
///     ul!(node)
/// }
///
/// let s = render!(component(frag!(
///     li!("one"),
///     li!("two"),
/// )));
///
/// assert_eq!(s, "<ul><li>one</li><li>two</li></ul>");
/// ```
///
/// This example returns multiple nodes from a function which returns only one value.
///
/// ```
/// # use laby::*;
/// fn component() -> impl Render {
///     frag!(
///         li!("one"),
///         li!("two"),
///     )
/// }
///
/// let s = render!(ul!(component()));
/// assert_eq!(s, "<ul><li>one</li><li>two</li></ul>");
/// ```
pub use laby_macros::frag;

/// Wraps a `match` or `if` expression returning [`Render`][1] into one.
///
/// This macro allows a `match` or `if` expression to return different types of [`Render`][1]
/// implementations. This would otherwise be disallowed because all branches of a `match` or `if`
/// expression must return the same type of [`Render`][1] implementation.
///
/// This macro was named `frag_match` because it uses a set of [`Option`] variables for each
/// variant and the [`frag!`][2] macro for rendering.
///
/// # Expansion
///
/// ```ignore
/// // frag_match!(match $expr { $pat => $expr, ... })
/// {
///     let mut variant1 = None, mut variant2 = None, ..;
///
///     match $expr {
///         $pat => variant1 = Some($expr),
///         $pat => variant2 = Some($expr), ..
///     }
///
///     frag!(variant1, variant2, ..)
/// }
/// ```
///
/// # Examples
///
/// This example adds different types of nodes to a [`Vec<T>`][3] using the [`frag_match!`] macro.
///
/// ```
/// # use laby::*;
/// let mut vec = Vec::new();
///
/// for value in ["div", "span", "img"] {
///     vec.push(frag_match!(match value {
///         "div" => div!(),
///         "span" => span!(),
///         "img" => img!(),
///         _ => unreachable!(),
///     }));
/// }
///
/// let s = render!(iter!(vec));
/// assert_eq!(s, "<div></div><span></span><img>");
/// ```
///
/// [1]: laby_common::Render
/// [2]: laby_macros::frag
/// [3]: alloc::vec::Vec
pub use laby_macros::frag_match;

/// Wraps multiple values implementing [`Render`][1] into one, with whitespace as the delimiter.
///
/// This macro behaves similarly to the [`frag!`] macro. The only difference is that all wrapped
/// values will be rendered sequentially in the order of arguments, but with a single whitespace
/// character `' '` to delimit each value.
///
/// It is intended to be used to generate an interpolated string for the `class` attribute in an
/// HTML markup.
///
/// [1]: laby_common::Render
///
/// # Example
///
/// The following example generates a class string with several values interpolated. Note that
/// `four` is not included because it is [`None`], but the whitespace that delimits `four` is
/// still rendered regardless.
///
/// ```
/// # use laby::*;
/// let two = Some("two");
/// let four: Option<&str> = None;
/// let six = 6;
///
/// let s = classes!("one", two, "three", four, "five", six);
/// assert_eq!(render!(s), "one two three  five 6");
/// ```
pub use laby_macros::classes;
