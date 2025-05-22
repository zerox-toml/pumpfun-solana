use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use raydium_amm::program::RaydiumAmm;

#[derive(Accounts)]
pub struct RaydiumSwap<'info> {
    /// CHECK: This is the Raydium AMM program
    pub amm_program: Program<'info, RaydiumAmm>,
    
    /// CHECK: Raydium AMM account
    #[account(mut)]
    pub amm_account: AccountInfo<'info>,
    
    /// CHECK: Raydium AMM authority
    pub amm_authority: AccountInfo<'info>,
    
    /// CHECK: Raydium AMM open orders
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    
    /// CHECK: Raydium AMM target orders
    #[account(mut)]
    pub amm_target_orders: AccountInfo<'info>,
    
    /// CHECK: Raydium pool coin token account
    #[account(mut)]
    pub pool_coin_token_account: AccountInfo<'info>,
    
    /// CHECK: Raydium pool pc token account
    #[account(mut)]
    pub pool_pc_token_account: AccountInfo<'info>,
    
    /// CHECK: Serum program
    pub serum_program: AccountInfo<'info>,
    
    /// CHECK: Serum market
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    
    /// CHECK: Serum bids
    #[account(mut)]
    pub serum_bids: AccountInfo<'info>,
    
    /// CHECK: Serum asks
    #[account(mut)]
    pub serum_asks: AccountInfo<'info>,
    
    /// CHECK: Serum event queue
    #[account(mut)]
    pub serum_event_queue: AccountInfo<'info>,
    
    /// CHECK: Serum base vault
    #[account(mut)]
    pub serum_base_vault: AccountInfo<'info>,
    
    /// CHECK: Serum quote vault
    #[account(mut)]
    pub serum_quote_vault: AccountInfo<'info>,
    
    /// CHECK: Serum vault signer
    pub serum_vault_signer: AccountInfo<'info>,
    
    /// User's source token account
    #[account(mut)]
    pub user_source_token_account: Account<'info, TokenAccount>,
    
    /// User's destination token account
    #[account(mut)]
    pub user_destination_token_account: Account<'info, TokenAccount>,
    
    /// User's authority
    pub user_authority: Signer<'info>,
    
    /// Token program
    pub token_program: Program<'info, Token>,
}

pub fn swap<'info>(
    ctx: Context<'_, '_, '_, 'info, RaydiumSwap<'info>>,
    amount_in: u64,
    minimum_amount_out: u64,
) -> Result<()> {
    let cpi_accounts = raydium_amm::cpi::accounts::Swap {
        amm_program: ctx.accounts.amm_program.to_account_info(),
        amm_account: ctx.accounts.amm_account.to_account_info(),
        amm_authority: ctx.accounts.amm_authority.to_account_info(),
        amm_open_orders: ctx.accounts.amm_open_orders.to_account_info(),
        amm_target_orders: ctx.accounts.amm_target_orders.to_account_info(),
        pool_coin_token_account: ctx.accounts.pool_coin_token_account.to_account_info(),
        pool_pc_token_account: ctx.accounts.pool_pc_token_account.to_account_info(),
        serum_program: ctx.accounts.serum_program.to_account_info(),
        serum_market: ctx.accounts.serum_market.to_account_info(),
        serum_bids: ctx.accounts.serum_bids.to_account_info(),
        serum_asks: ctx.accounts.serum_asks.to_account_info(),
        serum_event_queue: ctx.accounts.serum_event_queue.to_account_info(),
        serum_base_vault: ctx.accounts.serum_base_vault.to_account_info(),
        serum_quote_vault: ctx.accounts.serum_quote_vault.to_account_info(),
        serum_vault_signer: ctx.accounts.serum_vault_signer.to_account_info(),
        user_source_token_account: ctx.accounts.user_source_token_account.to_account_info(),
        user_destination_token_account: ctx.accounts.user_destination_token_account.to_account_info(),
        user_authority: ctx.accounts.user_authority.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.amm_program.to_account_info(),
        cpi_accounts,
    );

    raydium_amm::cpi::swap(cpi_ctx, amount_in, minimum_amount_out)?;

    Ok(())
}

#[derive(Accounts)]
pub struct RaydiumAddLiquidity<'info> {
    /// CHECK: This is the Raydium AMM program
    pub amm_program: Program<'info, RaydiumAmm>,
    
    /// CHECK: Raydium AMM account
    #[account(mut)]
    pub amm_account: AccountInfo<'info>,
    
    /// CHECK: Raydium AMM authority
    pub amm_authority: AccountInfo<'info>,
    
    /// CHECK: Raydium AMM open orders
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    
    /// CHECK: Raydium AMM target orders
    #[account(mut)]
    pub amm_target_orders: AccountInfo<'info>,
    
    /// CHECK: Raydium pool coin token account
    #[account(mut)]
    pub pool_coin_token_account: AccountInfo<'info>,
    
    /// CHECK: Raydium pool pc token account
    #[account(mut)]
    pub pool_pc_token_account: AccountInfo<'info>,
    
    /// User's coin token account
    #[account(mut)]
    pub user_coin_token_account: Account<'info, TokenAccount>,
    
    /// User's pc token account
    #[account(mut)]
    pub user_pc_token_account: Account<'info, TokenAccount>,
    
    /// User's authority
    pub user_authority: Signer<'info>,
    
    /// Token program
    pub token_program: Program<'info, Token>,
}

pub fn add_liquidity<'info>(
    ctx: Context<'_, '_, '_, 'info, RaydiumAddLiquidity<'info>>,
    max_coin_amount: u64,
    max_pc_amount: u64,
) -> Result<()> {
    let cpi_accounts = raydium_amm::cpi::accounts::AddLiquidity {
        amm_program: ctx.accounts.amm_program.to_account_info(),
        amm_account: ctx.accounts.amm_account.to_account_info(),
        amm_authority: ctx.accounts.amm_authority.to_account_info(),
        amm_open_orders: ctx.accounts.amm_open_orders.to_account_info(),
        amm_target_orders: ctx.accounts.amm_target_orders.to_account_info(),
        pool_coin_token_account: ctx.accounts.pool_coin_token_account.to_account_info(),
        pool_pc_token_account: ctx.accounts.pool_pc_token_account.to_account_info(),
        user_coin_token_account: ctx.accounts.user_coin_token_account.to_account_info(),
        user_pc_token_account: ctx.accounts.user_pc_token_account.to_account_info(),
        user_authority: ctx.accounts.user_authority.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.amm_program.to_account_info(),
        cpi_accounts,
    );

    raydium_amm::cpi::add_liquidity(cpi_ctx, max_coin_amount, max_pc_amount)?;

    Ok(())
} 