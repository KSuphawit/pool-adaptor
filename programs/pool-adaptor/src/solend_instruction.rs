use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::pubkey::PUBKEY_BYTES;
use anchor_lang::solana_program::sysvar;
use spl_token_lending::instruction::*;

/// Creates a `WithdrawObligationCollateralAndRedeemReserveCollateral` instruction.
#[allow(clippy::too_many_arguments)]
pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral(
    program_id: Pubkey,
    collateral_amount: u64,
    reserve_collateral_pubkey: Pubkey,
    user_collateral_pubkey: Pubkey,
    reserve_pubkey: Pubkey,
    obligation_pubkey: Pubkey,
    lending_market_pubkey: Pubkey,
    user_liquidity_token_pubkey: Pubkey,
    reserve_collateral_mint_pubkey: Pubkey,
    reserve_liquidity_supply_pubkey: Pubkey,
    obligation_owner_pubkey: Pubkey,
    user_transfer_authority_pubkey: Pubkey,
) -> Instruction {
    let (lending_market_authority_pubkey, _bump_seed) = Pubkey::find_program_address(
        &[&lending_market_pubkey.to_bytes()[..PUBKEY_BYTES]],
        &program_id,
    );

    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(reserve_collateral_pubkey, false),
            AccountMeta::new(user_collateral_pubkey, false),
            AccountMeta::new(reserve_pubkey, false),
            AccountMeta::new(obligation_pubkey, false),
            AccountMeta::new_readonly(lending_market_pubkey, false),
            AccountMeta::new_readonly(lending_market_authority_pubkey, false),
            AccountMeta::new(user_liquidity_token_pubkey, false),
            AccountMeta::new(reserve_collateral_mint_pubkey, false),
            AccountMeta::new(reserve_liquidity_supply_pubkey, false),
            AccountMeta::new_readonly(obligation_owner_pubkey, true),
            AccountMeta::new_readonly(user_transfer_authority_pubkey, true),
            AccountMeta::new_readonly(sysvar::clock::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: LendingInstruction::WithdrawObligationCollateralAndRedeemReserveCollateral { collateral_amount }.pack(),
    }
}