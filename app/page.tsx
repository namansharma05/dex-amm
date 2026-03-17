"use client";

import DexterCard from "./components/dexterCard";
import MintSol from "./components/mintSol";
import MintUsdt from "./components/mintUsdt";
import WalletCard from "./components/walletCard";

export default function Home() {
  return (
    <div className="min-h-screen bg-bg1 text-foreground">
      <main className="mx-auto flex items-center justify-center min-h-screen flex-col gap-10 px-6 py-16">
        <WalletCard />
        <div className="flex gap-10">
          <DexterCard />
          <div className="flex items-center justify-center flex-col gap-10">
            <MintSol />
            <MintUsdt />
          </div>
        </div>
      </main>
    </div>
  );
}
