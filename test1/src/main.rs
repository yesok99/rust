use syn::{parse_str, ItemStruct};
use quote::quote;
fn main() {
    let code = r#"
        struct Person {
            name: String,
            age: u32,
        }
    "#;

    // 使用 syn 解析字符串为结构体 AST

    let parsed: ItemStruct = parse_str(code).unwrap();

    println!("Struct name: {}", parsed.ident); // ✅ 输出: Person
    for fields in &parsed.fields {
        println!("fields: {}", quote! { #fields });
    }
    
    for attr in &parsed.attrs {
        println!("Attribute: {}", quote! { #attr });
    }

    println!("Struct generics: {:#?}", quote!(#parsed.generics));
}