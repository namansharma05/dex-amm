use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[account]
pub struct SolVault {}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        mint::authority = mint_b,
        mint::decimals = 9,
        mint::freeze_authority = mint_b,
        mint::token_program = token_program,
        seeds = [b"mint_b"],
        bump,
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        space = 8,
        seeds = [b"sol_vault"],
        bump,
    )]
    pub sol_vault_account: Account<'info, SolVault>,

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

#[derive(Accounts)]
pub struct SwapTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        mint::authority = mint_b,
        mint::decimals = 9,
        mint::freeze_authority = mint_b,
        seeds = [b"mint_b"],
        bump,
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [b"sol_vault"],
        bump,
    )]
    pub sol_vault_account: Account<'info, SolVault>,

    #[account(
        mut,
        token::mint = mint_b,
        token::authority = usdt_vault_account,
        token::token_program = token_program,
        seeds = [b"usdt_token"], // PDA is derived from "usdt_token"
        bump,
    )]
    pub usdt_vault_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        token::mint = mint_b,
        token::authority = signer,
        token::token_program = token_program,
        seeds = [b"usdt_token", signer.key().as_ref()],
        bump,
    )]
    pub user_usdt_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
