mod solana; 
mod serializer;
mod process; // 导出 process 模块

fn main()  {
    
    serializer::main();
    
    let result = process::main();
    println!("{:?}", result);
    match result {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {:?}", e),
    }

    // let s = "aa AA hello world";
    // let bytes = s.as_bytes();
    // println!("{:?}", bytes);    

    // let a = new ;
    // let b = a.write_all(&mut [0x02]);

}
