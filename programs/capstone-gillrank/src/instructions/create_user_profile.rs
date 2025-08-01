use anchor_lang::prelude::*;

use crate::state::UserProfile;

#[derive(Accounts)]
pub struct CreateUserProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + UserProfile::INIT_SPACE,
        seeds = [b"profile", user.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateUserProfile<'info> {
    // create user profile
    pub fn create(&mut self, bump: &CreateUserProfileBumps) -> Result<()> {
        self.user_profile.set_inner(UserProfile {
            owner: self.user.key(),
            wins: 0,
            loses: 0,
            nfts: 0,
            bump: bump.user_profile,
        });

        Ok(())
    }
}
