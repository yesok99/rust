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

    let b = Box::new(0x12345678i32);
    
    // 获取地址
    let box_addr = &b as *const Box<i32> as usize;
    let heap_addr = &*b as *const i32 as usize;
    
    println!("Box 栈地址: 0x{:x}", box_addr);
    println!("堆上数据地址: 0x{:x}", heap_addr);
    println!("堆上存储的值: 0x{:x}", *b);

}