use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, FnArg, Ident, ItemFn, PatType};

#[proc_macro_attribute]
pub fn command_handler(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = input.sig.ident;
    let vis = input.vis;
    let block = input.block;
    let attrs = input.attrs;
    let args_declaration = input.sig.inputs;
    let mut args: Punctuated<Ident, Comma> = Punctuated::new();
    for arg in &args_declaration {
        match arg {
            FnArg::Typed(PatType { pat, .. }) => {
                let arg_name = match &**pat {
                    syn::Pat::Ident(id) => id.ident.clone(),
                    _ => panic!("expected ident argument")
                };
                args.push(arg_name);
            }
            _ => panic!("expected a typed argument")
        }
    }

    quote! {
        #(#attrs)*
        #vis fn #fn_name(#args_declaration) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
            async fn #fn_name(#args_declaration) #block

            Box::pin(#fn_name(#args))
        }
    }
    .into()
}
