use anchor_lang::prelude::*;

declare_id!("F5qfUhqkqEovEkQnxziJAVkY8spYevZU6Jd8VFCujYbJ");

#[program]
pub mod solana_quest_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
