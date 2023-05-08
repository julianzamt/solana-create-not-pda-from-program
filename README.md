# Solana Create A Not PDA Account From A Program
This program proves that is possible to create a Not PDA Account from inside a Solana Program,
having the account to be created as a signer of the original tx. Also, that is not necessarily the FeePayer the account that 
must be used to pay for those creations.

## Usage
Pre-Requirements:
* solana cli
* nodeJS

### From the root folder:
1 - cd create-not-pda/cli && npm install && cd ../..
2 - solana-test-validator -r (This will start a local validator)  

### In another terminal:
3 - `cd create-not-pda/ && cargo build-sbf && solana program deploy target/deploy/create-not-pda.so --url localhost && cd ..`  
4 - Paste the returned programId into the PROGRAM_ID field in create-not-pda/cli/constants.ts   
5 - In create-not-pda/cli/constant.ts, insert your test private key. Try: $cat ~/.config/solana/id.json (If you don't have one yet, run `solana-keygen new` first)  
6 - Open create-not-pda/cli/index.ts. You'll find function calls to send txs to the deployed contracts.  
7 - Comment and uncomment fns to have a taste of what you can and cannot do.  
8 - npx ts-node cli/index.ts