use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

macro_rules! tileitem_push {
    ($var:expr, $($name:ident: $item_type:ty),*) => {
        $(
            let parsed = ::syn::Field {
                attrs: vec![],
                vis: ::syn::Visibility::Inherited,
                mutability: ::syn::FieldMutability::None,
                ident: Some(::syn::Ident::new(stringify!($name), ::proc_macro2::Span::call_site())),
                colon_token: Some(::syn::parse_str(":").unwrap()),
                ty: ::syn::parse_str(stringify!($item_type)).unwrap()
            };
            $var.push(parsed);
        )*
    }
}

/// I just need to insert these a lot and it's a big block
#[proc_macro_attribute]
pub fn tileitem(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as DeriveInput);
    let Data::Struct(internal_data) = &mut input.data else {
        return r#"compile_error!("`#[tileitem]` can only be applied to structs")"#
            .parse()
            .unwrap();
    };
    let Fields::Named(fields) = &mut internal_data.fields else {
        return r#"compile_error!("`#[tileitem]` can only be applied to non empty/tuple structs")"#
            .parse()
            .unwrap();
    };
    let real_fields = &mut fields.named;
    tileitem_push!(
        real_fields,
        tile_id: Option<String>,
        item_id: Option<String>,
        tile_texture: Option<String>,
        item_texture: Option<String>,
        variants: Vec<::std::collections::HashMap<String, VariantInternal>>,
        tile_drops: Option<Vec<String>>,
        autotile: Option<Vec<AutotileInternal>>
    );

    TokenStream::from(quote! {
        #input
    })
}
