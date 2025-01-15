use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Token,Mint,MintTo},
};
use anchor_spl::token;
use anchor_spl::associated_token;
use anchor_spl::associated_token::AssociatedToken;

declare_id!("7fM8upzdwQoXLDprTj1B32JqbRHNH3aVts9q2y8Dyp36");

// ================================================
// RUST SMART CONTRACT FOR SOLANA: LOTTERY TICKETS
// ================================================
//
// This Rust file is part of the Lottery Tickets Anchor project.
// It demonstrates a Solana smart contract using the Anchor framework.
//
// ------------------------------------------
// Key Features:
// 1. Mint Lottery Tickets
// 2. Use Anchor SPL libraries for token operations
// 3. Implement Program Derived Accounts (PDAs)
// 4. Ensure best practices in Solana token management
// ------------------------------------------
//
// ------------------------------------------
// File Structure:
// - Imports: Required libraries and modules
// - Declare ID: Program-specific identifier
// - #[program]: Main entry point for contract logic
// - Structs: Context and account configurations
// ------------------------------------------
//
// ## Detailed Documentation
// 1. Each ticket is uniquely identified using a combination of numbers and a bump seed.
// 2. This program ensures compatibility with Solana's token standards.
// 3. Extensively commented to serve as a learning resource.
// ================================================
#[program]
pub mod lottery_tickets_anchor {
    use super::*;

    // ------------------------------------------
    // FUNCTION: mint_ticket
    // ------------------------------------------
    //
    // This function is responsible for minting a lottery ticket.
    //
    // Parameters:
    // - ctx: Execution context for the function.
    // - numbers: Array of 6 numbers serving as a unique identifier.
    // - bump: Seed for deriving the PDA.
    //
    // Steps:
    // 1. Log the inputs for debugging purposes.
    // 2. Create associated token accounts.
    // 3. Mint a token to the created account.
    //
    // ------------------------------------------
    pub fn mint_ticket(
        ctx: Context<CreateMint>,
        numbers: [u8; 6],
        bump: u8,
    ) -> Result<()> {
        msg!("Running program with ID: {:?}", ctx.program_id);
        msg!("Numbers: {:?}", numbers);
        msg!("Bump: {:?}", bump);

        msg!("Ticket account created");
        msg!("Initializing mint account...");
        msg!("Mint: {}", &ctx.accounts.mint_account.key());
        msg!("Creating token account...");
        msg!("Token Address: {}", &ctx.accounts.token_account.key());    
        associated_token::create(
            CpiContext::new(
                ctx.accounts.associated_token_program.to_account_info(),
                associated_token::Create {
                    payer: ctx.accounts.payer.to_account_info(),
                    associated_token: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                    mint: ctx.accounts.mint_account.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
            ),
        )?;
        msg!("Minting token to token account...");
        msg!("Mint: {}", &ctx.accounts.mint_account.to_account_info().key());   
        msg!("Token Address: {}", &ctx.accounts.token_account.key());     
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint_account.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            1,
        )?;


        msg!("Token mint process completed successfully.");

        Ok(())
    }
}

// ------------------------------------------
// STRUCT: CreateMint
// ------------------------------------------
//
// This struct defines the context for the mint_ticket function.
// It includes account configurations and program relationships.
// ------------------------------------------
#[derive(Accounts)]
#[instruction(numbers : [u8;6])]
pub struct CreateMint<'info> {
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = payer.key(),
        mint::freeze_authority = payer.key(),
        seeds=[numbers.as_ref()],
        bump,
    )]
    pub mint_account: Account<'info, Mint>,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    
    #[account(mut)]
    pub payer: Signer<'info>,

    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,

}
