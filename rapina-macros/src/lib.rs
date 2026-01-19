use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, LitStr, Pat, parse_macro_input};

#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    route_macro(attr, item)
}

#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    route_macro(attr, item)
}

#[proc_macro_attribute]
pub fn put(attr: TokenStream, item: TokenStream) -> TokenStream {
    route_macro(attr, item)
}

#[proc_macro_attribute]
pub fn delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    route_macro(attr, item)
}

fn route_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _path = parse_macro_input!(attr as LitStr);
    let func = parse_macro_input!(item as ItemFn);

    let func_name = &func.sig.ident;
    let func_block = &func.block;
    let func_output = &func.sig.output;
    let func_vis = &func.vis;

    let args: Vec<_> = func.sig.inputs.iter().collect();

    let expanded = if args.is_empty() {
        quote! {
            #func_vis async fn #func_name(
                _req: hyper::Request<hyper::body::Incoming>,
                _params: rapina::extract::PathParams,
            ) #func_output #func_block
        }
    } else {
        let mut extractions = Vec::new();
        let mut arg_names = Vec::new();

        for arg in &args {
            if let FnArg::Typed(pat_type) = arg {
                if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    let arg_name = &pat_ident.ident;
                    let arg_type = &pat_type.ty;

                    arg_names.push(arg_name.clone());
                    extractions.push(quote! {
                        let #arg_name = <#arg_type as rapina::extract::FromRequest>::from_request(req, &params).await.unwrap();
                    });
                }
            }
        }

        let inner_block = &func.block;

        quote! {
            #func_vis async fn #func_name(
                req: hyper::Request<hyper::body::Incoming>,
                params: rapina::extract::PathParams,
            ) #func_output {
                #(#extractions)*
                #inner_block
            }
        }
    };

    TokenStream::from(expanded)
}
