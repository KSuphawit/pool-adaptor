use anchor_lang::context::CpiContext;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::{invoke, invoke_signed};
use spl_token_lending::instruction::*;
use crate::AccountInfo;
use crate::solend_instruction::withdraw_obligation_collateral_and_redeem_reserve_collateral;


pub fn solend_refresh_reserve<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, SolendRefreshReserve<'info>>,
) -> Result<()> {
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
    ).map_err(Into::into)
}


pub fn solend_deposit_reserve_liquidity_and_obligation_collateral<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, SolendDepositReserveLiquidityAndObligationCollateral<'info>>,
    amount: u64,
) -> Result<()> {
    let ix_deposit = deposit_reserve_liquidity_and_obligation_collateral(
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
        &ix_deposit,
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
    ).map_err(Into::into)
}

pub fn solend_withdraw_obligation_collateral_and_redeem_reserve_collateral<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, SolendWithdrawObligationCollateralAndRedeemReserveCollateral<'info>>,
    amount: u64,
) -> Result<()> {
    //Refresh Reserve
    let ix_refresh_reserve = refresh_reserve(
        spl_token_lending::id(),
        ctx.accounts.reserve.key(),
        ctx.accounts.reserve_liquidity_pyth_oracle.key(),
        ctx.accounts.reserve_liquidity_switchboard_oracle.key(),
    );

    invoke(
        &ix_refresh_reserve,
        &[
            ctx.accounts.reserve.clone(),
            ctx.accounts.reserve_liquidity_pyth_oracle,
            ctx.accounts.reserve_liquidity_switchboard_oracle,
            ctx.accounts.clock.clone(),
        ],
    )?;

    //Refresh Reserve
    let ix_refresh_reserve_2 = refresh_reserve(
        spl_token_lending::id(),
        ctx.accounts.reserve_2.key(),
        ctx.accounts.reserve_liquidity_pyth_oracle_2.key(),
        ctx.accounts.reserve_liquidity_switchboard_oracle_2.key(),
    );

    invoke(
        &ix_refresh_reserve_2,
        &[
            ctx.accounts.reserve_2.clone(),
            ctx.accounts.reserve_liquidity_pyth_oracle_2,
            ctx.accounts.reserve_liquidity_switchboard_oracle_2,
            ctx.accounts.clock.clone(),
        ],
    )?;



    //Refresh Obligation
    let reserves = ctx.remaining_accounts;

    let ix_refresh_obligation = refresh_obligation(
        spl_token_lending::id(),
        ctx.accounts.obligation.key(),
        reserves.iter().map(|info| info.key()).collect(),
    );

    let mut refresh_obligation_accounts_info = vec![ctx.accounts.obligation.clone(), ctx.accounts.clock.clone()];
    refresh_obligation_accounts_info.extend(reserves);
    refresh_obligation_accounts_info.push(ctx.program);
    invoke(&ix_refresh_obligation, &refresh_obligation_accounts_info)?;

    // //Withdraw and redeem
    let ix_withdraw = withdraw_obligation_collateral_and_redeem_reserve_collateral(
        spl_token_lending::id(),
        amount,
        ctx.accounts.reserve_collateral.key(),
        ctx.accounts.user_collateral.key(),
        ctx.accounts.reserve.key(),
        ctx.accounts.obligation.key(),
        ctx.accounts.lending_market.key(),
        ctx.accounts.user_liquidity_token.key(),
        ctx.accounts.reserve_collateral_mint.key(),
        ctx.accounts.reserve_liquidity_supply.key(),
        ctx.accounts.obligation_owner.key(),
        ctx.accounts.user_transfer_authority.key(),
    );

    invoke_signed(
        &ix_withdraw,
        &[
            ctx.accounts.reserve_collateral,
            ctx.accounts.user_collateral,
            ctx.accounts.reserve.clone(),
            ctx.accounts.obligation.clone(),
            ctx.accounts.lending_market,
            ctx.accounts.lending_market_authority,
            ctx.accounts.user_liquidity_token,
            ctx.accounts.reserve_collateral_mint,
            ctx.accounts.reserve_liquidity_supply,
            ctx.accounts.obligation_owner,
            ctx.accounts.user_transfer_authority,
            ctx.accounts.clock.clone(),
            ctx.accounts.token_program,
        ],
        ctx.signer_seeds,
    ).map_err(Into::into)
}

