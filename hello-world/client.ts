import * as web3 from "@solana/web3.js";

const programId = new web3.PublicKey(
    "HZXNCxAxJKHXBPrMauJKBDeDpw29fk7Bx3CKF8jBGqZB"
  );
  
  async function sayHello(
    payer: web3.Keypair
  ): Promise<web3.TransactionSignature> {
    const transaction = new web3.Transaction();
  
    const instruction = new web3.TransactionInstruction({
      keys: [],
      programId,
    });
  
    transaction.add(instruction);
    const signature = await web3.sendAndConfirmTransaction(
      pg.connection,
      transaction,
      [payer]
    );
  
    return signature;
  }
  
  async function main() {
    const transactionSignature = sayHello(pg.wallet.keypair);
  
    console.log(
      `Transaction: https://explorer.solana.com/tx/${transactionSignature}?cluster=devnet`
    );
  }
  
  main();
  