use anchor_lang::prelude::*;
use anchor_spl::token::spl_token::state::Mint;

#[derive(Accounts)]
pub struct Deposite<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>
}