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

use node::{Element, Node};
use proc_macro::TokenStream;

mod build;
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
                #[doc = include_str!(concat!("../docs/", stringify!($name), ".md"))]
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

fn get_element(tag: &str) -> Option<Element> {
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
