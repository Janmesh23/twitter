
use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

use crate::errors::TwitterError;
use crate::states::*;

pub fn add_comment(ctx: Context<AddCommentContext>, comment_content: String) -> Result<()> {
    // length check (tests use 500 bytes)
    if comment_content.chars().count() > COMMENT_LENGTH {
        return Err(TwitterError::CommentTooLong.into());
    }

    let comment = &mut ctx.accounts.comment;
    let tweet = &ctx.accounts.tweet;

    // Fill comment data
    comment.comment_author = *ctx.accounts.comment_author.key;
    comment.parent_tweet = tweet.key();
    comment.content = comment_content;
    comment.bump = ctx.bumps.comment;

    Ok(())
}

#[derive(Accounts)]
#[instruction(comment_content: String)]
pub struct AddCommentContext<'info> {
    /// The author who pays and signs
    #[account(mut)]
    pub comment_author: Signer<'info>,

    /// The PDA for this specific comment
    /// Seeds: COMMENT_SEED, author, sha256(content), parent tweet
    #[account(
        init,
        payer = comment_author,
        space = 8 + Comment::INIT_SPACE,
        seeds = [
            COMMENT_SEED.as_bytes(),
            comment_author.key().as_ref(),
            &hash(comment_content.as_bytes()).to_bytes()[..],
            tweet.key().as_ref(),
        ],
        bump
    )]
    pub comment: Account<'info, Comment>,

    /// Parent tweet must exist
    pub tweet: Account<'info, Tweet>,

    pub system_program: Program<'info, System>,
}