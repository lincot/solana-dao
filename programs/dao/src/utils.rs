use anchor_lang::prelude::*;

pub fn close(account: AccountInfo, destination: AccountInfo) -> Result<()> {
    **destination.try_borrow_mut_lamports()? += account.lamports();
    **account.try_borrow_mut_lamports()? = 0;
    *account.try_borrow_mut_data()? = &mut [];
    Ok(())
}
