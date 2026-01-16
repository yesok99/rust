fn main() {
    // 示例1：引用切片
    let tt1 = &[&[1, 2, 3, 4]];
    println!("tt1 类型: {:?}", std::any::type_name::<typeof(tt1)>());
    // 输出: &[&[i32; 4]]
    
    // 示例2：数组的数组
    let tt2 = [[1, 2, 3, 4]];
    println!("tt2 类型: {:?}", std::any::type_name::<typeof(tt2)>());
    // 输出: [[i32; 4]; 1]
    
    // 查看大小
    println!("tt1 大小: {} bytes", std::mem::size_of_val(tt1));
    println!("tt2 大小: {} bytes", std::mem::size_of_val(tt2));
}