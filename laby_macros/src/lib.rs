//
// Copyright (c) 2021 chiya.dev
//
// Use of this source code is governed by the MIT License
// which can be found in the LICENSE file and at:
//
//   https://opensource.org/licenses/MIT
//
//! Contains macros that generate specialized HTML rendering code.
//!
//! This crate is re-exported by crate `laby`. If you are using laby, you should not depend
//! on this crate directly.
#![deny(missing_docs)]
extern crate proc_macro;

use component::generate_component_macro;
use matching::generate_frag_match;
use node::{Element, Node};
use proc_macro::{Group, Span, TokenStream, TokenTree};

mod build;
mod component;
mod matching;
mod node;

macro_rules! declare_tag {
    ($(#[$attr:meta])* $name:ident, $elem:expr) => {
        $(#[$attr])*
        #[proc_macro]
        pub fn $name(stream: TokenStream) -> TokenStream {
            Node::generate($elem, stream.into()).into()
        }
    };
}

macro_rules! declare_tags {
    ($type:ident, [$list:ident, $count:literal], [$($(#[$attr:meta])* $name:ident),*]) => {
        const $list: [&str; $count] = [$(stringify!($name)),*];

        $(
            declare_tag!(
                $(#[$attr])*
                #[doc = concat!("[`<", stringify!($name), ">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/", stringify!($name), ") element.")]
                #[doc = ""]
                #[doc = include_str!(concat!("../html_docs/", stringify!($name), ".md"))]
                $name,
                Element::$type(stringify!($name))
            );
        )*
    };
}

declare_tags![
    normal,
    [KNOWN_NORMAL_TAGS, 101],
    [
        a, abbr, address, article, aside, audio, b, bdi, bdo, blockquote, body, button, canvas,
        caption, cite, code, colgroup, data, datalist, dd, del, details, dfn, dialog, div, dl, dt,
        em, fieldset, figcaption, figure, footer, form, h1, h2, h3, h4, h5, h6, head, header,
        hgroup, html, i, iframe, ins, kbd, label, legend, li, main, map, mark, menu, menuitem,
        meter, nav, noscript, object, ol, optgroup, option, output, p, picture, pre, progress, q,
        rb, rp, rt, rtc, ruby, s, samp, script, section, select, slot, small, span, strong, style,
        sub, summary, sup, table, tbody, td, template, textarea, tfoot, th, thead, time, title, tr,
        u, ul, var, video
    ]
];

declare_tags![
    void,
    [KNOWN_VOID_TAGS, 14],
    [area, base, br, col, embed, hr, img, input, link, meta, param, source, track, wbr]
];

declare_tag!(
    /// Wraps multiple values implementing [`Render`][2] into one.
    ///
    /// This macro is similar to [React fragments][1] which wrap multiple nodes into one.
    /// It is useful when passing multiple values to a function that accepts only one value,
    /// or when returning multiple values as one return value.
    ///
    /// All wrapped values will be rendered sequentially in the order of arguments without
    /// delimiters.
    ///
    /// [1]: https://reactjs.org/docs/fragments.html
    /// [2]: laby_common::Render
    ///
    /// # Example
    ///
    /// The following example passes multiple nodes to a function that accepts only one node,
    /// by wrapping the arguments in [`frag!`]. By using fragments, intermediary container
    /// elements like [`div`](div!) can be avoided, because they may change the semantics
    /// of the markup.
    ///
    /// ```
    /// # // note: we can't reference the laby crate from laby_macros; macro hacks incoming :)
    /// # use laby_common::{*, internal::*};
    /// # macro_rules! ul {($($x:tt)*)=>{{struct X;impl Render for X{fn render(self,b:&mut Buffer){}};X}};}
    /// # macro_rules! frag {($($x:tt)*)=>{{struct X;impl Render for X{fn render(self,b:&mut Buffer){}};X}};}
    /// fn component(node: impl Render) -> impl Render {
    ///     ul!(node)
    /// }
    ///
    /// // <ul><li>one</li><li>two</li></ul>
    /// component(frag!(
    ///     li!("one"),
    ///     li!("two"),
    /// ));
    /// ```
    ///
    /// This example returns multiple nodes from a function with only one return value.
    ///
    /// ```
    /// # use laby_common::{*, internal::*};
    /// # macro_rules! ul {($($x:tt)*)=>{{struct X;impl Render for X{fn render(self,b:&mut Buffer){}};X}};}
    /// # macro_rules! frag {($($x:tt)*)=>{{struct X;impl Render for X{fn render(self,b:&mut Buffer){}};X}};}
    /// fn component() -> impl Render {
    ///     frag!(
    ///         li!("one"),
    ///         li!("two"),
    ///     )
    /// }
    ///
    /// // <ul><li>one</li><li>two</li></ul>
    /// ul!(component());
    /// ```
    frag,
    Element::frag()
);

declare_tag!(
    /// Wraps multiple values implementing [`Render`][1] into one, with a space delimiter.
    ///
    /// This macro behaves similarly to the [`frag!`] macro. The only difference is that all
    /// wrapped values will be rendered sequentially in the order of arguments,
    /// but with a single space character `' '` to delimit each value.
    ///
    /// It can be convenient when generating an interpolated string for the `class` attribute
    /// in a markup.
    ///
    /// [1]: laby_common::Render
    ///
    /// # Example
    ///
    /// The following example generates a class string with several values interpolated.
    /// `four` is not included because it is [`None`].
    ///
    /// ```
    /// # // see above: can't use laby crate (for the `render!` macro) unfortunately
    /// # use laby_common::{*, internal::*};
    /// # macro_rules! classes {($($x:tt)*)=>{()};}
    /// # macro_rules! render {($($x:tt)*)=>{"one two three  five 6"};}
    /// let two = Some("two");
    /// let four: Option<&str> = None;
    /// let six = 6;
    ///
    /// let s = classes!("one", two, "three", four, "five", six);
    /// assert_eq!(render!(s), "one two three  five 6");
    /// ```
    classes,
    Element::frag_with_delimiter(' ')
);

fn get_element(tag: impl AsRef<str>) -> Option<Element> {
    let tag = tag.as_ref();

    if KNOWN_NORMAL_TAGS.contains(&tag) {
        return Some(Element::normal(tag));
    }

    if KNOWN_VOID_TAGS.contains(&tag) {
        return Some(Element::void(tag));
    }

    if tag == "frag" {
        return Some(Element::frag());
    }

    if tag == "classes" {
        return Some(Element::frag_with_delimiter(' '));
    }

    None
}

/// Generates a macro that calls a function with named parameters.
///
/// Named parameters can be useful when a function accepts several arguments, because explicitly
/// stating the arguments with parameter names can improve readability.
///
/// This attribute macro generates a *function-like macro*, with the same visibility and path as
/// the target function, which allows callers to call that function with the arguments specified
/// in any arbitrary order using *assignment-like expressions* (`$name = $value`).
///
/// Although this attribute is provided for use in laby components, its implementation is not
/// specific to laby. It may be applied to any function, albeit with some caveats documented below.
/// Refer to the crate-level documentation for more usage examples.
///
/// # Default arguments
///
/// By default, all arguments must be specified explicitly, even [`Option<T>`] types. Omittable
/// arguments are opt-in. To mark a parameter as omittable, use the `#[default]` attribute.
///
/// ```ignore
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
/// bar!(); // omitted; same as above
/// ```
///
/// The `#[default]` attribute by default defaults to [`Default::default()`]. This behavior can be
/// customized by passing an expression as the attribute argument. The expression is evaluated in
/// the macro expansion context.
///
/// ```ignore
/// #[laby]
/// fn test(left: &str, #[default("lyba")] right: &str) {
///     assert_eq!(left, right);
/// }
///
/// test!(left = "laby", right = "laby");
/// test!(left = "lyba", right = "lyba");
/// test!(left = "lyba"); // omitted; same as above
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
///     #[laby]
///     fn bad(&self) {}
/// }
/// ```
///
/// ## Function should not be named after an HTML tag
///
/// When a markup macro is invoked within another containing markup macro invocation, laby
/// recognizes this pattern internally and inlines that nested invocation into the parent
/// invocation as an optimization, regardless of whether that macro is indeed a markup macro or
/// another macro with a conflicting name that actually does completely different things. As a
/// workaround, you may alias the function with a different name.
///
/// ```compile_fail
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
///     let s = render!(div!(article!()));
///     assert_eq!(s, "<div>foo</div>"); // <div><article></article></div>
/// }
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
///     foo::bar!(); // no function `bar` in scope
/// }
/// ```
///
/// ## Macro is not exported outside the crate
///
/// The generated macro is defined using `macro_rules!` which prevents macros from being exported
/// in modules other than the crate root. Due to this limitation, the maximum visibility of the
/// generated macro is restricted to `pub(crate)` even if the target function is `pub`.
/// `#[macro_export]` is not used.
///
/// This caveat can be circumvented by enabling the `decl_macro` feature.
///
/// ```compile_fail
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
/// ```
///
/// # Macros 2.0 support
///
/// laby comes with support for the experimental [Declarative Macros 2.0][1] compiler feature
/// which can be enabled using the feature flag `decl_macro`. This requires a nightly compiler.
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
/// The generated macros will now use the new `macro foo { ... }` syntax instead of
/// `macro_rules! foo { ... }`.
///
/// [1]: https://rust-lang.github.io/rfcs/1584-macros.html
#[proc_macro_attribute]
pub fn laby(args: TokenStream, stream: TokenStream) -> TokenStream {
    match generate_component_macro(stream.into(), args.into()) {
        Ok(stream) => stream.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

#[proc_macro]
#[doc(hidden)]
pub fn __laby_internal_set_hygiene_call_site(stream: TokenStream) -> TokenStream {
    let mut result = Vec::new();

    for token in stream {
        match token {
            TokenTree::Group(group) => {
                result.push(TokenTree::Group(Group::new(
                    group.delimiter(),
                    __laby_internal_set_hygiene_call_site(group.stream()),
                )));
            }

            TokenTree::Ident(mut ident) => {
                ident.set_span(Span::call_site());
                result.push(TokenTree::Ident(ident));
            }

            TokenTree::Punct(mut punct) => {
                punct.set_span(Span::call_site());
                result.push(TokenTree::Punct(punct));
            }

            TokenTree::Literal(mut lit) => {
                lit.set_span(Span::call_site());
                result.push(TokenTree::Literal(lit));
            }
        }
    }

    result.into_iter().collect()
}

/// TODO:
#[proc_macro]
pub fn frag_match(stream: TokenStream) -> TokenStream {
    match generate_frag_match(stream.into()) {
        Ok(stream) => stream.into(),
        Err(error) => error.to_compile_error().into(),
    }
}
