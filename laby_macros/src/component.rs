//
// Copyright (c) 2021 chiya.dev
//
// Use of this source code is governed by the MIT License
// which can be found in the LICENSE file and at:
//
//   https://opensource.org/licenses/MIT
//
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::{
    parse::{Nothing, Parse, ParseStream, Parser},
    parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
    token::Comma,
    AttrStyle, Attribute, Expr, ExprPath, FnArg, Ident, ItemFn, Pat, PatIdent, PatType, Signature,
    Visibility,
};

pub fn generate_component_macro(
    stream: TokenStream,
    args: TokenStream,
) -> syn::Result<TokenStream> {
    Parser::parse2(Nothing::parse, args)?;

    let mut func = Parser::parse2(ItemFn::parse, stream)?;
    let func_sig = func.sig.clone();
    let func_name = func_sig.ident.clone();

    for arg in func.sig.inputs.iter_mut() {
        let PatType { ref mut attrs, .. } = unwrap_typed_arg(arg)?;

        pop_default_attr(attrs)?;
        pop_other_attr(attrs)?;
    }

    if cfg!(feature = "decl_macro") {
        let vis = func.vis.clone();

        Ok(quote!(
            #func

            #[allow(unused_macros)]
            #vis macro #func_name($($x:tt)*) {
                ::laby::__laby_internal_call_fn_named!(#func_sig, $($x)*)
            }
        ))
    } else {
        let vis = match func.vis.clone() {
            // maximum pub(crate) visibility due to macro_rules! limitations
            Visibility::Public(_) => parse_quote!(pub(crate)),
            vis => vis,
        };

        Ok(quote!(
            #func

            #[allow(unused_imports)]
            #vis use #func_name::call as #func_name;

            #[doc(hidden)]
            mod #func_name {
                #[allow(unused_macros)]
                macro_rules! call {
                    ($($x:tt)*) => {
                        ::laby::__laby_internal_call_fn_named!(#func_sig, $($x)*)
                    };
                }

                pub(crate) use call;
            }
        ))
    }
}

fn unwrap_typed_arg(arg: &mut FnArg) -> syn::Result<&mut PatType> {
    match arg {
        FnArg::Typed(ref mut pat) => Ok(pat),
        FnArg::Receiver(ref recv) => Err(syn::Error::new(
            recv.span(),
            "parameter cannot be a receiver",
        )),
    }
}

fn unwrap_named_pat(pat: &Pat) -> syn::Result<&PatIdent> {
    match pat {
        Pat::Ident(ref ident) => Ok(ident),
        ref pat => Err(syn::Error::new(
            pat.span(),
            "pattern must be bound to a name",
        )),
    }
}

fn pop_default_attr(attrs: &mut Vec<Attribute>) -> syn::Result<Option<TokenStream>> {
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

fn pop_other_attr(attrs: &mut Vec<Attribute>) -> syn::Result<Option<TokenStream>> {
    for i in 0..attrs.len() {
        if {
            let ref attr = attrs[i];
            attr.path.is_ident("other") && matches!(attr.style, AttrStyle::Outer)
        } {
            let attr = attrs.remove(i);

            return if attr.tokens.is_empty() {
                Ok(Some(quote!(::laby::frag!)))
            } else {
                Ok(Some(attr.tokens))
            };
        }
    }

    Ok(None)
}

/// Named argument macro involves multiple macros under the hood.
///
/// The first macro `#[laby]` attribute generates a declarative macro in the caller crate using
/// `macro_rules!`. That generated macro expands to an expression that calls an internal hidden
/// procedural macro with the original function signature encoded as an argument
/// (represented by this struct). That internal macro then expands to an expression that actually
/// calls the original function with the proper arguments.
struct EncodedInput {
    sig: Signature,
    args: Punctuated<Expr, Comma>,
}

impl Parse for EncodedInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let sig = input.parse()?;
        input.parse::<Comma>()?;
        let args = input.call(Punctuated::parse_terminated)?;
        Ok(Self { sig, args })
    }
}

