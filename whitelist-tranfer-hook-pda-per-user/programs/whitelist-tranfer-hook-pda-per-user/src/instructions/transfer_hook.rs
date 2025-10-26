use std::cell::RefMut;
use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{
            transfer_hook::TransferHookAccount, 
            BaseStateWithExtensionsMut, 
            PodStateWithExtensionsMut
        }, 
        pod::PodAccount
    }, 
    token_interface::{
        Mint, 
        TokenAccount
    }
};

use crate::WhitelistPDA;

#[derive(Accounts)]
pub struct TransferHook<'info> {
    #[account(
        token::mint = mint, 
        token::authority = owner,
    )]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        token::mint = mint,
    )]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: source token account owner, can be SystemAccount or PDA owned by another program
    pub owner: UncheckedAccount<'info>,
    /// CHECK: ExtraAccountMetaList Account,
    #[account(
        seeds = [b"extra-account-metas", mint.key().as_ref()], 
        bump
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,

     ///CHECK : This is safe because we are only using pubkey. 
    pub target_address :UncheckedAccount<'info>,

    #[account(
        seeds = [b"whitelist",target_address.key().as_ref()], 
        bump
    )]
    pub whitelist: Account<'info, WhitelistPDA>,
}

impl<'info> TransferHook<'info> {
    /// This function is called when the transfer hook is executed.
    pub fn transfer_hook(&mut self, _amount: u64) -> Result<()> {
        // Fail this instruction if it is not called from within a transfer hook
        self.check_is_transferring()?;
        require!(self.whitelist.is_whitelisted == true,ErrorCode::SenderNotWhitelisted);

        msg!("Tranfer Approved , receiver is whitelisted");
        Ok(())
    }

    /// Checks if the transfer hook is being executed during a transfer operation.
    fn check_is_transferring(&mut self) -> Result<()> {
        // Ensure that the source token account has the transfer hook extension enabled

        // Get the account info of the source token account
        let source_token_info = self.source_token.to_account_info();
        // Borrow the account data mutably
        let mut account_data_ref: RefMut<&mut [u8]> = source_token_info.try_borrow_mut_data()?;

        // Unpack the account data as a PodStateWithExtensionsMut
        // This will allow us to access the extensions of the token account
        // We use PodStateWithExtensionsMut because TokenAccount is a POD (Plain Old Data) type
        let mut account = PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref)?;
        // Get the TransferHookAccount extension
        // Search for the TransferHookAccount extension in the token account
        // The returning struct has a `transferring` field that indicates if the account is in the middle of a transfer operation
        let account_extension = account.get_extension_mut::<TransferHookAccount>()?;
    
        // Check if the account is in the middle of a transfer operation
        if !bool::from(account_extension.transferring) {
            panic!("TransferHook: Not transferring");
        }
    
        Ok(())
    }
}