use crate::*;
use solana_program::{msg, program_error::ProgramError};

#[derive(Debug)]
pub enum CreateNotPdaInstruction {
    CreateNotPDA {},
}

impl CreateNotPdaInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        msg!("Unpacking instruction...");

        let (tag, _) = input
            .split_first()
            .ok_or(errors::CreateNotPdaError::InvalidInstruction)?;

        Ok(match tag {
            0 => Self::CreateNotPDA {},
            _ => return Err(errors::CreateNotPdaError::InvalidInstruction.into()),
        })
    }
}
