use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn remove_reaction(ctx: Context<RemoveReactionContext>) -> Result<()> {
    let tweet = &mut ctx.accounts.tweet;
    let tweet_reaction = &ctx.accounts.tweet_reaction;

    // -------------------------------------------------------------------------------------------
    // TODO We can see that unlike the add_reaction function, the remove_reaction function does not
    // include reaction within its input arguments, that is because we have reaction type stored within
    // the tweet_reaction Account. Accordingly check what reaction we want to remove and
    // modify the number of Likes/Dislikes within the Tweet Account accordingly.
    // Return an error in case of over/underflow.

    // HINT: tweet.likes = tweet.likes.checked_sub(1).ok_or(TwitterError::MinLikesReached)?

    // -------------------------------------------------------------------------------------------

    match tweet_reaction.reaction { // Check the reaction type stored in the tweet_reaction account
        ReactionType::Like => {
            tweet.likes = tweet.likes.checked_sub(1).ok_or(TwitterError::MinLikesReached)?;
        }
        ReactionType::Dislike => {
            tweet.dislikes = tweet.dislikes.checked_sub(1).ok_or(TwitterError::MinDislikesReached)?;
        }
    }

    Ok(())
}
#[derive(Accounts)]
pub struct RemoveReactionContext<'info> {
    #[account(mut)]
    pub reaction_author: Signer<'info>,
    #[account(
        mut,
        close=reaction_author,
        seeds = [
            TWEET_REACTION_SEED.as_bytes(),
            reaction_author.key().as_ref(),
            tweet.key().as_ref(),
        ],
        bump = tweet_reaction.bump,
    // -------------------------------------------------------------------------------------------
    // TODO fill the seeds for proper generating of PDA and don`t forget to check bump

    // HINT: check how seeds are used within the AddReactionContext
    // -------------------------------------------------------------------------------------------
    )]
    pub tweet_reaction: Account<'info, Reaction>,
    // -------------------------------------------------------------------------------------------
    // TODO fill the required account macro below

    // HINT:
    // - account should be mutable
    // - seeds are :    tweet.topic[..tweet.topic_length as usize].as_ref()
    //                  TWEET_SEED.as_bytes(),
    //                  tweet.tweet_author.key().as_ref()
    // - lastly, check the correctness of bump using: bump = tweet.bump
    // -------------------------------------------------------------------------------------------
    #[account(
        mut,
        seeds = [ 
            tweet.topic[..tweet.topic_length as usize].as_ref(),
            TWEET_SEED.as_bytes(),
            tweet.tweet_author.key().as_ref()
        ],
        bump = tweet.bump 
    )]
    pub tweet: Account<'info, Tweet>,
}
