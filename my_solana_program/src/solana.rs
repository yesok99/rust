use solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint,
        entrypoint::ProgramResult,
        msg,
        pubkey::Pubkey,
    };
use borsh::{BorshDeserialize, BorshSerialize};
    #[derive(BorshDeserialize, BorshSerialize)]
    pub struct GreetingAccount {
        /// number of greetings
        pub counter: u32,       // 示例字段（根据需求调整）
        pub message: String,    // 可变长度字段
        pub is_active: bool,    // 布尔类型
    }

    entrypoint!(process_instruction);

    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Hello, Solana!");
        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;
        msg!("First account: {:?}", account.key);

        if !instruction_data.is_empty() && instruction_data[0] == 1 {
            msg!("Instruction type 1 executed!");
        }

        // let greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
        // // greeting_account.counter += 1;
        // // greeting_account.serialize(&mut *account.data.borrow_mut())?;

        // // msg!("Greeted {} time(s)!", greeting_account.counter);
        // msg!(
        //     "Loaded: counter={}, message={}", 
        //     greeting_account.counter, 
        //     greeting_account.message
        // );

        

        Ok(())
    }

