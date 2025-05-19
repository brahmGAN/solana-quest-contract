use anchor_lang::{prelude::*, solana_program::system_instruction};
use std::mem::size_of;

declare_id!("F5qfUhqkqEovEkQnxziJAVkY8spYevZU6Jd8VFCujYbJ");

#[program]
pub mod entry_fee_quest
{
    use super::*;

    pub fn initialize(ctx: Context<InitializeContext>, funds_handler: Pubkey, entry_fee: u64) -> Result<()> 
    {
        let initialize_account = &mut ctx.accounts.initialize_account; 
        let entry_fee_account = &mut ctx.accounts.entry_fee_account;
        require!(initialize_account.initialize == false,ErrorCode::AlreadyInitialized);
        entry_fee_account.funds_handler = funds_handler;
        entry_fee_account.entry_fee = entry_fee;  
        entry_fee_account.owner = ctx.accounts.payer.key(); 
        initialize_account.initialize = true; 
        Ok(())
    }

    pub fn quest_entry_fee(ctx: Context<QuestEntryFeeContext>, nonce: String) -> Result<()>
    {
        let entry_fee_account = &mut ctx.accounts.entry_fee_account; 
        let if_paid_account = &mut ctx.accounts.if_paid_account; 
        let funds_handler_pubkey = &ctx.accounts.funds_handler_pubkey; 
        let entry_fee = entry_fee_account.entry_fee;
        require!(ctx.accounts.payer.lamports() >= entry_fee,ErrorCode::InsufficientBalance);
        require!(entry_fee_account.funds_handler == funds_handler_pubkey.key(), ErrorCode::UnauthorizedFundsHandler);

        if_paid_account.if_paid = true; 

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
        
        msg!("user:{}", *ctx.accounts.payer.key);
        msg!("Nonce:{}", nonce);
        
        emit!(EntryFeeEvent{
            user: *ctx.accounts.payer.key, 
            nonce: nonce 
        });
        
        Ok(())
    }

    pub fn set_entry_fee(ctx: Context<SetEntryFeeContext>, entry_fee: u64) -> Result<()>
    {
        let entry_fee_account = &mut ctx.accounts.entry_fee_account; 
        require!(entry_fee_account.owner == ctx.accounts.payer.key(), ErrorCode::NotAuthorized);

        entry_fee_account.entry_fee = entry_fee; 

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeContext<'info>
{   
    #[account(
        init, 
        payer = payer, 
        seeds = [b"initialize"], 
        bump, 
        space = size_of::<Initialize>() + 8 
    )]
    pub initialize_account: Account<'info, Initialize>,

    #[account(
        init_if_needed, 
        payer = payer, 
        seeds = [b"entry_fee_account"], 
        bump, 
        space = size_of::<EntryFee>() + 8 
    )]
    pub entry_fee_account: Account<'info,EntryFee>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(nonce:String)]
pub struct QuestEntryFeeContext<'info>
{
    #[account(
        init_if_needed, 
        payer = payer, 
        seeds = [b"entry_fee_account"], 
        bump, 
        space = size_of::<EntryFee>() + 8 
    )]
    pub entry_fee_account: Account<'info,EntryFee>,

    #[account(
        init_if_needed, 
        payer = payer, 
        seeds = [nonce.as_bytes()], 
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

#[derive(Accounts)]
pub struct SetEntryFeeContext<'info>
{
    #[account(
        init_if_needed, 
        payer = payer, 
        seeds = [b"entry_fee_account"], 
        bump, 
        space = size_of::<EntryFee>() + 8 
    )]
    pub entry_fee_account: Account<'info,EntryFee>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


//seeds: "initialize_account"
#[account] 
pub struct Initialize 
{
    pub initialize: bool, 
}

//seeds: "entry_fee_account"
#[account] 
pub struct EntryFee 
{
    pub funds_handler: Pubkey, 
    pub entry_fee: u64, 
    pub owner: Pubkey 
}

//seeds: nonce passed as paremeter 
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

    #[msg("Not Authorized!")]
    NotAuthorized,
}