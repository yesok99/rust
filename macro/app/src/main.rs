use my_derive::{Hello, add, my_macro, hook};

// 1.派生宏
#[derive(Hello)]
#[hello(prefix = "Hi")]
struct User {
    id: u64,
    name: String,
}

// 2.属性宏
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
    // 1.派生宏
    User::hello();

    // 2.函数宏
    let a = add!(1, 2);
    let b = add!(10 * 2, 3 + 4);

    println!("{}, {}", a, b);

    // 3.属性宏
    let result = business_logic(21);
    println!("最终结果: {}", result);
    test();
}
