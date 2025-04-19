use anchor_lang::prelude::*;

declare_id!("HNiA4HhC5VJVjW3aiqDnawQmWskSaG37LjdYBrdqqqkt");

#[program]

pub mod day1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
