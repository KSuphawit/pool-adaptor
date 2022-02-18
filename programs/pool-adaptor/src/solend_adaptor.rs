use anchor_lang::context::CpiContext;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::{invoke, invoke_signed};
use spl_token_lending::instruction::*;
use crate::AccountInfo;


pub fn solend_refresh_reserve<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, SolendRefreshReserve<'info>>,
) -> ProgramResult {
    let ix = refresh_reserve(
        spl_token_lending::id(),
        ctx.accounts.reserve.key(),
        ctx.accounts.reserve_liquidity_pyth_oracle.key(),
        ctx.accounts.reserve_liquidity_switchboard_oracle.key(),
    );

    invoke(
        &ix,
        &[
            ctx.accounts.reserve,
            ctx.accounts.reserve_liquidity_pyth_oracle,
            ctx.accounts.reserve_liquidity_switchboard_oracle,
            ctx.accounts.clock,
        ],
    )
}


pub fn solend_deposit_reserve_liquidity_and_obligation_collateral<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, SolendDepositReserveLiquidityAndObligationCollateral<'info>>,
    amount: u64,
) -> ProgramResult {
    let ix = deposit_reserve_liquidity_and_obligation_collateral(
        spl_token_lending::id(),
        amount,
        ctx.accounts.source_liquidity.key(),
        ctx.accounts.user_collateral.key(),
        ctx.accounts.reserve.key(),
        ctx.accounts.reserve_liquidity_supply.key(),
        ctx.accounts.reserve_collateral_mint.key(),
        ctx.accounts.lending_market.key(),
        ctx.accounts.destination_deposit_collateral.key(),
        ctx.accounts.obligation.key(),
        ctx.accounts.obligation_owner.key(),
        ctx.accounts.reserve_liquidity_pyth_oracle.key(),
        ctx.accounts.reserve_liquidity_switchboard_oracle.key(),
        ctx.accounts.user_transfer_authority.key(),
    );

    invoke_signed(
        &ix,
        &[
            ctx.accounts.source_liquidity,
            ctx.accounts.user_collateral,
            ctx.accounts.reserve,
            ctx.accounts.reserve_liquidity_supply,
            ctx.accounts.reserve_collateral_mint,
            ctx.accounts.lending_market,
            ctx.accounts.lending_market_authority,
            ctx.accounts.destination_deposit_collateral,
            ctx.accounts.obligation,
            ctx.accounts.obligation_owner,
            ctx.accounts.reserve_liquidity_pyth_oracle,
            ctx.accounts.reserve_liquidity_switchboard_oracle,
            ctx.accounts.user_transfer_authority,
            ctx.accounts.clock,
            ctx.accounts.token_program,
        ],
        ctx.signer_seeds,
    )
}

#[derive(Accounts)]
pub struct SolendRefreshReserve<'info> {
    pub reserve: AccountInfo<'info>,
    pub reserve_liquidity_pyth_oracle: AccountInfo<'info>,
    pub reserve_liquidity_switchboard_oracle: AccountInfo<'info>,
    pub clock: AccountInfo<'info>,
}


#[derive(Accounts)]
pub struct SolendDepositReserveLiquidityAndObligationCollateral<'info> {
    pub source_liquidity: AccountInfo<'info>,
    pub user_collateral: AccountInfo<'info>,
    pub reserve: AccountInfo<'info>,
    pub reserve_liquidity_supply: AccountInfo<'info>,
    pub reserve_collateral_mint: AccountInfo<'info>,
    pub lending_market: AccountInfo<'info>,
    pub lending_market_authority: AccountInfo<'info>,
    pub destination_deposit_collateral: AccountInfo<'info>,
    pub obligation: AccountInfo<'info>,
    pub obligation_owner: AccountInfo<'info>,
    pub reserve_liquidity_pyth_oracle: AccountInfo<'info>,
    pub reserve_liquidity_switchboard_oracle: AccountInfo<'info>,
    pub user_transfer_authority: AccountInfo<'info>,
    pub clock: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}