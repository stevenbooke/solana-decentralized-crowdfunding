use anchor_lang::prelude::*;

declare_id!("9E2TAd91tJHTvd4zfozq9hAEZf3jpdi4Guy5BxmLTg1b");

#[program]
pub mod solana_decentralized_crowdfunding {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
