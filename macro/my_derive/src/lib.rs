use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Expr, Token};
use syn::punctuated::Punctuated;
use syn::{
    Lit, Meta,ItemFn
};


// 1. ==== 派生宏 结构体 枚举等 ====
/* 
#[proc_macro_derive(Hello)]
pub fn derive_hello(input: TokenStream) -> TokenStream {
    // 1️⃣ TokenStream → AST
    let ast = parse_macro_input!(input as DeriveInput);

    // 2️⃣ 拿到 struct 名字
    let name = &ast.ident;

    // 3️⃣ 生成代码
    let expanded = quote! {
        impl #name {
            pub fn hello() {
                println!("Hello from {}", stringify!(#name));
            }
        }
    };

    // 4️⃣ TokenStream 返回给编译器
    expanded.into()
}
    */

#[proc_macro_derive(Hello, attributes(hello))]
pub fn derive_hello(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let mut prefix = "Hello".to_string();

    for attr in &ast.attrs {
        if attr.path().is_ident("hello") {
            let nested: Result<syn::punctuated::Punctuated<Meta, Token![,]>, _> =
                attr.parse_args_with(syn::punctuated::Punctuated::parse_terminated);

            if let Ok(nested) = nested {
                for meta in nested {
                    if let Meta::NameValue(nv) = &meta {
                        if nv.path.is_ident("prefix") {
                            // 方法 1：从 Expr 中提取 Lit
                            if let Expr::Lit(expr_lit) = &nv.value {
                                if let Lit::Str(lit) = &expr_lit.lit {
                                    prefix = lit.value();
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    quote! {
        impl #name {
            pub fn hello() {
                println!("{} {}", #prefix, stringify!(#name));
            }
        }
    }
    .into()
}

// 2. ===== 新增：函数式宏 add!(x, y) =====
#[proc_macro]
pub fn add(input: TokenStream) -> TokenStream {
    // 解析：x, y
    let args = parse_macro_input!(input with Punctuated::<Expr, Token![,]>::parse_terminated);

    if args.len() != 2 {
        return syn::Error::new_spanned(
            args,
            "add! expects exactly two arguments",
        )
        .to_compile_error()
        .into();
    }

    let x = &args[0];
    let y = &args[1];

    quote! {
        (#x) + (#y)
    }
    .into()
}

// 3.属性宏

#[proc_macro_attribute]
pub fn 
my_macro(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let original_body = &input.block; // 获取原函数体

    let expanded = quote! {
        // 替换原函数体，而不是生成新函数
        fn #fn_name() {
            println!("[LOG] 调用函数 {}", stringify!(#fn_name));
            #original_body  // 插入原函数体
            println!("[LOG] 函数执行完毕");
        }
    };

    expanded.into()
}


#[proc_macro_attribute]
pub fn hook(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_vis = &input.vis;  // 保留原函数可见性
    let fn_generics = &input.sig.generics;
    let fn_inputs = &input.sig.inputs;
    let fn_output = &input.sig.output;
    let original_body = &input.block;

    // 生成钩子函数
    let expanded = quote! {
        #fn_vis fn #fn_name #fn_generics(#fn_inputs) #fn_output {
            // 前置钩子
            println!("[HOOK] 前置钩子: 函数 {} 被调用", stringify!(#fn_name));

            // 执行原函数体
            let result = (move || #original_body)();

            // 后置钩子
            println!("[HOOK] 后置钩子: 函数 {} 执行完成", stringify!(#fn_name));
            result
        }
    };

    expanded.into()
}