"use client";

import { createWalletTransactionSigner } from "@solana/client";
import {
  appendTransactionMessageInstruction,
  createSolanaRpc,
  createTransactionMessage,
  pipe,
  sendTransactionWithoutConfirmingFactory,
  setTransactionMessageFeePayer,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
} from "@solana/kit";
import { useWalletConnection } from "@solana/react-hooks";
import { useState, useRef, useEffect } from "react";
import { getSwapTokensInstructionAsync } from "../generated/dexter";

const TOKENS = {
  SOL: {
    id: "SOL",
    name: "SOL",
    icon: (
      <div className="w-8 h-8 rounded-full bg-gradient-to-tr from-[#9945FF] to-[#14F195] flex items-center justify-center shadow-sm shrink-0">
        <svg fill="white" viewBox="0 0 393.3 331.4" className="w-4 h-4">
          <path d="M64.6 237.9c2.4-2.4 5.7-3.8 9.2-3.8h317.4c5.8 0 8.7 7 4.6 11.1l-62.7 62.7c-2.4 2.4-5.7 3.8-9.2 3.8H6.5c-5.8 0-8.7-7-4.6-11.1l62.7-62.7zM64.6 3.8C67.1 1.4 70.4 0 73.8 0h317.4c5.8 0 8.7 7 4.6 11.1l-62.7 62.7c-2.4 2.4-5.7 3.8-9.2 3.8H6.5c-5.8 0-8.7-7-4.6-11.1L64.6 3.8zm254.1 117.1c-2.4-2.4-5.7-3.8-9.2-3.8H-2.1c-5.8 0-8.7 7-4.6 11.1l62.7 62.7c2.4 2.4 5.7 3.8 9.2 3.8h317.4c5.8 0 8.7-7 4.6-11.1l-62.7-62.7z" />
        </svg>
      </div>
    ),
  },
  USDT: {
    id: "USDT",
    name: "USDT",
    icon: (
      <div className="w-8 h-8 rounded-full bg-[#26A17B] flex items-center justify-center shadow-sm shrink-0">
        <svg viewBox="0 0 24 24" className="w-4 h-4 fill-white">
          <path d="M13.43 11.41V16H10.57V11.41C6.01 11.17 2.65 9.87 2.65 8.28C2.65 6.69 6.01 5.39 10.57 5.14V0H13.43V5.14C17.99 5.39 21.35 6.69 21.35 8.28C21.35 9.87 17.99 11.17 13.43 11.41ZM12 10.46C15.82 10.46 19.03 9.55 19.03 8.28C19.03 7.02 15.82 6.1 12 6.1C8.18 6.1 4.97 7.02 4.97 8.28C4.97 9.55 8.18 10.46 12 10.46Z" />
        </svg>
      </div>
    ),
  },
};

function TokenDropdown({
  value,
  onChange,
  options = ["SOL", "USDT"],
  triggerClassName = "",
}: {
  value: string;
  onChange: (val: "SOL" | "USDT") => void;
  options?: string[];
  triggerClassName?: string;
}) {
  const [isOpen, setIsOpen] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (
        dropdownRef.current &&
        !dropdownRef.current.contains(event.target as Node)
      ) {
        setIsOpen(false);
      }
    }
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const currentToken = value ? TOKENS[value as keyof typeof TOKENS] : null;

  return (
    <div className="relative" ref={dropdownRef}>
      <button
        onClick={() => setIsOpen(!isOpen)}
        className={`flex items-center gap-2 p-1 pr-3 rounded-full font-bold transition-all shadow-sm shrink-0 cursor-pointer outline-none select-none border border-transparent hover:border-border-low ${triggerClassName}`}
      >
        {currentToken ? (
          currentToken.icon
        ) : (
          <div className="w-8 h-8 bg-black/10 dark:bg-white/10 rounded-full flex items-center justify-center shrink-0"></div>
        )}
        <span className="text-base font-bold whitespace-nowrap">
          {currentToken ? currentToken.name : "Select"}
        </span>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth="3"
          strokeLinecap="round"
          strokeLinejoin="round"
          className={`opacity-60 transition-transform duration-200 ${isOpen ? "rotate-180" : ""}`}
        >
          <path d="m6 9 6 6 6-6" />
        </svg>
      </button>

      {isOpen && (
        <div className="absolute top-full right-0 mt-2 w-52 bg-card border border-border-low rounded-2xl shadow-[0_16px_40px_-12px_rgba(0,0,0,0.2)] p-2 z-50 flex flex-col gap-1 backdrop-blur-3xl animate-in fade-in zoom-in-95 duration-200 origin-top-right">
          {options.map((opt) => {
            const token = TOKENS[opt as keyof typeof TOKENS];
            if (!token) return null;
            return (
              <button
                key={opt}
                onClick={() => {
                  onChange(opt as "SOL" | "USDT");
                  setIsOpen(false);
                }}
                className={`flex items-center gap-3 p-2.5 rounded-xl hover:bg-muted/10 transition-all w-full text-left group ${value === opt ? "bg-muted/5 shadow-inner" : ""}`}
              >
                <div className="group-hover:scale-110 transition-transform duration-200">
                  {token.icon}
                </div>
                <div className="flex flex-col">
                  <span className="font-bold text-base text-foreground">
                    {token.name}
                  </span>
                  <span className="text-[10px] text-muted font-bold opacity-60 uppercase">
                    {opt === "SOL" ? "Solana" : "Tether USD"}
                  </span>
                </div>
                {value === opt && (
                  <div className="ml-auto w-5 h-5 bg-primary/10 rounded-full flex items-center justify-center">
                    <svg
                      className="w-3 h-3 text-primary"
                      viewBox="0 0 24 24"
                      fill="none"
                      stroke="currentColor"
                      strokeWidth="4"
                      strokeLinecap="round"
                      strokeLinejoin="round"
                    >
                      <polyline points="20 6 9 17 4 12" />
                    </svg>
                  </div>
                )}
              </button>
            );
          })}
        </div>
      )}
    </div>
  );
}

