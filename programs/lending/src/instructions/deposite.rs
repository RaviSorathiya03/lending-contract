use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked}};

use crate::states::{Bank, User};

#[derive(Accounts)]
pub struct Deposite<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut, 
        seeds = [mint.key().as_ref()],
        bump, 
    )]
    pub bank: Account<'info, Bank>,

    #[account(
        mut, 
        seeds = [b"treasury", mint.key().as_ref()],
        bump
    )]
    pub bank_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut, 
        seeds = [signer.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, User>,

    #[account(
        mut, 
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub user_token_account: Interface<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

pub fn process_deposit(
    ctx: Context<Deposite>,
    amount: u64
) -> Result<()>{
    //transfereing the token from the user token account to the bank token account
    let transfer_cpi_accounts = TransferChecked{
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.bank_token_account.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(), 
        mint: ctx.accounts.mint.to_account_info()
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, transfer_cpi_accounts);
    let decimals = ctx.accounts.mint.decimals;
    token_interface::transfer_checked(cpi_ctx, amount, decimals)?;

    //updating the state of the bank and the user
    let bank = &mut ctx.accounts.bank;

    if bank.total_deposites == 0{
        bank.total_deposites = amount;
        bank.total_deposite_shares = amount;
    } 

    let deposite_ratio = amount.checked_div(bank.total_deposites).unwrap();
    let user_shares = amount.checked_mul(deposite_ratio);

    let user = &mut ctx.accounts.user_account;

    match ctx.accounts.mint.to_account_info().key(){
        key if key == user.usdc_address => {
            user.desposited_usdc += amount;
            user.deposited_usdc_share += amount;
        },
        _=> {
            user.deposited_sol += amount;
            user.deposited_sol_share += user_shares;
        }
    }
    bank.total_deposites += amount;
    bank.total_deposite_shares += user_shares;
    Ok(())
}
