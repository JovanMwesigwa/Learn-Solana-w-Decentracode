use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("BmNhEDbi5h2i9k2Z7wNg2t354Ez6VEaqwmkEnMxcUVUB");

#[program]
mod heroes {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {}
}

// 1. handling state / State management (accounting system) ✅
// 2. CRUD functions | Changing state | Mutating (accounts)
// 3. Testing
// 4. Deploying

// Funtionality -> Add a hero (Info: name, age, universe())  CRUD

// Hero Account info
#[account]
pub struct Hero {
    name: String,
    age: i32,
    bio: String,
    created_at: i64,
}

// 2. Constants to calculate the size of Hero account
const DISCRIMINATOR_LENGTH: usize = 8;
const MAX_NAME_LENGTH: usize = 50 * 4; // maximum is 50
const AGE_LENGTH: usize = 4;
const MAX_BIO_LENGTH: usize = 240 * 4; // maximum is 240 * 4
const TIMESTAMP_LENGTH: usize = 8;

// 3. Add a implementation to find the total size of the Hero Account
impl Hero {
    const LEN: usize =
        DISCRIMINATOR_LENGTH + MAX_NAME_LENGTH + AGE_LENGTH + MAX_BIO_LENGTH + TIMESTAMP_LENGTH;
}
