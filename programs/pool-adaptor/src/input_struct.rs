use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RefreshReserveInput<'info> {
	/// CHECK: this is safe
	pub pool_program: AccountInfo<'info>,
	/// CHECK: this is safe
	#[account(mut)]
	pub reserve: AccountInfo<'info>,
	/// CHECK: this is safe
	#[account(mut)]
	pub reserve_liquidity_pyth_oracle: AccountInfo<'info>,
	/// CHECK: this is safe
	#[account(mut)]
	pub reserve_liquidity_switchboard_oracle: AccountInfo<'info>,
	/// CHECK: this is safe
	pub clock: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DepositReserveLiquidityAndObligationCollateralInput<'info> {
	/// CHECK: this is safe
	pub pool_program: AccountInfo<'info>,
	/// CHECK: this is safe
	#[account(mut)]
	pub source_liquidity: AccountInfo<'info>,
	/// CHECK: this is safe
	#[account(mut)]
	pub user_collateral: AccountInfo<'info>,
	/// CHECK: this is safe
	#[account(mut)]
	pub reserve: AccountInfo<'info>,
	/// CHECK: this is safe
	#[account(mut)]
	pub reserve_liquidity_supply: AccountInfo<'info>,
	/// CHECK: this is safe
	#[account(mut)]
	pub reserve_collateral_mint: AccountInfo<'info>,
	/// CHECK: this is safe
	pub lending_market: AccountInfo<'info>,
	/// CHECK: this is safe
	pub lending_market_authority: AccountInfo<'info>,
	/// CHECK: this is safe
	#[account(mut)]
	pub destination_deposit_collateral: AccountInfo<'info>,
	/// CHECK: this is safe
	#[account(mut)]
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
pub struct WithdrawObligationCollateralAndRedeemReserveCollateralInput<'info> {
	/// CHECK: this is safe
	pub pool_program: AccountInfo<'info>,
	/// CHECK: this is safe
	#[account(mut)]
	pub reserve_collateral: AccountInfo<'info>,
	/// CHECK: this is safe
	#[account(mut)]
	pub user_collateral: AccountInfo<'info>,
	/// CHECK: this is safe
	#[account(mut)]
	pub reserve: AccountInfo<'info>,
	/// CHECK: this is safe
	#[account(mut)]
	pub reserve_2: AccountInfo<'info>,
	/// CHECK: this is safe
	#[account(mut)]
	pub obligation: AccountInfo<'info>,
	/// CHECK: this is safe
	pub lending_market: AccountInfo<'info>,
	/// CHECK: this is safe
	pub lending_market_authority: AccountInfo<'info>,
	/// CHECK: this is safe
	#[account(mut)]
	pub user_liquidity_token: AccountInfo<'info>,
	/// CHECK: this is safe
	#[account(mut)]
	pub reserve_collateral_mint: AccountInfo<'info>,
	/// CHECK: this is safe
	#[account(mut)]
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
