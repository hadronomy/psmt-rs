extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::quote;
use syn::{self, parse_macro_input, Item};

/// Implements `ExecutableCommand` to an **enum**.
///
/// This allows to execute a specific subcommand
/// by calling `my_subcommand_enum.exec()`.
///
/// ```ignore
/// struct TestCommand;
///
/// // For `executable_cmd` to work
/// // the trait ExecutableCommand needs to be implemented
/// // in every subcommand
/// impl TestCommand {
///     pub fn exec(&self) -> Result<(), &'static str> {
///         todo!("Execute test command");
///     }
/// }
///
/// /// Subcommands enum used by the main clap
/// /// command
/// #[derive(Subcommand, ExecutableCommand)]
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
#[proc_macro_derive(ExecutableCommand)]
#[proc_macro_error]
pub fn executable_cmd_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);
    impl_executable_cmd(&input)
}

/// Implements the `executable_cmd` attribute macro
fn impl_executable_cmd(input: &Item) -> TokenStream {
    let name;
    let impl_matches = match &input {
        Item::Enum(e) => {
            // TODO: Check that the enum derives from Subcommand
            name = &e.ident;
            e.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                quote! {
                    #name::#variant_name(cmd) => cmd.exec()
                }
            })
        }
        _ => abort_call_site!("`executable_cmd` attribute can only be used in enums"),
    };
    let gen = quote! {
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
