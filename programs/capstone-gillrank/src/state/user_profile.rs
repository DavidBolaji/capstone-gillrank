use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserProfile {
    pub owner: Pubkey,   // ownwer of profile
    pub wins: u16,       // total no of wins
    pub loses: u16,      // total no of loses
    pub nfts: u16,       // total no of nfts
    pub bump: u8         // profile bump
}