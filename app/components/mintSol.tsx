"use client";

import {
  appendTransactionMessageInstruction,
  createSolanaRpc,
  createTransactionMessage,
  pipe,
  sendTransactionWithoutConfirmingFactory,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
} from "@solana/kit";
import { useWalletConnection } from "@solana/react-hooks";
import { getMintSolInstructionAsync } from "../generated/dexter/instructions/mintSol";
import { createWalletTransactionSigner } from "@solana/client";
import { useState } from "react";

export default function MintSol() {
  const { wallet } = useWalletConnection();
  const [amount, setAmount] = useState<string>("");
  const [isLoading, setIsLoading] = useState(false);
  const [status, setStatus] = useState<"idle" | "success" | "error">("idle");

  const handleMintSol = async () => {
    if (!wallet) return;
    const rpcUrl = process.env.NEXT_PUBLIC_SOLANA_RPC;
    if (!rpcUrl) return;

    setIsLoading(true);
    setStatus("idle");

    try {
      const rpc = createSolanaRpc(rpcUrl as any);
      const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
      const { signer } = createWalletTransactionSigner(wallet);

      const instruction = await getMintSolInstructionAsync({
        signer,
        amount: BigInt(parseFloat(amount) * 1_000_000_000),
      });

      const transactionMessage = pipe(
        createTransactionMessage({ version: 0 }),
        (tx) => setTransactionMessageFeePayerSigner(signer, tx),
        (tx) =>
          setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
        (tx) => appendTransactionMessageInstruction(instruction, tx)
      );

      const transactionSign =
        await signTransactionMessageWithSigners(transactionMessage);

      await sendTransactionWithoutConfirmingFactory({ rpc })(transactionSign, {
        commitment: "confirmed",
      });

      console.log("SOL Token Minted");
      setStatus("success");
      setTimeout(() => setStatus("idle"), 3000);
    } catch (e) {
      console.error("Error Minting SOL", e);
      setStatus("error");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="w-full max-w-[400px] bg-card border border-border-low rounded-[24px] shadow-[0_8px_32px_rgba(0,0,0,0.05)] overflow-hidden transition-all hover:shadow-[0_12px_48px_rgba(0,0,0,0.08)]">
      <div className="p-6 flex flex-col gap-5">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-full bg-gradient-to-tr from-[#9945FF] to-[#14F195] flex items-center justify-center shadow-sm">
            <svg fill="white" viewBox="0 0 393.3 331.4" className="w-5 h-5">
              <path d="M64.6 237.9c2.4-2.4 5.7-3.8 9.2-3.8h317.4c5.8 0 8.7 7 4.6 11.1l-62.7 62.7c-2.4 2.4-5.7 3.8-9.2 3.8H6.5c-5.8 0-8.7-7-4.6-11.1l62.7-62.7zM64.6 3.8C67.1 1.4 70.4 0 73.8 0h317.4c5.8 0 8.7 7 4.6 11.1l-62.7 62.7c-2.4 2.4-5.7 3.8-9.2 3.8H6.5c-5.8 0-8.7-7-4.6-11.1L64.6 3.8zm254.1 117.1c-2.4-2.4-5.7-3.8-9.2-3.8H-2.1c-5.8 0-8.7 7-4.6 11.1l62.7 62.7c2.4 2.4 5.7 3.8 9.2 3.8h317.4c5.8 0 8.7-7 4.6-11.1l-62.7-62.7z" />
            </svg>
          </div>
          <div>
            <h2 className="text-lg font-bold text-foreground">Mint SOL</h2>
            <p className="text-xs text-muted font-medium">
              Faucet tokens for testing
            </p>
          </div>
        </div>

        <div className="bg-muted/5 border border-transparent rounded-[16px] p-4 flex flex-col gap-1 focus-within:border-primary/20 transition-all group">
          <div className="flex justify-between items-center text-xs font-bold text-muted uppercase tracking-wider">
            <span>Amount</span>
          </div>
          <div className="flex items-center gap-3 mt-1">
            <input
              type="number"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              placeholder="0.0"
              className="bg-transparent border-none text-3xl font-bold outline-none w-full text-foreground placeholder:text-muted/20 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
            />
            <span className="text-xl font-bold text-muted/40">SOL</span>
          </div>
        </div>

        <button
          onClick={handleMintSol}
          disabled={!wallet || isLoading || !amount || parseFloat(amount) <= 0}
          className={`w-full py-4 rounded-[18px] font-bold text-base transition-all active:scale-[0.98] shadow-sm flex items-center justify-center gap-2 ${
            status === "success"
              ? "bg-green-500/10 text-green-500 hover:bg-green-500/20"
              : status === "error"
                ? "bg-red-500/10 text-red-500 hover:bg-red-500/20"
                : "bg-primary text-white hover:opacity-90 disabled:bg-muted/10 disabled:text-muted/40 disabled:cursor-not-allowed cursor-pointer"
          }`}
        >
          {isLoading ? (
            <div className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
          ) : status === "success" ? (
            <>
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="3"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <polyline points="20 6 9 17 4 12" />
              </svg>
              Minted Successfully
            </>
          ) : status === "error" ? (
            "Error Minting"
          ) : (
            "Mint Test SOL"
          )}
        </button>
      </div>
    </div>
  );
}
