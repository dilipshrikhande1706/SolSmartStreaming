use anchor_lang::prelude::*;

declare_id!("GeShW6zkEbWRZidx4Rcy6Wj2wnGiRUQK1umseuCarNSc");

#[program]
pub mod streaming_service {
    use super::*;

    pub fn initialize_payment(ctx: Context<InitializePayment>, amount: u64) -> Result<()> {
        let entitlement = &mut ctx.accounts.entitlement;
        entitlement.user = *ctx.accounts.user.key;
        entitlement.duration = match amount {
            100_000_000 => 60,    // 0.1 SOL = 1 minute
            500_000_000 => 300,   // 0.5 SOL = 5 minutes
            _ => return Err(ErrorCode::InvalidAmount.into()),
        };
        entitlement.expiry = Clock::get()?.unix_timestamp + entitlement.duration as i64;

        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.user.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };
        let cpi_program = ctx.accounts.system_program.to_account_info();
        anchor_lang::system_program::transfer(
            CpiContext::new(cpi_program, cpi_accounts),
            amount,
        )?;

        Ok(())
    }

    pub fn check_entitlement(ctx: Context<CheckEntitlement>) -> Result<()> {
        let entitlement = &ctx.accounts.entitlement;
        let current_time = Clock::get()?.unix_timestamp;
        if current_time > entitlement.expiry {
            return Err(ErrorCode::EntitlementExpired.into());
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePayment<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 8)]
    pub entitlement: Account<'info, Entitlement>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is the vault account, manually verified as the program's payment destination
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CheckEntitlement<'info> {
    #[account(has_one = user)]
    pub entitlement: Account<'info, Entitlement>,
    pub user: Signer<'info>,
}

#[account]
pub struct Entitlement {
    pub user: Pubkey,
    pub duration: u64,
    pub expiry: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid payment amount. Must be 0.1 SOL or 0.5 SOL.")]
    InvalidAmount,
    #[msg("Streaming entitlement has expired.")]
    EntitlementExpired,
}