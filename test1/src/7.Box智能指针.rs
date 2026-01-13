fn main() {
    let b = Box::new(5);
    
    // 1. 打印 Box 在栈上的地址（&b 的类型是 &Box<i32>）
    println!("栈地址 &b = {:p}", &b);
    
    // 2. 打印 Box 指向的堆内存地址（b 本身是 *const i32 指针）
    println!("堆地址 b = {:p}", b);
    
    // 3. 解引用堆内存内容    println!("堆内容 *b = {}", *b);
    
    // 4. 额外：通过指针直接访问（unsafe 演示，实际无需这样写）
    let raw_ptr = &*b as *const i32;
    println!("通过原始指针访问: {}", unsafe { *raw_ptr });

}