use anchor_lang::prelude::*;

declare_id!("EodQCMKrb2eBsrw8ynv75zvrzjuvtPdx3LmVXi2r12dq");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
