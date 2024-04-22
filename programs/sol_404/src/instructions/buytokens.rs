// 转账usdc 可以获得jelly token, 检测当前账户的jelly token数目 以及拥有的nft数目, 给他mint nft

use std::default;

use anchor_lang::{accounts::program, prelude::*};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, TokenAccount, mint_to, transfer, Mint, MintTo, Transfer, Token}
};

use crate::error::*;


pub fn buytokens(
    ctx: Context<BuyTokens>,
    amount: u64
) -> Result<()>{
    let bta = &ctx.accounts.buyer_token_account;
    let bta_balance = &bta.amount;

    if *bta_balance < amount {
        return err!(TokenUseError::NotEnoughBalance);
    }

    let seeds = b"jelly";
    let bump = ctx.bumps.jelly_token_mint;
    let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

    
    let token_balance = & ctx.accounts.buyer_token_account.amount;
    
    let mut nft_mint = (*token_balance % 10 + amount) % 10;

    // transfer nft
    //这里是有问题的 转nft 但是确是buyer_usdc_account
    while nft_mint > 0{
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.buyer_usdc_account.to_account_info(), // mint account of token to mint
            to: ctx.accounts.jelly_usdc_account.to_account_info(), // worker token account to mint to
            authority: ctx.accounts.buyer.to_account_info(), // pda is used as both address of mint and mint authority
        }
    );
    transfer(cpi_ctx, 1)?;
    nft_mint -= 1;
    };


    // mint token to buyer
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.jelly_token_mint.to_account_info(), // mint account of token to mint
            to: ctx.accounts.buyer_token_account.to_account_info(), // worker token account to mint to
            authority: ctx.accounts.jelly_token_mint.to_account_info(), // pda is used as both address of mint and mint authority
        },
        signer,
    );

    // Mint token, accounting for decimals of mint
    let transfer_jelly_amount = amount
        .checked_mul(10u64.pow(ctx.accounts.usdc_token_mint.decimals as u32))
        .unwrap();

    mint_to(cpi_ctx, transfer_jelly_amount)?;
    
    

    // 从buyer账户转钱给我们
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.buyer_usdc_account.to_account_info(), // mint account of token to mint
            to: ctx.accounts.jelly_usdc_account.to_account_info(), // worker token account to mint to
            authority: ctx.accounts.payer.to_account_info(), // pda is used as both address of mint and mint authority
        }
    );

    //这里应该和我们的token值多少钱相关
    // Mint token, accounting for decimals of mint
    let transfer_token_amount = amount
        .checked_mul(10u64.pow(ctx.accounts.jelly_token_mint.decimals as u32))
        .unwrap();

    transfer(cpi_ctx, transfer_token_amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct BuyTokens<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = jelly_token_mint,
        associated_token::authority = payer
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = usdc_token_mint,
        associated_token::authority = payer
    )]
    pub buyer_usdc_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub jelly_usdc_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"jelly"],
        bump,
    )]
    pub jelly_token_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"usdc"],
        bump,
    )]
    pub usdc_token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub jelly_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}