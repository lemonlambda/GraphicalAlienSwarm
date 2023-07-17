use proc_macro::TokenStream;
use regex::Regex;

/// I just need to insert these a lot and it's a big block
#[proc_macro_attribute]
pub fn tileitem(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_string = &attr.to_string();
    let attr = if attr.to_string().is_empty() {
        vec!["VariantInternal", "AutotileInternal"]
    } else {
        let re = Regex::new(r#"(,)|(, )"#).expect("Failed to create regex");
        re.split(attr_string).collect::<Vec<&str>>()
    };
    item.to_string().replace("}", r#"
        tile_id: Option<String>,
        item_id: Option<String>,
        tile_texture: Option<String>,
        item_texture: Option<String>,
        variants: Vec<::std::iter::Map<String, VariantInternal>>,
        tile_drops: Option<Vec<String>>,
        autotile: Option<Vec<AutotileInternal>>
    }"#).replace("VariantInternal", attr[0]).replace("AutotileInternal", attr[1]).parse().unwrap()
    
}