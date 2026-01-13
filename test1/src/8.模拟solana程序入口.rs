// 1. 定义必要的类型（简化版 Solana 类型）
type Pubkey = [u8; 32];
type AccountInfo = Vec<u8>; // 简化为字节数组
type ProgramResult = Result<(), ProgramError>;

#[derive(Debug)]
enum ProgramError {
    InvalidInstruction,
}

// 2. 定义宏（模拟 solana_program::entrypoint!）
macro_rules! entrypoint {
    ($func:ident) => {
        // 生成符合 C ABI 的入口函数
        #[no_mangle]
        unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
            // 反序列化输入数据            let (program_id, accounts, instruction_data) = deserialize(input);
            // 调用用户定义的函数
            let program_id = [0u8; 32];
            let accounts = vec![vec![1, 2, 3], vec![4, 5, 6]];
            let instruction_data = vec![7, 8, 9];
            match $func(&program_id, &accounts, &instruction_data) {
                Ok(()) => 0, // 成功返回 0
                Err(e) => error_to_u64(e), // 错误转为错误码
            }
        }
    };
}

// 3. 模拟反序列化函数unsafe fn deserialize(input: *mut u8) -> (Pubkey, Vec<AccountInfo>, Vec<u8>) {
    // 实际 Solana 会解析复杂结构，这里简化为固定值
    // let program_id = [0u8; 32];
    // let accounts = vec![vec![1, 2, 3], vec![4, 5, 6]];
    // let instruction_data = vec![7, 8, 9];
    // (program_id, accounts, instruction_data)


// 4. 错误转错误码
fn error_to_u64(e: ProgramError) -> u64 {
    match e {
        ProgramError::InvalidInstruction => 1,
    }
}

// 5. 用户定义的函数（模拟 process_instruction）
fn a(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    println!("Program ID: {:?}", program_id);
    println!("Accounts: {:?}", accounts);
    println!("Instruction Data: {:?}", instruction_data);
    Ok(())
}

// 6. 使用宏注册入口点
entrypoint!(a);

// 7. 测试入口点（模拟 Solana 运行时调用）
fn main() {
    let mut dummy_input = [0u8; 100]; // 模拟 input 数据
    let result = unsafe { entrypoint(dummy_input.as_mut_ptr()) };
    println!("Program exit code: {}", result);
}