//
// Copyright (c) 2021 chiya.dev
//
// Use of this source code is governed by the MIT License
// which can be found in the LICENSE file and at:
//
//   https://opensource.org/licenses/MIT
//
//! laby is a small *macro* library for writing fast HTML templates in Rust.
//! It focuses on three things:
//!
//! - **Simplicity**: laby has minimal dependencies, works out of the box without any
//! configuration, and can be easily extended to add extra functionality where necessary.
//! - **Performance**: laby generates specialized code that generate HTML. It requires no
//! heap allocation at runtime other than the rendering buffer that the resulting HTML gets
//! rendered to. Any operation that involves extra heap allocations is opt-in.
//! All rendering code is statically type checked at compile time and inlined for performance.
//! - **Familiarity**: laby provides macros that accept any valid Rust code;
//! learning a new [DSL][1] for HTML templating is not necessary. Macros can be nested, composed,
//! formatted by [rustfmt][2], separated into components and returned by functions just like
//! normal Rust code.
//!
//! Much of laby's high-performance code was inherited from [sailfish][3], an extremely fast
//! HTML templating engine for Rust. However, whereas sailfish processes HTML template files
//! *with special syntax*, laby provides macros that are embedded *into your code directly*.
//! Which library to adopt is up to your coding style and personal preference.
//!
//! laby targets *Rust stable* and supports embedded environments with [no_std][4].
//! No configuration is required.
//!
//! # Installation
//!
//! In your project, add the following line to your `Cargo.toml` in the `[dependencies]` section:
//!
//! ```toml
//! [dependencies]
//! laby = "0"
//! ```
//!
//! Additionally, you may want to import laby into your code like this:
//!
//! ```
//! use laby::*;
//! ```
//!
//! This is purely for convenience because laby exports a large amount of macros, each of
//! which represent an [HTML tag][5]. Of course, it is possible to import only the macros
//! you use individually. The rest of this guide assumes that you have imported the necessary
//! macros already.
//!
//! laby does not provide integration support for popular web frameworks. It returns a plain old
//! [`String`][7] as the rendered result, so you are encouraged to write your own macro that
//! writes that [`String`][7] to the response stream. Most web frameworks can do this
//! out of the box.
//!
//! # Basics
//!
//! laby provides procedural macros that generate specialized Rust code at compile time,
//! which in turn generate HTML code when rendered at runtime. In order to use laby effectively,
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
//! a [`String`][7] representation. The result is compared to another string which is spread
//! over multiple lines for readability. This code compiles and runs successfully.
//!
//! Notice how the children of a node are passed as normal positional arguments,
//! while the attributes of a node are configured as assignment expressions. This is a perfectly
//! valid Rust syntax, which means it can be formatted using [rustfmt][2].
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
//! This is, in essence, all that laby macros do; they simply declare a new specialized struct
//! for a tree of nodes, implement the [`Render`] trait for that struct, construct that struct,
//! and return the constructed value.
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
//! laby accepts any valid expression in place of attribute names and values and child nodes,
//! and can access variables in the local scope just like normal code. It is not limited to only
//! string literals.
//!
//! The only requirement is for the expression to evaluate to a value that
//! implements the [`Render`] trait. Refer to the [list of foreign impls](Render#foreign-impls)
//! to see which types implement this trait out of the box. The evaluated value is stored in the
//! specialized struct and rendered when the [`render!`] macro is called.
//! Consider the following example.
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
//! Note, that these expressions are evaluated where the node is *constructed*
//! (i.e. `let n = ...`), not where the [`render!`] macro is called.
//!
//! Additionally, the apostrophes in the article contents are escaped with the HTML entity
//! `&#39;`. laby escapes all templated expressions by default unless the [`raw!`] macro is used.
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
//! expressions. When that struct is constructed (i.e. `_article { ... }`), the compiler is able
//! to infer the generic type arguments from the field assignments and monomorphize the struct.
//! Iff all field expressions evaluate to a value that implements the [`Render`] trait,
//! then that trait will also be implemented for the generated struct, allowing for it to be
//! rendered by [`render!`].
//!
//! # Componentization
//!
//! Writing a large template for rendering an entire HTML document quickly becomes unwieldy and
//! unmaintainable, so it is often necessary to break up the document into several smaller
//! components. There are two popular techniques around this problem: *include* and *inherit*.
//! laby supports both patterns, using only the language features provided by Rust.
//!
//! In practice, these patterns are often mixed and matched together to form a complete and
//! coherent document. Examples of both approaches are explored below.
//!
//! #### Template inheritance
//!
//! This is a [top-down approach][6] that breaks down a large document into small components.
//! This leads to a consistent but rigid structure that is difficult to extend or change easily.
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
//! In fact, the macros [`iter!`], [`raw!`] and [`disp!`] are implemented in this way.
//! They are not magic; they are simply extensions of laby's core rendering system. You can even
//! ignore laby's HTML macros and write your own transformations to implement the
//! [`Render`] trait. ~~(But why would you?)~~
//!
//! # License
//!
//! laby is written by [chiya.dev][0], licensed under the [MIT License][9].
//! Portions of code were taken from [sailfish][3] which is written by [Ryohei Machida][10],
//! also licensed under the [MIT License][8]. Documentation for HTML tags were taken
//! from [MDN][11], licensed under [CC-BY-SA 2.5][12].
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
#![no_std]
#![deny(missing_docs)]
extern crate alloc;

mod doctype;
mod helpers;

pub use doctype::*;
pub use helpers::*;
pub use laby_common::*;
pub use laby_macros::*;
