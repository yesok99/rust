use crate::solana::process_instruction; //绝对路径导出
// use my_solana_program::solana::process_instruction;
// use lib::process_instruction; // 现在可以访问
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};
use solana_program::entrypoint::ProgramResult;


pub fn main() -> ProgramResult {
    
    let program_id = Pubkey::new_unique();
    let mut lamports = 100;
    let mut data = vec![0u8; 10];
    let account_pubkey = Pubkey::new_unique();
    let account = AccountInfo::new(
        &account_pubkey,
        true,
        true,
        &mut lamports,
        &mut data,
        &program_id,
        false,
        0,
    );

    let accounts = vec![account];
    let instruction_data = vec![1];

    // let instruction_data = vec![
    //     0x01, 0x00, 0x00, 0x00,   // counter: 1 (u32)
    //     0x05, 0x00, 0x00, 0x00,   // message长度: 5 (u32)
    //     0x48, 0x65, 0x6C, 0x6C, 0x6F, // "Hello" 
    //     0x01                      // is_active: true
    //     ];
    
    process_instruction(&program_id, &accounts, &instruction_data)?;
    Ok(())
}
