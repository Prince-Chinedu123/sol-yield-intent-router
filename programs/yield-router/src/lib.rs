use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("2QnXvmbZCBLtVA4ZE61Fp2VTm3UD6xv8xtw1Pg7UN6gz");

#[program]
pub mod yield_router {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.admin = ctx.accounts.admin.key();
        vault.total_locked = 0;
        vault.is_staked = false;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.vault_pda.to_account_info(),
            },
        );
        system_program::transfer(cpi_context, amount)?;
        vault.total_locked = vault.total_locked.checked_add(amount).unwrap();
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require_keys_eq!(
            ctx.accounts.admin.key(),
            vault.admin,
            ErrorCode::Unauthorized
        );
        vault.is_staked = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = admin, 
        space = 8 + 32 + 8 + 1, 
        seeds = [b"state_v3"], 
        bump
    )]
    pub vault: Account<'info, VaultAccount>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, seeds = [b"state_v3"], bump)]
    pub vault: Account<'info, VaultAccount>,
    #[account(mut, seeds = [b"vault_pda"], bump)]
    pub vault_pda: UncheckedAccount<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut, seeds = [b"state_v3"], bump)]
    pub vault: Account<'info, VaultAccount>,
    pub admin: Signer<'info>,
}

#[account]
pub struct VaultAccount {
    pub admin: Pubkey,
    pub total_locked: u64,
    pub is_staked: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
}

