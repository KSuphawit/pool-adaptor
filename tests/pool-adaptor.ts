import * as anchor from '@project-serum/anchor';
import {BN, Program} from '@project-serum/anchor';
import {PoolAdaptor} from '../target/types/pool_adaptor';
import {SYSVAR_CLOCK_PUBKEY} from "@solana/web3.js";
import {TOKEN_PROGRAM_ID} from "@solana/spl-token";


// setting
const provider = anchor.Provider.env();
anchor.setProvider(provider);

// program
const program = anchor.workspace.PoolAdaptor as Program<PoolAdaptor>;
const SOLEND_DEV_PROGRAM_ID = new anchor.web3.PublicKey("ALend7Ketfx5bxh6ghsCDXAoDrhvEmsXT3cynB6aPLgx")


// refresh_reserve
const refreshReserve0 = new anchor.web3.PublicKey("FNNkz4RCQezSSS71rW2tvqZH1LCkTzaiG7Nd1LeA5x5y")
const refreshReserve1 = new anchor.web3.PublicKey("5SSkXsEKQepHHAewytPVwdej4epN1nxgLVM84L4KXgy7")
const refreshReserve2 = new anchor.web3.PublicKey("CZx29wKMUxaJDq6aLVQTdViPL754tTR64NAgQBUGxxHb")

// deposit
const deposit0 = new anchor.web3.PublicKey("39XieSM6DMrE3sANqGeuASxe1Yw1hKPftSMKCdLm1e6W")
const deposit1 = new anchor.web3.PublicKey("GoKbUcCHTTWS68mMa2j6v1E1n1gSosNCyUQpg2Z7Vhqj")
const deposit2 = new anchor.web3.PublicKey("FNNkz4RCQezSSS71rW2tvqZH1LCkTzaiG7Nd1LeA5x5y")
const deposit3 = new anchor.web3.PublicKey("HixjFJoeD2ggqKgFHQxrcJFjVvE5nXKuUPYNijFg7Kc5")
const deposit4 = new anchor.web3.PublicKey("E2PSSXsXJGdpqhhaV3rYPpuy1inRCQAWxcdykA1DTmYr")
const deposit5 = new anchor.web3.PublicKey("GvjoVKNjBvQcFaSKUW1gTE7DxhSpjHbE69umVR5nPuQp")
const deposit6 = new anchor.web3.PublicKey("EhJ4fwaXUp7aiwvZThSUaGWCaBQAJe3AEaJJJVCn3UCK")
const deposit7 = new anchor.web3.PublicKey("FiUyeMAnZYkLCbrPGwjJWQwzYJ5AjD6p9N9fx6VxDPMt")
const deposit8 = new anchor.web3.PublicKey("7WYn4tFoq38sVAEpdbdMsJ97YMZiexur9cUAetWnAhSt")
const deposit9 = new anchor.web3.PublicKey("4YPTC5LFiWNjcJjnaKSnZQgXEnGWgDcxwSLdozvCUUqE")
const deposit10 = new anchor.web3.PublicKey("5SSkXsEKQepHHAewytPVwdej4epN1nxgLVM84L4KXgy7")
const deposit11 = new anchor.web3.PublicKey("CZx29wKMUxaJDq6aLVQTdViPL754tTR64NAgQBUGxxHb")
const deposit12 = new anchor.web3.PublicKey("4YPTC5LFiWNjcJjnaKSnZQgXEnGWgDcxwSLdozvCUUqE")



// withdraw
const withdraw0 = new anchor.web3.PublicKey("FiUyeMAnZYkLCbrPGwjJWQwzYJ5AjD6p9N9fx6VxDPMt")
const withdraw1 = new anchor.web3.PublicKey("GoKbUcCHTTWS68mMa2j6v1E1n1gSosNCyUQpg2Z7Vhqj")
const withdraw2 = new anchor.web3.PublicKey("FNNkz4RCQezSSS71rW2tvqZH1LCkTzaiG7Nd1LeA5x5y")
const withdraw3 = new anchor.web3.PublicKey("7WYn4tFoq38sVAEpdbdMsJ97YMZiexur9cUAetWnAhSt")
const withdraw4 = new anchor.web3.PublicKey("GvjoVKNjBvQcFaSKUW1gTE7DxhSpjHbE69umVR5nPuQp")
const withdraw5 = new anchor.web3.PublicKey("EhJ4fwaXUp7aiwvZThSUaGWCaBQAJe3AEaJJJVCn3UCK")
const withdraw6 = new anchor.web3.PublicKey("39XieSM6DMrE3sANqGeuASxe1Yw1hKPftSMKCdLm1e6W")
const withdraw7 = new anchor.web3.PublicKey("E2PSSXsXJGdpqhhaV3rYPpuy1inRCQAWxcdykA1DTmYr")
const withdraw8 = new anchor.web3.PublicKey("HixjFJoeD2ggqKgFHQxrcJFjVvE5nXKuUPYNijFg7Kc5")
const withdraw9 = new anchor.web3.PublicKey("4YPTC5LFiWNjcJjnaKSnZQgXEnGWgDcxwSLdozvCUUqE")
const withdraw10 = new anchor.web3.PublicKey("4YPTC5LFiWNjcJjnaKSnZQgXEnGWgDcxwSLdozvCUUqE")


