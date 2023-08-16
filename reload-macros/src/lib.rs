extern crate proc_macro;
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn hot_reload_setup(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: ItemFn = parse_macro_input!(item as ItemFn);

    let fn_name = &ast.sig.ident;

    let inner_fn_name_str = format!("hot_reloaded_inner_{fn_name}");
    let inner_fn_name = Ident::new(&inner_fn_name_str, Span::call_site());

    quote! {

        #[no_mangle]
        pub fn #inner_fn_name(app: &mut ReloadableApp) {
            #ast

            #fn_name(app);
        }

        #[allow(non_camel_case_types)]
        struct #fn_name;

        impl hot_reload::ReloadableSetup for #fn_name {
            fn setup_function_name() -> &'static str {
                #inner_fn_name_str
            }

            fn default_function(app: &mut ReloadableApp) {
                #inner_fn_name(app);
            }
        }

    }
    .into()
}

#[proc_macro_attribute]
pub fn hot_bevy_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: ItemFn = parse_macro_input!(item as ItemFn);

    let fn_name: &proc_macro2::Ident = &ast.sig.ident;

    quote! {
        pub fn #fn_name(options: hot_reload::HotReloadOptions) {
            hot_reload::run_reloadabe_app(options);
        }

        #[no_mangle]
        pub fn hot_reload_internal_main(plugin: hot_reload::HotReloadPlugin) {
            #ast

            #fn_name(plugin);
        }
    }
    .into()
}

/*
#[proc_macro]
#[allow(unused_variables)]
pub fn hot_bevy(input: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(input as Expr);
    let (function, options) = match &attrs {
        Expr::Path(path) => (path, None),
        Expr::Tuple(tuple) => {
            let function = tuple
                .elems
                .first()
                .expect("Must have first attribute of tuple in hot_bevy macro");
            let Expr::Path(function) = function else {
                panic!(
                    "First attribute in hot_bevy macro tuple must be a path to the main function"
                );
            };
            let options = tuple.elems.last();
            (function, options)
        }
        _ => panic!("Invalid input to hot_bevy macro"),
    };
    #[cfg(not(feature = "bypass"))]
    {
        let options = match options {
            Some(v) => quote!(#v),
            None => {
                let n: Expr = parse_str("None").expect("This should work");
                quote!(#n)
            }
        };

        quote! {
            println!("Starting hot reload shell");
            hot_reload::run_reloadabe_app(#options);
        }
        .into()
    }
    #[cfg(feature = "bypass")]
    {
        quote! {
            println!("bypassing hot reload");
            let mut app = #function();
            app.run()
        }
        .into()
    }
}
*/
