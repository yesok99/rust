use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // === 第1步：创建底层数据 ===
    // 在堆上分配 100 字节的数据
    let mut heap_data = vec![0u8; 10];
    heap_data[0] = 9;
    heap_data[1] = 5;
    heap_data[9] = 100;
    
    println!("=== 原始数据 ===");
    println!("heap_data 地址: {:p}", heap_data.as_ptr());
    println!("heap_data 长度: {}", heap_data.len());
    
    // === 第2步：创建可变切片引用 ===
    let slice_ref: &mut [u8] = &mut heap_data[..];
    
    println!("\n=== 切片引用 ===");
    println!("slice_ref 指针: {:p}", slice_ref.as_ptr());
    println!("slice_ref 长度: {}", slice_ref.len());
    
    // === 第3步：包装成 RefCell ===
    // 注意：这里 RefCell 还没有被 Rc 包装
    let ref_cell = RefCell::new(slice_ref);
    
    println!("\n=== RefCell 内存布局 ===");
    println!("ref_cell 栈地址: {:p}", &ref_cell);
    println!("ref_cell 大小: {} 字节", std::mem::size_of_val(&ref_cell));
    
    // === 第4步：包装成 Rc ===
    // 这里发生堆分配：Rc 的控制块被分配到堆上
    let rc_refcell: Rc<RefCell<&mut [u8]>> = Rc::new(ref_cell);
    
    println!("\n=== Rc<RefCell> 创建后 ===");
    println!("rc_refcell 栈地址: {:p}", &rc_refcell);
    println!("Rc 强引用计数: {}", Rc::strong_count(&rc_refcell));
    
    // === 第5步：克隆 Rc（共享所有权）===
    let rc_clone = Rc::clone(&rc_refcell);
    
    println!("\n=== 克隆 Rc 后 ===");
    println!("rc_clone 栈地址: {:p}", &rc_clone);
    println!("Rc 强引用计数: {}", Rc::strong_count(&rc_refcell));
    
    // === 第6步：通过不同 Rc 访问和修改数据 ===
    {
        // 通过第一个 Rc 借用并修改
        let mut borrowed1 = rc_refcell.borrow_mut();
        borrowed1[0] = 255;
        println!("\n=== 通过 rc_refcell 修改 ===");
        println!("修改 borrowed1[0] = 255");
    }  // borrowed1 离开作用域，释放借用
    
    {
        // 通过克隆的 Rc 借用并查看
        let borrowed2 = rc_clone.borrow();
        println!("\n=== 通过 rc_clone 查看 ===");
        println!("borrowed2[0] = {}", borrowed2[0]);
        println!("borrowed2[9] = {}", borrowed2[9]);
    }
    
    // === 第7步：查看内存地址 ===
    println!("\n=== 内存地址总结 ===");
    
    // 获取 Rc 的堆指针（需要 unsafe）
    unsafe {
        let rc_ptr: *const _ = &*rc_refcell;
        println!("Rc 堆控制块地址: {:p}", rc_ptr);
        
        // 通过偏移访问 RefCell
        let refcell_addr = rc_ptr as *const RefCell<&mut [u8]>;
        println!("RefCell 在堆中的地址: {:p}", refcell_addr);
        
        // 获取切片指针
        let borrowed = rc_refcell.borrow();
        let slice_ptr = borrowed.as_ptr();
        println!("切片数据指针: {:p}", slice_ptr);
    }
    
    // === 第8步：验证修改 ===
    println!("\n=== 最终数据验证 ===");
    println!("heap_data[0] = {}", heap_data[0]);  // 应该为 255
    println!("heap_data[1] = {}", heap_data[1]);  // 应该为 2
    
    // === 第9步：生命周期结束时自动清理 ===
    println!("\n=== 清理过程 ===");
    // 1. rc_clone 被 drop，强引用计数减1
    // 2. rc_refcell 被 drop，强引用计数为0
    // 3. Rc 控制块被释放，触发 RefCell 的 drop
    // 4. RefCell 被 drop（但切片引用不拥有数据）
    // 5. heap_data 离开作用域，实际数据被释放
}