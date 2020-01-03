extern crate proc_macro;

use crate::proc_macro::TokenStream;
use heck::ShoutySnakeCase;
use quote::{format_ident, quote};
use syn;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, ItemStruct, Token};

struct Params {
    var_name: Option<Ident>,
    buffer_size: Expr,
    code: Expr,
    tag_id: Expr,
    tag_size: Expr,
}

impl Parse for Params {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        match parse_params(input) {
            Some(p) => Ok(p),
            None => Err(input.error("cannot parse parameters")),
        }
    }
}

#[proc_macro_attribute]
pub fn mailbox_request(attrs: TokenStream, item: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    match create_mailbox_request(attrs, item) {
        Ok(s) => s,
        Err(e) => panic!("cannot parse struct: {}", e),
    }
}

fn create_mailbox_request(
    attrs: TokenStream,
    item: TokenStream,
) -> syn::parse::Result<TokenStream> {
    let struct_ast: ItemStruct = syn::parse(item)?;
    let params: syn::parse::Result<Params> = syn::parse(attrs);
    let name = &struct_ast.ident;
    let fields = struct_ast.fields.iter();
    let struct_attrs = &struct_ast.attrs;
    let gen = quote! {
        #(#struct_attrs)*
        #[repr(C, align(16))]
        struct #name {
            header: MailboxRequestHeader,
            #(#fields),*,
            end_tag: u32,
        }
    };
    if let Ok(p) = params {
        let header_name = if let Some(var_name) = p.var_name {
            var_name
        } else {
            format_ident!("{}_HEADER", name.to_string().to_shouty_snake_case())
        };
        let buffer_size = p.buffer_size;
        let code = p.code;
        let tag_id = p.tag_id;
        let tag_size = p.tag_size;
        let gen = quote! {
            #gen
            const #header_name: MailboxRequestHeader = MailboxRequestHeader {
                buffer_size: #buffer_size,
                code: #code,
                tag_id: #tag_id,
                tag_size: #tag_size,
                unknown: 0,
            };
        };
        Ok(TokenStream::from(gen))
    } else {
        println!("param error");
        Ok(TokenStream::from(gen))
    }
}

fn parse_params(attrs: ParseStream) -> Option<Params> {
    let buffer_size: Expr = parse_param(attrs)?;
    let code: Expr = parse_param(attrs)?;
    let tag_id: Expr = parse_param(attrs)?;
    let tag_size: Expr = parse_param(attrs)?;
    let var_name: Option<Ident> = parse_param(attrs);
    Some(Params {
        var_name,
        buffer_size,
        code,
        tag_id,
        tag_size,
    })
}

fn parse_param<T: syn::parse::Parse>(attrs: ParseStream) -> Option<T> {
    if let Ok(ret) = attrs.parse::<T>() {
        // optional trailing comma
        let _ = attrs.parse::<Token![,]>();
        Some(ret)
    } else {
        None
    }
}
