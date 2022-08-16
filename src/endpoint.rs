use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::VecDeque;
use syn::{AttributeArgs, ImplItemMethod, Stmt, TypePath};

macro_rules! unwrap_darling {
    ($condition:expr) => {
        match $condition {
            Ok(v) => v,
            Err(e) => return TokenStream::from(e.write_errors()).into(),
        }
    };
}

#[derive(Default, FromMeta)]
struct EndpointArgs {
    #[darling(default)]
    params: Option<TypePath>,
    #[darling(default = "default_name")]
    name: String,
}

fn default_name() -> String {
    "endpoint".to_string()
}

pub fn parse_attr(args: AttributeArgs, item: BodyItem) -> proc_macro2::TokenStream {
    embed(quote_fn_endpoint(args), item)
}

pub enum BodyItem {
    ItemFn(syn::ItemFn),
    ItemImpl(syn::ItemImpl),
}

fn quote_fn_endpoint(args: AttributeArgs) -> proc_macro2::TokenStream {
    let endpoint = unwrap_darling!(parse_endpoint(&args));
    let path_params = unwrap_darling!(extract_path_params(&endpoint));
    let path_params = path_params
        .iter()
        .map(|d| syn::Ident::from_string(d).unwrap());
    let path_params2 = path_params.clone();
    let args = unwrap_darling!(EndpointArgs::from_list(&args));
    let fn_name = syn::Ident::from_string(&args.name).unwrap();

    if let Some(params) = args.params {
        let params_ty = params.path.get_ident().unwrap();
        quote! {
            fn #fn_name(params: &#params_ty) -> String {
                #(
                    let #path_params = &params.#path_params;
                )*
                format!(#endpoint, #(#path_params2 = #path_params2),*)
            }
        }
    } else {
        quote! {
            fn #fn_name() -> String {
                format!(#endpoint)
            }
        }
    }
}

fn embed(fn_endpoint: proc_macro2::TokenStream, item: BodyItem) -> proc_macro2::TokenStream {
    match item {
        BodyItem::ItemFn(mut item) => {
            // e.g.
            // fn something() {
            //     fn endpoint() { ... }
            // }
            let fn_endpoint = syn::parse::<Stmt>(fn_endpoint.into()).unwrap();
            item.block.stmts.insert(0, fn_endpoint);
            quote!(#item)
        }
        BodyItem::ItemImpl(mut item) => {
            // e.g)
            // impl Something {
            //     fn endpoint() { ... }
            // }
            let fn_endpoint = syn::parse::<ImplItemMethod>(fn_endpoint.into()).unwrap();
            item.items.push(syn::ImplItem::Method(fn_endpoint));
            quote!(#item)
        }
    }
}

fn parse_endpoint(args: &AttributeArgs) -> darling::Result<String> {
    let endpoint = args
        .first()
        .ok_or_else(|| darling::Error::missing_field("endpoint"))
        .map(String::from_nested_meta)??;
    Ok(endpoint)
}

fn extract_path_params(endpoint: &str) -> darling::Result<Vec<String>> {
    let mut result = Vec::new();
    let mut current = VecDeque::new();
    let mut is_dyn = false;
    for char in endpoint.chars() {
        match char {
            '{' => {
                is_dyn = true;
            }
            '}' => {
                is_dyn = false;
                result.push(current.drain(0..).collect());
            }
            _ if is_dyn => {
                current.push_back(char);
            }
            _ => {}
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_endpoint() {
        assert_eq!(
            extract_path_params("/static").unwrap(),
            Vec::<String>::new()
        );
        assert_eq!(extract_path_params("/static/{id}").unwrap(), vec!["id"]);
        assert_eq!(
            extract_path_params("/static/{id}/{second}").unwrap(),
            vec!["id", "second"]
        );
    }
}
