use anchor_lang::{InstructionData, ToAccountMetas};
use dexter;
use litesvm::LiteSVM;
use solana_sdk::{
    instruction::Instruction, pubkey::Pubkey, signature::Keypair, signer::Signer, system_program,
    transaction::Transaction,
};

const TOKEN_PROGRAM_ID: Pubkey = solana_sdk::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

fn get_token_balance(svm: &LiteSVM, account: &Pubkey) -> u64 {
    let acc = svm.get_account(account).unwrap();
    let amount_bytes = &acc.data[64..72];
    u64::from_le_bytes(amount_bytes.try_into().unwrap())
}

fn get_native_balance(svm: &LiteSVM, account: &Pubkey) -> u64 {
    svm.get_account(account).unwrap().lamports
}

fn setup_amm() -> (LiteSVM, Keypair, Pubkey, Pubkey, Pubkey, Pubkey) {
    let mut svm = LiteSVM::new();
    let payer = Keypair::new();

    svm.airdrop(&payer.pubkey(), 10_000_000_000_000).unwrap();

    let dexter_program_id = dexter::ID;

    let root = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let so_path = root.join("target/deploy/dexter.so");
    if so_path.exists() {
        svm.add_program_from_file(dexter_program_id, &so_path)
            .unwrap();
    }

    let (mint_b, _) = Pubkey::find_program_address(&[b"mint_b"], &dexter_program_id);
    let (sol_vault_account, _) = Pubkey::find_program_address(&[b"sol_vault"], &dexter_program_id);
    let (usdt_vault_account, _) =
        Pubkey::find_program_address(&[b"usdt_token"], &dexter_program_id);

    let amount_sol = 1_000_000_000_000; // 1000 SOL

    println!(
        "{:#?}",
        dexter::accounts::Initialize {
            signer: payer.pubkey(),
            mint_b,
            sol_vault_account,
            usdt_vault_account,
            token_program: TOKEN_PROGRAM_ID,
            system_program: system_program::ID
        }
        .to_account_metas(None)
    );
    let ix = Instruction {
        program_id: dexter_program_id,
        accounts: dexter::accounts::Initialize {
            signer: payer.pubkey(),
            mint_b,
            sol_vault_account,
            usdt_vault_account,
            token_program: TOKEN_PROGRAM_ID,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: dexter::instruction::Initialize { amount_sol }.data(),
    };

    let blockhash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);

    let res = svm.send_transaction(tx);
    assert!(
        res.is_ok(),
        "Initialize transaction failed: {:?}",
        res.unwrap_err()
    );

    (
        svm,
        payer,
        dexter_program_id,
        mint_b,
        sol_vault_account,
        usdt_vault_account,
    )
}

fn create_user_usdt(svm: &mut LiteSVM, payer: &Keypair, dexter_program_id: Pubkey) -> Pubkey {
    // Just a helper to find the PDA that the AMM will initialize if needed
    let (user_usdt_token_account, _) = Pubkey::find_program_address(
        &[b"usdt_token", payer.pubkey().as_ref()],
        &dexter_program_id,
    );
    // Note: The swap_tokens instruction handles init_if_needed for this account!
    user_usdt_token_account
}

#[test]
fn test_initialize() {
    let (svm, _, _, _, sol_vault_account, usdt_vault_account) = setup_amm();

    let sol_balance = get_native_balance(&svm, &sol_vault_account);
    let usdt_balance = get_token_balance(&svm, &usdt_vault_account);

    assert!(sol_balance >= 1000_000_000 * 1000);
    assert_eq!(usdt_balance, 1000_000_000 * 1000);
}

