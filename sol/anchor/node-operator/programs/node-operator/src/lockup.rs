#[account]
pub struct Vesting {
    /// The owner of this Vesting account.
    pub beneficiary: Pubkey,
    /// The mint of the SPL token locked up.
    pub mint: Pubkey,
    /// Address of the account's token vault.
    pub vault: Pubkey,
    /// The owner of the token account funding this account.
    pub grantor: Pubkey,
    /// The outstanding SRM deposit backing this vesting account. All
    /// withdrawals will deduct this balance.
    pub outstanding: u64,
    /// The starting balance of this vesting account, i.e., how much was
    /// originally deposited.
    pub start_balance: u64,
    /// The unix timestamp at which this vesting account was created.
    pub created_ts: i64,
    /// The time at which vesting begins.
    pub start_ts: i64,
    /// The time at which all tokens are vested.
    pub end_ts: i64,
    /// The number of times vesting will occur. For example, if vesting
    /// is once a year over seven years, this will be 7.
    pub period_count: u64,
    /// The amount of tokens in custody of whitelisted programs.
    pub whitelist_owned: u64,
    /// Signer nonce.
    pub nonce: u8,
    /// The program that determines when the locked account is **realized**.
    /// In addition to the lockup schedule, the program provides the ability
    /// for applications to determine when locked tokens are considered earned.
    /// For example, when earning locked tokens via the staking program, one
    /// cannot receive the tokens until unstaking. As a result, if one never
    /// unstakes, one would never actually receive the locked tokens.
    pub realizor: Option<Realizor>,
}

#[account]
pub struct NodeOperatorVesting {
    pub recipient: PubKey,
    pub mint: PubKey,
    pub vault: PubKey,
    pub outstanding_balance: u64,
    pub initial_balance: u64,
    pub created_at_timestamp: u64,
    pub started_at_timestamp: u64,
    pub ended_at_timestamp: u64,
    pub vesting_count: u64,
    pub realizor: Option<Realizor>,
}


// Create a vesting account


// Withdrawing from a vesting account.


// Realizing of locked X tokens
// Realizor checks if the below requirements are fulfilled.\
- Vesting scheduled
- node has been running:
    if running uninteruptedly, 
        release for the period
    else if interrupted:
        skip the interrupted epoch

    
