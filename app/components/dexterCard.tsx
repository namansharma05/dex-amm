"use client";

import { useState, useRef, useEffect } from "react";

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
        className={`flex items-center gap-2 p-1 pr-3 rounded-full font-bold transition-all shadow-sm shrink-0 cursor-pointer outline-none select-none ${triggerClassName}`}
      >
        {currentToken ? (
          currentToken.icon
        ) : (
          <div className="w-8 h-8 bg-black/10 dark:bg-white/10 rounded-full flex items-center justify-center shrink-0"></div>
        )}
        <span className="text-base whitespace-nowrap">
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
          className={`opacity-80 transition-transform duration-200 ${isOpen ? "rotate-180" : ""}`}
        >
          <path d="m6 9 6 6 6-6" />
        </svg>
      </button>

      {isOpen && (
        <div className="absolute top-full right-0 mt-2 w-52 bg-card border border-border-low rounded-2xl shadow-[0_16px_40px_-12px_rgba(0,0,0,0.15)] p-2 z-50 flex flex-col gap-1 backdrop-blur-3xl animate-in fade-in zoom-in-95 duration-200 origin-top-right">
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
                className={`flex items-center gap-3 p-2 rounded-xl hover:bg-muted/10 transition-colors w-full text-left group ${value === opt ? "bg-muted/5" : ""}`}
              >
                <div className="group-hover:scale-110 transition-transform duration-200">
                  {token.icon}
                </div>
                <span className="font-bold text-base text-foreground">
                  {token.name}
                </span>
                {value === opt && (
                  <svg
                    className="ml-auto w-5 h-5 text-primary"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    strokeWidth="3"
                    strokeLinecap="round"
                    strokeLinejoin="round"
                  >
                    <polyline points="20 6 9 17 4 12" />
                  </svg>
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
  const buyToken =
    sellToken === "SOL" ? "USDT" : sellToken === "USDT" ? "SOL" : "";

  const handleSwap = () => {
    if (sellToken === "SOL") setSellToken("USDT");
    else if (sellToken === "USDT") setSellToken("SOL");
  };

  return (
    <div className="w-full max-w-[480px] mx-auto p-2 bg-card border border-border-low rounded-[24px] shadow-[0_24px_64px_-16px_rgba(0,0,0,0.1)] relative">
      <div className="p-3 flex flex-col gap-1">
        {/* Sell Section */}
        <div className="bg-muted/5 border border-transparent rounded-[16px] p-4 flex flex-col gap-2 hover:border-border-low transition-all group">
          <div className="flex justify-between items-center text-sm font-semibold text-muted">
            <span>Sell</span>
          </div>
          <div className="flex justify-between items-center gap-4">
            <input
              type="number"
              placeholder="0"
              className="bg-transparent border-none text-4xl md:text-5xl font-medium outline-none w-full text-foreground placeholder:text-muted/20"
            />
            <TokenDropdown
              value={sellToken}
              onChange={(val) => setSellToken(val)}
              triggerClassName="flex items-center justify-between bg-card hover:bg-muted/5 text-foreground border border-border-low w-[125px]"
            />
          </div>
          <div className="flex justify-between items-center text-sm font-medium text-muted/60">
            <span>$0</span>
          </div>
        </div>

        {/* Swap Arrow */}
        <div className="relative -my-6 z-10 flex justify-center">
          <button
            onClick={handleSwap}
            className="bg-card border-[4px] border-card rounded-xl p-1 shadow-md hover:scale-110 active:scale-90 transition-all group cursor-pointer"
          >
            <div className="bg-muted/5 group-hover:bg-muted/10 p-1.5 rounded-lg transition-colors">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2.5"
                strokeLinecap="round"
                strokeLinejoin="round"
                className="text-primary"
              >
                <path d="M12 5v14M19 12l-7 7-7-7" />
              </svg>
            </div>
          </button>
        </div>

        {/* Buy Section */}
        <div className="bg-cream/40 border border-transparent rounded-[16px] p-4 flex flex-col gap-2 hover:border-border-low transition-all group">
          <div className="flex justify-between items-center text-sm font-semibold text-muted">
            <span>Buy</span>
          </div>
          <div className="flex justify-between items-center gap-4">
            <input
              type="number"
              placeholder="0"
              className="bg-transparent border-none text-4xl md:text-5xl font-medium outline-none w-full text-foreground placeholder:text-muted/20"
            />
            <TokenDropdown
              value={buyToken}
              onChange={(val) => {
                if (val === "SOL") setSellToken("USDT");
                else if (val === "USDT") setSellToken("SOL");
              }}
              triggerClassName="flex items-center justify-between bg-card hover:bg-muted/5 text-foreground border border-border-low w-[125px]"
            />
          </div>
          <div className="flex justify-between items-center text-sm font-medium text-muted/60">
            <span>$0</span>
          </div>
        </div>

        {/* Main Action Button */}
        <button className="w-full bg-primary/10 text-primary hover:bg-primary/20 disabled:bg-muted/5 disabled:text-muted/30 py-4 mt-1 rounded-[20px] font-bold text-lg transition-all active:scale-[0.98] shadow-sm">
          Add Funds to Swap
        </button>
      </div>
    </div>
  );
}
