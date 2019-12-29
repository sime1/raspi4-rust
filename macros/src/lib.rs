extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn mailbox_request(attrs: TokenStream, item: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: syn::parse::Result<syn::ItemStruct> = syn::parse(item);
    return match ast {
        Ok(s) => create_mailbox_request(attrs, s),
        Err(e) => panic!("cannot parse struct: {}", e)
    }
}

fn create_mailbox_request(_: TokenStream, item: syn::ItemStruct) -> TokenStream {
    let name = &item.ident;
    let fields = item.fields.iter();
    let attrs = item.attrs;
    let gen = quote! {
        #(#attrs)*
        #[repr(C, align(16))]
        struct #name {
            header: MailboxRequestHeader,
            #(#fields),*,
            end_tag: u32,
        }
    };
    gen.into()
}