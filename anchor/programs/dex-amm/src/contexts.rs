use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        token::mint = mint_a,
        token::authority = sol_vault_account,
        token::token_program = token_program,
        seeds = [b"sol_token", signer.key().as_ref()], // PDA is derived from "sol_token" + program's signer wallet address
        bump,
    )]
    pub sol_vault_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = signer,
        token::mint = mint_b,
        token::authority = usdt_vault_account,
        token::token_program = token_program,
        seeds = [b"usdt_token", signer.key().as_ref()],
        bump,

    )]
    pub usdt_vault_account: InterfaceAccount<'info, TokenAccount>,

    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}
