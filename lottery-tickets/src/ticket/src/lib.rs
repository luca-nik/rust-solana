use {
    solana_program::{
        account_info::{next_account_info, AccountInfo}, 
        entrypoint, 
        entrypoint::ProgramResult, 
        msg, 
        native_token::LAMPORTS_PER_SOL,
        program::invoke,
        pubkey::Pubkey,
        system_instruction::create_account,
        system_program,
        hash::hash,
        hash::Hash,
        program::invoke_signed,
    },
    spl_token::{
        instruction as token_instruction,
    },
    spl_associated_token_account::{
        instruction as token_account_instruction,
    },
    borsh::{BorshDeserialize},
};

use crate::numbers::LotteryNumbers;

mod numbers;


entrypoint!(process_instruction);


fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    // Program ID is my program ID
    // Accounts are the accounts needed to perform the minting of the NFT
    // Instruction Data is the encoded string of numbers representing the numbers of the lottery

    const RENT_EXCEMPTION : u64 = 27074400;

    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let payer_ticket_account = next_account_info(accounts_iter)?;
    let ticket_mint_account = next_account_info(accounts_iter)?;
    let rent = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let associated_token_program = next_account_info(accounts_iter)?;

    /* Given the string of integers, i need to create a mint account and an associated mint account */
    let bump = instruction_data[0];
    let numbers_data = &instruction_data[1..];

    // Deserialize the instruction data to obtain the array of lottery numbers
    let lottery_numbers: LotteryNumbers = LotteryNumbers::try_from_slice(numbers_data)?;
    msg!("Numbers in the ticket: {:?}", lottery_numbers.values);

    msg!("Creating ticket mint account...");
    let create_account_instruction = create_account(
        &payer.key,
        &ticket_mint_account.key,
        RENT_EXCEMPTION,
        82, // Mint account size (bytes)
        &token_program.key,
    );

    invoke_signed(
        &create_account_instruction,
        &[payer.clone(), ticket_mint_account.clone()],
        &[&[&lottery_numbers.values.as_ref(), &[bump]]],
    )?;
    
    msg!("Initializing ticket mint account...");
    msg!("Ticket: {}", ticket_mint_account.key);

    // Initialize the mint account
    let initialize_mint_instruction = token_instruction::initialize_mint(
        &token_program.key,
        &ticket_mint_account.key,
        payer.key,
        Some(payer.key),
        0, // 0 Decimal for NFT
    )?;
    
    invoke_signed(
        &initialize_mint_instruction,
        &[ticket_mint_account.clone(), payer.clone(), token_program.clone(), rent.clone()],
        &[&[&lottery_numbers.values.as_ref(), &[bump]]],
    )?;
    
    msg!("Creating payer's associated ticket account...");
    invoke(
        &token_account_instruction::create_associated_token_account(
            &payer.key,
            &payer.key,
            &ticket_mint_account.key,
            &token_program.key,
        ),
        &[
            ticket_mint_account.clone(),
            payer_ticket_account.clone(),
            payer.clone(),
            token_program.clone(),
            associated_token_program.clone(),
        ]
    )?;

    msg!("Minting ticket to ticket account ...");
    msg!("Payer's associated ticket account: {}", payer_ticket_account.key);
    invoke(
        &token_instruction::mint_to(
            &token_program.key,
            &ticket_mint_account.key,
            &payer_ticket_account.key,
            &payer.key,
            &[&payer.key],
            1,
        )?,
        &[
            ticket_mint_account.clone(),
            payer.clone(),
            payer_ticket_account.clone(),
            token_program.clone(),
            rent.clone(),
        ]
    )?;

    msg!("Minting of the ticket completed");

    Ok(())
}
