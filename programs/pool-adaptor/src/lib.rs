mod solend_adaptor;

use crate::solend_adaptor::*;
use anchor_lang::prelude::*;

declare_id!("8hqwuvN3Wcyoj2haZgdMzzrwLLX47ERxHZunTn87Ee3p");

#[program]
pub mod pool_adaptor {
    use super::*;

    pub fn refresh_reserve(ctx: Context<RefreshReserveInput>) -> ProgramResult {

        let cpi_account = SolendRefreshReserve {
            reserve: ctx.accounts.reserve.clone(),
            reserve_liquidity_pyth_oracle: ctx.accounts.reserve_liquidity_pyth_oracle.clone(),
            reserve_liquidity_switchboard_oracle: ctx.accounts.reserve_liquidity_switchboard_oracle.clone(),
            clock: ctx.accounts.clock.clone()
        };

        let cpi_context =  CpiContext::new(ctx.accounts.pool_program.clone(), cpi_account);

        solend_refresh_reserve(cpi_context)
    }

    pub fn deposit_reserve_liquidity_and_obligation_collateral(ctx: Context<DepositReserveLiquidityAndObligationCollateralInput>, amount: u64) -> ProgramResult {

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
            reserve_liquidity_switchboard_oracle: ctx.accounts.reserve_liquidity_switchboard_oracle.clone(),
            user_transfer_authority: ctx.accounts.user_transfer_authority.clone(),
            clock: ctx.accounts.clock.clone(),
            token_program: ctx.accounts.token_program.clone(),
        };

        let cpi_context = CpiContext::new(ctx.accounts.pool_program.clone(), cpi_account);

        solend_deposit_reserve_liquidity_and_obligation_collateral(cpi_context, amount)
    }
}


#[derive(Accounts)]
pub struct RefreshReserveInput<'info> {
    pub pool_program: AccountInfo<'info>,
    #[account(mut)]
    pub reserve: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_liquidity_pyth_oracle: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_liquidity_switchboard_oracle: AccountInfo<'info>,
    pub clock: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DepositReserveLiquidityAndObligationCollateralInput<'info> {
    pub pool_program: AccountInfo<'info>,
    #[account(mut)]
    pub source_liquidity: AccountInfo<'info>,
    #[account(mut)]
    pub user_collateral: AccountInfo<'info>,
    #[account(mut)]
    pub reserve: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_liquidity_supply: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_collateral_mint: AccountInfo<'info>,
    pub lending_market: AccountInfo<'info>,
    pub lending_market_authority: AccountInfo<'info>,
    #[account(mut)]
    pub destination_deposit_collateral: AccountInfo<'info>,
    #[account(mut)]
    pub obligation: AccountInfo<'info>,
    pub obligation_owner: AccountInfo<'info>,
    pub reserve_liquidity_pyth_oracle: AccountInfo<'info>,
    pub reserve_liquidity_switchboard_oracle: AccountInfo<'info>,
    pub user_transfer_authority: AccountInfo<'info>,
    pub signer: Signer<'info>,
    pub clock: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}