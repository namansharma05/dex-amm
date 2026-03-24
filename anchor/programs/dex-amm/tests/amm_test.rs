use anchor_lang::{InstructionData, ToAccountMetas};
use dexter::{accounts as dexter_accounts, instruction as dexter_instructions};
use litesvm::LiteSVM;
use solana_sdk::{
    instruction::Instruction, pubkey::Pubkey, signature::Keypair, signer::Signer,
    transaction::Transaction,
};
use anchor_spl::token_interface::spl_token_2022;
use std::fs;

#[test]
fn test_amm_flow_with_litesvm() {
    let mut svm = LiteSVM::new();
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap(); // 10 SOL

    let program_id = dexter::id();
    
    // Load the program binary (built with 'anchor build')
    let program_bytes = fs::read("../../target/deploy/dexter.so").expect("Could not find dexter.so in target/deploy. Please run 'anchor build' first.");
    let _ = svm.add_program(program_id, &program_bytes);

    // Construct PDAs
    let (mint_b, _) = Pubkey::find_program_address(&[b"mint_b"], &program_id);
    let (sol_vault, _) = Pubkey::find_program_address(&[b"sol_vault"], &program_id);
    let (usdt_vault, _) = Pubkey::find_program_address(&[b"usdt_token"], &program_id);
    let (user_usdt, _) = Pubkey::find_program_address(&[b"usdt_token", payer.pubkey().as_ref()], &program_id);

    // 1. Initialize Pool
    println!("Step 1: Initializing pool...");
    let init_accounts = dexter_accounts::Initialize {
        signer: payer.pubkey(),
        mint_b,
        sol_vault_account: sol_vault,
        usdt_vault_account: usdt_vault,
        token_program: spl_token_2022::id(),
        system_program: solana_sdk::system_program::id(),
    }.to_account_metas(None);

    let init_ix = Instruction {
        program_id,
        accounts: init_accounts,
        data: dexter_instructions::Initialize {}.data(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[init_ix],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    svm.send_transaction(tx).expect("Failed to initialize pool");

    // Verify Initialization
    let sol_balance = svm.get_balance(&sol_vault).unwrap();
    println!("Initial SOL Vault Balance: {} lamports (includes rent)", sol_balance);
    // 953520 is the rent-exempt minimum for 9 bytes (8 + 1)
    assert!(sol_balance >= 1000, "Balance should at least include our 1000 lamport deposit"); 

    // 2. Swap SOL for USDT
    println!("Step 2: Swapping 5000 SOL for USDT...");
    let amount_in_sol = 5000;
    
    // Get user starting balance (optional)
    let swap_sol_accounts = dexter_accounts::SwapTokens {
        signer: payer.pubkey(),
        mint_b,
        sol_vault_account: sol_vault,
        usdt_vault_account: usdt_vault,
        user_usdt_token_account: user_usdt,
        token_program: spl_token_2022::id(),
        system_program: solana_sdk::system_program::id(),
    }.to_account_metas(None);

    let swap_sol_ix = Instruction {
        program_id,
        accounts: swap_sol_accounts,
        data: dexter_instructions::SwapTokens {
            amount_in: amount_in_sol,
            token_name: "SOL".to_string(),
        }.data(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[swap_sol_ix],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    svm.send_transaction(tx).expect("Failed to swap SOL for USDT");

    let post_swap_sol = svm.get_balance(&sol_vault).unwrap();
    println!("Post-Swap SOL Vault Balance: {} lamports", post_swap_sol);
    assert_eq!(post_swap_sol, sol_balance + amount_in_sol);

    println!("Success: All AMM flow tests passed!");
}
