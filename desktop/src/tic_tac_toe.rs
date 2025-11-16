use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn other(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Playing,
    Won(Player),
    Draw,
}

#[derive(Debug, Clone)]
pub struct TicTacToe {
    board: [Option<Player>; 9],
    current_player: Player,
    pub game_state: GameState,
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            board: [None; 9],
            current_player: Player::X,
            game_state: GameState::Playing,
        }
    }

    pub fn get_cell(&self, position: usize) -> Option<Player> {
        if position < 9 {
            self.board[position]
        } else {
            None
        }
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn make_move(&mut self, position: usize) -> bool {
        if position >= 9 || self.board[position].is_some() || self.game_state != GameState::Playing
        {
            return false;
        }

        self.board[position] = Some(self.current_player);

        if self.check_winner() {
            self.game_state = GameState::Won(self.current_player);
        } else if self.is_board_full() {
            self.game_state = GameState::Draw;
        } else {
            self.current_player = self.current_player.other();
        }

        true
    }

    fn check_winner(&self) -> bool {
        let winning_positions = [
            [0, 1, 2], // Top row
            [3, 4, 5], // Middle row
            [6, 7, 8], // Bottom row
            [0, 3, 6], // Left column
            [1, 4, 7], // Middle column
            [2, 5, 8], // Right column
            [0, 4, 8], // Diagonal \
            [2, 4, 6], // Diagonal /
        ];

        for positions in &winning_positions {
            if let (Some(a), Some(b), Some(c)) = (
                self.board[positions[0]],
                self.board[positions[1]],
                self.board[positions[2]],
            ) {
                if a == b && b == c {
                    return true;
                }
            }
        }

        false
    }

    fn is_board_full(&self) -> bool {
        self.board.iter().all(|cell| cell.is_some())
    }

    pub fn reset(&mut self) {
        self.board = [None; 9];
        self.current_player = Player::X;
        self.game_state = GameState::Playing;
    }
}
