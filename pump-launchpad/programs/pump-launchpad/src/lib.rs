use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};

// Program ID - will be updated after deployment
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// Constants
pub const VIRTUAL_SOL_RESERVE: u64 = 30_000_000_000; // 30 SOL in lamports
pub const VIRTUAL_TOKEN_RESERVE: u64 = 1_073_000_000_000_000_000; // 1.073B tokens
pub const TOTAL_SUPPLY: u64 = 1_000_000_000_000_000_000; // 1B tokens
pub const MIGRATION_THRESHOLD: u64 = 69_000_000_000; // 69 SOL in lamports
pub const CREATOR_FEE_BPS: u64 = 100; // 1% = 100 basis points

#[program]
pub mod pump_launchpad {
    use super::*;

    // Initialize a new bonding curve for a token
    pub fn initialize_token(
        ctx: Context<InitializeToken>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        let bonding_curve = &mut ctx.accounts.bonding_curve;
        let creator = &ctx.accounts.creator;

        bonding_curve.creator = creator.key();
        bonding_curve.mint = ctx.accounts.mint.key();
        bonding_curve.name = name;
        bonding_curve.symbol = symbol;
        bonding_curve.uri = uri;
        bonding_curve.virtual_sol_reserve = VIRTUAL_SOL_RESERVE;
        bonding_curve.virtual_token_reserve = VIRTUAL_TOKEN_RESERVE;
        bonding_curve.real_sol_reserve = 0;
        bonding_curve.real_token_reserve = TOTAL_SUPPLY;
        bonding_curve.total_supply = TOTAL_SUPPLY;
        bonding_curve.migrated = false;
        bonding_curve.created_at = Clock::get()?.unix_timestamp;

        msg!("Token initialized: {}", bonding_curve.name);
        Ok(())
    }

