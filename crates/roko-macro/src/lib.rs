use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Expr, ItemFn};

#[proc_macro_attribute]
#[allow(clippy::redundant_clone)]
pub fn cmd(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut parsed: ItemFn = syn::parse(item).unwrap();

    if parsed.sig.asyncness.is_none() {
        panic!("cmd must be async");
    }

    let name = parsed.sig.ident;

    parsed.sig.ident = syn::Ident::new(&format!("{}_future", name), name.span());

    let gen = parsed.sig.generics.clone();
    let args = parsed.sig.inputs.clone();

    let new_name = parsed.sig.ident.clone();

    let args_call = args.clone();

    let args_call = args_call
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Typed(pat) => {
                let pat = pat.pat.clone();
                match &*pat {
                    syn::Pat::Ident(name) => name.ident.clone(),
                    _ => todo!("pattern matching"),
                }
            }
            _ => todo!(),
        })
        .collect::<Vec<_>>();

    quote! {
        pub fn #name #gen (#args) -> Box<dyn std::future::Future<Output = Option<Msg>> + std::marker::Unpin> {
            #parsed
            Box::new(Box::pin(#new_name(
                #(#args_call),*
            )))
        }
    }.into()
}

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

fn get_model_from_attributes(attrs: &[syn_rsx::Node]) -> Option<proc_macro2::TokenStream> {
    attrs.iter().find_map(|attr| {
        if let syn_rsx::Node::Attribute(attr) = attr {
            get_model_attribute(attr)
        } else {
            None
        }
    })
}

fn get_model_attribute(attrs: &syn_rsx::NodeAttribute) -> Option<proc_macro2::TokenStream> {
    if attrs.key.to_string() == "model" {
        let value = attrs.value.as_ref().unwrap();

        let value: syn::Expr = syn::parse(value.as_ref().to_token_stream().into()).unwrap();

        let result = match value {
            Expr::Block(block) => {
                let stmt = &block.block.stmts[0];
                quote! { #stmt }
            }
            _ => {
                quote! { #value.to_string() }
            }
        };

        Some(result)
    } else {
        None
    }
}

fn transform(node: &syn_rsx::Node) -> proc_macro2::TokenStream {
    match node {
        syn_rsx::Node::Element(el) => {
            let tag = el.name.to_token_stream();

            let attrs = el.attributes.iter().map(transform);
            let children = el.children.iter().map(transform);

            let model = get_model_from_attributes(&el.attributes);

            if let Some(model) = model {
                quote! {
                    #tag(#model, vec![#(#attrs),*], vec![#(#children),*])
                }
            } else {
                quote! {
                    #tag(vec![#(#attrs),*], vec![#(#children),*])
                }
            }
        }
        syn_rsx::Node::Attribute(attr) => {
            let name = attr.key.to_string();

            let mut needs_rc = false;
            let mut is_custom = false;
            let mut ignore = false;

            let constructor = match name.as_str() {
                "onclick" => {
                    needs_rc = true;
                    quote! {OnClick}
                }
                "onmount" => {
                    needs_rc = true;
                    quote! {OnMount}
                }
                "onunmount" => {
                    needs_rc = true;
                    quote! {OnUnmount}
                }
                "model" => {
                    ignore = true;
                    quote! {Model}
                }
                _ => {
                    is_custom = true;
                    quote! {Custom}
                }
            };

            if let Some(value) = &attr.value {
                // Idk how to pattern match on syn::expr::Expr so I did that
                let value: syn::Expr = syn::parse(value.as_ref().to_token_stream().into()).unwrap();

                let result = match value {
                    Expr::Block(block) => {
                        let stmt = &block.block.stmts[0];
                        quote! { #stmt }
                    }
                    _ => {
                        quote! { #value.to_string() }
                    }
                };

                if needs_rc {
                    quote! { roko_html::Attribute::#constructor(std::sync::Arc::new(#result)) }
                } else if is_custom {
                    quote! { roko_html::Attribute::Custom(#name.to_string(), #result.to_string()) }
                } else if ignore {
                    quote! {}
                } else {
                    quote! { roko_html::Attribute::#constructor(#result) }
                }
            } else {
                quote! { roko_html::Attribute::#name }
            }
        }
        syn_rsx::Node::Text(text) => {
            let text = text.value.as_ref().to_token_stream();
            quote! {
                roko_html::text(#text)
            }
        }
        syn_rsx::Node::Block(block) => {
            let block = block.value.as_ref();
            quote! { #block.into() }
        }
        _ => todo!("fragmento"),
    }
}

#[proc_macro]
pub fn html(item: TokenStream) -> TokenStream {
    let html = syn_rsx::parse(item).unwrap();

    let res = transform(&html[0]);
    quote! {#res}.into()
}
