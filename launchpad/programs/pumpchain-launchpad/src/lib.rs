use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};

// Program ID (replace with actual after deployment)
declare_id!("PumpLchPad111111111111111111111111111111111");

/// PumpChain Launchpad - Token launch with bonding curve
#[program]
pub mod pumpchain_launchpad {
    use super::*;

    /// Initialize a new token launch
    pub fn initialize_launch(ctx: Context<InitializeLaunch>, params: LaunchParams) -> Result<()> {
        let launch = &mut ctx.accounts.launch;
        
        launch.creator = ctx.accounts.creator.key();
        launch.mint = ctx.accounts.mint.key();
        launch.token_name = params.name;
        launch.token_symbol = params.symbol;
        launch.token_uri = params.uri;
        
        // Bonding curve parameters
        launch.initial_price = params.initial_price;
        launch.slope = params.slope;
        launch.target_market_cap = params.target_market_cap;
        
        launch.total_supply = 0;
        launch.sol_raised = 0;
        launch.is_migrated = false;
        launch.created_at = Clock::get()?.unix_timestamp;
        
        emit!(LaunchCreated {
            launch: launch.key(),
            creator: launch.creator,
            mint: launch.mint,
            name: launch.token_name.clone(),
            symbol: launch.token_symbol.clone(),
        });
        
        Ok(())
    }

    /// Buy tokens from the bonding curve
    pub fn buy_tokens(ctx: Context<BuyTokens>, sol_amount: u64) -> Result<()> {
        let launch = &mut ctx.accounts.launch;
        
        require!(!launch.is_migrated, LaunchpadError::AlreadyMigrated);
        
        // Calculate token amount based on bonding curve
        let token_amount = calculate_token_amount(
            launch.total_supply,
            sol_amount,
            launch.initial_price,
            launch.slope,
        )?;
        
        // Transfer SOL from buyer to launch
        anchor_lang::system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.buyer.to_account_info(),
                    to: ctx.accounts.launch_sol_account.to_account_info(),
                },
            ),
            sol_amount,
        )?;
        
        // Mint tokens to buyer
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.buyer_token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
                &[&[b"mint_authority", &[ctx.bumps.mint_authority]]],
            ),
            token_amount,
        )?;
        
        // Update launch state
        launch.total_supply += token_amount;
        launch.sol_raised += sol_amount;
        
        // Check if migration threshold reached
        let current_market_cap = launch.sol_raised; // Simplified
        if current_market_cap >= launch.target_market_cap {
            launch.is_migrated = true;
            emit!(LaunchMigrated {
                launch: launch.key(),
                market_cap: current_market_cap,
            });
        }
        
        emit!(TokensPurchased {
            launch: launch.key(),
            buyer: ctx.accounts.buyer.key(),
            sol_amount,
            token_amount,
        });
        
        Ok(())
    }

    /// Sell tokens back to the bonding curve
    pub fn sell_tokens(ctx: Context<SellTokens>, token_amount: u64) -> Result<()> {
        let launch = &mut ctx.accounts.launch;
        
        require!(!launch.is_migrated, LaunchpadError::AlreadyMigrated);
        
        // Calculate SOL amount based on bonding curve
        let sol_amount = calculate_sol_amount(
            launch.total_supply,
            token_amount,
            launch.initial_price,
            launch.slope,
        )?;
        
        // Burn tokens from seller
        token::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Burn {
                    mint: ctx.accounts.mint.to_account_info(),
                    from: ctx.accounts.seller_token_account.to_account_info(),
                    authority: ctx.accounts.seller.to_account_info(),
                },
            ),
            token_amount,
        )?;
        
        // Transfer SOL back to seller
        **ctx.accounts.launch_sol_account.to_account_info().try_borrow_mut_lamports()? -= sol_amount;
        **ctx.accounts.seller.try_borrow_mut_lamports()? += sol_amount;
        
        // Update launch state
        launch.total_supply -= token_amount;
        launch.sol_raised -= sol_amount;
        
        emit!(TokensSold {
            launch: launch.key(),
            seller: ctx.accounts.seller.key(),
            token_amount,
            sol_amount,
        });
        
        Ok(())
    }

    /// Migrate to Raydium AMM when target reached
    pub fn migrate_to_raydium(ctx: Context<MigrateToRaydium>) -> Result<()> {
        let launch = &ctx.accounts.launch;
        
        require!(launch.is_migrated, LaunchpadError::MigrationNotReady);
        require!(!launch.raydium_pool_created, LaunchpadError::AlreadyMigrated);
        
        // TODO: Create Raydium pool
        // 1. Create AMM pool
        // 2. Add liquidity (SOL + tokens)
        // 3. Burn LP tokens
        // 4. Disable bonding curve
        
        emit!(RaydiumMigration {
            launch: launch.key(),
            pool: ctx.accounts.raydium_pool.key(),
        });
        
        Ok(())
    }
}

