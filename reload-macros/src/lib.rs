extern crate proc_macro;
extern crate quote;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_str, Expr, ItemFn};

#[proc_macro_attribute]
pub fn hot_bevy_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: ItemFn = parse_macro_input!(item as ItemFn);

    let fn_name: &proc_macro2::Ident = &ast.sig.ident;

    quote! {

        #ast

        #[cfg(not(feature = "bypass"))]
        #[no_mangle]
        pub fn internal_hot_reload_setup() -> Option<Box<App>> {
            let mut app = #fn_name();

            let (sn, rc) = app_grabber();

            app.set_runner(move |app| {
                sn.send(app);
            });

            app.run();

            rc.recv().map(Box::new).ok()
        }


        #[cfg(not(feature = "bypass"))]
        #[no_mangle]
        pub fn internal_trigger_update(app: &mut Box<App>, readers: &mut hot_reload::WindowAndInputEventReaders) {
            app.update();
        }
    }
    .into()
}

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
