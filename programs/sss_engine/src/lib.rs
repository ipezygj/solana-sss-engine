use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

declare_id!("11111111111111111111111111111111");

#[program]
pub mod sss_engine {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        config: StablecoinConfig,
    ) -> Result<()> {
        let stablecoin_data = &mut ctx.accounts.stablecoin_data;
        
        stablecoin_data.authority = ctx.accounts.authority.key();
        stablecoin_data.config = config.clone();

        msg!("SSS-Engine Initialized: {}", stablecoin_data.config.name);
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct StablecoinConfig {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
    pub enable_permanent_delegate: bool,
    pub enable_transfer_hook: bool,
    pub default_account_frozen: bool,
}

#[account]
pub struct StablecoinData {
    pub authority: Pubkey,
    pub config: StablecoinConfig,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + (4 + 32) + (4 + 10) + (4 + 100) + 1 + 1 + 1 + 1 + 1,
        seeds = [b"stablecoin_data", mint.key().as_ref()],
        bump
    )]
    pub stablecoin_data: Account<'info, StablecoinData>,
    
    #[account(mut)]
    pub mint: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}
