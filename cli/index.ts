import * as web3 from '@solana/web3.js';
import { createNotPDAs } from './functions';

const connection = new web3.Connection('http://127.0.0.1:8899');

async function main() {
  let first_not_pda = web3.Keypair.generate();
  let second_not_pda = web3.Keypair.generate();

  let firstCreationPayer = web3.Keypair.fromSecretKey(
    Buffer.from([
      132, 127, 30, 185, 29, 3, 227, 17, 167, 240, 145, 102, 239, 203, 111, 226,
      135, 201, 170, 15, 188, 97, 114, 109, 182, 46, 29, 183, 68, 29, 117, 92,
      231, 54, 148, 113, 156, 8, 113, 80, 149, 97, 88, 249, 85, 183, 63, 168,
      37, 168, 95, 76, 147, 220, 222, 45, 127, 202, 186, 199, 204, 234, 13, 116,
    ])
  );
  let secondCreationPayer = web3.Keypair.generate();

  let txhash = await connection.requestAirdrop(
    firstCreationPayer.publicKey,
    1e9
  );

  let blockHash = await connection.getLatestBlockhashAndContext();

  await connection.confirmTransaction({
    blockhash: blockHash.value.blockhash,
    lastValidBlockHeight: blockHash.value.lastValidBlockHeight,
    signature: txhash,
  });

  txhash = await connection.requestAirdrop(secondCreationPayer.publicKey, 1e9);

  blockHash = await connection.getLatestBlockhashAndContext();

  await connection.confirmTransaction({
    blockhash: blockHash.value.blockhash,
    lastValidBlockHeight: blockHash.value.lastValidBlockHeight,
    signature: txhash,
  });

  console.log('first_not_pda: ', first_not_pda.publicKey.toBase58());
  console.log('first_not_pda private key: ', first_not_pda.secretKey);

  await createNotPDAs(
    connection,
    first_not_pda,
    second_not_pda,
    firstCreationPayer,
    secondCreationPayer
  );
}

main()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
