use anchor_lang::prelude::*;
declare_id!("B1cV6QaeD1iCrAeHKNSfCbJ9wBG6SYNjzYTV84nBsBKq");

#[program]
pub mod counter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.authority = ctx.accounts.authority.key();
        counter.count = 0;
        msg!("Counter created for authority: {}", counter.key());
        Ok(())
    }
    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        Ok(())
    }
    pub fn reset(ctx: Context<Reset>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        Ok(())
    }
    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count -=1;
        Ok(())
    }
    pub fn close(ctx: Context<Close>) -> Result<()> {
    // Anchor 会自动关闭账户并把 lamports 转给 destination（这里是 user）
        Ok(())
    }   
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8,
        seeds = [b"counter", authority.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info,Counter>,
    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info,Counter>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(
        mut,
        close = authority,  // 关闭后 lamports 返还给 user
        seeds = [b"counter", authority.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Reset<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [b"counter", authority.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info,Counter>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}