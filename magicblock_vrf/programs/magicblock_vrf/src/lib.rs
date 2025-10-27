pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("DeKyCagVzospP15FxRjwavBPzzwuNHfSCEij1dCPUoXi");

#[program]
pub mod magicblock_vrf {

    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user(&ctx.bumps)
    }

    pub fn update_user(ctx : Context<UpdateUser>,new_data : u8)-> Result<()>{
        ctx.accounts.update_user(new_data)
    }

    pub fn close_user(ctx : Context<CloseUser>)-> Result<()>{
        ctx.accounts.close_user()
    }
    pub fn delegate(ctx: Context<Delegate>) -> Result<()> {
        ctx.accounts.delegate()?;
        
        Ok(())
    }

    pub fn undelegate(ctx: Context<Undelegate>) -> Result<()> {
        ctx.accounts.undelegate()?;
        
        Ok(())
    }

    pub fn consume_randomness(ctx : Context<ConsumeRandomness>, randomness : [u8;32])-> Result<()>{
        ctx.accounts.consume_randomness(randomness)
    }
}
