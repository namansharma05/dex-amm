use ::anchor_lang::prelude::*;

#[account]
pub struct SolVault {
    pub initialized: bool,
}
