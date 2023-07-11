use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::Expr;

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

fn transform(node: &syn_rsx::Node) -> proc_macro2::TokenStream {
    match node {
        syn_rsx::Node::Element(el) => {
            let tag = el.name.to_string();
            let attrs = el.attributes.iter().map(transform);
            let children = el.children.iter().map(transform);

            quote! {
                roko_html::Html::Node(roko_html::Node{
                    tag: #tag,
                    attributes: vec![#(#attrs),*],
                    children: vec![#(#children),*],
                })
            }
        }
        syn_rsx::Node::Attribute(attr) => {
            let name = attr.key.to_string();

            let mut needs_rc = false;

            let constructor = match name.as_str() {
                "onclick" => {
                    needs_rc = true;
                    quote! {OnClick}
                }
                "class" => quote! {Class},
                "style" => quote! {Style},
                _ => panic!("unknown attribute"),
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
                } else {
                    quote! { roko_html::Attribute::#constructor(#result) }
                }
            } else {
                quote! { roko_html::Attribute::#name }
            }
        }
        syn_rsx::Node::Text(text) => {
            let text = text.value.as_ref().to_token_stream().to_string();
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
