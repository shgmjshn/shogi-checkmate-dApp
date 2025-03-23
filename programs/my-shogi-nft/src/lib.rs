use anchor_lang::prelude::*;

declare_id!("EA7qTJVQeghgYfoFBNL9mAQNxXXvq8hrpHhsqsip1Kgg");

#[program]
pub mod my_shogi_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