/// Calculate token amount using bonding curve formula
fn calculate_token_amount(
    current_supply: u64,
    sol_amount: u64,
    initial_price: u64,
    slope: u64,
) -> Result<u64> {
    // Simplified bonding curve: tokens = sol / (price + slope * supply)
    let price = initial_price.saturating_add(
        slope.saturating_mul(current_supply) / 1_000_000
    );
    
    let tokens = sol_amount.saturating_mul(1_000_000) / price.max(1);
    
    Ok(tokens)
}

/// Calculate SOL amount for token sale
fn calculate_sol_amount(
    current_supply: u64,
    token_amount: u64,
    initial_price: u64,
    slope: u64,
) -> Result<u64> {
    // Reverse of buy calculation
    let price = initial_price.saturating_add(
        slope.saturating_mul(current_supply) / 1_000_000
    );
    
    let sol = token_amount.saturating_mul(price) / 1_000_000;
    
    Ok(sol)
}

// Account Structures

#[derive(Accounts)]
pub struct InitializeLaunch<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    
    #[account(
        init,
        payer = creator,
        space = 8 + LaunchAccount::SIZE
    )]
    pub launch: Account<'info, LaunchAccount>,
    
    #[account(
        init,
        payer = creator,
        mint::decimals = 6,
        mint::authority = mint_authority,
    )]
    pub mint: Account<'info, Mint>,
    
    /// CHECK: PDA used as mint authority
    #[account(seeds = [b"mint_authority"], bump)]
    pub mint_authority: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    #[account(mut)]
    pub launch: Account<'info, LaunchAccount>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    #[account(
        mut,
        seeds = [b"launch_sol", launch.key().as_ref()],
        bump
    )]
    /// CHECK: Launch SOL account
    pub launch_sol_account: UncheckedAccount<'info>,
    
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint,
        associated_token::authority = buyer,
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: PDA used as mint authority
    #[account(seeds = [b"mint_authority"], bump)]
    pub mint_authority: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SellTokens<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(mut)]
    pub launch: Account<'info, LaunchAccount>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    #[account(
        mut,
        seeds = [b"launch_sol", launch.key().as_ref()],
        bump
    )]
    /// CHECK: Launch SOL account
    pub launch_sol_account: UncheckedAccount<'info>,
    
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = seller,
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct MigrateToRaydium<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub launch: Account<'info, LaunchAccount>,
    
    /// CHECK: Raydium pool account
    pub raydium_pool: UncheckedAccount<'info>,
    
    // Additional Raydium accounts...
}

// Data Structures

#[account]
pub struct LaunchAccount {
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub token_name: String,
    pub token_symbol: String,
    pub token_uri: String,
    pub initial_price: u64,
    pub slope: u64,
    pub target_market_cap: u64,
    pub total_supply: u64,
    pub sol_raised: u64,
    pub is_migrated: bool,
    pub raydium_pool_created: bool,
    pub created_at: i64,
}

impl LaunchAccount {
    pub const SIZE: usize = 32 + 32 + 4 + 32 + 4 + 10 + 4 + 200 + 8 + 8 + 8 + 8 + 8 + 1 + 1 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LaunchParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub initial_price: u64,
    pub slope: u64,
    pub target_market_cap: u64,
}

// Events

#[event]
pub struct LaunchCreated {
    pub launch: Pubkey,
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
}

#[event]
pub struct TokensPurchased {
    pub launch: Pubkey,
    pub buyer: Pubkey,
    pub sol_amount: u64,
    pub token_amount: u64,
}

#[event]
pub struct TokensSold {
    pub launch: Pubkey,
    pub seller: Pubkey,
    pub token_amount: u64,
    pub sol_amount: u64,
}

#[event]
pub struct LaunchMigrated {
    pub launch: Pubkey,
    pub market_cap: u64,
}

#[event]
pub struct RaydiumMigration {
    pub launch: Pubkey,
    pub pool: Pubkey,
}

// Errors

#[error_code]
pub enum LaunchpadError {
    #[msg("Launch has already migrated to AMM")]
    AlreadyMigrated,
    #[msg("Migration threshold not reached")]
    MigrationNotReady,
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
}