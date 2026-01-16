/*
1. 没有借用之前，borrow = 0
2. borrow后，borrow = 1
3. borrow_mut后，borrow = -1 
如果修改数据push，则堆重新分配地址。


*/
use std::cell::RefCell;
use std::rc::Rc;

// 可变借用和不可变借用不能在同一个作用域
fn borrow(){

    let data = Rc::new(RefCell::new(vec![9,8,7]));

    if !data.borrow().is_empty() {

        print!("data:{:?}\n", data.borrow());
    }

    {
        let mut b_mut  = data.borrow_mut();

        (*b_mut)[0] = 999;
        
    }

    let c = data.borrow();

    print!("{:?}",c);
}

fn main() {
    let shared_data = RefCell::new(vec![7, 9, 3, 8]);
    
    // 查看 RefCell 的栈地址
    println!("RefCell 栈地址: {:p}", &shared_data);
    
    // 查看内部 Vec 的堆地址
    let vec_ref = shared_data.borrow();
    println!("Vec 堆指针: {:p}", vec_ref.as_ptr());
    println!("Vec 长度: {}, 容量: {}", vec_ref.len(), vec_ref.capacity());
    
    // 通过指针查看堆数据
    unsafe {
        let ptr = vec_ref.as_ptr();
        println!("堆数据[0]: {}", *ptr);
        println!("堆数据[1]: {}", *ptr.add(1));
        println!("堆数据[2]: {}", *ptr.add(2));
    }

    drop(vec_ref);
    let mut vec_ref = shared_data.borrow_mut();
    vec_ref.push(10);
    println!("Vec 堆指针: {:p}", vec_ref.as_ptr());
    println!("Vec 长度: {}, 容量: {}", vec_ref.len(), vec_ref.capacity());
    
    // 通过指针查看堆数据
    unsafe {
        let ptr = vec_ref.as_ptr();
        println!("堆数据[0]: {}", *ptr);
        println!("堆数据[1]: {}", *ptr.add(1));
        println!("堆数据[2]: {}", *ptr.add(2));
    }


}