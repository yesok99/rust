fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    println!("创建闭包，捕获 x = {}, 地址: {:p}", x, &x);
    
    move |y| {
        let result = x + y;
        println!("闭包调用: x = {} (地址: ?), y = {} (地址: {:p}), 结果 = {}", 
                 x, y, &y, result);
        result
    }
}

fn main() {
    let add_five = make_adder(5);
    println!("5 + 3 = {}", add_five(3));
    println!("5 + 7 = {}", add_five(7));
}