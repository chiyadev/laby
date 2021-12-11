//
// Copyright (c) 2021 chiya.dev
//
// Use of this source code is governed by the MIT License
// which can be found in the LICENSE file and at:
//
//   https://opensource.org/licenses/MIT
//
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, Parser},
    parse_quote, Block, Expr, ExprBlock, ExprIf, ExprMatch,
};

pub fn generate_frag_match(stream: TokenStream) -> syn::Result<TokenStream> {
    generate_from_if(stream.clone()).or_else(|_| generate_from_match(stream))
}

fn generate_from_match(stream: TokenStream) -> syn::Result<TokenStream> {
    let mut expr = Parser::parse2(ExprMatch::parse, stream)?;
    let mut vars = Vec::new();
    let mut decls = Vec::new();

    for arm in expr.arms.iter_mut() {
        let name = format_ident!("__variant_{}", decls.len() + 1);
        let ref body = arm.body;

        decls.push(quote!(let mut #name = None));
        arm.body = Box::new(parse_quote!(#name = Some(#body)));
        vars.push(name);
    }

    Ok(quote!({
        #(#decls;)*
        #expr
        ::laby::frag!(#(#vars),*)
    }))
}

fn generate_from_if(stream: TokenStream) -> syn::Result<TokenStream> {
    let mut expr = Parser::parse2(ExprIf::parse, stream)?;
    let mut vars = Vec::new();
    let mut decls = Vec::new();

    {
        let mut add = |block: &mut Block| {
            let name = format_ident!("__variant_{}", decls.len() + 1);

            decls.push(quote!(let mut #name = None));
            *block = parse_quote!({ #name = Some(#block) });
            vars.push(name);
        };

        let mut expr = &mut expr;
        loop {
            add(&mut expr.then_branch);

            match expr.else_branch {
                Some((_, ref mut branch)) => match **branch {
                    Expr::If(ref mut branch) => expr = branch,

                    Expr::Block(ExprBlock { ref mut block, .. }) => {
                        add(block);
                        break;
                    }

                    _ => break,
                },

                _ => break,
            }
        }
    }

    Ok(quote!({
        #(#decls;)*
        #expr
        ::laby::frag!(#(#vars),*)
    }))
}
