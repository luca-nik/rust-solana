import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    getAssociatedTokenAddress,
    TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import {
    Connection,
    Keypair,
    PublicKey,
    SystemProgram,
    SYSVAR_RENT_PUBKEY,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
} from '@solana/web3.js';
import {
    createKeypairFromFile,
} from './util';
import {
    createNumbersInstruction,
} from './numbers'
import fs from 'mz/fs';
import os from 'os';
import path from 'path';
import yaml from 'yaml';


// Path to local Solana CLI config file.
const CONFIG_FILE_PATH = path.resolve(
    os.homedir(),
    '.config',
    'solana',
    'cli',
    'config.yml',
);

let connection: Connection;
let localKeypair: Keypair;
let programKeypair: Keypair;
let programId: PublicKey;


const PROGRAM_PATH = path.resolve(__dirname, '../../dist/program');

export async function connect() {
    connection = new Connection('http://127.0.0.1:8899', 'confirmed');

    console.log(`Successfully connected to Solana dev net.`);
}

async function checkAddressExists(publicKey: PublicKey): Promise<boolean> {
    try {
      // Fetch account information for the specified public key
      const accountInfo = await connection.getAccountInfo(publicKey);
      
      // If accountInfo is not null, the account exists
      return accountInfo !== null;
    } catch (error) {
      console.error("Error checking if address exists:", error);
      return false;
    }
  }


// Use local keypair for client, load my data
export async function getLocalAccount() {
    //in config.yml there is the address to my account keypair
    const configYml = await fs.readFile(CONFIG_FILE_PATH, {encoding: 'utf8'});
    const keypairPath = await yaml.parse(configYml).keypair_path;
    localKeypair = await createKeypairFromFile(keypairPath);

    console.log(`Local account loaded successfully.`);
    console.log(`Local account's address is:`);
    console.log(`   ${localKeypair.publicKey}`);
}

export async function runProgram(lotteryNumbers : Uint8Array) {
    
    // In this example the payer is the account on the machine
    let payer = localKeypair

    // setup pda, it will host the ticket mint
    let [mint_address, bump] = await PublicKey.findProgramAddressSync(
      [Buffer.from(lotteryNumbers)],
      programId,
    );

    // Check if the ticket already exists
    const exists = await checkAddressExists(mint_address);
    if (exists) {
        console.log(`Ticket address ${mint_address.toBase58()} already exists. Change numbers. Exiting the program.`);
        process.exit(1); // Exit the program with status code 0 (success)
    }
    console.log(`Crafitng the ticket ${mint_address.toBase58()} onchain`);


    // Get the address of the account owned by payer that hosts the NFT
    const ticketAddress = await getAssociatedTokenAddress(
        mint_address,
        payer.publicKey
    );

    // Set up the transaction
    const instruction = new TransactionInstruction({
        keys: [
            // Ticket Payer
            {
                pubkey: payer.publicKey,
                isSigner: true,
                isWritable: false,
            },
            // Ticket account
            {
                pubkey: ticketAddress,
                isSigner: false,
                isWritable: true,
            },
            // PDA
            {
                pubkey: mint_address,
                isSigner: false,
                isWritable: true,
            },
            // Rent account
            {
                pubkey: SYSVAR_RENT_PUBKEY,
                isSigner: false,
                isWritable: false,
            },
            // System program
            {
                pubkey: SystemProgram.programId,
                isSigner: false,
                isWritable: false,
            },
            // Token program
            {
                pubkey: TOKEN_PROGRAM_ID,
                isSigner: false,
                isWritable: false,
            },
            // Associated token program
            {
                pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
                isSigner: false,
                isWritable: false,
            },
        ],
        programId: programId,
        data: Buffer.concat([Buffer.from(new Uint8Array([bump])), Buffer.from(lotteryNumbers)]),
    })
    
    await sendAndConfirmTransaction(
        connection,
        new Transaction().add(instruction),
        [payer], // Pass singer
    )
}

export async function main() {
    await connect();
    await getLocalAccount();

    // Load program ID
    programKeypair = await createKeypairFromFile(
        path.join(PROGRAM_PATH, 'ticket-keypair.json')
    );
    programId = programKeypair.publicKey;
    console.log(`Program ID: ${programId.toBase58()}`);

    // Set numbers and run
    let numbers = new Uint8Array([1,2,3,2,5,17]);

    // Run the program
    await runProgram(numbers); 
}

main().then(
    () => process.exit(),
    err => {
      console.error(err);
      process.exit(-1);
    },
  );