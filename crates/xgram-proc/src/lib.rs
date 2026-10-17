use proc_macro::TokenStream;
use quote::quote;
use syn::{
    FnArg,
    Ident,
    ItemFn,
    PatType,
    ReturnType,
    parse_macro_input,
    parse_quote,
    punctuated::Punctuated,
    token::Comma
};

/// Procedural macros for wrapping command handlers.
/// ```rust,ignore
/// #[command_handler]
/// async fn foo(ctx: CommandContext) -> Foo {
///     /* function body */
/// }
///
/// // Resolves into:
///
/// fn foo(ctx: CommandContext) -> Pin<Box<dyn Future<Output = Foo> + Send>> {
///     async fn foo(ctx: CommandContext) -> Foo {
///         /* function body */
///     }
///     
///     Box::pin(foo(ctx))
/// }
/// ```
#[proc_macro_attribute]
pub fn command_handler(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = input.sig.ident;
    let vis = input.vis;
    let block = input.block;
    let attrs = input.attrs;
    let args_declaration = input.sig.inputs;
    let ret = match input.sig.output {
        ReturnType::Default => parse_quote!(()),
        ReturnType::Type(_, ty) => ty
    };

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
        #vis fn #fn_name(#args_declaration) -> ::std::pin::Pin<::std::boxed::Box<dyn ::core::future::Future<Output = #ret> + ::core::marker::Send>> {
            async fn #fn_name(#args_declaration) -> #ret #block

            ::std::boxed::Box::pin(#fn_name(#args))
        }
    }.into()
}
