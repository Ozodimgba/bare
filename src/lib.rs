use pinocchio::{
    account_info::AccountInfo, entrypoint, msg, program_error::ProgramError, pubkey::Pubkey, ProgramResult
};
use sanity::declare_program;

declare_program! {
    name = "spl_token",
    id = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    idl_path = "idls/spl_token.json",  
    idl_version = 1
}

declare_program! {
    name = "jupiter",
    id = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4",
    idl_path = "idls/jupiter.json",
    idl_version = 2,
}

declare_program! {
    name = "raydium",
    id = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4",
    idl_path = "idls/ray.json",
    idl_version = 1,
}

declare_program! {
    name = "pump",
    id = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4",
    idl_path = "idls/pump.json",
    idl_version = 2,
}


entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = instruction_data[0];
    
    match instruction {
        0 => handle_token_transfer(accounts)?,
        1 => handle_token_mint(accounts)?,
        _ => return Err(ProgramError::InvalidInstructionData),
    }
    
    Ok(())
}

fn handle_token_transfer(accounts: &[AccountInfo]) -> ProgramResult {
    let source_account = &accounts[0];
    let destination_account = &accounts[1];
    let authority_account = &accounts[2];
    
    let amount: u64 = 1000;
    let amount_bytes = amount.to_le_bytes().to_vec(); // args

    spl_token::transfer(
        source_account,
        destination_account, 
        authority_account,
        amount_bytes
    )?;

    msg!("Transfer completed successfully");
    Ok(())
}

fn handle_token_mint(accounts: &[AccountInfo]) -> ProgramResult {
    let mint_account = &accounts[0];
    let destination_account = &accounts[1];
    let authority_account = &accounts[2];
    
    let amount: u64 = 5000;
    let amount_bytes = amount.to_le_bytes().to_vec(); // args

    
    spl_token::mintTo(
        mint_account,
        destination_account,
        authority_account,
        amount_bytes
    )?;

    /// raydium::createPool(poolCreator, ammConfig, poolState, tokenMint0, tokenMint1, tokenVault0, tokenVault1, observationState, tokenProgram, systemProgram, rent, sqrtPriceX64)
    /// jupiter::route(token_program, user_transfer_authority, user_source_token_account, user_destination_token_account, destination_token_account, destination_mint, platform_fee_account, event_authority, program, route_plan, in_amount, quoted_out_amount, slippage_bps, platform_fee_bps)
    /// pump::create(mint, mint_authority, bonding_curve, associated_bonding_curve, global, mpl_token_metadata, metadata, user, system_program, token_program, associated_token_program, rent, event_authority, program, name, symbol, uri, creator)
    
    msg!("Mint completed successfully");
    Ok(())
}