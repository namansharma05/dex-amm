"use client";

import DexterCard from "./components/dexterCard";
import WalletCard from "./components/walletCard";

export default function Home() {
  return (
    <div className="min-h-screen bg-bg1 text-foreground">
      <main className="mx-auto flex items-center justify-center min-h-screen flex-col gap-10 px-6 py-16">
        <WalletCard />
        <div className="flex justify-center w-full">
          <DexterCard />
        </div>
      </main>
    </div>
  );
}
