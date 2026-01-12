# 第 1 步：创建 proc-macro crate
cargo new my_derive --lib
## 修改 my_derive/Cargo.toml
[package]
name = "my_derive"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
syn = { version = "2", features = ["full"] }
quote = "1"
proc-macro2 = "1"

### 说明：

proc-macro = true：告诉编译器这是“编译期插件”

syn：解析 Rust 语法

quote：生成 Rust 代码

proc-macro2：稳定 TokenStream（间接用）

# 第 2 步：写最小 derive 宏
my_derive/src/lib.rs

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

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

# 第 3 步：创建使用宏的 crate

cargo new app

app/Cargo.toml
[dependencies]
my_derive = { path = "../my_derive" }

app/src/main.rs
use my_derive::Hello;

#[derive(Hello)]
struct User {
    id: u64,
    name: String,
}

fn main() {
    User::hello();
}

# 第 4 步：运行

在 workspace 根目录：
cargo run -p app

