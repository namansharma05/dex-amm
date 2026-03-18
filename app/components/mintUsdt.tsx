"use client";

import { createWalletTransactionSigner } from "@solana/client";
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
import { getMintUsdtInstructionAsync } from "../generated/dexter/instructions/mintUsdt";
import { useState } from "react";

export default function MintUsdt() {
  const { wallet } = useWalletConnection();
  const [amount, setAmount] = useState<string>("100");
  const [isLoading, setIsLoading] = useState(false);
  const [status, setStatus] = useState<"idle" | "success" | "error">("idle");

  const handleMintUsdt = async () => {
    if (!wallet) return;
    const rpcUrl = process.env.NEXT_PUBLIC_SOLANA_RPC;
    if (!rpcUrl) return;

    setIsLoading(true);
    setStatus("idle");

    try {
      const rpc = createSolanaRpc(rpcUrl as any);
      const { signer } = createWalletTransactionSigner(wallet);
      const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
      
      const instruction = await getMintUsdtInstructionAsync({
        signer,
        amount: BigInt(parseFloat(amount) * 1_000_000), // Assuming 6 decimals for USDT
      });

      const transactionMessage = pipe(
        createTransactionMessage({ version: 0 }),
        (tx) => setTransactionMessageFeePayerSigner(signer, tx),
        (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
        (tx) => appendTransactionMessageInstruction(instruction, tx)
      );

      const transactionSign = await signTransactionMessageWithSigners(transactionMessage);
      
      await sendTransactionWithoutConfirmingFactory({ rpc })(transactionSign, {
        commitment: "confirmed",
      });
      
      console.log("USDT Token Minted");
      setStatus("success");
      setTimeout(() => setStatus("idle"), 3000);
    } catch (e) {
      console.error("Error Minting USDT", e);
      setStatus("error");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="w-full max-w-[400px] bg-card border border-border-low rounded-[24px] shadow-[0_8px_32px_rgba(0,0,0,0.05)] overflow-hidden transition-all hover:shadow-[0_12px_48px_rgba(0,0,0,0.08)]">
      <div className="p-6 flex flex-col gap-5">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-full bg-[#26A17B] flex items-center justify-center shadow-sm shrink-0">
            <svg viewBox="0 0 24 24" className="w-5 h-5 fill-white">
              <path d="M13.43 11.41V16H10.57V11.41C6.01 11.17 2.65 9.87 2.65 8.28C2.65 6.69 6.01 5.39 10.57 5.14V0H13.43V5.14C17.99 5.39 21.35 6.69 21.35 8.28C21.35 9.87 17.99 11.17 13.43 11.41ZM12 10.46C15.82 10.46 19.03 9.55 19.03 8.28C19.03 7.02 15.82 6.1 12 6.1C8.18 6.1 4.97 7.02 4.97 8.28C4.97 9.55 8.18 10.46 12 10.46Z" />
            </svg>
          </div>
          <div>
            <h2 className="text-lg font-bold text-foreground">Mint USDT</h2>
            <p className="text-xs text-muted font-medium">Test USDT for liquidity</p>
          </div>
        </div>

        <div className="bg-muted/5 border border-transparent rounded-[16px] p-4 flex flex-col gap-1 focus-within:border-primary/20 transition-all group">
          <div className="flex justify-between items-center text-xs font-bold text-muted uppercase tracking-wider">
            <span>Amount</span>
            <span className="text-primary/60 cursor-pointer hover:text-primary transition-colors">Max</span>
          </div>
          <div className="flex items-center gap-3 mt-1">
            <input
              type="number"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              placeholder="0.0"
              className="bg-transparent border-none text-3xl font-bold outline-none w-full text-foreground placeholder:text-muted/20 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
            />
            <span className="text-xl font-bold text-muted/40">USDT</span>
          </div>
        </div>

        <button
          onClick={handleMintUsdt}
          disabled={!wallet || isLoading || !amount || parseFloat(amount) <= 0}
          className={`w-full py-4 rounded-[18px] font-bold text-base transition-all active:scale-[0.98] shadow-sm flex items-center justify-center gap-2 ${
            status === "success"
              ? "bg-green-500/10 text-green-500 hover:bg-green-500/20"
              : status === "error"
              ? "bg-red-500/10 text-red-500 hover:bg-red-500/20"
              : "bg-[#26A17B] text-white hover:opacity-90 disabled:bg-muted/10 disabled:text-muted/40 disabled:cursor-not-allowed cursor-pointer"
          }`}
        >
          {isLoading ? (
            <div className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
          ) : status === "success" ? (
            <>
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round"><polyline points="20 6 9 17 4 12"/></svg>
              Minted Successfully
            </>
          ) : status === "error" ? (
            "Error Minting"
          ) : (
            "Mint Test USDT"
          )}
        </button>
      </div>
    </div>
  );
}

