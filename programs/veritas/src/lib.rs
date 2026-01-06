use anchor_lang::prelude::*;

declare_id!("4ZVTFnkgbMzMFJacXTGVwEqFr8GbbcGuMAioX4Qxa1Jx");

#[program]
pub mod veritas {
    use super::*;

    pub fn register_node(ctx: Context<RegisterNode>, device_id: String) -> Result<()> {
        require!(device_id.len() <= 32, ErrorCode::IdTooLong);
        let node = &mut ctx.accounts.node;
        
        node.owner = *ctx.accounts.signer.key;
        node.device_id = device_id;
        node.is_active = true;
        node.reward_score = 0;
        node.last_ping = Clock::get()?.unix_timestamp;
        node.bump = ctx.bumps.node;
        
        msg!("Level 1.1 Success: Node registered for owner {}", node.owner);
        Ok(())
    }

    pub fn submit_data_hash(ctx: Context<SubmitData>, data_hash: [u8; 32]) -> Result<()> {
        let node = &mut ctx.accounts.node;
        node.last_ping = Clock::get()?.unix_timestamp;
        node.reward_score += 1; // Fixed: Added semicolon here
        
        msg!("Heartbeat received from node {}: {:?}", node.device_id, data_hash);
        Ok(())
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        let node = &mut ctx.accounts.node;
        
        require!(node.reward_score > 0, ErrorCode::NoRewardsToClaim);
        
        msg!("Rewards claimed for node {} with score {}", node.device_id, node.reward_score);
        
        node.reward_score = 0;
        Ok(())
    }
}

// 1. Unified Account Struct (Defined ONLY ONCE)
#[account]
pub struct NodeAccount {
    pub owner: Pubkey,
    pub device_id: String,
    pub is_active: bool,
    pub reward_score: u64,
    pub last_ping: i64,
    pub bump: u8,
}

impl NodeAccount {
    const LEN: usize = 8 + 32 + 36 + 1 + 8 + 8 + 1;
}

#[derive(Accounts)]
#[instruction(device_id: String)]
pub struct RegisterNode<'info> {
    #[account(
        init,
        payer = signer,
        space = NodeAccount::LEN,
        seeds = [b"node", signer.key().as_ref(), device_id.as_bytes()],
        bump
    )]
    pub node: Account<'info, NodeAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitData<'info> {
    #[account(
        mut,
        seeds = [b"node", signer.key().as_ref(), node.device_id.as_bytes()],
        bump = node.bump,
        has_one = owner, 
    )]
    pub node: Account<'info, NodeAccount>,
    pub signer: Signer<'info>, // Use 'signer' for the user
    /// CHECK: Target owner verified by has_one
    pub owner: SystemAccount<'info>, 
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(
        mut,
        seeds = [b"node", signer.key().as_ref(), node.device_id.as_bytes()],
        bump = node.bump,
        has_one = owner,
    )]
    pub node: Account<'info, NodeAccount>,
    #[account(mut)]
    pub signer: Signer<'info>, // Use 'signer' here
    /// CHECK: Target owner verified by has_one
    pub owner: SystemAccount<'info>, // Use 'owner' here (no duplication)
}

#[error_code]
pub enum ErrorCode {
    #[msg("The provided Device ID is too long. Max 32 characters")]
    IdTooLong,
    #[msg("No rewards available to claim")]
    NoRewardsToClaim,
}