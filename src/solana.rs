use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("4axWNQPo1BuJvCwS5yYf1AJ6sF3AeaH7fMGSHTMEUM5A");
#[program]
mod heroes {
    use super::*;
    pub fn create_hero(
        ctx: Context<CreateHero>,
        name: String,
        age: i32,
        bio: String,
    ) -> Result<()> {
        let hero = &mut ctx.accounts.hero;
        let author = &mut ctx.accounts.author;
        let clock = Clock::get().unwrap();

        // 🔐 Guard from invalid and too long data...
        if name.chars().count() > 50 {
            // throw an error
            return Err(ErrorCode::NameTooLong.into());
        }

        if bio.chars().count() > 240 {
            // Throw an error
            return Err(ErrorCode::BioTooLong.into());
        }

        // Create the hero...
        hero.name = name;
        hero.age = age;
        hero.bio = bio;
        hero.author = author.key();
        hero.created_at = clock.unix_timestamp;

        Ok(())
    }
}

// Result will return -> Ok(result) | Err(error)

// Funtionality -> Add a hero (Info: name, age, universe())  CRUD
// Create the Hero... (name, age, bio..)
#[derive(Accounts)]
pub struct CreateHero<'info> {
    #[account(init, payer=author, space=Hero::LEN )]
    pub hero: Account<'info, Hero>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Hero Account info
#[account]
pub struct Hero {
    name: String,
    age: i32,
    bio: String,
    author: Pubkey,
    created_at: i64,
}

// 2. Constants to calculate the size of Hero account
const DISCRIMINATOR_LENGTH: usize = 8;
const MAX_NAME_LENGTH: usize = 50 * 4; // maximum is 50
const AGE_LENGTH: usize = 4;
const MAX_BIO_LENGTH: usize = 240 * 4; // maximum is 240 * 4
const TIMESTAMP_LENGTH: usize = 8;
const AUTHOR_LENGTH: usize = 32;

// 3. Add a implementation to find the total size of the Hero Account
impl Hero {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + MAX_NAME_LENGTH
        + AGE_LENGTH
        + MAX_BIO_LENGTH
        + TIMESTAMP_LENGTH
        + AUTHOR_LENGTH;
}

#[error_code]
pub enum ErrorCode {
    #[msg("The name MUST not exceed 50 characters")]
    NameTooLong,
    #[msg("You bio MUST not exceed 240 characters")]
    BioTooLong,
}

// 1. handling state / State management (accounting system) ✅
// 2. CRUD functions | Changing state | Mutating (accounts)
// 3. Testing
// 4. Deploying
