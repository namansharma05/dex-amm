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
    use anchor_spl::token_interface::{self, Burn, MintTo};

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

        token_interface::mint_to(cpi_context_sol, 1000_000_000 * 1000)?;
        token_interface::mint_to(cpi_context_usdt, 1000_000_000 * 1000)?;
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

    pub fn swap_tokens(ctx: Context<SwapTokens>, amount_in: u64, token_name: String) -> Result<()> {
        let signer_seeds_mint_a: &[&[&[u8]]] = &[&[b"mint_a", &[ctx.bumps.mint_a]]];
        let signer_seeds_mint_b: &[&[&[u8]]] = &[&[b"mint_b", &[ctx.bumps.mint_b]]];

        let cpi_program_id = ctx.accounts.token_program.to_account_info();

        if token_name == "SOL" {
            // user is swapping SOL for USDT
            require!(amount_in > 0, errors::DexError::InvalidAmount);
            require!(
                ctx.accounts.sol_vault_account.amount >= amount_in,
                errors::DexError::InvalidAmount
            );
            // x * y = k
            let new_sol_vault_amount = ctx.accounts.sol_vault_account.amount + amount_in;
            let new_usdt_vault_amount = (ctx.accounts.sol_vault_account.amount
                * ctx.accounts.usdt_vault_account.amount)
                / new_sol_vault_amount;

            let amount_out = ctx.accounts.usdt_vault_account.amount - new_usdt_vault_amount;

            // mint amount_in SOL to the pool
            let cpi_accounts_sol = MintTo {
                mint: ctx.accounts.mint_a.to_account_info(),
                to: ctx.accounts.sol_vault_account.to_account_info(),
                authority: ctx.accounts.mint_a.to_account_info(),
            };

            let cpi_context_sol = CpiContext::new(cpi_program_id.clone(), cpi_accounts_sol)
                .with_signer(signer_seeds_mint_a);

            token_interface::mint_to(cpi_context_sol, amount_in)?;

            // burn amount_out USDT from the pool
            let cpi_accounts_usdt = Burn {
                mint: ctx.accounts.mint_b.to_account_info(),
                from: ctx.accounts.usdt_vault_account.to_account_info(),
                authority: ctx.accounts.mint_b.to_account_info(),
            };

            let cpi_context_usdt = CpiContext::new(cpi_program_id.clone(), cpi_accounts_usdt)
                .with_signer(signer_seeds_mint_b);

            token_interface::burn(cpi_context_usdt, amount_out)?;

            msg!("amount_in_sol minted in the pool: {}", amount_in);
            msg!("amount_out_usdt burned from the pool: {}", amount_out);

            // mint amount_out USDT to the user
            let cpi_accounts_usdt_mint_to_user = MintTo {
                mint: ctx.accounts.mint_b.to_account_info(),
                to: ctx.accounts.user_usdt_token_account.to_account_info(),
                authority: ctx.accounts.mint_b.to_account_info(),
            };

            let cpi_context_usdt_mint_to =
                CpiContext::new(cpi_program_id.clone(), cpi_accounts_usdt_mint_to_user)
                    .with_signer(signer_seeds_mint_b);

            token_interface::mint_to(cpi_context_usdt_mint_to, amount_out)?;

            // burn amount_in SOL from the user
            let cpi_accounts_sol_burn = Burn {
                mint: ctx.accounts.mint_a.to_account_info(),
                from: ctx.accounts.user_sol_token_account.to_account_info(),
                authority: ctx.accounts.user_sol_token_account.to_account_info(),
            };

            let cpi_context_sol_burn =
                CpiContext::new(cpi_program_id.clone(), cpi_accounts_sol_burn)
                    .with_signer(signer_seeds_mint_a);

            token_interface::burn(cpi_context_sol_burn, amount_in)?;
        } else if token_name == "USDT" {
            // user is swapping USDT for SOL
            require!(amount_in > 0, errors::DexError::InvalidAmount);
            require!(
                ctx.accounts.usdt_vault_account.amount >= amount_in,
                errors::DexError::InvalidAmount
            );
            // x * y = k
            let new_usdt_vault_amount = ctx.accounts.usdt_vault_account.amount + amount_in;
            let new_sol_vault_amount = (ctx.accounts.sol_vault_account.amount
                * ctx.accounts.usdt_vault_account.amount)
                / new_usdt_vault_amount;

            let amount_out = ctx.accounts.sol_vault_account.amount - new_sol_vault_amount;

            // mint amount_in USDT to the pool
            let cpi_accounts_usdt_mint_to_pool = MintTo {
                mint: ctx.accounts.mint_b.to_account_info(),
                to: ctx.accounts.usdt_vault_account.to_account_info(),
                authority: ctx.accounts.mint_b.to_account_info(),
            };

            let cpi_context_usdt_mint_to_pool =
                CpiContext::new(cpi_program_id.clone(), cpi_accounts_usdt_mint_to_pool)
                    .with_signer(signer_seeds_mint_b);

            token_interface::mint_to(cpi_context_usdt_mint_to_pool, amount_in)?;

            // burn amount_out SOL from the pool
            let cpi_accounts_sol = Burn {
                mint: ctx.accounts.mint_a.to_account_info(),
                from: ctx.accounts.sol_vault_account.to_account_info(),
                authority: ctx.accounts.mint_a.to_account_info(),
            };

            let cpi_context_sol = CpiContext::new(cpi_program_id.clone(), cpi_accounts_sol)
                .with_signer(signer_seeds_mint_a);

            token_interface::burn(cpi_context_sol, amount_out)?;

            msg!("amount_in_usdt minted in the pool: {}", amount_in);
            msg!("amount_out_sol burned from the pool: {}", amount_out);

            // mint amount_out SOL to the user
            let cpi_accounts_sol_mint_to_user = MintTo {
                mint: ctx.accounts.mint_a.to_account_info(),
                to: ctx.accounts.user_sol_token_account.to_account_info(),
                authority: ctx.accounts.mint_a.to_account_info(),
            };

            let cpi_context_sol_mint_to_user =
                CpiContext::new(cpi_program_id.clone(), cpi_accounts_sol_mint_to_user)
                    .with_signer(signer_seeds_mint_a);

            token_interface::mint_to(cpi_context_sol_mint_to_user, amount_out)?;

            // burn amount_in USDT from the user
            let cpi_accounts_usdt_burn = Burn {
                mint: ctx.accounts.mint_b.to_account_info(),
                from: ctx.accounts.user_usdt_token_account.to_account_info(),
                authority: ctx.accounts.user_usdt_token_account.to_account_info(),
            };

            let cpi_context_usdt_burn =
                CpiContext::new(cpi_program_id.clone(), cpi_accounts_usdt_burn)
                    .with_signer(signer_seeds_mint_b);

            token_interface::burn(cpi_context_usdt_burn, amount_in)?;
        }
        Ok(())
    }

    pub fn mint_sol(ctx: Context<MintSol>, amount: u64) -> Result<()> {
        let signer_seeds_mint_a: &[&[&[u8]]] = &[&[b"mint_a", &[ctx.bumps.mint_a]]];
        let cpi_program_id = ctx.accounts.token_program.to_account_info();
        let cpi_accounts_sol = MintTo {
            mint: ctx.accounts.mint_a.to_account_info(),
            to: ctx.accounts.user_sol_token_account.to_account_info(),
            authority: ctx.accounts.mint_a.to_account_info(),
        };
        let cpi_context_sol = CpiContext::new(cpi_program_id.clone(), cpi_accounts_sol)
            .with_signer(signer_seeds_mint_a);
        token_interface::mint_to(cpi_context_sol, amount)?;
        Ok(())
    }

    pub fn mint_usdt(ctx: Context<MintUsdt>, amount: u64) -> Result<()> {
        let signer_seeds_mint_b: &[&[&[u8]]] = &[&[b"mint_b", &[ctx.bumps.mint_b]]];
        let cpi_program_id = ctx.accounts.token_program.to_account_info();
        let cpi_accounts_usdt = MintTo {
            mint: ctx.accounts.mint_b.to_account_info(),
            to: ctx.accounts.user_usdt_token_account.to_account_info(),
            authority: ctx.accounts.mint_b.to_account_info(),
        };
        let cpi_context_usdt = CpiContext::new(cpi_program_id.clone(), cpi_accounts_usdt)
            .with_signer(signer_seeds_mint_b);
        token_interface::mint_to(cpi_context_usdt, amount)?;
        Ok(())
    }
}
