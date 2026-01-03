use anchor_lang::prelude::*;

declare_id!("9bCZC8RLBvooM8L5DsNhv1tazXV1oMdiWvaDFpd6wZLk");

#[program]
pub mod veritas {
    use super::*;

    pub fn register_node(ctx: Context<RegisterNode>, device_id: String) -> Result<()> {
        require!(device_id.len()<=32, ErrorCode::IdTooLong);
        let node = &mut ctx.accounts.node;
        node.owner = *ctx.accounts.signer.key;
        node.device_id = device_id.clone();
        node.is_active = true;
        node.last_ping = Clock::get()?.unix_timestamp;
        node.bump = ctx.bumps.node;
        msg!("Level 1.1 Success: Node {} registered for owner {}", device_id, node.owner);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(device_id: String)]
pub struct RegisterNode<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + Node::LEN,
        seeds = [b"node", signer.key().as_ref(), device_id.as_bytes()],
        bump
    )]
    pub node: Account<'info, Node>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Node {
    pub owner: Pubkey,
    pub device_id: String,
    pub is_active: bool,
    pub last_ping: i64,
    pub bump: u8,
}

impl Node {
    const LEN: usize = 32 + 100 + 1 + 8 + 1;
}
#[error_code]
pub enum ErrorCode{
    #[msg("The provided Device ID is too long. Max 32 characters")]
    IdTooLong,
}