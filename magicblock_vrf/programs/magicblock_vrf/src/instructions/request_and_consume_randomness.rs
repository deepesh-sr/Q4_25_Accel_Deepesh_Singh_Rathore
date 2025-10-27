use anchor_lang::prelude::*;
use ephemeral_vrf_sdk::anchor::vrf;
use ephemeral_vrf_sdk::instructions::{create_request_randomness_ix, RequestRandomnessParams};
use ephemeral_vrf_sdk::types::SerializableAccountMeta;

use crate::User;

#[vrf]
#[derive(Accounts)]
pub struct RequestRandomness<'info> {
    
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account_pda.bump
    )]
    pub user_account_pda: Account<'info, User>,

    /// CHECK: The oracle queue
    #[account(mut, address = ephemeral_vrf_sdk::consts::DEFAULT_QUEUE)]
    pub oracle_queue: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ConsumeRandomness<'info> {
    /// This check ensure that the vrf_program_identity (which is a PDA) is a singer
    /// enforcing the callback is executed by the VRF program trough CPI
    #[account(address = ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY)]
    pub vrf_program_identity: Signer<'info>,
    #[account(mut)]
    pub user_account_pda: Account<'info, User>,
}
impl <'info> ConsumeRandomness <'info>{
    pub fn consume_randomness(
        &mut self,
        randomness: [u8; 32],
    ) -> Result<()> {
        let rnd_u8 = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 1, 6);
        msg!("Consuming random number: {:?}", rnd_u8);
        self.user_account_pda.random_number = rnd_u8; // Update the player's last result
        Ok(())
    }
}

impl <'info> RequestRandomness <'info> {
    pub fn request_randomness(&mut self,client_seed : u8)-> Result<()>{
        msg!("Requesting randomness...");
        let ix = create_request_randomness_ix(RequestRandomnessParams {
            payer: self.user.key(),
            oracle_queue: self.oracle_queue.key(),
            callback_program_id: crate::ID,
            callback_discriminator:crate::instruction::ConsumeRandomness::DISCRIMINATOR.to_vec() ,
            caller_seed: [client_seed; 32],
            // Specify any account that is required by the callback
            accounts_metas: Some(
                vec![SerializableAccountMeta {
                pubkey: self.user_account_pda.key(),
                is_signer: false,
                is_writable: true,
            }]),
            ..Default::default()
        });
            self.invoke_signed_vrf(&self.user.to_account_info(), &ix)?;
        Ok(())
    }
}

