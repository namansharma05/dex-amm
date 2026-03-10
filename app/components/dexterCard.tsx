export default function DexterCard() {
  return (
    <div className="w-full flex items-center justify-center">
      <div className="w-[400px] h-[200px] bg-card rounded-lg shadow-lg">
        <div className="flex flex-col items-center justify-center">
          <div className="flex items-center justify-center">
            <input type="text" placeholder="Enter amount" />
          </div>
          <div className="swap-emoji-class">
            <button>swap emoji comes here</button>
          </div>
          <div className="flex items-center justify-center">
            <input type="text" placeholder="Enter amount" />
          </div>
        </div>
      </div>
    </div>
  );
}