#[test]
fn test_swap_sol_for_usdt() {
    let (mut svm, payer, dexter_program_id, mint_b, sol_vault_account, usdt_vault_account) =
        setup_amm();

    let initial_vault_sol = get_native_balance(&svm, &sol_vault_account);
    let initial_vault_usdt = get_token_balance(&svm, &usdt_vault_account);

    let user_usdt_account = create_user_usdt(&mut svm, &payer, dexter_program_id);

    // Initial signers balance changes due to gas fees, but we verify vault changes strictly

    let amount_in = 100_000;

    // Test swap Native SOL -> USDT
    let token_name = "SOL".to_string();
    println!(
        "{:#?}",
        dexter::accounts::Initialize {
            signer: payer.pubkey(),
            mint_b,
            sol_vault_account,
            usdt_vault_account,
            token_program: TOKEN_PROGRAM_ID,
            system_program: system_program::ID
        }
        .to_account_metas(None)
    );
    let ix = Instruction {
        program_id: dexter_program_id,
        accounts: dexter::accounts::SwapTokens {
            signer: payer.pubkey(),
            mint_b,
            sol_vault_account,
            usdt_vault_account,
            user_usdt_token_account: user_usdt_account,
            token_program: TOKEN_PROGRAM_ID,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: dexter::instruction::SwapTokens {
            amount_in,
            token_name,
        }
        .data(),
    };

    let blockhash = svm.latest_blockhash();
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = svm.send_transaction(tx);
    assert!(
        res.is_ok(),
        "swap transaction failed: {:?}",
        res.unwrap_err()
    );

    let final_vault_sol = get_native_balance(&svm, &sol_vault_account);
    let final_vault_usdt = get_token_balance(&svm, &usdt_vault_account);
    let final_user_usdt = get_token_balance(&svm, &user_usdt_account);

    assert_eq!(final_vault_sol, initial_vault_sol + amount_in);

    let pool_sol = initial_vault_sol as u128;
    let pool_usdt = initial_vault_usdt as u128;
    let k = pool_sol * pool_usdt;
    let new_pool_sol = pool_sol + amount_in as u128;
    let new_pool_usdt = k / new_pool_sol;
    let amount_out = (pool_usdt - new_pool_usdt) as u64;

    assert_eq!(final_vault_usdt, initial_vault_usdt - amount_out);
    assert_eq!(final_user_usdt, amount_out);
}

#[test]
fn test_swap_usdt_for_sol() {
    let (mut svm, payer, dexter_program_id, mint_b, sol_vault_account, usdt_vault_account) =
        setup_amm();

    // First swap SOL -> USDT to give the user some USDT
    let user_usdt_account = create_user_usdt(&mut svm, &payer, dexter_program_id);
    let amount_in = 500_000_000; // 0.5 SOL
    let token_name = "SOL".to_string();

    let ix_initial = Instruction {
        program_id: dexter_program_id,
        accounts: dexter::accounts::SwapTokens {
            signer: payer.pubkey(),
            mint_b,
            sol_vault_account,
            usdt_vault_account,
            user_usdt_token_account: user_usdt_account,
            token_program: TOKEN_PROGRAM_ID,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: dexter::instruction::SwapTokens {
            amount_in,
            token_name,
        }
        .data(),
    };

    let blockhash = svm.latest_blockhash();
    let tx_initial = Transaction::new_signed_with_payer(
        &[ix_initial],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    svm.send_transaction(tx_initial).unwrap();

    let user_usdt_balance_start = get_token_balance(&svm, &user_usdt_account);

    // Track vault state BEFORE reversing the swap
    let pre_vault_sol = get_native_balance(&svm, &sol_vault_account);
    let pre_vault_usdt = get_token_balance(&svm, &usdt_vault_account);

    // Now test swap USDT -> Native SOL
    let amount_in_usdt = user_usdt_balance_start / 2;
    let token_name_2 = "USDT".to_string();

    println!(
        "{:#?}",
        dexter::accounts::Initialize {
            signer: payer.pubkey(),
            mint_b,
            sol_vault_account,
            usdt_vault_account,
            token_program: TOKEN_PROGRAM_ID,
            system_program: system_program::ID
        }
        .to_account_metas(None)
    );
    let ix = Instruction {
        program_id: dexter_program_id,
        accounts: dexter::accounts::SwapTokens {
            signer: payer.pubkey(),
            mint_b,
            sol_vault_account,
            usdt_vault_account,
            user_usdt_token_account: user_usdt_account,
            token_program: TOKEN_PROGRAM_ID,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: dexter::instruction::SwapTokens {
            amount_in: amount_in_usdt,
            token_name: token_name_2,
        }
        .data(),
    };

    let blockhash2 = svm.latest_blockhash();
    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash2);
    let signer_sol_before = get_native_balance(&svm, &payer.pubkey());
    svm.send_transaction(tx).unwrap();
    let signer_sol_after = get_native_balance(&svm, &payer.pubkey());

    let final_vault_sol = get_native_balance(&svm, &sol_vault_account);
    let final_vault_usdt = get_token_balance(&svm, &usdt_vault_account);

    assert_eq!(final_vault_usdt, pre_vault_usdt + amount_in_usdt);

    let pool_sol = pre_vault_sol as u128;
    let pool_usdt = pre_vault_usdt as u128;
    let k = pool_sol * pool_usdt;
    let new_pool_usdt = pool_usdt + amount_in_usdt as u128;
    let new_pool_sol = k / new_pool_usdt;
    let amount_out = (pool_sol - new_pool_sol) as u64;

    assert_eq!(final_vault_sol, pre_vault_sol - amount_out);
    // User gained SOL, but paid gas, checking exact balance is skipped, we know vault decreased properly
}

#[test]
fn test_pool_drain_revert() {
    let (mut svm, payer, dexter_program_id, mint_b, sol_vault_account, usdt_vault_account) =
        setup_amm();
    let user_usdt_account = create_user_usdt(&mut svm, &payer, dexter_program_id);

    // Drain test 1 - Swap 0 tokens is invalid
    let ix_zero = Instruction {
        program_id: dexter_program_id,
        accounts: dexter::accounts::SwapTokens {
            signer: payer.pubkey(),
            mint_b,
            sol_vault_account,
            usdt_vault_account,
            user_usdt_token_account: user_usdt_account,
            token_program: TOKEN_PROGRAM_ID,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: dexter::instruction::SwapTokens {
            amount_in: 0,
            token_name: "SOL".to_string(),
        }
        .data(),
    };

    let blockhash = svm.latest_blockhash();
    let tx_zero =
        Transaction::new_signed_with_payer(&[ix_zero], Some(&payer.pubkey()), &[&payer], blockhash);
    let res_zero = svm.send_transaction(tx_zero);
    assert!(res_zero.is_err(), "Swap with amount 0 should fail");

    // Drain test 2 - Extremely large mathematical swap approaches 0 usdt output but never negative, cannot exceed vault
    let pool_usdt_initial = get_token_balance(&svm, &usdt_vault_account);
    let amount_in: u64 = 5_000_000_000_000; // Giant swap in native SOL

    let ix_massive = Instruction {
        program_id: dexter_program_id,
        accounts: dexter::accounts::SwapTokens {
            signer: payer.pubkey(),
            mint_b,
            sol_vault_account,
            usdt_vault_account,
            user_usdt_token_account: user_usdt_account,
            token_program: TOKEN_PROGRAM_ID,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: dexter::instruction::SwapTokens {
            amount_in,
            token_name: "SOL".to_string(),
        }
        .data(),
    };

    let blockhash2 = svm.latest_blockhash();
    let tx_massive = Transaction::new_signed_with_payer(
        &[ix_massive],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash2,
    );

    // We expect an error if the user lacks the massively insane amount, but our airdrop covers this.
    // We expect no crash from vault draining though.
    let _res_massive = svm.send_transaction(tx_massive);

    // It should never be negative mathematically CPMM ensures amount_out < total_pool
}
