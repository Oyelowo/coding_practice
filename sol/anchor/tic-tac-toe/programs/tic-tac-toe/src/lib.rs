use anchor_lang::prelude::*;
use num_derive::*;
use num_traits::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod tic_tac_toe {
    use super::*;

    pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.players = [ctx.accounts.player_one.key(), player_two];
        game.turn = 1;
        Ok(())
    }

    pub fn play(ctx: Context<Play>, tile: Tile) -> Result<()> {
        let game = &mut ctx.accounts.game;
        /*
        We've checked in the accounts struct that the player account has signed
        the transaction, but we do not check that it is the player we expect.
        That's what the require check in play is for.
        */
        require!(
            game.current_player() == ctx.accounts.player.key(),
            TicTacToeError::NotPlayersTurn
        );
        game.play(&tile)
    }
}

/*
NOTE: SIZE
(What about using mem::size_of<Game>()? This almost works but not quite.
The issue is that borsh will always serialize an option as 1 byte for the
variant identifier and then additional x bytes for the content if it's Some.
Rust uses null-pointer optimization to make Option's variant identifier 0 bytes
when it can, so an option is sometimes just as big as its contents.
This is the case with Sign. This means the MAXIMUM_SIZE could be expressed
as mem::size_of<Game>() + 9.)


*/

/*
In addition to the game's size, we have to add another 8 to the space.
This is space for the discriminator which anchor sets automatically.
In short, the discriminator is how anchor can differentiate between
different accounts of the same program.
*/
#[derive(Accounts)]
pub struct SetupGame<'info> {
    #[account(init, payer = player_one, space = Game::MAXIMUM_SIZE + 8)]
    pub game: Account<'info, Game>,

    #[account(mut)]
    pub player_one: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Tile {
    row: u8,
    column: u8,
}

#[error_code]
pub enum TicTacToeError {
    GameAlreadyOver,
    TileAlreadySet,
    TileOutOfBounds,
    NotPlayersTurn,
}

impl Default for GameState {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(
    AnchorSerialize, AnchorDeserialize, FromPrimitive, ToPrimitive, Copy, Clone, PartialEq, Eq,
)]
pub enum Sign {
    X,
    O,
}

#[derive(Accounts)]
pub struct Play<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    pub player: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Tie,
    Won { winner: Pubkey },
}

#[account]
#[derive(Default)]
pub struct Game {
    players: [Pubkey; 2], // Pubkey as a vector has a length of 32 so 2*32 = 64 ✅
    turn: u8,             // u8 as a vector has a length of 1 so 1 = 1 ✅
    board: [[Option<Sign>; 3]; 3], // 9 * (1 + 1) = 18 |  board's default (9 * None) as a vector has a length of 9 != 18 ❌. So, the default wont guess right
    state: GameState, // 32 + 1 | state's default as a vector is 1 != 33 ❌. So, the default wont guess right
}

impl Game {
    /*
    We have found out that init currently only allocates 75 bytes for our
    account data but the account can grow to (64 + 1 + 18 + 33) = 116 bytes.
     We can add this number to our Game impl like this:
    */
    const MAXIMUM_SIZE: usize = 116;
    pub fn is_active(&self) -> bool {
        self.state == GameState::Active
    }
    fn current_player_index(&self) -> usize {
        ((self.turn - 1) % 2) as usize
    }
    pub fn current_player(&self) -> Pubkey {
        self.players[self.current_player_index()]
    }

    pub fn play(&mut self, tile: &Tile) -> Result<()> {
        if !self.is_active() {
            return err!(TicTacToeError::GameAlreadyOver);
        }
        match tile {
            tile @ Tile {
                row: 0..=2,
                column: 0..=2,
            } => match self.board[tile.row as usize][tile.column as usize] {
                Some(_) => return err!(TicTacToeError::TileAlreadySet),
                None => {
                    self.board[tile.row as usize][tile.column as usize] =
                        Some(Sign::from_usize(self.current_player_index()).unwrap());
                }
            },
            _ => return err!(TicTacToeError::TileOutOfBounds),
        }

        self.update_state();

        if let GameState::Active = self.state {
            self.turn += 1;
        }

        Ok(())
    }

    fn is_winning_trio(&self, trio: [(usize, usize); 3]) -> bool {
        let [first, second, third] = trio;
        self.board[first.0][first.1].is_some()
            && self.board[first.0][first.1] == self.board[second.0][second.1]
            && self.board[first.0][first.1] == self.board[third.0][third.1]
    }

    fn update_state(&mut self) {
        for i in 0..=2 {
            // three of the same in one row
            if self.is_winning_trio([(i, 0), (i, 1), (i, 2)]) {
                self.state = GameState::Won {
                    winner: self.current_player(),
                };
                return;
            }
            // three of the same in one column
            if self.is_winning_trio([(0, i), (1, i), (2, i)]) {
                self.state = GameState::Won {
                    winner: self.current_player(),
                };
                return;
            }
        }

        // three of the same in one diagonal
        if self.is_winning_trio([(0, 0), (1, 1), (2, 2)])
            || self.is_winning_trio([(0, 2), (1, 1), (2, 0)])
        {
            self.state = GameState::Won {
                winner: self.current_player(),
            };
            return;
        }

        // reaching this code means the game has not been won,
        // so if there are unfilled tiles left, it's still active
        for row in 0..=2 {
            for column in 0..=2 {
                if self.board[row][column].is_none() {
                    return;
                }
            }
        }

        // game has not been won
        // game has no more free tiles
        // -> game ends in a tie
        self.state = GameState::Tie;
    }
}