const reserveLiquidityPythOracle = new anchor.web3.PublicKey("5SSkXsEKQepHHAewytPVwdej4epN1nxgLVM84L4KXgy7");
const reserveLiquiditySwitchboardOracle = new anchor.web3.PublicKey("CZx29wKMUxaJDq6aLVQTdViPL754tTR64NAgQBUGxxHb");

const reserve2 = new anchor.web3.PublicKey("5VVLD7BQp8y3bTgyF5ezm1ResyMTR3PhYsT4iHFU8Sxz")
const reserveLiquidityPythOracle2 = new anchor.web3.PublicKey("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix");
const reserveLiquiditySwitchboardOracle2 = new anchor.web3.PublicKey("AdtRGGhmqvom3Jemp5YNrxd9q9unX36BZk1pujkkXijL")

describe('pool-adaptor', () => {

    it('deposit_reserve_liquidity_and_obligation_collateral', async () => {
        const depositAmount = Math.pow(10, 6)

        const txDepositReserveLiquidityAndObligationCollateral = await program.rpc.depositReserveLiquidityAndObligationCollateral(
            new BN(depositAmount),
            {
                accounts: {
                    poolProgram: SOLEND_DEV_PROGRAM_ID,
                    sourceLiquidity: deposit0,
                    userCollateral: deposit1,
                    reserve: deposit2,
                    reserveLiquiditySupply: deposit3,
                    reserveCollateralMint: deposit4,
                    lendingMarket: deposit5,
                    lendingMarketAuthority: deposit6,
                    destinationDepositCollateral: deposit7,
                    obligation: deposit8,
                    obligationOwner: deposit9,
                    reserveLiquidityPythOracle: deposit10,
                    reserveLiquiditySwitchboardOracle: deposit11,
                    userTransferAuthority: deposit12,
                    clock: SYSVAR_CLOCK_PUBKEY,
                    tokenProgram: TOKEN_PROGRAM_ID
                }
            }
        )

        console.log("txDepositReserveLiquidityAndObligationCollateral : ", txDepositReserveLiquidityAndObligationCollateral)
    });


    it('withdraw_obligation_collateral_and_redeem_reserve_collateral', async () => {
        const withdrawAmount = 2 * Math.pow(10, 6)

        const txWithdrawObligationCollateralAndRedeemReserveCollateral = await program.rpc.withdrawObligationCollateralAndRedeemReserveCollateral(
            new BN(withdrawAmount),
            {
                accounts: {
                    poolProgram: SOLEND_DEV_PROGRAM_ID,
                    reserveCollateral: withdraw0,
                    userCollateral: withdraw1,
                    reserve: withdraw2,
                    reserve2: reserve2,
                    obligation: withdraw3,
                    lendingMarket: withdraw4,
                    lendingMarketAuthority: withdraw5,
                    userLiquidityToken: withdraw6,
                    reserveCollateralMint: withdraw7,
                    reserveLiquiditySupply: withdraw8,
                    obligationOwner: withdraw9,
                    userTransferAuthority: withdraw10,
                    reserveLiquidityPythOracle: reserveLiquidityPythOracle,
                    reserveLiquidityPythOracle2: reserveLiquidityPythOracle2,
                    reserveLiquiditySwitchboardOracle: reserveLiquiditySwitchboardOracle,
                    reserveLiquiditySwitchboardOracle2: reserveLiquiditySwitchboardOracle2,
                    clock: SYSVAR_CLOCK_PUBKEY,
                    tokenProgram: TOKEN_PROGRAM_ID
                },
                remainingAccounts: [
                    {pubkey: reserve2, isSigner: false, isWritable: false},
                    {pubkey: withdraw2, isSigner: false, isWritable: false},
                    {pubkey: withdraw2, isSigner: false, isWritable: false}
                ]
            }
        )

        console.log("txWithdrawObligationCollateralAndRedeemReserveCollateral : ", txWithdrawObligationCollateralAndRedeemReserveCollateral)
    });
});