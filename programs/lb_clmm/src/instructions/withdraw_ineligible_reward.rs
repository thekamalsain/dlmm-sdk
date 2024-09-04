use crate::{
    state::{bin::BinArray, lb_pair::LbPair},
    utils::remaining_accounts_util::RemainingAccountsInfo,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    memo::Memo,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

#[event_cpi]
#[derive(Accounts)]
#[instruction(reward_index: u64)]
pub struct WithdrawIneligibleReward<'info> {
    #[account(mut)]
    pub lb_pair: AccountLoader<'info, LbPair>,

    #[account(mut)]
    pub reward_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub reward_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub funder_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub funder: Signer<'info>,

    #[account(
        mut,
        has_one = lb_pair
    )]
    pub bin_array: AccountLoader<'info, BinArray>,

    pub token_program: Interface<'info, TokenInterface>,
    pub memo_program: Program<'info, Memo>,
}

pub fn handle(
    ctx: Context<WithdrawIneligibleReward>,
    index: u64,
    remaining_accounts_info: RemainingAccountsInfo,
) -> Result<()> {
    Ok(())
}