    // Buy tokens with SOL
    pub fn buy(ctx: Context<Buy>, amount_sol: u64) -> Result<()> {
        require!(amount_sol > 0, ErrorCode::InvalidAmount);
        
        let bonding_curve = &mut ctx.accounts.bonding_curve;
        require!(!bonding_curve.migrated, ErrorCode::AlreadyMigrated);

        // Calculate tokens to receive using constant product formula
        let tokens_out = calculate_tokens_out(
            amount_sol,
            bonding_curve.virtual_sol_reserve,
            bonding_curve.virtual_token_reserve,
        );

        require!(tokens_out > 0, ErrorCode::InsufficientLiquidity);
        require!(tokens_out <= bonding_curve.real_token_reserve, ErrorCode::InsufficientTokens);

        // Calculate creator fee (1%)
        let fee = amount_sol.checked_mul(CREATOR_FEE_BPS).unwrap().checked_div(10000).unwrap();
        let amount_after_fee = amount_sol.checked_sub(fee).unwrap();

        // Transfer SOL from buyer to program
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.bonding_curve.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(cpi_context, amount_sol)?;

        // Transfer creator fee
        if fee > 0 {
            **bonding_curve.to_account_info().try_borrow_mut_lamports()? -= fee;
            **ctx.accounts.creator.to_account_info().try_borrow_mut_lamports()? += fee;
        }

        // Update bonding curve state
        bonding_curve.virtual_sol_reserve = bonding_curve.virtual_sol_reserve
            .checked_add(amount_after_fee)
            .unwrap();
        bonding_curve.virtual_token_reserve = bonding_curve.virtual_token_reserve
            .checked_sub(tokens_out)
            .unwrap();
        bonding_curve.real_sol_reserve = bonding_curve.real_sol_reserve
            .checked_add(amount_after_fee)
            .unwrap();
        bonding_curve.real_token_reserve = bonding_curve.real_token_reserve
            .checked_sub(tokens_out)
            .unwrap();

        // Transfer tokens to buyer
        let seeds = &[
            b"bonding_curve",
            bonding_curve.mint.as_ref(),
            &[ctx.bumps.bonding_curve],
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.bonding_curve_token_account.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: bonding_curve.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(
            CpiContext::new_with_signer(cpi_program, cpi_accounts, signer),
            tokens_out,
        )?;

        // Check if migration threshold reached
        if bonding_curve.virtual_sol_reserve >= MIGRATION_THRESHOLD && !bonding_curve.migrated {
            bonding_curve.migrated = true;
            msg!("Migration threshold reached! Ready for Raydium.");
        }

        msg!("Bought {} tokens for {} SOL", tokens_out, amount_sol);
        Ok(())
    }

    // Sell tokens for SOL
    pub fn sell(ctx: Context<Sell>, amount_tokens: u64) -> Result<()> {
        require!(amount_tokens > 0, ErrorCode::InvalidAmount);

        let bonding_curve = &mut ctx.accounts.bonding_curve;
        require!(!bonding_curve.migrated, ErrorCode::AlreadyMigrated);

        // Calculate SOL to receive
        let sol_out = calculate_sol_out(
            amount_tokens,
            bonding_curve.virtual_sol_reserve,
            bonding_curve.virtual_token_reserve,
        );

        require!(sol_out > 0, ErrorCode::InsufficientLiquidity);
        require!(sol_out <= bonding_curve.real_sol_reserve, ErrorCode::InsufficientSol);

        // Calculate creator fee (1%)
        let fee = sol_out.checked_mul(CREATOR_FEE_BPS).unwrap().checked_div(10000).unwrap();
        let sol_after_fee = sol_out.checked_sub(fee).unwrap();

        // Transfer tokens from seller to bonding curve
        let cpi_accounts = Transfer {
            from: ctx.accounts.seller_token_account.to_account_info(),
            to: ctx.accounts.bonding_curve_token_account.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(
            CpiContext::new(cpi_program, cpi_accounts),
            amount_tokens,
        )?;

        // Update bonding curve state
        bonding_curve.virtual_sol_reserve = bonding_curve.virtual_sol_reserve
            .checked_sub(sol_out)
            .unwrap();
        bonding_curve.virtual_token_reserve = bonding_curve.virtual_token_reserve
            .checked_add(amount_tokens)
            .unwrap();
        bonding_curve.real_sol_reserve = bonding_curve.real_sol_reserve
            .checked_sub(sol_out)
            .unwrap();
        bonding_curve.real_token_reserve = bonding_curve.real_token_reserve
            .checked_add(amount_tokens)
            .unwrap();

        // Transfer SOL to seller
        **bonding_curve.to_account_info().try_borrow_mut_lamports()? -= sol_after_fee;
        **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? += sol_after_fee;

        // Transfer creator fee
        if fee > 0 {
            **bonding_curve.to_account_info().try_borrow_mut_lamports()? -= fee;
            **ctx.accounts.creator.to_account_info().try_borrow_mut_lamports()? += fee;
        }

        msg!("Sold {} tokens for {} SOL", amount_tokens, sol_after_fee);
        Ok(())
    }
}

// Calculate tokens out using constant product formula: x * y = k
fn calculate_tokens_out(amount_sol: u64, virtual_sol: u64, virtual_tokens: u64) -> u64 {
    let new_sol_reserve = virtual_sol + amount_sol;
    let new_token_reserve = (virtual_sol as u128)
        .checked_mul(virtual_tokens as u128)
        .unwrap()
        .checked_div(new_sol_reserve as u128)
        .unwrap() as u64;
    virtual_tokens - new_token_reserve
}

// Calculate SOL out using constant product formula
fn calculate_sol_out(amount_tokens: u64, virtual_sol: u64, virtual_tokens: u64) -> u64 {
    let new_token_reserve = virtual_tokens + amount_tokens;
    let new_sol_reserve = (virtual_sol as u128)
        .checked_mul(virtual_tokens as u128)
        .unwrap()
        .checked_div(new_token_reserve as u128)
        .unwrap() as u64;
    virtual_sol - new_sol_reserve
}

#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    
    #[account(
        init,
        payer = creator,
        space = 8 + BondingCurve::SIZE,
        seeds = [b"bonding_curve", mint.key().as_ref()],
        bump
    )]
    pub bonding_curve: Account<'info, BondingCurve>,
    
    pub mint: Account<'info, Mint>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"bonding_curve", bonding_curve.mint.as_ref()],
        bump
    )]
    pub bonding_curve: Account<'info, BondingCurve>,
    
    /// CHECK: Creator account for fee distribution
    #[account(mut, address = bonding_curve.creator)]
    pub creator: AccountInfo<'info>,
    
    #[account(
        mut,
        associated_token::mint = bonding_curve.mint,
        associated_token::authority = bonding_curve,
    )]
    pub bonding_curve_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = bonding_curve.mint,
        associated_token::authority = buyer,
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Sell<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"bonding_curve", bonding_curve.mint.as_ref()],
        bump
    )]
    pub bonding_curve: Account<'info, BondingCurve>,
    
    /// CHECK: Creator account for fee distribution
    #[account(mut, address = bonding_curve.creator)]
    pub creator: AccountInfo<'info>,
    
    #[account(
        mut,
        associated_token::mint = bonding_curve.mint,
        associated_token::authority = bonding_curve,
    )]
    pub bonding_curve_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = bonding_curve.mint,
        associated_token::authority = seller,
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct BondingCurve {
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub virtual_sol_reserve: u64,
    pub virtual_token_reserve: u64,
    pub real_sol_reserve: u64,
    pub real_token_reserve: u64,
    pub total_supply: u64,
    pub migrated: bool,
    pub created_at: i64,
}

impl BondingCurve {
    pub const SIZE: usize = 32 + 32 + 64 + 64 + 128 + 8 + 8 + 8 + 8 + 8 + 1 + 8;
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("Insufficient tokens")]
    InsufficientTokens,
    #[msg("Insufficient SOL")]
    InsufficientSol,
    #[msg("Already migrated")]
    AlreadyMigrated,
}