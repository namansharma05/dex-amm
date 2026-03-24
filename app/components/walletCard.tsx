"use client";

import { useWalletConnection } from "@solana/react-hooks";

export default function WalletCard() {
  const { connectors, connect, disconnect, wallet, status } =
    useWalletConnection();

  const address = wallet?.account.address.toString();

  return (
    <section className="w-full max-w-3xl bg-card border border-border-low rounded-[32px] shadow-[0_32px_80px_-32px_rgba(0,0,0,0.1)] overflow-hidden transition-all hover:shadow-[0_40px_100px_-40px_rgba(0,0,0,0.15)] group relative">
      <div className="absolute top-0 left-0 w-full h-1.5 bg-gradient-to-r from-primary/20 via-primary to-primary/20 opacity-50"></div>

      <div className="p-8 space-y-8">
        <div className="flex items-center justify-between gap-6">
          <div className="space-y-2">
            <h2 className="text-2xl font-bold tracking-tight text-foreground">
              Wallet Connection
            </h2>
            <p className="text-sm font-medium text-muted max-w-md leading-relaxed">
              Connect your Solana wallet to interact with the Dexter AMM and
              manage liquidity.
            </p>
          </div>
          <div
            className={`flex items-center gap-2.5 px-4 py-2 rounded-full border transition-all duration-500 ${
              status === "connected"
                ? "bg-green-500/10 border-green-500/20 text-green-500"
                : "bg-muted/10 border-border-low text-muted"
            }`}
          >
            <span
              className={`w-2.5 h-2.5 rounded-full ${status === "connected" ? "bg-green-500 animate-pulse" : "bg-muted/40"}`}
            />
            <span className="text-xs font-bold uppercase tracking-wider">
              {status === "connected"
                ? "Connected"
                : status === "connecting"
                  ? "Connecting"
                  : "Disconnected"}
            </span>
          </div>
        </div>

        <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
          {connectors.map((connector) => (
            <button
              key={connector.id}
              onClick={() => connect(connector.id)}
              disabled={
                status === "connecting" ||
                (status === "connected" &&
                  wallet?.connector.id === connector.id)
              }
              className={`group relative flex flex-col items-start gap-4 p-5 rounded-[22px] border transition-all duration-300 cursor-pointer overflow-hidden ${
                status === "connected" && wallet?.connector.id === connector.id
                  ? "bg-primary/5 border-primary/20 ring-1 ring-primary/10 shadow-sm"
                  : "bg-muted/5 border-transparent hover:border-border-low hover:bg-muted/10 hover:-translate-y-1 hover:shadow-md"
              } disabled:cursor-default`}
            >
              <div className="flex w-full items-center justify-between">
                <span className="font-bold text-lg">{connector.name}</span>
                {status === "connected" &&
                  wallet?.connector.id === connector.id && (
                    <div className="bg-primary text-white p-1 rounded-full">
                      <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="14"
                        height="14"
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
              </div>
              <span className="text-xs font-bold text-muted/60 uppercase tracking-wide">
                {status === "connecting"
                  ? "Linking..."
                  : status === "connected" &&
                      wallet?.connector.id === connector.id
                    ? "Connected"
                    : "Connect Wallet"}
              </span>
            </button>
          ))}
        </div>

        <div className="flex flex-wrap items-center justify-between gap-4 border-t border-border-low pt-8">
          <div className="flex items-center gap-3">
            <div className="flex flex-col gap-1">
              <span className="text-[10px] font-bold text-muted uppercase tracking-widest pl-1">
                Wallet Address
              </span>
              <div className="group/addr relative">
                <span className="inline-flex items-center rounded-xl bg-muted/5 border border-border-low px-4 py-2.5 font-mono text-xs font-bold text-foreground/80 hover:bg-muted/10 transition-colors">
                  {address
                    ? `${address.slice(0, 12)}...${address.slice(-12)}`
                    : "Not linked"}
                </span>
              </div>
            </div>
          </div>

          <div className="flex items-center gap-3">
            <button
              onClick={() => disconnect()}
              disabled={status !== "connected"}
              className="px-6 py-3 rounded-2xl font-bold text-sm text-muted hover:text-foreground hover:bg-muted/10 transition-all disabled:opacity-0 disabled:pointer-events-none cursor-pointer"
            >
              Disconnect
            </button>
          </div>
        </div>
      </div>
    </section>
  );
}
