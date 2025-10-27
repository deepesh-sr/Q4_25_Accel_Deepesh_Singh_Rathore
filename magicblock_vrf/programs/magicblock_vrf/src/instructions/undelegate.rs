use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::{anchor::commit, ephem::{commit_accounts, commit_and_undelegate_accounts}};

use crate::User;

#[commit]
#[derive(Accounts)]

pub struct Undelegate<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account_pda.bump
    )]
    pub user_account_pda: Account<'info, User>,

    pub system_program: Program<'info, System>,
}

impl<'info> Undelegate<'info> {
    pub fn undelegate(&mut self) -> Result<()> {
        commit_and_undelegate_accounts(
            &self.user.to_account_info(),
            vec![&self.user_account_pda.to_account_info()],
            &self.magic_context,
            &self.magic_program,
        )?;

        Ok(())
    }
}
