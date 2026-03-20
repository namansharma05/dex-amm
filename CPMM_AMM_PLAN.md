# DEX AMM (CPMM) Implementation Plan and Code Review

## 1. Overview
This document outlines the plan for creating a Constant Product Market Maker (CPMM) token swap program on Solana, and reviews the current implementation in `anchor/programs/dex-amm/src/lib.rs`.

A CPMM uses the mathematical formula `x * y = k` to determine token swap amounts. When a user swaps `amount_in` of Token A for Token B, the pool's new balance of Token B is calculated such that the product of the two balances remains constant (ignoring fees). The amount of Token B given to the user is the difference between the old pool balance and the new pool balance.

## 2. Implementation Plan for a standard CPMM
A standard DEX AMM swap instruction should follow these steps:
1. **Validate Inputs:** Check that `amount_in` is greater than 0.
2. **Calculate Output Amount:**
   - Cast pool balances to `u128` to avoid integer overflow during multiplication.
   - `k = pool_a_balance * pool_b_balance`
   - `new_pool_a_balance = pool_a_balance + amount_in`
   - `new_pool_b_balance = k / new_pool_a_balance`
   - `amount_out = pool_b_balance - new_pool_b_balance`
3. **Execute Incoming Transfer:**
   - Transfer `amount_in` of Token A from the User's Token Account to the Pool's Vault A.
   - This CPI relies on the user's signature interacting securely with the token program.
4. **Execute Outgoing Transfer:**
   - Transfer `amount_out` of Token B from the Pool's Vault B to the User's Token Account.
   - This CPI requires the Pool's Vault PDA signature (`with_signer`).

## 3. Code Review of Current `lib.rs`

The current code attempts to implement the AMM but utilizes a non-standard minting and burning model rather than transferring tokens. While it accurately uses the basic CPMM formula `x * y = k`, there are several critical issues in the code that will cause the program to fail during execution if deployed.

### ✅ What is Correct:
- The math conceptually tries to implement constant product (`x * y = k`) logic for finding the correct output amount.
- Required checks are present to ensure `amount_in > 0` and that the pool has a valid balance to trade against.

### ❌ Critical Bugs & Flaws:

**1. Incorrect Authority for Burning User Tokens:**
When attempting to burn tokens from the user's accounts, the code incorrectly sets the `authority` parameter to the token account itself instead of the token account's owner (the user's wallet):
```rust
from: ctx.accounts.user_sol_token_account.to_account_info(),
authority: ctx.accounts.user_sol_token_account.to_account_info(), // Incorrect! Should be the User.
```
This will result in a CPI error, as the token account cannot act as its own authority.

**2. Incorrect Signer Used for User Operations:**
When burning the user's tokens, the program attempts to sign the cross-program invocation (CPI) using the Program Derived Address (PDA) seeds associated with the mint (`signer_seeds_mint_a` or `signer_seeds_mint_b`):
```rust
let cpi_context_sol_burn = CpiContext::new(cpi_program_id.clone(), cpi_accounts_sol_burn)
    .with_signer(signer_seeds_mint_a); // Incorrect!
```
The user's wallet is the authority over their token accounts and has already signed the transaction. The CPI does not need a PDA `with_signer` for burning user funds; it should just pass the regular context trusting the user's existing signature on the transaction.

**3. Math Overflow Risks:**
The AMM formula multiplication operates on `u64` types natively:
```rust
let new_usdt_vault_amount = (ctx.accounts.sol_vault_account.amount * ctx.accounts.usdt_vault_account.amount) / new_sol_vault_amount;
```
If the token balances grow large enough (which is extremely common due to token decimals, e.g., $10^6$ tokens with $10^9$ decimals equals $10^{15}$ units), multiplying them will exceed the maximum value of a `u64` ($1.8 \times 10^{19}$), causing the program to panic and crash during swaps. You must upcast the variables to `u128` during the multiplication step, and downcast back to `u64`.

**4. Anti-pattern: Using Mint/Burn instead of Transfer:**
Currently, instead of transferring existing tokens between the user and the pool, the swap logic:
- Mints new tokens to the pool and burns tokens from the pool.
- Mints new tokens to the user and burns tokens from the user.

While this mathematically keeps the total circulating supply stable, it is highly non-standard. It requires the DEX program to be the permanent `Mint Authority` over the tokens, which fundamentally breaks the standard composition of decentralized finance (DeFi), where users independently trade tokens they own. A standard CPMM should make use of the `Transfer` CPI rather than `MintTo` and `Burn`.

## 4. Specific Recommendations for Refactoring
- **Restructure CPMM Logic:** Switch from a `MintTo` and `Burn` methodology inside `swap_tokens` to using `Transfer` logic instead.
- **Inbound Transfers:** Use `token::transfer` to move `amount_in` from the user's account to the vault PDA without relying on PDA seeds inside the CPI.
- **Outbound Transfers:** Use `token::transfer` with PDA seeds to move `amount_out` from the vault PDA to the user.
- **Safe Math:** Explicitly cast vault amounts to `u128` for the `let new_vault_amount` calculations to prevent panic crashes on large token values.
