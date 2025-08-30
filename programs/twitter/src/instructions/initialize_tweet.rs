// programs/twitter/src/instructions/initialize_tweet.rs
use anchor_lang::prelude::*;
use crate::errors::TwitterError;
use crate::states::*;

pub fn initialize_tweet(
    ctx: Context<InitializeTweet>,
    topic: String,
    content: String,
) -> Result<()> {
    // Validate topic length
    if topic.chars().count() > TOPIC_LENGTH {
        return Err(TwitterError::TopicTooLong.into());
    }

    // Validate content length
    if content.chars().count() > CONTENT_LENGTH {
        return Err(TwitterError::ContentTooLong.into());
    }

    // Fill in Tweet account data
    let tweet = &mut ctx.accounts.tweet;
    tweet.tweet_author = *ctx.accounts.tweet_authority.key;
    tweet.topic = topic;
    tweet.content = content;
    tweet.likes = 0;
    tweet.dislikes = 0;
    tweet.bump = ctx.bumps.tweet;

    Ok(())
}

#[derive(Accounts)]
#[instruction(topic: String)]
pub struct InitializeTweet<'info> {
    #[account(
        init,
        payer = tweet_authority,
        space = 8 + Tweet::INIT_SPACE,
        seeds = [
            topic.as_bytes(),               // tests compute PDA with topic first
            TWEET_SEED.as_bytes(),
            tweet_authority.key().as_ref()
        ],
        bump
    )]
    pub tweet: Account<'info, Tweet>,

    #[account(mut)]
    pub tweet_authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
