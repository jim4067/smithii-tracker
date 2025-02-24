use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;
// use std::convert::TryInto;

#[inline(always)]
pub fn process_track(_accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let timestamp = u64::from_le_bytes(
        instruction_data
            .try_into()
            .map_err(|_error| ProgramError::InvalidAccountData)?,
    );

    log!("timestamp -> {}", timestamp); // ! debug

    // todo: check if PDA is valid

    Ok(())
}
