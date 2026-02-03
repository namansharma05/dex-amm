use anchor_lang::prelude::*;
use anchor_spl::{
    token::TokenAccount,
    token_interface::{Mint, TokenInterface},
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        token::mint = mint,
        token::authority = sol_vault,
        token::token_program = token_program,
        seeds = [b"sol_token", signer.key().as_ref()], // PDA is derived from "sol_token" + program's signer wallet address
        bump,
    )]
    pub sol_vault: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}
