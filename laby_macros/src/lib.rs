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

declare_tag!(frag, Element::frag());
declare_tag!(classes, Element::frag_with_delimiter(' '));

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

#[proc_macro]
pub fn frag_match(stream: TokenStream) -> TokenStream {
    match generate_frag_match(stream.into()) {
        Ok(stream) => stream.into(),
        Err(error) => error.to_compile_error().into(),
    }
}
