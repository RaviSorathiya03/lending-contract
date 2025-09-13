use anchor_lang::prelude::*;
mod states;
mod instructions;
use instruction::*;
declare_id!("GJwhyVddz4g84PnpRETVuuMZiEd8LvQnPoX4SCXEQEkY");

#[program]
pub mod lending {
    use crate::instructions::{process_deposit, process_init_bank, process_init_user};

    use super::*;

    pub fn init_bank(
        ctx: Context<InitBank>,
        liquidation_threshold: u64, 
        max_ltv: u64
    ) -> Result<()>{
        process_init_bank(ctx, liquidation_threshold, max_ltv)
    }

    pub fn init_user(
        ctx: Context<InitUser>,
        usdc_address: Pubkey
    ) -> Result<()>{
        process_init_user(ctx, usdc_address)
    }

    pub fn deposite(
        ctx: Context<Deposite>,
        amount: u64
    ) -> Result<()>{
        process_deposit(ctx, amount)
    }
}

