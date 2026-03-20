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
    use anchor_spl::token_interface::{self, MintTo, Transfer};

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
        let signer_seeds_sol_vault: &[&[&[u8]]] =
            &[&[b"sol_token", &[ctx.bumps.sol_vault_account]]];
        let signer_seeds_usdt_vault: &[&[&[u8]]] =
            &[&[b"usdt_token", &[ctx.bumps.usdt_vault_account]]];

        let cpi_program_id = ctx.accounts.token_program.to_account_info();

        if token_name == "SOL" {
            // user is swapping SOL for USDT
            require!(amount_in > 0, errors::DexError::InvalidAmount);
            require!(
                ctx.accounts.sol_vault_account.amount > 0
                    && ctx.accounts.usdt_vault_account.amount > 0,
                errors::DexError::InvalidAmount
            );

            // x * y = k
            let pool_sol = ctx.accounts.sol_vault_account.amount as u128;
            let pool_usdt = ctx.accounts.usdt_vault_account.amount as u128;
            let amount_in_u128 = amount_in as u128;

            let k = pool_sol.checked_mul(pool_usdt).unwrap();
            let new_pool_sol = pool_sol.checked_add(amount_in_u128).unwrap();
            let new_pool_usdt = k.checked_div(new_pool_sol).unwrap();

            let amount_out_u128 = pool_usdt.checked_sub(new_pool_usdt).unwrap();
            let amount_out = amount_out_u128 as u64;

            require!(amount_out > 0, errors::DexError::InvalidAmount);
            require!(
                ctx.accounts.usdt_vault_account.amount >= amount_out,
                errors::DexError::InvalidAmount
            );

            // Transfer amount_in SOL from User to Pool
            let cpi_accounts_sol = Transfer {
                from: ctx.accounts.user_sol_token_account.to_account_info(),
                to: ctx.accounts.sol_vault_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            };
            let cpi_context_sol = CpiContext::new(cpi_program_id.clone(), cpi_accounts_sol);
            token_interface::transfer(cpi_context_sol, amount_in)?;

            // Transfer amount_out USDT from Pool to User
            let cpi_accounts_usdt = Transfer {
                from: ctx.accounts.usdt_vault_account.to_account_info(),
                to: ctx.accounts.user_usdt_token_account.to_account_info(),
                authority: ctx.accounts.usdt_vault_account.to_account_info(),
            };
            let cpi_context_usdt = CpiContext::new(cpi_program_id.clone(), cpi_accounts_usdt)
                .with_signer(signer_seeds_usdt_vault);
            token_interface::transfer(cpi_context_usdt, amount_out)?;

            msg!("Swapped {} SOL for {} USDT", amount_in, amount_out);
        } else if token_name == "USDT" {
            // user is swapping USDT for SOL
            require!(amount_in > 0, errors::DexError::InvalidAmount);
            require!(
                ctx.accounts.sol_vault_account.amount > 0
                    && ctx.accounts.usdt_vault_account.amount > 0,
                errors::DexError::InvalidAmount
            );

            // x * y = k
            let pool_sol = ctx.accounts.sol_vault_account.amount as u128;
            let pool_usdt = ctx.accounts.usdt_vault_account.amount as u128;
            let amount_in_u128 = amount_in as u128;

            let k = pool_sol.checked_mul(pool_usdt).unwrap();
            let new_pool_usdt = pool_usdt.checked_add(amount_in_u128).unwrap();
            let new_pool_sol = k.checked_div(new_pool_usdt).unwrap();

            let amount_out_u128 = pool_sol.checked_sub(new_pool_sol).unwrap();
            let amount_out = amount_out_u128 as u64;

            require!(amount_out > 0, errors::DexError::InvalidAmount);
            require!(
                ctx.accounts.sol_vault_account.amount >= amount_out,
                errors::DexError::InvalidAmount
            );

            // Transfer amount_in USDT from User to Pool
            let cpi_accounts_usdt = Transfer {
                from: ctx.accounts.user_usdt_token_account.to_account_info(),
                to: ctx.accounts.usdt_vault_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            };
            let cpi_context_usdt = CpiContext::new(cpi_program_id.clone(), cpi_accounts_usdt);
            token_interface::transfer(cpi_context_usdt, amount_in)?;

            // Transfer amount_out SOL from Pool to User
            let cpi_accounts_sol = Transfer {
                from: ctx.accounts.sol_vault_account.to_account_info(),
                to: ctx.accounts.user_sol_token_account.to_account_info(),
                authority: ctx.accounts.sol_vault_account.to_account_info(),
            };
            let cpi_context_sol = CpiContext::new(cpi_program_id.clone(), cpi_accounts_sol)
                .with_signer(signer_seeds_sol_vault);
            token_interface::transfer(cpi_context_sol, amount_out)?;

            msg!("Swapped {} USDT for {} SOL", amount_in, amount_out);
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