pub fn generate_component_macro_call(stream: TokenStream) -> syn::Result<TokenStream> {
    let EncodedInput { mut sig, args } = Parser::parse2(EncodedInput::parse, stream)?;

    // named params
    struct NamedParam {
        default: Option<TokenStream>,
        arg: Option<Expr>,
    }

    let mut params = HashMap::new();
    let mut param_names = Vec::new();

    // children accumulation
    struct OtherParam {
        ident: Ident,
        wrapper: TokenStream,
        args: Vec<Expr>,
    }

    let mut other = None;

    // parse function signature to populate above fields
    for arg in sig.inputs.iter_mut() {
        let PatType {
            ref mut attrs,
            ref pat,
            ..
        } = unwrap_typed_arg(arg)?;

        let PatIdent { ref ident, .. } = unwrap_named_pat(pat)?;
        param_names.push(ident.clone());

        match pop_other_attr(attrs)? {
            // other param
            Some(wrapper) => match other {
                Some(_) => {
                    return Err(syn::Error::new(
                        pat.span(),
                        "`other` attribute cannot be applied on multiple parameters",
                    ));
                }

                _ => {
                    other = Some(OtherParam {
                        ident: ident.clone(),
                        wrapper,
                        args: vec![],
                    });
                }
            },

            // named param
            _ => {
                let key = ident.to_string();
                let value = NamedParam {
                    default: pop_default_attr(attrs)?,
                    arg: None,
                };

                if let Some(_) = params.insert(key.clone(), value) {
                    return Err(syn::Error::new(ident.span(), "duplicate parameter name"));
                }
            }
        }
    }

    // parse arguments and assign arguments to params
    for expr in args {
        match expr {
            // named param
            Expr::Assign(expr) if expr.attrs.len() == 0 => {
                let name = match *expr.left {
                    Expr::Path(ExprPath {
                        ref attrs,
                        ref path,
                        ..
                    }) if attrs.len() == 0 => path.get_ident().map(Clone::clone),
                    _ => None,
                };

                match name {
                    Some(name) => match params.get_mut(&name.to_string()) {
                        Some(NamedParam { ref mut arg, .. }) => {
                            *arg = Some(*expr.right);
                        }

                        _ => {
                            return Err(syn::Error::new(
                                name.span(),
                                format!("unknown parameter: {}", name),
                            ));
                        }
                    },

                    _ => {
                        return Err(syn::Error::new(
                            expr.span(),
                            "invalid parameter name; expected ident",
                        ));
                    }
                }
            }

            // other param
            expr => match other {
                Some(OtherParam { ref mut args, .. }) => {
                    args.push(expr);
                }

                _ => {
                    return Err(syn::Error::new(
                        expr.span(),
                        "invalid expression; expected assignment",
                    ));
                }
            },
        }
    }

    let func = sig.ident;
    let mut func_args = Vec::new();

    // map params to respective arguments
    for name in param_names {
        // other param
        if let Some(OtherParam { ref ident, .. }) = other {
            if *ident == name {
                let OtherParam { wrapper, args, .. } = unsafe {
                    // SAFETY: we just checked above that Option is Some
                    std::mem::replace(&mut other, None).unwrap_unchecked()
                };

                func_args.push(quote!(#wrapper(#(#args),*)));
                continue;
            }
        }

        // named param
        let NamedParam { arg, default, .. } = params.remove(&name.to_string()).unwrap();

        match arg.as_ref().map(ToTokens::to_token_stream).or(default) {
            Some(value) => {
                func_args.push(value);
            }

            _ => {
                return Err(syn::Error::new(
                    Span::call_site(),
                    format!("missing parameter: {}", name),
                ));
            }
        }
    }

    Ok(quote!(#func(#(#func_args),*)))
}
