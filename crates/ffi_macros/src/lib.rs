#![crate_type = "proc-macro"]

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Field, Fields, ItemFn, ItemStruct, Type, TypePath};

#[proc_macro_attribute]
pub fn ffi_func(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let js_name = &input.sig.ident.to_string().to_case(Case::Camel);
    let vis = &input.vis;
    let attrs = &input.attrs;
    let sig = &input.sig;
    let body = &input.block;

    let output = quote! {
        #(#attrs)*
        #[cfg_attr(feature = "ffi_wasm", wasm_bindgen(js_name = #js_name))]
        #[cfg_attr(feature = "ffi_uniffi", uniffi::export)]
        #vis #sig #body
    };

    output.into()
}

#[proc_macro_attribute]
pub fn ffi_record(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemStruct);

    if let Fields::Named(ref mut fields) = input.fields {
        for field in &mut fields.named {
            if is_option_type(field) {
                add_option_field_attributes(field);
            }
        }
    }

    let struct_attrs = quote! {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
        #[cfg_attr(feature = "ffi_wasm", derive(Tsify))]
        #[cfg_attr(
            feature = "ffi_wasm",
            tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
        )]
        #[cfg_attr(feature = "ffi_wasm", serde(rename_all = "camelCase"))]
        #[cfg_attr(feature = "ffi_uniffi", derive(uniffi::Record))]
    };

    // Combine original attributes with new ones
    let combined = quote! {
        #struct_attrs
        #input
    };

    combined.into()
}

fn is_option_type(field: &Field) -> bool {
    if let Type::Path(TypePath { path, .. }) = &field.ty {
        if let Some(segment) = path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

fn add_option_field_attributes(field: &mut Field) {
    let wasm_attr: syn::Attribute =
        syn::parse_quote!(#[cfg_attr(feature = "ffi_wasm", tsify(optional))]);
    field.attrs.push(wasm_attr);

    let field_name = field.ident.to_token_stream().to_string();

    if field_name != "genesis_id" && field_name != "genesis_hash" {
        let uniffi_attr: syn::Attribute =
            syn::parse_quote!(#[cfg_attr(feature = "ffi_uniffi", uniffi(default = None))]);
        field.attrs.push(uniffi_attr);
    }
}
