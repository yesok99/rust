
fn create_closure() -> impl Fn() {
    let msg = String::from("Hello");
    
    // 必须用 move，否则 msg 会在函数结束时被丢弃
    move || println!("{}", msg)
}

fn closure_traits() {
    let s = String::from("test");
    
    // FnOnce - 消耗所有权
    let fnonce = move || {
        let _ = s;  // 消耗 s
    };
    fnonce();  // 只能调用一次
    
    // FnMut - 可变借用
    let mut s2 = String::from("test");
    let mut fnmut = move || {
        s2.push_str(" modified");
    };
    fnmut();  // 可以多次调用
    
    // Fn - 不可变借用
    let s3 = String::from("test");
    let fn_ = move || {
        println!("{}", s3);
    };
    fn_();  // 可以多次调用
    fn_();
}

fn inspect_closure() {
    use std::mem;
    
    let s = String::from("hello");
    
    // 没有 move
    let closure1 = || println!("{}", s);
    println!("无move闭包大小: {} 字节", mem::size_of_val(&closure1));
    // 较小，只包含指针
    
    closure1();
    // 有 move
    let closure2 = move || println!("{}", s);
    println!("有move闭包大小: {} 字节", mem::size_of_val(&closure2));
    // 较大，包含整个 String
    
    
    closure2();
    
    // s 在这里仍然可用（对于 closure1 是借用，对于 closure2 是复制？）
    // 实际上：String 没有 Copy，所以 closure2 移动了 s
    // println!("s: {}", s);  // ❌ 对于 closure2 来说，s 已被移动
}

fn main() {

    // let closure = create_closure();
    // closure();  // 输出: Hello

    // closure_traits()

    inspect_closure();
}