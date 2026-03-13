use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, program::invoke_signed, system_instruction};
use anchor_spl::token_interface::{TokenInterface, TokenAccount, Mint, TransferChecked, transfer_checked};
use spl_token_2022::{extension::ExtensionType, instruction as token22_ix};

declare_id!("11111111111111111111111111111111");

#[program]
pub mod sss_engine {
    use super::*;

    // 1. INITIALIZE (SSS-1 & SSS-2 Core)
    pub fn initialize(ctx: Context<Initialize>, config: StablecoinConfig) -> Result<()> {
        let stablecoin_data = &mut ctx.accounts.stablecoin_data;
        stablecoin_data.authority = ctx.accounts.authority.key();
        stablecoin_data.config = config.clone();
        // CPI-logiikka Token-2022 alustamiseksi menisi tähän...
        msg!("SSS-Engine: Token {} forged.", config.symbol);
        Ok(())
    }

    // 2. TOGGLE BLACKLIST (SSS-2 Compliance)
    pub fn toggle_blacklist(ctx: Context<ToggleBlacklist>, is_blacklisted: bool) -> Result<()> {
        require!(ctx.accounts.stablecoin_data.config.enable_transfer_hook, ErrorCode::ComplianceModuleDisabled);
        
        let blacklist_entry = &mut ctx.accounts.blacklist_entry;
        blacklist_entry.is_blacklisted = is_blacklisted;
        
        msg!("Address {} blacklist status set to: {}", ctx.accounts.target_account.key(), is_blacklisted);
        Ok(())
    }

    // 3. SEIZE FUNDS (SSS-2 Permanent Delegate Action)
    pub fn seize(ctx: Context<Seize>, amount: u64) -> Result<()> {
        require!(ctx.accounts.stablecoin_data.config.enable_permanent_delegate, ErrorCode::ComplianceModuleDisabled);

        let cpi_accounts = TransferChecked {
            from: ctx.accounts.source_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.destination_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(), // Authority toimii Permanent Delegatena
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, amount, ctx.accounts.stablecoin_data.config.decimals)?;
        
        msg!("Seized {} tokens from {}", amount, ctx.accounts.source_account.key());
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

#[account]
pub struct BlacklistEntry {
    pub is_blacklisted: bool,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 200,
        seeds = [b"stablecoin_data", mint.key().as_ref()],
        bump
    )]
    pub stablecoin_data: Account<'info, StablecoinData>,
    #[account(mut)]
    pub mint: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct ToggleBlacklist<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(has_one = authority)]
    pub stablecoin_data: Account<'info, StablecoinData>,
    /// CHECK: Kohdeosoite, jota ollaan asettamassa mustalle listalle
    pub target_account: AccountInfo<'info>,
    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + 1,
        seeds = [b"blacklist", stablecoin_data.key().as_ref(), target_account.key().as_ref()],
        bump
    )]
    pub blacklist_entry: Account<'info, BlacklistEntry>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Seize<'info> {
    pub authority: Signer<'info>,
    #[account(has_one = authority)]
    pub stablecoin_data: Account<'info, StablecoinData>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub source_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub destination_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("This stablecoin preset does not have the compliance module enabled.")]
    ComplianceModuleDisabled,
}
