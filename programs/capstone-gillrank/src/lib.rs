#![allow(unexpected_cfgs)]
#![allow(deprecated)]
#![allow(unused_imports)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("VS9y7jwTSYpmKzgTwckAFibhGyvuEWqgMXQf21ehAXq");

#[program]
pub mod capstone_gillrank {

    use super::*;

    pub fn create_user_profile(ctx: Context<CreateUserProfile>) ->Result<()> {
        let bumps = ctx.bumps;
        ctx.accounts.create(&bumps)?;
        Ok(())
    }
}
