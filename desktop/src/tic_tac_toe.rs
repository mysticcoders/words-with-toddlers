use rand::Rng;
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameMode {
    OnePlayer,
    TwoPlayer,
}

const WINNING_POSITIONS: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

#[derive(Debug, Clone)]
pub struct TicTacToe {
    board: [Option<Player>; 9],
    current_player: Player,
    pub game_state: GameState,
    pub winning_line: Option<[usize; 3]>,
    pub mode: GameMode,
}

impl TicTacToe {
    pub fn new(mode: GameMode) -> Self {
        TicTacToe {
            board: [None; 9],
            current_player: Player::X,
            game_state: GameState::Playing,
            winning_line: None,
            mode,
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

    pub fn is_computer_turn(&self) -> bool {
        self.mode == GameMode::OnePlayer
            && self.current_player == Player::O
            && self.game_state == GameState::Playing
    }

    pub fn make_move(&mut self, position: usize) -> bool {
        if position >= 9 || self.board[position].is_some() || self.game_state != GameState::Playing
        {
            return false;
        }

        self.board[position] = Some(self.current_player);

        if let Some(line) = self.check_winner() {
            self.game_state = GameState::Won(self.current_player);
            self.winning_line = Some(line);
        } else if self.is_board_full() {
            self.game_state = GameState::Draw;
        } else {
            self.current_player = self.current_player.other();
        }

        true
    }

    /// Computes the computer's move and plays it
    pub fn computer_move(&mut self) {
        if !self.is_computer_turn() {
            return;
        }

        let position = self.pick_computer_move();
        self.make_move(position);
    }

    /// Picks the best move for the computer (O) using simple strategy:
    /// 1. Win if possible
    /// 2. Block opponent from winning
    /// 3. Take center
    /// 4. Take a corner
    /// 5. Take any open spot
    fn pick_computer_move(&self) -> usize {
        let me = Player::O;
        let opponent = Player::X;

        // Try to win
        if let Some(pos) = self.find_winning_move(me) {
            return pos;
        }

        // Block opponent
        if let Some(pos) = self.find_winning_move(opponent) {
            return pos;
        }

        // Take center
        if self.board[4].is_none() {
            return 4;
        }

        // Take a corner (randomized to add variety)
        let corners = [0, 2, 6, 8];
        let open_corners: Vec<usize> = corners.iter().copied().filter(|&c| self.board[c].is_none()).collect();
        if !open_corners.is_empty() {
            let idx = rand::thread_rng().gen_range(0..open_corners.len());
            return open_corners[idx];
        }

        // Take any open spot
        self.board.iter().position(|c| c.is_none()).unwrap_or(0)
    }

    /// Finds a move that would complete a winning line for the given player
    fn find_winning_move(&self, player: Player) -> Option<usize> {
        for positions in &WINNING_POSITIONS {
            let cells: Vec<Option<Player>> = positions.iter().map(|&p| self.board[p]).collect();
            let player_count = cells.iter().filter(|&&c| c == Some(player)).count();
            let empty_count = cells.iter().filter(|&&c| c.is_none()).count();

            if player_count == 2 && empty_count == 1 {
                for &p in positions {
                    if self.board[p].is_none() {
                        return Some(p);
                    }
                }
            }
        }
        None
    }

    fn check_winner(&self) -> Option<[usize; 3]> {
        for positions in &WINNING_POSITIONS {
            if let (Some(a), Some(b), Some(c)) = (
                self.board[positions[0]],
                self.board[positions[1]],
                self.board[positions[2]],
            ) {
                if a == b && b == c {
                    return Some(*positions);
                }
            }
        }

        None
    }

    fn is_board_full(&self) -> bool {
        self.board.iter().all(|cell| cell.is_some())
    }

    pub fn reset(&mut self) {
        self.board = [None; 9];
        self.current_player = Player::X;
        self.game_state = GameState::Playing;
        self.winning_line = None;
    }
}
