// 宏的定义
macro_rules! create_fn {
    ($name:ident, $body:expr) => {
        fn $name() -> i32 {
            $body
        }
    };
}

macro_rules! create_struct {
    ($name:ident{$($field_name:ident: $field_type:ty),* $(,)?}) => {
        #[derive(Debug)]
        struct $name {
            $($field_name: $field_type),*
        }

        impl $name {
            fn new($($field_name: $field_type),*) -> Self {
                Self {
                    $($field_name),*
                }
            }
        }
    };
}

#[allow(dead_code)]
fn main() {
    // 使用宏创建函数
    create_fn!(one, 1);
    create_fn!(ten, 10);

    println!("{}", one());   // 1
    println!("{}", ten());   // 10

    // 使用宏创建结构体
    create_struct!(Student{name: String, age: u8});
    
    let student = Student::new(String::from("lucky"), 18);

    println!("{:#?}", student);

    // 下面这两行，就不会提示 name和age不读取
    // 方案一：
    // println!("姓名: {}", student.name);  // ✅ 读取 name
    // println!("年龄: {}", student.age);   // ✅ 读取 age

    // 方案二：
    /*
        #[allow(dead_code)]  // 允许死代码
        struct Student {
            name: String,
            age: u8,
        }
        // 或者
        #[allow(dead_code)]
        fn main() {
            // 整个函数允许死代码
        }
    
     */

    //方案3：使用下划线前缀
    /*
        struct Student {
            _name: String,  // 下划线表示"故意未使用"
            _age: u8,
        }
    */
 

}