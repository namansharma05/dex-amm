use ::anchor_lang::prelude::*;

#[error_code]
pub enum DexError {
    #[msg("Invalid amount")]
    InvalidAmount,
}
