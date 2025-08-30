
use anchor_lang::prelude::*;

use crate::states::*;

pub fn remove_comment(_ctx: Context<RemoveCommentContext>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct RemoveCommentContext<'info> {
    #[account(mut)]
    pub comment_author: Signer<'info>,
    
    #[account(
        mut,
        close = comment_author,
        has_one = comment_author
    )]
    pub comment: Account<'info, Comment>,
}