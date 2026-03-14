use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        mint::authority = mint_a,
        mint::decimals = 8,
        mint::freeze_authority = mint_a,
        seeds = [b"mint_a"],
        bump,
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        mint::authority = mint_b,
        mint::decimals = 8,
        mint::freeze_authority = mint_b,
        seeds = [b"mint_b"],
        bump,
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        token::mint = mint_a,
        token::authority = sol_vault_account,
        token::token_program = token_program,
        seeds = [b"sol_token"], // PDA is derived from "sol_token"
        bump,
    )]
    pub sol_vault_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = signer,
        token::mint = mint_b,
        token::authority = usdt_vault_account,
        token::token_program = token_program,
        seeds = [b"usdt_token"], // PDA is derived from "usdt_token"
        bump,
    )]
    pub usdt_vault_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}
