extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{self, parse_macro_input, spanned::Spanned, Item};

#[proc_macro_attribute]
pub fn executable_cmd(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);
    impl_executable_cmd(&input)
}

// #ast
// impl ExecutableCommand for #name {
//     fn exec(&self) -> Result<(), &'static str> {
//         match &self {
//             #name::Test(cmd) => cmd.exec(),
//         }
//     }
// }

fn impl_executable_cmd(input: &Item) -> TokenStream {
    let impl_matches = match &input {
        Item::Enum(e) => {
            // TODO: Check that the enum derives from Subcommand
            let recurse = e.variants.iter().map(|variant| {
                let enum_name = &e.ident;
                let variant_name = &variant.ident;
                quote_spanned! {
                    variant.span() => #enum_name::#variant_name(cmd) => cmd.exec()
                }
            });
            recurse
        }
        _ => panic!("`executable_cmd` attribute can only be used in enums"),
    };
    let gen = quote! {
        #input
        impl ExecutableCommand for Command {
            fn exec(&self) -> Result<(), &'static str> {
                match &self {
                    #(#impl_matches,)*
                }
            }
        }
    };
    gen.into()
}
