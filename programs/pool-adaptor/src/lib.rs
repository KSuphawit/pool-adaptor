mod solend_adaptor;
mod solend_instruction;
mod input_struct;
use input_struct::*;
use anchor_lang::solana_program::program::invoke_signed;
use solend_adaptor::*;
use anchor_lang::prelude::*;

declare_id!("4MEHAqSFJM7uN2skG7Uj3L8REayCjSKbyovfRa9kSEW9");

#[program]
pub mod pool_adaptor {
    use super::*;

    pub fn refresh_reserve(ctx: Context<RefreshReserveInput>) -> Result<()> {
        let cpi_account = SolendRefreshReserve {
            reserve: ctx.accounts.reserve.clone(),
            reserve_liquidity_pyth_oracle: ctx.accounts.reserve_liquidity_pyth_oracle.clone(),
            reserve_liquidity_switchboard_oracle: ctx
                .accounts
                .reserve_liquidity_switchboard_oracle
                .clone(),
            clock: ctx.accounts.clock.clone(),
        };

        let cpi_context = CpiContext::new(ctx.accounts.pool_program.clone(), cpi_account);

        solend_refresh_reserve(cpi_context)
    }

    pub fn deposit_reserve_liquidity_and_obligation_collateral(
        ctx: Context<DepositReserveLiquidityAndObligationCollateralInput>,
        amount: u64,
    ) -> Result<()> {
        let cpi_account = SolendDepositReserveLiquidityAndObligationCollateral {
            source_liquidity: ctx.accounts.source_liquidity.clone(),
            user_collateral: ctx.accounts.user_collateral.clone(),
            reserve: ctx.accounts.reserve.clone(),
            reserve_liquidity_supply: ctx.accounts.reserve_liquidity_supply.clone(),
            reserve_collateral_mint: ctx.accounts.reserve_collateral_mint.clone(),
            lending_market: ctx.accounts.lending_market.clone(),
            lending_market_authority: ctx.accounts.lending_market_authority.clone(),
            destination_deposit_collateral: ctx.accounts.destination_deposit_collateral.clone(),
            obligation: ctx.accounts.obligation.clone(),
            obligation_owner: ctx.accounts.obligation_owner.clone(),
            reserve_liquidity_pyth_oracle: ctx.accounts.reserve_liquidity_pyth_oracle.clone(),
            reserve_liquidity_switchboard_oracle: ctx
                .accounts
                .reserve_liquidity_switchboard_oracle
                .clone(),
            user_transfer_authority: ctx.accounts.user_transfer_authority.clone(),
            clock: ctx.accounts.clock.clone(),
            token_program: ctx.accounts.token_program.clone(),
        };

        let cpi_context = CpiContext::new(ctx.accounts.pool_program.clone(), cpi_account);

        solend_deposit_reserve_liquidity_and_obligation_collateral(cpi_context, amount)
    }

    pub fn withdraw_obligation_collateral_and_redeem_reserve_collateral<'info>(
        ctx: Context<'_, '_, '_, 'info, WithdrawObligationCollateralAndRedeemReserveCollateralInput<'info>>,
        amount: u64,
    ) -> Result<()> {
        let cpi_account = SolendWithdrawObligationCollateralAndRedeemReserveCollateral {
            reserve_collateral: ctx.accounts.reserve_collateral.clone(),
            user_collateral: ctx.accounts.user_collateral.clone(),
            reserve: ctx.accounts.reserve.clone(),
            reserve_2: ctx.accounts.reserve_2.clone(),
            obligation: ctx.accounts.obligation.clone(),
            lending_market: ctx.accounts.lending_market.clone(),
            lending_market_authority: ctx.accounts.lending_market_authority.clone(),
            user_liquidity_token: ctx.accounts.user_liquidity_token.clone(),
            reserve_collateral_mint: ctx.accounts.reserve_collateral_mint.clone(),
            reserve_liquidity_supply: ctx.accounts.reserve_liquidity_supply.clone(),
            obligation_owner: ctx.accounts.obligation_owner.clone(),
            user_transfer_authority: ctx.accounts.user_transfer_authority.clone(),
            reserve_liquidity_pyth_oracle: ctx.accounts.reserve_liquidity_pyth_oracle.clone(),
            reserve_liquidity_pyth_oracle_2: ctx.accounts.reserve_liquidity_pyth_oracle_2.clone(),
            reserve_liquidity_switchboard_oracle: ctx.accounts.reserve_liquidity_switchboard_oracle.clone(),
            reserve_liquidity_switchboard_oracle_2: ctx.accounts.reserve_liquidity_switchboard_oracle_2.clone(),
            clock: ctx.accounts.clock.clone(),
            token_program: ctx.accounts.token_program.clone(),
        };

        let cpi_context = CpiContext::new(ctx.accounts.pool_program.clone(), cpi_account)
            .with_remaining_accounts(ctx.remaining_accounts.to_vec());

        solend_withdraw_obligation_collateral_and_redeem_reserve_collateral(cpi_context, amount)
    }
}