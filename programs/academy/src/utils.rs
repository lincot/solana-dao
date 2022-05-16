use anchor_lang::{__private::CLOSED_ACCOUNT_DISCRIMINATOR, prelude::*};
use std::io::Write;

pub fn close(account: AccountInfo, destination: AccountInfo) -> Result<()> {
    **destination.try_borrow_mut_lamports()? += account.lamports();
    **account.try_borrow_mut_lamports()? = 0;

    let data: &mut [u8] = &mut account.try_borrow_mut_data()?;
    let mut cursor = std::io::Cursor::new(data);
    cursor
        .write_all(&CLOSED_ACCOUNT_DISCRIMINATOR)
        .map_err(|_| ErrorCode::AccountDidNotSerialize)?;

    Ok(())
}
