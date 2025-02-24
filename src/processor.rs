use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

#[inline(always)]
pub fn process_track(_accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let _timestamp = u64::from_le_bytes(
        instruction_data
            .try_into()
            .map_err(|_error| ProgramError::InvalidAccountData)?,
    );

    Ok(())
}
