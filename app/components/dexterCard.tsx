export default function DexterCard() {
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
            <button className="flex items-center gap-2 bg-primary text-white p-1 pr-3 rounded-full font-bold shadow-lg shadow-primary/20 hover:scale-105 active:scale-95 transition-all shrink-0">
              <div className="w-8 h-8 bg-white/20 rounded-full flex items-center justify-center overflow-hidden">
                <svg
                  viewBox="0 0 24 24"
                  fill="none"
                  xmlns="http://www.w3.org/2000/svg"
                  className="w-5 h-5 fill-white"
                >
                  <path d="M12 2C6.477 2 2 6.477 2 12s4.477 10 10 10 10-4.477 10-10S17.523 2 12 2z" />
                </svg>
              </div>
              <span className="text-base">Select token</span>
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
                className="opacity-80"
              >
                <path d="m6 9 6 6 6-6" />
              </svg>
            </button>
          </div>
          <div className="flex justify-between items-center text-sm font-medium text-muted/60">
            <span>$0</span>
          </div>
        </div>

        {/* Swap Arrow */}
        <div className="relative -my-4 z-10 flex justify-center">
          <button className="bg-card border-[4px] border-card rounded-xl p-2 shadow-md hover:scale-110 active:scale-90 transition-all group">
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
            <button className="flex items-center gap-2 bg-card hover:bg-muted/5 text-foreground border border-border-low p-1 pr-3 rounded-full font-bold transition-all shadow-sm shrink-0">
              <div className="w-8 h-8 bg-blue-500 rounded-full flex items-center justify-center">
                <svg
                  viewBox="0 0 24 24"
                  fill="none"
                  xmlns="http://www.w3.org/2000/svg"
                  className="w-5 h-5 fill-white"
                >
                  <path d="M12 2C6.477 2 2 6.477 2 12s4.477 10 10 10 10-4.477 10-10S17.523 2 12 2z" />
                </svg>
              </div>
              <span className="text-base">SED</span>
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
                className="opacity-60"
              >
                <path d="m6 9 6 6 6-6" />
              </svg>
            </button>
          </div>
          <div className="flex justify-between items-center text-sm font-medium text-muted/60">
            <span>$0</span>
          </div>
        </div>

        {/* Main Action Button */}
        <button className="w-full bg-primary/10 text-primary hover:bg-primary/20 disabled:bg-muted/5 disabled:text-muted/30 py-4 rounded-[20px] font-bold text-lg transition-all active:scale-[0.98] shadow-sm">
          Connect Wallet
        </button>
      </div>
    </div>
  );
}
