# Product Requirements Document (PRD): Dexter AMM (CPMM)

## 1. Executive Summary
**Dexter** is a decentralized exchange (DEX) Automated Market Maker (AMM) on the Solana blockchain. It utilizes the Constant Product Market Maker (CPMM) mathematical model (`x * y = k`) to enable permissionless, trustless token swapping between pairs of assets (e.g., SOL and USDT). The goal is to provide a highly efficient, secure, and fully decentralized swapping protocol for users and liquidity providers.

## 2. Objectives & Goals
### Primary Goals
- **Seamless Token Swapping:** Enable users to seamlessly swap between two tokens with on-chain price discovery driven by the ratio of tokens in the pool.
- **Robust Security:** Ensure all user funds and pool reserves are handled securely, leveraging Solana's Program Derived Addresses (PDAs) and proper CPI (Cross-Program Invocation) authority delegations.
- **Architectural Correctness:** Re-architect the current experimental mint/burn approach into a standard, industry-accepted Vault Transfer approach.

### Secondary Goals (Future Enhancements)
- **Liquidity Provisioning (LP):** Allow external users to provide liquidity in exchange for LP tokens, earning a yield from swap fees.
- **Dynamic Fees & Slippage:** Implement features to protect users against high price impact (slippage) and gather protocol/LP fees.

## 3. Core Features & Requirements

### 3.1. Pool Initialization
- **Description:** Initialize a new market/pool for a specific pair of tokens (Token A and Token B).
- **Requirements:**
  - Create Vault Token Accounts for Token A and Token B, strictly owned by a program PDA.
  - Set the initial exchange rate by depositing the starting balance of both tokens.
  - *Future:* Mint the initial LP tokens to the creator.

### 3.2. Token Swapping (CPMM)
- **Description:** Users can swap an amount of Token A for Token B, or vice-versa.
- **Mathematical Model:** `Reserve A * Reserve B = Constant (k)`.
- **Requirements:**
  - **Inputs:** The user specifies strictly the `amount_in` and the `token_name` (or `mint` address) they wish to swap from.
  - **Logic:** 
    - The program casts pool vault amounts to `u128` to calculate the invariant `k`.
    - Computes `amount_out` based on the formula: `amount_out = Reserve Out - (k / (Reserve In + amount_in))`.
    - Validates that pool has sufficient liquidity and inputs are `> 0`.
  - **Execution:**
    - Transfers `amount_in` from the User's Token Account to the Pool's Vault via standard `token::transfer` authenticated by the user's wallet signature.
    - Transfers `amount_out` from the Pool's Vault to the User's Token Account via standard `token::transfer` authenticated by the PDA's `signer_seeds`.

### 3.3. Security & Validation Rules
- **No Direct Minting/Burning:** The DEX should NOT act as the mint authority for traded assets unless it specifically represents a virtual or testnet token factory. Real-world AMMs must utilize transfers.
- **Overflow Protection:** All multiplication operations involving token balances must cast factors to `u128` to prevent `u64` capacity overflows (particularly prominent given SPL token decimal standards).
- **Authority Verification:** Any deduction of user assets must strictly be checked against the wallet owner's signature, ensuring no unauthorized withdrawal algorithms are exploitable.

## 4. Technical Architecture

### 4.1. Anchor Program Structure
- **State Accounts:**
  - `AmmPool`: Stores metadata, PDA bumps, fee structures, and the addresses of the two Vault token accounts.
- **Contexts / Instructions:**
  - `InitializePool`: Sets up the PDA ownership for the vaults.
  - `Swap`: Handles the core mathematical logic and CPI `Token::Transfer` requests.

### 4.2. Token Flow Diagram (Swap)
1. User calls `swap_tokens(amount_in: 100, target: "SOL")`.
2. Dexter calculates `amount_out` required to maintain `x * y = k`.
3. Dexter initiates CPI: `Transfer 100 SOL from User -> Vault A`.
4. Dexter initiates CPI: `Transfer amount_out USDT from Vault B -> User` (using Vault PDA seeds to sign).
5. State balances update organically based on transfer success.

## 5. Implementation Roadmap

### Phase 1: Core Re-architecture
- Refactor the current `lib.rs` file to replace `MintTo` and `Burn` CPIs with `Transfer` CPIs.
- Correct the user authority delegation on inbound transfers so they are properly authenticated by the transaction signer.
- Upgrade mathematical calculations to securely perform multiplication using `u128` type casting.

### Phase 2: Refine and Test
- Write Rust unit tests using `solana-program-test` or basic anchor Typescript integration tests to confirm robust math and CPI signing behavior.
- Validate that attempts to drain the pool mathematically revert successfully.

### Phase 3: Liquidity Provisioning (Optional Expansion)
- Build an algorithm to mint an SPL LP Token representing proportional shares in the Constant Product Pool.
- Add `add_liquidity` and `remove_liquidity` instructions, dynamically managing LP token supply based on deposits and withdrawals.
- Implement a `swap_fee` directly contributing to compounding Vault balances.
