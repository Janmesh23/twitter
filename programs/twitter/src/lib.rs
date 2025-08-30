use anchor_lang::prelude::*;

declare_id!("TyWkgqkz7VywfCCqUkkYd1oQ1ZMWNTsyXbRMTCUMkvV");



#[program]
pub mod twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
