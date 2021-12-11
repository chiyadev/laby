//
// Copyright (c) 2021 chiya.dev
//
// Use of this source code is governed by the MIT License
// which can be found in the LICENSE file and at:
//
//   https://opensource.org/licenses/MIT
//
use crate::build::build_node;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    punctuated::Punctuated,
    token::{Comma, Semi},
    Ident,
};

#[derive(Clone)]
pub struct Element {
    pub tag: String,
    pub void: bool,
    pub frag: bool,
    pub delimiter: String,
}

impl Element {
    pub fn normal(tag: impl AsRef<str>) -> Self {
        Self {
            tag: tag.as_ref().into(),
            void: false,
            frag: false,
            delimiter: String::new(),
        }
    }

    pub fn void(tag: impl AsRef<str>) -> Self {
        Self {
            tag: tag.as_ref().into(),
            void: true,
            frag: false,
            delimiter: String::new(),
        }
    }

    pub fn frag() -> Self {
        Self {
            tag: "frag".into(),
            void: false,
            frag: true,
            delimiter: String::new(),
        }
    }

    pub fn frag_with_delimiter(del: impl Into<String>) -> Self {
        Self {
            tag: "frag".into(),
            void: false,
            frag: true,
            delimiter: del.into(),
        }
    }
}

pub struct Node {
    pub element: Element,
    pub decl: NodeDecl,
    pub render: NodeRender,
    pub ctor: NodeCtor,
}

impl Node {
    pub fn new(element: Element) -> Self {
        let ident = format_ident!("_{}", element.tag);
        let decl = NodeDecl::new(ident.clone());
        let render = NodeRender::new(ident.clone());
        let ctor = NodeCtor::new(ident.clone());

        Self {
            element,
            decl,
            render,
            ctor,
        }
    }

    pub fn generate(element: Element, stream: TokenStream) -> TokenStream {
        match build_node(element, stream) {
            Ok(node) => node.to_token_stream(),
            Err(error) => error.to_compile_error(),
        }
    }

    pub fn store_generic(&mut self, value: TokenStream, bounds: TokenStream) -> Ident {
        let id = self.decl.fields.len() + 1;
        let name = format_ident!("t{}", id);
        let ty = format_ident!("T{}", id);

        self.decl.generics.push(quote!(#ty));
        self.decl.fields.push(quote!(#name: #ty));

        self.render.generics.push(quote!(#ty));
        self.render.generics_bound.push(quote!(#ty: #bounds));
        self.render.stmts.push(quote!(let #name = self.#name));

        self.ctor.fields.push(quote!(#name: #value));
        name
    }

    pub fn store_concrete(&mut self, value: TokenStream, ty: TokenStream) -> Ident {
        let id = self.decl.fields.len() + 1;
        let name = format_ident!("t{}", id);

        self.decl.fields.push(quote!(#name: #ty));
        self.render.stmts.push(quote!(let #name = self.#name));
        self.ctor.fields.push(quote!(#name: #value));
        name
    }
}

impl ToTokens for Node {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            decl, render, ctor, ..
        } = self;

        quote!({
            #decl
            #render
            #ctor
        })
        .to_tokens(tokens)
    }
}

pub struct NodeDecl {
    ident: Ident,
    generics: Punctuated<TokenStream, Comma>,
    fields: Punctuated<TokenStream, Comma>,
}

impl NodeDecl {
    pub fn new(ident: Ident) -> Self {
        Self {
            ident,
            generics: Punctuated::new(),
            fields: Punctuated::new(),
        }
    }
}

impl ToTokens for NodeDecl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            ident,
            generics,
            fields,
        } = self;

        quote!(
            struct #ident<#generics> {
                #fields
            }
        )
        .to_tokens(tokens)
    }
}

pub struct NodeRender {
    ident: Ident,
    generics: Punctuated<TokenStream, Comma>,
    generics_bound: Punctuated<TokenStream, Comma>,
    stmts: Punctuated<TokenStream, Semi>,
    buffer: String,
}

impl NodeRender {
    pub fn new(ident: Ident) -> Self {
        Self {
            ident,
            generics: Punctuated::new(),
            generics_bound: Punctuated::new(),
            stmts: Punctuated::new(),
            buffer: String::new(),
        }
    }

    pub fn push_str(&mut self, value: impl AsRef<str>) {
        self.buffer.push_str(value.as_ref());
    }

    pub fn push_expr(&mut self, value: TokenStream) {
        self.flush();
        self.stmts.push(value);
    }

    pub fn flush(&mut self) {
        if self.buffer.len() != 0 {
            let literal = Literal::string(&self.buffer);

            self.stmts.push(quote!(buffer.push_str(#literal)));
            self.buffer.clear();
        }
    }
}

impl ToTokens for NodeRender {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            ident,
            generics,
            generics_bound,
            stmts,
            buffer,
        } = self;

        assert!(buffer.len() == 0, "render buffer not flushed");

        quote!(
            impl<#generics_bound> ::laby::Render for #ident<#generics> {
                #[inline]
                fn render(self, buffer: &mut ::laby::internal::Buffer) {
                    #stmts;
                }
            }
        )
        .to_tokens(tokens)
    }
}

pub struct NodeCtor {
    ident: Ident,
    fields: Punctuated<TokenStream, Comma>,
}

impl NodeCtor {
    pub fn new(ident: Ident) -> Self {
        Self {
            ident,
            fields: Punctuated::new(),
        }
    }
}

impl ToTokens for NodeCtor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { ident, fields } = self;

        quote!(
            #ident {
                #fields
            }
        )
        .to_tokens(tokens)
    }
}
