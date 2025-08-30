// programs/twitter/src/instructions/add_reaction.rs
use anchor_lang::prelude::*;
use crate::errors::TwitterError;
use crate::states::*;

pub fn add_reaction(ctx: Context<AddReactionContext>, reaction: ReactionType) -> Result<()> {
    let tweet_reaction = &mut ctx.accounts.tweet_reaction;
    let tweet = &mut ctx.accounts.tweet;

    // Populate reaction account fields
    tweet_reaction.reaction_author = *ctx.accounts.reaction_author.key;
    tweet_reaction.parent_tweet = tweet.key();
    tweet_reaction.reaction = reaction.clone();
    tweet_reaction.bump = ctx.bumps.tweet_reaction;

    // Increment appropriate counter on tweet with overflow protection
    match reaction {
        ReactionType::Like => {
            tweet.likes = tweet.likes.checked_add(1).ok_or(TwitterError::MaxLikesReached)?;
        }
        ReactionType::Dislike => {
            tweet.dislikes = tweet.dislikes.checked_add(1).ok_or(TwitterError::MaxDislikesReached)?;
        }
    }

    Ok(())
}

#[derive(Accounts)]
pub struct AddReactionContext<'info> {
    // The signer who pays for the new reaction account
    #[account(mut)]
    pub reaction_author: Signer<'info>,

    // PDA for reaction: [TWEET_REACTION_SEED, author, parent_tweet]
    #[account(
        init,
        payer = reaction_author,
        space = 8 + Reaction::INIT_SPACE,
        seeds = [
            TWEET_REACTION_SEED.as_bytes(),
            reaction_author.key().as_ref(),
            tweet.key().as_ref()
        ],
        bump
    )]
    pub tweet_reaction: Account<'info, Reaction>,

    // Parent tweet that must already exist
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,

    pub system_program: Program<'info, System>,
}