export default function DexterCard() {
  const [sellToken, setSellToken] = useState<"SOL" | "USDT" | "">("SOL");
  const [sellAmount, setSellAmount] = useState<string>("");
  const [isLoading, setIsLoading] = useState(false);

  const buyToken =
    sellToken === "SOL" ? "USDT" : sellToken === "USDT" ? "SOL" : "";
  const { wallet } = useWalletConnection();

  const handleSwapTokens = () => {
    if (sellToken === "SOL") setSellToken("USDT");
    else if (sellToken === "USDT") setSellToken("SOL");
  };

  const handleSwap = async () => {
    if (!wallet || !sellToken || !sellAmount) return;
    setIsLoading(true);

    const rpcUrl = process.env.NEXT_PUBLIC_SOLANA_RPC;
    const rpc = createSolanaRpc(rpcUrl as any);
    try {
      const { signer } = createWalletTransactionSigner(wallet);
      const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

      const instruction = await getSwapTokensInstructionAsync({
        signer,
        amountIn: BigInt(
          parseFloat(sellAmount) * (sellToken === "SOL" ? 1e9 : 1e6)
        ),
        tokenName: sellToken,
      });

      const transactionMessage = pipe(
        createTransactionMessage({ version: "legacy" }),
        (tx) => setTransactionMessageFeePayer(signer.address, tx),
        (tx) =>
          setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
        (tx) => appendTransactionMessageInstruction(instruction, tx)
      );

      const transactionSign =
        await signTransactionMessageWithSigners(transactionMessage);

      await sendTransactionWithoutConfirmingFactory({ rpc })(transactionSign, {
        commitment: "confirmed",
      });
      console.log("Swap successful");
    } catch (e) {
      console.error("Error swapping", e);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="w-full max-w-[480px] mx-auto p-1.5 bg-card border border-border-low rounded-[32px] shadow-[0_32px_80px_-16px_rgba(0,0,0,0.12)] relative overflow-hidden group/card">
      {/* Decorative background glow */}
      <div className="absolute -top-[100px] -right-[100px] w-64 h-64 bg-primary/5 blur-[100px] rounded-full pointer-events-none group-hover/card:bg-primary/10 transition-colors duration-700"></div>

      <div className="p-4 flex flex-col gap-1.5 relative z-10">
        {/* Card Header */}
        <div className="flex justify-between items-center px-2 mb-2">
          <div className="flex gap-4">
            <span className="text-base font-bold text-foreground relative">
              Swap
              <div className="absolute -bottom-1 left-0 w-full h-0.5 bg-primary rounded-full"></div>
            </span>
          </div>
        </div>

        {/* Sell Section */}
        <div className="bg-muted/5 border border-transparent rounded-[24px] p-5 flex flex-col gap-3 hover:border-border-low/50 transition-all focus-within:bg-muted/10 focus-within:border-primary/10 group/input">
          <div className="flex justify-between items-center text-sm font-bold text-muted uppercase tracking-tight">
            <span>Sell</span>
          </div>
          <div className="flex justify-between items-center gap-4">
            <input
              type="number"
              placeholder="0"
              value={sellAmount}
              onChange={(e) => setSellAmount(e.target.value)}
              className="bg-transparent border-none text-4xl md:text-5xl font-bold outline-none w-full text-foreground placeholder:text-muted/20 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
            />
            <TokenDropdown
              value={sellToken}
              onChange={(val) => setSellToken(val)}
              triggerClassName="flex items-center justify-between bg-card hover:bg-muted/5 text-foreground border border-border-low/50 shadow-sm w-[130px] p-1.5 pr-4"
            />
          </div>
          <div className="flex justify-between items-center text-sm font-bold text-muted/40">
            <span>~$0.00</span>
            <div className="flex items-center gap-1">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="14"
                height="14"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="3"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <path d="M19 7V4a1 1 0 0 0-1-1H5a2 2 0 0 0 0 4h15a1 1 0 0 1 1 1v4h-3a2 2 0 0 0 0 4h3a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1" />
                <path d="M3 5v14a2 2 0 0 0 2 2h15a1 1 0 0 0 1-1v-4" />
              </svg>
              <span>0.00</span>
            </div>
          </div>
        </div>

        {/* Swap Arrow */}
        <div className="relative -my-5 z-10 flex justify-center">
          <button
            onClick={handleSwapTokens}
            className="bg-card border-[6px] border-card rounded-2xl p-0.5 shadow-xl hover:scale-110 active:scale-95 transition-all group cursor-pointer"
          >
            <div className="bg-muted/5 group-hover:bg-primary/10 p-2.5 rounded-xl transition-colors text-primary border border-border-low/50 group-hover:border-primary/20">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="22"
                height="22"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="3"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <path d="M12 5v14M19 12l-7 7-7-7" />
              </svg>
            </div>
          </button>
        </div>

        {/* Buy Section */}
        <div className="bg-muted/5 border border-transparent rounded-[24px] p-5 flex flex-col gap-3 transition-all">
          <div className="flex justify-between items-center text-sm font-bold text-muted uppercase tracking-tight">
            <span>Buy</span>
          </div>
          <div className="flex justify-between items-center gap-4">
            <input
              type="number"
              placeholder="0"
              readOnly
              className="bg-transparent border-none text-4xl md:text-5xl font-bold outline-none w-full text-foreground placeholder:text-muted/20 opacity-50 cursor-default"
            />
            <TokenDropdown
              value={buyToken}
              onChange={(val) => {
                if (val === "SOL") setSellToken("USDT");
                else if (val === "USDT") setSellToken("SOL");
              }}
              triggerClassName="flex items-center justify-between bg-card hover:bg-muted/5 text-foreground border border-border-low/50 shadow-sm w-[130px] p-1.5 pr-4"
            />
          </div>
          <div className="flex justify-between items-center text-sm font-bold text-muted/40">
            <span>~$0.00</span>
            <div className="flex items-center gap-1">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="14"
                height="14"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="3"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <path d="M19 7V4a1 1 0 0 0-1-1H5a2 2 0 0 0 0 4h15a1 1 0 0 1 1 1v4h-3a2 2 0 0 0 0 4h3a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1" />
                <path d="M3 5v14a2 2 0 0 0 2 2h15a1 1 0 0 0 1-1v-4" />
              </svg>
              <span>0.00</span>
            </div>
          </div>
        </div>

        {/* Info Rows */}
        <div className="px-2 py-1 flex flex-col gap-2 mt-2">
          <div className="flex justify-between items-center text-xs font-bold">
            <span className="text-muted">Exchange Rate</span>
            <span className="text-foreground">1 SOL = 154.2 USDT</span>
          </div>
          <div className="flex justify-between items-center text-xs font-bold">
            <span className="text-muted">Network Fee</span>
            <span className="text-foreground/80">~$0.002</span>
          </div>
        </div>

        {/* Main Action Button */}
        <button
          onClick={handleSwap}
          disabled={!wallet || isLoading || !sellAmount}
          className="w-full relative mt-10 group/btn overflow-hidden rounded-[24px] disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <div className="absolute inset-0 bg-gradient-to-r from-primary via-[#ff4d9e] to-primary animate-premium-gradient group-hover/btn:brightness-110 transition-all duration-700"></div>
          <div className="relative py-4 w-full h-full flex items-center justify-center font-bold text-lg text-white group-hover/btn:scale-105 transition-transform">
            {isLoading ? (
              <div className="w-6 h-6 border-3 border-white/30 border-t-white rounded-full animate-spin"></div>
            ) : !wallet ? (
              "Connect Wallet"
            ) : !sellAmount ? (
              "Enter Amount"
            ) : (
              "Swap Tokens"
            )}
          </div>
        </button>
      </div>
    </div>
  );
}
