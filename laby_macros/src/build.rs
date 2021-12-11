//
// Copyright (c) 2021 chiya.dev
//
// Use of this source code is governed by the MIT License
// which can be found in the LICENSE file and at:
//
//   https://opensource.org/licenses/MIT
//
use crate::{
    get_element,
    node::{Element, Node},
};
use laby_common::{internal::Buffer, Render};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::quote;
use syn::{
    parse::Parser, punctuated::Punctuated, spanned::Spanned, token::Comma, Expr, ExprAssign,
    ExprLit, Lit, Stmt,
};

pub fn build_node(element: Element, stream: TokenStream) -> syn::Result<Node> {
    let mut node = Node::new(element.clone());

    build_inner(element, stream, &mut node)?;
    node.render.flush();

    Ok(node)
}

fn build_inner(element: Element, stream: TokenStream, node: &mut Node) -> syn::Result<()> {
    let args = Parser::parse2(Punctuated::<Expr, Comma>::parse_terminated, stream)?;

    let mut attrs = Vec::new();
    let mut children = Vec::new();

    for expr in args {
        if let Expr::Assign(expr) = expr {
            attrs.push(expr);
        } else {
            children.push(expr);
        }
    }

    if element.frag && attrs.len() != 0 {
        return Err(syn::Error::new(
            attrs[0].span(),
            "invalid attribute in fragment",
        ));
    }

    if element.void && children.len() != 0 {
        return Err(syn::Error::new(
            children[0].span(),
            "invalid child in void element",
        ));
    }

    if !element.frag {
        node.render.push_str("<");
        node.render.push_str(&element.tag);

        for attr in attrs {
            match *attr.right {
                Expr::Macro(ref expr) => {
                    if let Some(ident) = expr.mac.path.get_ident() {
                        if ident == "bool" {
                            let mut attr = attr.clone();
                            attr.right = syn::parse2(expr.mac.tokens.clone())?;

                            set_boolean_attr(attr, node)?;
                            continue;
                        }
                    }
                }

                _ => {}
            };

            set_attr(attr, node)?;
        }

        node.render.push_str(">");
    }

    if !element.void {
        let mut separate = false;

        for child in children {
            if separate {
                node.render.push_str(&element.delimiter);
            } else {
                separate = true;
            }

            match child {
                Expr::Macro(ref expr) => {
                    if let Some(ident) = expr.mac.path.get_ident() {
                        if let Some(element) = get_element(ident.to_string()) {
                            // flatten nested markup
                            build_inner(element, expr.mac.tokens.clone(), node)?;
                            continue;
                        }
                    }
                }

                _ => {}
            };

            set_child(child, node)?;
        }

        if !element.frag {
            node.render.push_str("</");
            node.render.push_str(&element.tag);
            node.render.push_str(">");
        }
    }

    Ok(())
}

fn try_unwrap_literal(expr: &Expr) -> Option<&ExprLit> {
    match expr {
        Expr::Lit(lit) if lit.attrs.len() == 0 => Some(lit),
        Expr::Paren(paren) if paren.attrs.len() == 0 => try_unwrap_literal(&paren.expr),

        Expr::Block(block) if block.attrs.len() == 0 && block.block.stmts.len() == 1 => {
            if let Stmt::Expr(expr) = block.block.stmts.first()? {
                try_unwrap_literal(expr)
            } else {
                None
            }
        }

        _ => None,
    }
}

fn try_unwrap_ident(expr: &Expr) -> Option<&Ident> {
    match expr {
        Expr::Path(expr) if expr.attrs.len() == 0 => {
            if let Some(ident) = expr.path.get_ident() {
                Some(ident)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn try_render_literal(expr: &ExprLit) -> Option<String> {
    macro_rules! render {
        ($expr:expr) => {{
            let mut buffer = Buffer::new();
            Render::render($expr, &mut buffer);
            buffer.into_string()
        }};
    }

    if expr.attrs.len() == 0 {
        Some(match expr.lit {
            Lit::Str(ref v) => render!(v.value()),
            Lit::Byte(ref v) => render!(v.value()),
            Lit::Char(ref v) => render!(v.value()),
            Lit::Bool(ref v) => render!(v.value()),
            Lit::Int(ref v) => render!(v.base10_parse::<u128>().ok()?),
            Lit::Float(ref v) => render!(v.base10_parse::<f64>().ok()?),

            _ => return None,
        })
    } else {
        None
    }
}

fn set_attr(assign: ExprAssign, node: &mut Node) -> syn::Result<()> {
    let left = assign.left;
    let right = assign.right;

    node.render.push_str(" ");

    if let Some(value) = try_unwrap_literal(&left)
        .and_then(try_render_literal)
        .or_else(|| try_unwrap_ident(&left).map(ToString::to_string))
    {
        node.render.push_str(value);
    } else {
        let value = node.store_generic(quote!(#left), quote!(::laby::Render));
        node.render
            .push_expr(quote!(::laby::Render::render(#value, buffer)));
    }

    node.render.push_str("=\"");

    if let Some(value) = try_unwrap_literal(&right).and_then(try_render_literal) {
        node.render.push_str(value);
    } else {
        let value = node.store_generic(quote!(#right), quote!(::laby::Render));
        node.render
            .push_expr(quote!(::laby::Render::render(#value, buffer)));
    }

    node.render.push_str("\"");
    Ok(())
}

fn set_boolean_attr(assign: ExprAssign, node: &mut Node) -> syn::Result<()> {
    let left = assign.left;
    let right = assign.right;

    if let Some(enabled) = try_unwrap_literal(&right).and_then(|expr| match &expr.lit {
        Lit::Bool(v) => Some(v.value()),
        _ => None,
    }) {
        if enabled {
            node.render.push_str(" ");

            if let Some(value) = try_unwrap_literal(&left)
                .and_then(try_render_literal)
                .or_else(|| try_unwrap_ident(&left).map(ToString::to_string))
            {
                node.render.push_str(value);
            } else {
                let value = node.store_generic(quote!(#left), quote!(::laby::Render));
                node.render
                    .push_expr(quote!(::laby::Render::render(#value, buffer)));
            }
        }
    } else {
        let enabled = node.store_concrete(quote!(#right), quote!(::core::primitive::bool));

        if let Some(value) = try_unwrap_literal(&left)
            .and_then(try_render_literal)
            .or_else(|| try_unwrap_ident(&left).map(ToString::to_string))
        {
            let value = Literal::string(&format!(" {}", value));
            node.render.push_expr(quote!(
                if #enabled {
                    buffer.push_str(#value);
                }
            ));
        } else {
            let value = node.store_generic(quote!(#left), quote!(::laby::Render));
            node.render.push_expr(quote!(
                if #enabled {
                    buffer.push(' ');
                    ::laby::Render::render(#value, buffer);
                }
            ));
        }
    }

    Ok(())
}

fn set_child(expr: Expr, node: &mut Node) -> syn::Result<()> {
    if let Some(value) = try_unwrap_literal(&expr).and_then(try_render_literal) {
        node.render.push_str(value);
    } else {
        let value = node.store_generic(quote!(#expr), quote!(::laby::Render));
        node.render
            .push_expr(quote!(::laby::Render::render(#value, buffer)));
    }

    Ok(())
}
