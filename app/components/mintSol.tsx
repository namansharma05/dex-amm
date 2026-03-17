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

export default function MintSol() {
  const { wallet } = useWalletConnection();
  const handleMintSol = async () => {
    if (!wallet) return;
    const rpcUrl = process.env.NEXT_PUBLIC_SOLANA_RPC;
    if (!rpcUrl) return;
    const rpc = createSolanaRpc(rpcUrl as any);
    const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
    const { signer } = createWalletTransactionSigner(wallet);
    try {
      const instruction = await getMintSolInstructionAsync({
        signer,
        amount: 1000000000,
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
      try {
        await sendTransactionWithoutConfirmingFactory({ rpc })(
          transactionSign,
          { commitment: "confirmed" }
        );
      } catch (e) {
        console.error("Error Sending Transaction", e);
      }
    } catch (e) {
      console.error("Error Minting SOL", e);
    }
  };
  return (
    <div>
      <h1>Mint Sol</h1>
      <input type="number" placeholder="Amount" />
      <button onClick={handleMintSol}>Mint </button>
    </div>
  );
}
