use crate::{instructions::*};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke},
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::create_account,
    sysvar::Sysvar,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = CreateNotPdaInstruction::unpack(instruction_data)?;

        match instruction {
            CreateNotPdaInstruction::CreateNotPDA {} => {
                Self::process_create_not_pda(accounts, program_id)?;
            }
        }

        Ok(())
    }

    pub fn process_create_not_pda(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
        msg!("process_create_not_pda ix...");

        let account_info_iter = &mut accounts.iter();

        let first_not_pda = next_account_info(account_info_iter)?;
        let second_not_pda = next_account_info(account_info_iter)?;
        let first_creation_payer = next_account_info(account_info_iter)?;
        let second_creation_payer = next_account_info(account_info_iter)?;
        let _system_program = next_account_info(account_info_iter)?;

        let rent = Rent::get()?;
        let rent_minimum_balance = rent.minimum_balance(8);
        
        let space = 8;

        msg!("Creating first not pda...");
        invoke(
            &create_account(
                &first_creation_payer.key,
                &first_not_pda.key,
                rent_minimum_balance,
                space as u64,
                program_id,
            ),
            &[first_creation_payer.clone(), first_not_pda.clone()],
        )?;
        msg!("First not pda creation succeded.");

        msg!("Creating second not pda...");
        invoke(
            &create_account(
                &second_creation_payer.key,
                &second_not_pda.key,
                rent_minimum_balance,
                space as u64,
                program_id,
            ),
            &[second_creation_payer.clone(), second_not_pda.clone()],
        )?;
        msg!("Second not pda creation succeded.");

        Ok(())
    }
}
