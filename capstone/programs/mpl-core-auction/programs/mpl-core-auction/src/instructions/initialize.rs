use anchor_lang::prelude::*;

use crate::Config;

#[derive(Accounts)]
#[instruction(seed: u32)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump,
        space = 8 + Config::INIT_SPACE
    )]
    pub config: Account<'info, Config>,
    #[account(
        seeds = [b"tresuary", config.key().as_ref()],
        bump
    )]
    pub tresuary: SystemAccount<'info>,
    #[account(
        seeds = [b"vault", config.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}


impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, seed: u32, fee_bps: u8, min_duration_minutes: u32, max_duration_minutes: u32, bumps: &InitializeBumps) -> Result<()> {
        self.config.set_inner(Config{
            seed,
            admin: self.admin.key(),
            fee_bps,
            min_duration_minutes,
            max_duration_minutes,
            vault_bump: bumps.vault,
            tresuary_bump: bumps.tresuary,
            bump: bumps.config,
        });
        Ok(())
    }
}