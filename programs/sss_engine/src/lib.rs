use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};
use anchor_spl::token_interface::TokenInterface;
use spl_token_2022::{extension::ExtensionType, instruction as token22_ix};

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

        // 1. Määritetään tarvittavat laajennukset dynaamisesti (SSS-1 vs SSS-2)
        let mut extension_types = vec![];
        if config.enable_permanent_delegate {
            extension_types.push(ExtensionType::PermanentDelegate);
        }
        if config.enable_transfer_hook {
            extension_types.push(ExtensionType::TransferHook);
        }
        if config.default_account_frozen {
            extension_types.push(ExtensionType::DefaultAccountState);
        }

        // 2. Lasketaan tarvittava tila (Space) ja Vuokra (Rent)
        let space = ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(&extension_types).unwrap();
        let rent = Rent::get()?;
        let lamports = rent.minimum_balance(space);

        // 3. Luodaan Mint-tili System Programin kautta
        invoke(
            &system_instruction::create_account(
                ctx.accounts.authority.key,
                ctx.accounts.mint.key,
                lamports,
                space as u64,
                ctx.accounts.token_program.key,
            ),
            &[
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // 4. ALUSTETAAN LAAJENNUKSET (Ehdottomasti ennen Mintin alustusta!)
        if config.enable_permanent_delegate {
            invoke(
                &token22_ix::initialize_permanent_delegate(
                    ctx.accounts.token_program.key,
                    ctx.accounts.mint.key,
                    &ctx.accounts.authority.key, // Tässä master authority toimii delegate-roolissa
                )?,
                &[ctx.accounts.mint.to_account_info()],
            )?;
        }

        // (Tähän rakennetaan myöhemmin Transfer Hook & Freeze CPI -kutsut samalla logiikalla)

        // 5. Alustetaan lopuksi itse Mint
        invoke(
            &token22_ix::initialize_mint2(
                ctx.accounts.token_program.key,
                ctx.accounts.mint.key,
                &ctx.accounts.authority.key,
                Some(&ctx.accounts.authority.key), // Freeze authority
                config.decimals,
            )?,
            &[ctx.accounts.mint.to_account_info()],
        )?;

        msg!("SSS-Engine: Token {} (SSS-Preset) forged successfully.", config.symbol);
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
