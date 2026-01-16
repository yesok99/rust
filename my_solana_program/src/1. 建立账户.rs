#![allow(unexpected_cfgs)]

// use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use solana_program::sysvar::Sysvar;
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &solana_program::pubkey::Pubkey,
    accounts: &[solana_program::account_info::AccountInfo],
    data: &[u8],
) -> solana_program::entrypoint::ProgramResult {
    msg!("Hello World Rust program entrypoint");

    let accounts_iter = &mut accounts.iter();
    let account_user = solana_program::account_info::next_account_info(accounts_iter)?;
    let account_data = solana_program::account_info::next_account_info(accounts_iter)?;
    let system_program = solana_program::account_info::next_account_info(accounts_iter)?; // Program system
    let _ = solana_program::account_info::next_account_info(accounts_iter)?; // Program sysvar rent

    // 创建账户
    let rent_exemption = solana_program::rent::Rent::get()?.minimum_balance(data.len());

    let seeds = &[b"pda_seed", account_user.key.as_ref()];
    let (pda, bump_seed) = solana_program::pubkey::Pubkey::find_program_address(
        // &[&account_user.key.to_bytes()],
        seeds, program_id,
    );

    msg!("PDA: {}", pda);
    msg!("bump_seed: {}", bump_seed);
    msg!("Creating PDA account: {}", account_data.key);
    msg!("Rent exemption required: {} lamports", rent_exemption);

    let lamports = **account_data.try_borrow_lamports()?;

    if lamports == 0 {
        // if **account_data.try_borrow_lamports().unwrap() == 0 {
        solana_program::program::invoke_signed(
            &solana_program::system_instruction::create_account(
                account_user.key,
                account_data.key,
                rent_exemption,
                data.len() as u64,
                program_id,
            ),
            accounts,
            &[&[b"pda_seed", account_user.key.as_ref(), &[bump_seed]]],
        )?;
        // 写入数据
        // account_data.data.borrow_mut().copy_from_slice(data);

        msg!("账户构建成功");
        return Ok(());
    } else {
        msg!(
            "PDA账户已经存在，数据账户余额{},删除账户",
            lamports
        );
        // **account_user.lamports.borrow_mut() = account_user.lamports() + account_data.lamports();
        // **account_data.lamports.borrow_mut() = 0;
        // close(&account_data, &account_user);
    }

    // match data[0] {
    //     0 => close(&account_data, &account_user)?, // 关闭
    //     1 => write_data(
    //         &account_data,
    //         &account_user,
    //         &accounts,
    //         &data,
    //         rent_exemption,
    //     )?,
    //     _ => Err(ProgramError::InvalidInstructionData),
    // };

    if data[0] == 0 {
        close(&account_data, &account_user);
    } else {
        write_data(
            &account_data,
            &account_user,
            &accounts,
            &data,
            rent_exemption,
        );
    }
    msg!("完成✅");
    Ok(())
}

fn close<'a>(account_data: &'a AccountInfo, account_user: &'a AccountInfo) -> ProgramResult {
    **account_user.lamports.borrow_mut() = account_user.lamports() + account_data.lamports();
    **account_data.lamports.borrow_mut() = 0;

    Ok(())
}

fn write_data<'a>(
    account_data: &'a AccountInfo,
    account_user: &'a AccountInfo,
    accounts: &'a [AccountInfo],
    data: &[u8],
    rent_exemption: u64,
) -> ProgramResult {
    // 补足资金
    // Fund the data account to let it rent exemption.
    // let rent_exemption = solana_program::rent::Rent::get()?.minimum_balance(data.len());

    if rent_exemption > account_data.lamports() {
        solana_program::program::invoke(
            &solana_program::system_instruction::transfer(
                account_user.key,
                account_data.key,
                rent_exemption - account_data.lamports(),
            ),
            accounts,
        )?;
    }

    // Withdraw excess funds and return them to users. Since the funds in the pda account belong to the program, we do
    // not need to use instructions to transfer them here.
    if rent_exemption < account_data.lamports() {
        **account_user.lamports.borrow_mut() =
            account_user.lamports() + account_data.lamports() - rent_exemption;
        **account_data.lamports.borrow_mut() = rent_exemption;
    }
    // Realloc space.
    account_data.realloc(data.len(), false)?;
    // Overwrite old data with new data.
    account_data.data.borrow_mut().copy_from_slice(data);

    let payload = &data[1..];
    // 方法 1：直接打印整个 &str
    match core::str::from_utf8(payload) {
        Ok(s) => msg!("写入内容: {}", s),
        Err(_) => msg!("非法 UTF-8 字节"),
    }

    // 方法 2：想更保险，可逐字节 hex 打印
    msg!("原始字节: {:02X?}", payload);

    Ok(())
}
