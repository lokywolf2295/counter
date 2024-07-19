use anchor_lang::prelude::*;

declare_id!("4tFoBRn2KreUN5wemJXiHs1pAjvhMt4cmEJwUSNKmNLB");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
