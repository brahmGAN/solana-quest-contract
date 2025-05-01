use anchor_lang::{prelude::*, solana_program::system_instruction};
use std::mem::size_of;

declare_id!("F5qfUhqkqEovEkQnxziJAVkY8spYevZU6Jd8VFCujYbJ");

#[program]
pub mod solana_quest_contract 
{
    use super::*;

    pub fn initialize(ctx: Context<InitializeContext>, funds_handler: Pubkey) -> Result<()> 
    {
        let initialize_account = &mut ctx.accounts.initialize_account; 
        let funds_handler_account = &mut ctx.accounts.funds_handler_account;
        require!(initialize_account.initialize == false,ErrorCode::AlreadyInitialized);
        funds_handler_account.funds_handler = funds_handler; 
        initialize_account.initialize = true; 
        Ok(())
    }

    pub fn quest_entry_fee(ctx: Context<QuestEntryFeeContext>, nonce: String) -> Result<()>
    {
        let funds_handler_account = &mut ctx.accounts.funds_handler_account; 
        let if_paid_account = &mut ctx.accounts.if_paid_account; 
        let funds_handler_pubkey = &ctx.accounts.funds_handler_pubkey; 
        let entry_fee = 100000000;
        require!(ctx.accounts.payer.lamports() >= entry_fee,ErrorCode::InsufficientBalance);
        require!(funds_handler_account.funds_handler == funds_handler_pubkey.key(), ErrorCode::UnauthorizedFundsHandler);
        require!(if_paid_account.if_paid == false, ErrorCode::AlreadyPaid);

        let ix = system_instruction::transfer
        (
            &ctx.accounts.payer.key(), 
            &ctx.accounts.funds_handler_pubkey.key(), 
            entry_fee
        );

        anchor_lang::solana_program::program::invoke
        (
            &ix, 
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.funds_handler_pubkey.to_account_info(),
             ],
        )?;

        if_paid_account.if_paid = true; 
        
        msg!("user:{}", *ctx.accounts.payer.key);
        msg!("Nonce:{}", nonce.clone());
        
        emit!(EntryFeeEvent{
            user: *ctx.accounts.payer.key, 
            nonce: nonce 
        });

        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeContext<'info>
{   
    #[account(
        init, 
        payer = payer, 
        seeds = [b"initialize_account"], 
        bump, 
        space = size_of::<Initialize>() + 8 
    )]
    pub initialize_account: Account<'info, Initialize>,

    #[account(
        init_if_needed, 
        payer = payer, 
        seeds = [b"funds_handler_account"], 
        bump, 
        space = size_of::<FundsHandler>() + 8 
    )]
    pub funds_handler_account: Account<'info,FundsHandler>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct QuestEntryFeeContext<'info>
{
    #[account(
        init_if_needed, 
        payer = payer, 
        seeds = [b"funds_handler_account"], 
        bump, 
        space = size_of::<FundsHandler>() + 8 
    )]
    pub funds_handler_account: Account<'info,FundsHandler>,

    #[account(
        init_if_needed, 
        payer = payer, 
        seeds = [b"if_paid_account"], 
        bump, 
        space = size_of::<PaymentCheck>() + 8 
    )]
    pub if_paid_account: Account<'info,PaymentCheck>,

    #[account(mut)]
    pub funds_handler_pubkey: SystemAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account] 
pub struct Initialize 
{
    pub initialize: bool, 
}

#[account] 
pub struct FundsHandler 
{
    pub funds_handler: Pubkey, 
}

#[account] 
pub struct PaymentCheck 
{
    pub if_paid: bool, 
}

#[event] 
pub struct EntryFeeEvent
{
    pub user: Pubkey, 
    pub nonce: String
}

#[error_code]
pub enum ErrorCode
{
    #[msg("Already initialized!")]
    AlreadyInitialized,

    #[msg("Insufficient Balance!")]
    InsufficientBalance,

    #[msg("Unauthorized FundsHandler!")]
    UnauthorizedFundsHandler,

    #[msg("Already Paid!")]
    AlreadyPaid,
}