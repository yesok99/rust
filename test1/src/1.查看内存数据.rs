fn main() {
    let arr = vec![9, 2, 3, 4, 15];
    
    println!("=== 内存布局验证 ===");
    
    // 1. 栈上 Vec 结构体的地址
    println!("Vec 在栈上的地址: {:p}", &arr);
    
    // 2. 堆上数据的地址
    println!("数据在堆上的地址: {:p}", arr.as_ptr());
    
    // 3. 查看 Vec 的内存
    unsafe {
        let vec_addr = &arr as *const _ as *const u8;
        println!("\nVec 的内存内容（栈上）:");
        for i in 0..24 {  // Vec 大小为 24 字节
            if i % 8 == 0 {
                print!("\n[字节 {:2}-{:2}]: ", i, i+7);
            }
            print!("{:02x} ", *vec_addr.add(i));
        }
        println!();
        
        // 4. 查看堆上数据
        println!("\n堆上数据内容:");
        let data_ptr = arr.as_ptr() as *const u8;
        for i in 0..20 {  // 5 个 i32 = 20 字节
            if i % 4 == 0 {
                let index = i / 4;
                print!("\narr[{}]: ", index);
            }
            print!("{:02x} ", *data_ptr.add(i));
        }
        println!();
    }
}