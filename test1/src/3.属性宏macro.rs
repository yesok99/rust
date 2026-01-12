use my_macro::my_macro; 
use my_macro::hook; 

#[my_macro]
fn test() {
    println!("Hello from macro!");
}

#[hook]
fn business_logic(x: i32) -> i32 {

    println!("核心业务逻辑: {}", x);
    x * 2
}


fn main() {
    let result = business_logic(21);
    println!("最终结果: {}", result);
    test();
    business_logic(21);
}

/*
注册获取token，然后登陆, 邮箱验证
//creates.io token:****
//cargo login token
//cargo publish --manifest-path ./my_macro/Cargo.toml
//cd my_macro
//cargo publish

1. 补全 Cargo.toml 元数据（推荐）
[package]
name = "my_macro"
version = "0.1.0"
edition = "2021"
license = "MIT"  # 必须选择开源许可证（如MIT/Apache-2.0）
description = "A procedural macro for..."  # 简短描述
repository = "https://github.com/yourname/my_macro"  # 选填

2. 提交Git变更（强制要求）
git add .
git commit -m "Prepare for publishing"

3. 重新发布
cargo publish

完整发布流程示例

# 1. 修改Cargo.toml补全元数据
git add Cargo.toml
git commit -m "Add metadata"

# 2. 发布到crates.io
cargo publish

# 3. 后续更新
# 修改代码 → 递增version → git commit → cargo publish

如果已经发布失败1. 撤销本地修改

git reset --hard HEAD
git clean -fd

重新按正确流程操作
*/