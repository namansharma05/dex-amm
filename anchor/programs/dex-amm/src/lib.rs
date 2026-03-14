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
    use anchor_spl::token_interface::{self, MintTo};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let signer_seeds_mint_a: &[&[&[u8]]] = &[&[b"mint_a", &[ctx.bumps.mint_a]]];
        let signer_seeds_mint_b: &[&[&[u8]]] = &[&[b"mint_b", &[ctx.bumps.mint_b]]];

        let cpi_program_id = ctx.accounts.token_program.to_account_info();

        let cpi_accounts_sol = MintTo {
            mint: ctx.accounts.mint_a.to_account_info(),
            to: ctx.accounts.sol_vault_account.to_account_info(),
            authority: ctx.accounts.mint_a.to_account_info(),
        };

        let cpi_context_sol = CpiContext::new(cpi_program_id.clone(), cpi_accounts_sol)
            .with_signer(signer_seeds_mint_a);

        let cpi_accounts_usdt = MintTo {
            mint: ctx.accounts.mint_b.to_account_info(),
            to: ctx.accounts.usdt_vault_account.to_account_info(),
            authority: ctx.accounts.mint_b.to_account_info(),
        };

        let cpi_context_usdt =
            CpiContext::new(cpi_program_id, cpi_accounts_usdt).with_signer(signer_seeds_mint_b);

        token_interface::mint_to(cpi_context_sol, 100_000_000)?;
        token_interface::mint_to(cpi_context_usdt, 100_000_000)?;
        msg!(
            "sol token minted: {}",
            ctx.accounts.sol_vault_account.amount
        );
        msg!(
            "usdt token minted: {}",
            ctx.accounts.usdt_vault_account.amount
        );
        Ok(())
    }
}
