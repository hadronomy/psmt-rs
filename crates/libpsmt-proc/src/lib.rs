extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::{quote};
use syn::{self, parse_macro_input, Item};

/// Implements `ExecutableCommand` to an **enum**.
///
/// This allows to execute an enumeration of
/// subcommands by calling `my_enum.exec()`.
///
/// ```ignore
/// struct TestCommand;
///
/// // For `executable_cmd` to work
/// // exec needs to be implemented
/// // in every subcommand
/// impl TestCommand {
///     pub fn exec(&self) -> Result<(), &'static str> {
///         todo!("Execute test command");
///     }
/// }
///
/// /// Subcommands enum used by the main clap
/// /// command
/// #[derive(Subcommand)]
/// #[executable_cmd]
/// enum Command {
///     Test(TestCommand)
/// }
///
/// fn run() -> Result<Cli, &'static str> {
///     let cli = Cli::parse();
///     // This will run the exec
///     // that matches the enum value
///     match cli.commands.exec() {
///         Ok(_) => Ok(cli),
///         Err(msg) => Err(msg),
///     }
///  }
/// ```
///
#[proc_macro_attribute]
#[proc_macro_error]
pub fn executable_cmd(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);
    impl_executable_cmd(&input)
}

/// Implements the `executable_cmd` attribute macro
fn impl_executable_cmd(input: &Item) -> TokenStream {
    let name;
    let impl_matches = match &input {
        Item::Enum(e) => {
            // TODO: Check that the enum derives from Subcommand
            name = &e.ident;
            let recurse = e.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                let attributes = variant.attrs.iter().filter(|attr| {
                    let cfg_attr = attr.path.segments.iter().find(|element| {
                        element.ident == "cfg"
                    });
                    cfg_attr.is_some()
                });
                quote! {
                    #(#attributes)*
                    #name::#variant_name(cmd) => cmd.exec()
                }
            });
            recurse
        }
        _ => abort_call_site!("`executable_cmd` attribute can only be used in enums"),
    };
    let gen = quote! {
        #input
        impl ExecutableCommand for #name {
            fn exec(&self) -> eyre::Result<()> {
                match &self {
                    #(#impl_matches),*
                }
            }
        }
    };
    gen.into()
}
