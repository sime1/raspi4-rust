extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Expr, ItemStruct, Token};

struct Params {
    buffer_size: Expr,
    code: Expr,
    tag_id: Expr,
    tag_size: Expr,
}

impl Parse for Params {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        return match parse_params(input) {
            Some(p) => Ok(p),
            None => Err(input.error("cannot parse parameters")),
        };
    }
}

#[proc_macro_attribute]
pub fn mailbox_request(attrs: TokenStream, item: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    return match create_mailbox_request(attrs, item) {
        Ok(s) => s,
        Err(e) => panic!("cannot parse struct: {}", e),
    };
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
    return if let Ok(p) = params {
        let header_name = format_ident!("{}Header", name);
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
    };
}

fn parse_params(attrs: ParseStream) -> Option<Params> {
    let parser = Punctuated::<Expr, Token![,]>::parse_separated_nonempty;
    return if let Ok(params) = parser(attrs) {
        let mut i = params.iter();
        let buffer_size = i.next()?.clone();
        let code = i.next()?.clone();
        let tag_id = i.next()?.clone();
        let tag_size = i.next()?.clone();
        Some(Params {
            buffer_size: buffer_size,
            code: code,
            tag_id: tag_id,
            tag_size: tag_size,
        })
    } else {
        None
    };
}
