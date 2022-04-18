pub fn process_instruction(
    program_id: &Pubkey, 
    accounts: &[AccountInfo], 
    _instruction_data: &[u8], 
          ) -> ProgramResult {

 msg!("Hello John!");

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        msg!("Alicia would be back on Friday!");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut messaging_account = MessagingAccount::try_from_slice(&account.data.borrow())?;
    messaging_account.counter += 1;
    messaging_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Messaging {} time(s)", messaging_account.counter);

    Ok(())
}
