#![warn(dead_code, unused_variables)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

mod blueprints;
mod contexts;
mod errors;

use contexts::*;

declare_id!("7FqhXgUYkqLWCwMGv3R9tNd149oXwy9FqzS8d8HpU3W2");

#[program]
pub mod dexter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!(
            "created sol vault account: {:?}",
            ctx.accounts.sol_vault_account.key()
        );
        msg!(
            "sol vault account mint: {:?}",
            ctx.accounts.sol_vault_account.mint.key()
        );
        msg!(
            "created usdt vault account: {:?}",
            ctx.accounts.usdt_vault_account.key()
        );
        msg!(
            "usdt vault account mint: {:?}",
            ctx.accounts.usdt_vault_account.mint.key()
        );
        Ok(())
    }
}
