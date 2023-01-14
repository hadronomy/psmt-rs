extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn executable_cmd(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse(item).unwrap();
    impl_executable_cmd(&attr, &ast)
}

fn impl_executable_cmd(_attr: &TokenStream, ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        #ast
        impl ExecutableCommand for #name {
            fn exec(&self) -> Result<(), &'static str> {
                match &self {
                    #name::Test(cmd) => cmd.exec(),
                }
            }
        }
    };
    gen.into()
}
