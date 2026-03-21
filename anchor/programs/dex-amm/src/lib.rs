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
    use anchor_spl::token_interface::{self, MintTo, TransferChecked};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount_sol: u64) -> Result<()> {
        let signer_seeds_mint_b: &[&[&[u8]]] = &[&[b"mint_b", &[ctx.bumps.mint_b]]];

        let cpi_program_id = ctx.accounts.token_program.to_account_info();

        let cpi_accounts_usdt = MintTo {
            mint: ctx.accounts.mint_b.to_account_info(),
            to: ctx.accounts.usdt_vault_account.to_account_info(),
            authority: ctx.accounts.mint_b.to_account_info(),
        };

        let cpi_context_usdt =
            CpiContext::new(cpi_program_id, cpi_accounts_usdt).with_signer(signer_seeds_mint_b);

        token_interface::mint_to(cpi_context_usdt, 1000_000_000 * 1000)?;

        msg!(
            "usdt token minted to pool: {}",
            ctx.accounts.usdt_vault_account.amount
        );

        // Transfer initial liquidity of native SOL from signer to sol_vault_account
        let cpi_context_system = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.sol_vault_account.to_account_info(),
            },
        );
        require!(amount_sol > 0, errors::DexError::InvalidAmount);
        anchor_lang::system_program::transfer(cpi_context_system, amount_sol)?;

        msg!("sol deposited to pool: {}", amount_sol);
        Ok(())
    }

    pub fn swap_tokens(ctx: Context<SwapTokens>, amount_in: u64, token_name: String) -> Result<()> {
        let signer_seeds_usdt_vault: &[&[&[u8]]] =
            &[&[b"usdt_token", &[ctx.bumps.usdt_vault_account]]];

        let cpi_program_id = ctx.accounts.token_program.to_account_info();

        if token_name == "SOL" {
            // user is swapping native SOL for USDT
            require!(amount_in > 0, errors::DexError::InvalidAmount);
            require!(
                ctx.accounts.sol_vault_account.to_account_info().lamports() > 0
                    && ctx.accounts.usdt_vault_account.amount > 0,
                errors::DexError::InvalidAmount
            );

            // x * y = k
            let pool_sol = ctx.accounts.sol_vault_account.to_account_info().lamports() as u128;
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

            // Transfer amount_in Native SOL from User to Pool
            let cpi_context_system = CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.signer.to_account_info(),
                    to: ctx.accounts.sol_vault_account.to_account_info(),
                },
            );
            anchor_lang::system_program::transfer(cpi_context_system, amount_in)?;

            // Transfer amount_out USDT from Pool to User via SPL
            let cpi_accounts_usdt = TransferChecked {
                from: ctx.accounts.usdt_vault_account.to_account_info(),
                mint: ctx.accounts.mint_b.to_account_info(),
                to: ctx.accounts.user_usdt_token_account.to_account_info(),
                authority: ctx.accounts.usdt_vault_account.to_account_info(),
            };
            let cpi_context_usdt = CpiContext::new(cpi_program_id.clone(), cpi_accounts_usdt)
                .with_signer(signer_seeds_usdt_vault);
            token_interface::transfer_checked(cpi_context_usdt, amount_out, 9)?;

            msg!("Swapped {} native SOL for {} USDT", amount_in, amount_out);
        } else if token_name == "USDT" {
            // user is swapping USDT for Native SOL
            require!(amount_in > 0, errors::DexError::InvalidAmount);
            require!(
                ctx.accounts.sol_vault_account.to_account_info().lamports() > 0
                    && ctx.accounts.usdt_vault_account.amount > 0,
                errors::DexError::InvalidAmount
            );

            // x * y = k
            let pool_sol = ctx.accounts.sol_vault_account.to_account_info().lamports() as u128;
            let pool_usdt = ctx.accounts.usdt_vault_account.amount as u128;
            let amount_in_u128 = amount_in as u128;

            let k = pool_sol.checked_mul(pool_usdt).unwrap();
            let new_pool_usdt = pool_usdt.checked_add(amount_in_u128).unwrap();
            let new_pool_sol = k.checked_div(new_pool_usdt).unwrap();

            let amount_out_u128 = pool_sol.checked_sub(new_pool_sol).unwrap();
            let amount_out = amount_out_u128 as u64;

            require!(amount_out > 0, errors::DexError::InvalidAmount);
            require!(
                ctx.accounts.sol_vault_account.to_account_info().lamports() >= amount_out,
                errors::DexError::InvalidAmount
            );

            // Transfer amount_in USDT from User to Pool via SPL
            let cpi_accounts_usdt = TransferChecked {
                from: ctx.accounts.user_usdt_token_account.to_account_info(),
                mint: ctx.accounts.mint_b.to_account_info(),
                to: ctx.accounts.usdt_vault_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            };
            let cpi_context_usdt = CpiContext::new(cpi_program_id.clone(), cpi_accounts_usdt);
            token_interface::transfer_checked(cpi_context_usdt, amount_in, 9)?;

            // Transfer amount_out Native SOL from Pool to User
            **ctx
                .accounts
                .sol_vault_account
                .to_account_info()
                .try_borrow_mut_lamports()? -= amount_out;
            **ctx.accounts.signer.try_borrow_mut_lamports()? += amount_out;

            msg!("Swapped {} USDT for {} native SOL", amount_in, amount_out);
        }
        Ok(())
    }
}
