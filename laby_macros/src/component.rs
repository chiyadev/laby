//
// Copyright (c) 2021 chiya.dev
//
// Use of this source code is governed by the MIT License
// which can be found in the LICENSE file and at:
//
//   https://opensource.org/licenses/MIT
//
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Nothing, Parse, Parser},
    parse_quote,
    spanned::Spanned,
    AttrStyle, Attribute, FnArg, ItemFn, Pat, PatIdent, PatType, Visibility,
};

pub fn generate_component_macro(
    stream: TokenStream,
    args: TokenStream,
) -> syn::Result<TokenStream> {
    Parser::parse2(Nothing::parse, args)?;

    let mut func = Parser::parse2(ItemFn::parse, stream)?;
    let mut params = Vec::new();
    let mut args = Vec::new();

    for param in func.sig.inputs.iter_mut() {
        match param {
            FnArg::Receiver(recv) => {
                return Err(syn::Error::new(
                    recv.self_token.span,
                    "function cannot declare receiver parameters",
                ))
            }

            FnArg::Typed(PatType {
                ref mut attrs,
                ref pat,
                ..
            }) => match **pat {
                Pat::Ident(PatIdent {
                    ident: ref name, ..
                }) => {
                    if let Some(expr) = get_default_value(attrs)? {
                        params.push(quote!(
                            #[allow(unused_mut, unused_assignments)]
                            let mut #name = #expr;
                        ));
                    } else {
                        params.push(quote!(
                            #[allow(unused_mut)]
                            let mut #name;
                        ));
                    }

                    args.push(quote!(#name));
                }

                ref pat => {
                    return Err(syn::Error::new(
                        pat.span(),
                        "function parameters must be named",
                    ))
                }
            },
        }
    }

    if cfg!(feature = "decl_macro") {
        let name = func.sig.ident.clone();
        let vis = func.vis.clone();

        Ok(quote!(
            #func

            #[allow(unused_macros)]
            #vis macro #name($($name:ident = $value:expr),* $(,)?) {{
                #(#params)*
                $(::laby::__laby_internal_set_hygiene_call_site!($name) = $value;)*
                #name(#(#args),*)
            }}
        ))
    } else {
        let name = func.sig.ident.clone();
        let vis = match func.vis.clone() {
            Visibility::Public(_) => parse_quote!(pub(crate)),
            vis => vis,
        };

        Ok(quote!(
            #func

            #[allow(unused_imports)]
            #vis use #name::call as #name;

            #[doc(hidden)]
            mod #name {
                #[allow(unused_macros)]
                macro_rules! call {
                    ($($name:ident = $value:expr),* $(,)?) => {{
                        #(#params)*
                        $(::laby::__laby_internal_set_hygiene_call_site!($name) = $value;)*
                        #name(#(#args),*)
                    }};
                }

                pub(crate) use call;
            }
        ))
    }
}

fn get_default_value(attrs: &mut Vec<Attribute>) -> syn::Result<Option<TokenStream>> {
    for i in 0..attrs.len() {
        if {
            let ref attr = attrs[i];
            attr.path.is_ident("default") && matches!(attr.style, AttrStyle::Outer)
        } {
            let attr = attrs.remove(i);

            return if attr.tokens.is_empty() {
                Ok(Some(quote!(::core::default::Default::default())))
            } else {
                Ok(Some(attr.tokens))
            };
        }
    }

    Ok(None)
}
