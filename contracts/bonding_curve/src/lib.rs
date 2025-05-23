use anchor_lang::prelude::*;

pub mod errors;
pub mod utils;
pub mod instructions;
pub mod state;
pub mod consts;
pub mod raydium;

use crate::instructions::*;
use crate::raydium::*;

declare_id!("5mdPUgyK9mqosLtqZvfpY5pcpCqQBWHuS3XoU34CrJK3");

#[program]
pub mod bonding_curve {
    use super::*;

    pub fn initialize(ctx: Context<InitializeCurveConfiguration>, fee: f64) -> Result<()> {
        instructions::initialize(ctx, fee)
    }

    pub fn create_pool(ctx: Context<CreateLiquidityPool>) -> Result<()> {
        instructions::create_pool(ctx)
    }

    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
    ) -> Result<()> {
        instructions::add_liquidity(ctx)
    }

    pub fn remove_liquidity(ctx: Context<RemoveLiquidity>, bump: u8) -> Result<()> {
        instructions::remove_liquidity(ctx, bump)
    }

    pub fn buy(ctx: Context<Buy>, amount: u64) -> Result<()> {
        instructions::buy(ctx, amount)
    }

    pub fn sell(ctx: Context<Sell>, amount: u64, bump: u8) -> Result<()> {
        instructions::sell(ctx, amount, bump)
    }

    // Raydium CPI instructions
    pub fn raydium_swap(
        ctx: Context<RaydiumSwap>,
        amount_in: u64,
        minimum_amount_out: u64,
    ) -> Result<()> {
        raydium::swap(ctx, amount_in, minimum_amount_out)
    }

    pub fn raydium_add_liquidity(
        ctx: Context<RaydiumAddLiquidity>,
        max_coin_amount: u64,
        max_pc_amount: u64,
    ) -> Result<()> {
        raydium::add_liquidity(ctx, max_coin_amount, max_pc_amount)
    }
}

