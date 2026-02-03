#![warn(dead_code, unused_variables)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

mod blueprints;
mod contexts;
mod errors;
#[cfg(test)]
mod tests;

declare_id!("7FqhXgUYkqLWCwMGv3R9tNd149oXwy9FqzS8d8HpU3W2");

#[program]
pub mod dexter {}
