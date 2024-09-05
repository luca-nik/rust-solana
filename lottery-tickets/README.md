# Lottery Tickets

In this simple example, we craft a lottery ticket using a set of 6 numbers. 
Each lottery ticket is an NFT owned by the payer. 
The address of the NFT will be uniquely determined by the numbers that the payer has set. 

This is done by creating the NFT mint account as a PDA (Program Derived Address), employing the numbers of the payer as a seed.

##  Usage
### Run the Solana Test Validator:
In a separate terminal window, start the Solana local test validator by running the command: 
```bash 
solana-test-validator
```

### Build and Deploy the Program:
In another terminal window, execute the following command to build and deploy the Rust program to the local test validator:
```bash 
npm run rebuild-and-deploy
```
This command will compile the code and deploy it on the local Solana blockchain.

### Run the Client:
To interact with the deployed program, run the client using
```bash
npm run app
```
This command will execute the client code that interacts with the on-chain program.

### Check the Results:
Open the Solana Explorer and switch the network to "localhost". Check the tokens in your account to verify the results.