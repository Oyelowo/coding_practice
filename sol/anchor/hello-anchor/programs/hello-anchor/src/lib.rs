use anchor_lang::prelude::*;

// declare an id for the program
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// write business logic here
#[program]
pub mod hello_anchor {
    use super::*;

    pub fn set_data(ctx: Context<SetData>, data: MyAccount) -> Result<()> {
        // if ctx.accounts.my_account.amount > 0 {
        //     ctx.accounts.my_account.data = data;
        // }
        // if data.data >= 100 {
        //     return err!(MyError::DataTooLarge);
        // }
        require!(data.data < 100, MyError::DataTooLarge);
        ctx.accounts.my_account.set_inner(data);

        Ok(())
    }
}

#[error_code]
pub enum MyError {
    #[msg("MyAccount may only hold data below 100")]
    DataTooLarge,
}

// #[account]
// #[derive(Default)]
// pub struct MyAccount {
//     amount: u64,
//     data: u64,
// }

// validate incoming accounts here
#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[account]
#[derive(Default)]
pub struct MyAccount {
    pub data: u64,
    pub age: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Data {
    pub data: u64,
    pub age: u8,
}