#[derive(Accounts)]
pub struct SolendRefreshReserve<'info> {
    /// CHECK: this is safe
    pub reserve: AccountInfo<'info>,
    /// CHECK: this is safe
    pub reserve_liquidity_pyth_oracle: AccountInfo<'info>,
    /// CHECK: this is safe
    pub reserve_liquidity_switchboard_oracle: AccountInfo<'info>,
    /// CHECK: this is safe
    pub clock: AccountInfo<'info>,
}


#[derive(Accounts)]
pub struct SolendDepositReserveLiquidityAndObligationCollateral<'info> {
    /// CHECK: this is safe
    pub source_liquidity: AccountInfo<'info>,
    /// CHECK: this is safe
    pub user_collateral: AccountInfo<'info>,
    /// CHECK: this is safe
    pub reserve: AccountInfo<'info>,
    /// CHECK: this is safe
    pub reserve_liquidity_supply: AccountInfo<'info>,
    /// CHECK: this is safe
    pub reserve_collateral_mint: AccountInfo<'info>,
    /// CHECK: this is safe
    pub lending_market: AccountInfo<'info>,
    /// CHECK: this is safe
    pub lending_market_authority: AccountInfo<'info>,
    /// CHECK: this is safe
    pub destination_deposit_collateral: AccountInfo<'info>,
    /// CHECK: this is safe
    pub obligation: AccountInfo<'info>,
    /// CHECK: this is safe
    pub obligation_owner: AccountInfo<'info>,
    /// CHECK: this is safe
    pub reserve_liquidity_pyth_oracle: AccountInfo<'info>,
    /// CHECK: this is safe
    pub reserve_liquidity_switchboard_oracle: AccountInfo<'info>,
    /// CHECK: this is safe
    pub user_transfer_authority: AccountInfo<'info>,
    /// CHECK: this is safe
    pub clock: AccountInfo<'info>,
    /// CHECK: this is safe
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SolendWithdrawObligationCollateralAndRedeemReserveCollateral<'info> {
    /// CHECK: this is safe
    pub reserve_collateral: AccountInfo<'info>,
    /// CHECK: this is safe
    pub user_collateral: AccountInfo<'info>,
    /// CHECK: this is safe
    pub reserve: AccountInfo<'info>,
    /// CHECK: this is safe
    pub reserve_2: AccountInfo<'info>,
    /// CHECK: this is safe
    pub obligation: AccountInfo<'info>,
    /// CHECK: this is safe
    pub lending_market: AccountInfo<'info>,
    /// CHECK: this is safe
    pub lending_market_authority: AccountInfo<'info>,
    /// CHECK: this is safe
    pub user_liquidity_token: AccountInfo<'info>,
    /// CHECK: this is safe
    pub reserve_collateral_mint: AccountInfo<'info>,
    /// CHECK: this is safe
    pub reserve_liquidity_supply: AccountInfo<'info>,
    /// CHECK: this is safe
    pub obligation_owner: AccountInfo<'info>,
    /// CHECK: this is safe
    pub user_transfer_authority: AccountInfo<'info>,
    /// CHECK: this is safe
    pub reserve_liquidity_pyth_oracle: AccountInfo<'info>,
    /// CHECK: this is safe
    pub reserve_liquidity_pyth_oracle_2: AccountInfo<'info>,
    /// CHECK: this is safe
    pub reserve_liquidity_switchboard_oracle: AccountInfo<'info>,
    /// CHECK: this is safe
    pub reserve_liquidity_switchboard_oracle_2: AccountInfo<'info>,
    /// CHECK: this is safe
    pub clock: AccountInfo<'info>,
    /// CHECK: this is safe
    pub token_program: AccountInfo<'info>,
}