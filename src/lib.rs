use std::fmt;
use std::collections::HashSet;

mod lib_of_lib;
mod gamestate;

use lib_of_lib::piece::Piece;
use lib_of_lib::color::Color;
use gamestate::GameState; 


#[derive(Copy, Clone, PartialEq)]
pub struct Game {
    board: [[Option<(Piece, Color)>; 8]; 8],
    color: Color,
    state: GameState,
}



///This is a basic chess program with basic capabilities
///Since I was really interested in using a hashmap rahter 
///than defining every single position on the board I receieved
///a slight amount of help and inspiration from the some friends at the higher levels!



impl Game {
    /// Initialises a new board with pieces.
    /// begining with white pieces 
    pub fn new() -> Self {
        use Color::*;
        use Piece::*;
        Self {
            
            board: [
                [
                    Some((Bishop, White)),
                    Some((Knight, White)),
                    Some((Rook, White)),
                    Some((Queen, White)),
                    Some((King, White)),
                    Some((Knight, White)),
                    Some((Rook, White)),
                    Some((Bishop, White)),
                ],
                [
                    Some((Pawn, White)),
                    Some((Pawn, White)),
                    Some((Pawn, White)),
                    Some((Pawn, White)),
                    Some((Pawn, White)),
                    Some((Pawn, White)),
                    Some((Pawn, White)),
                    Some((Pawn, White)),
                ],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [
                    Some((Pawn, Black)),
                    Some((Pawn, Black)),
                    Some((Pawn, Black)),
                    Some((Pawn, Black)),
                    Some((Pawn, Black)),
                    Some((Pawn, Black)),
                    Some((Pawn, Black)),
                    Some((Pawn, Black)),
                ],
                [
                    Some((Bishop, Black)),
                    Some((Knight, Black)),
                    Some((Queen, Black)),
                    Some((Rook, Black)),
                    Some((King, Black)),
                    Some((Knight, Black)),
                    Some((Rook, Black)),
                    Some((Bishop, Black)),
                ],
            ],
            color: White,

            state: GameState::InProgress,
        }
    }

    /// If the current game state is in progress and the move is legal,
    /// move a piece and return the resulting state of the game.
    /// at all times the position of the king should be checked to see if the game is still 
    /// in progress
    pub fn make_move(&mut self, from: String, to: String) -> Option<GameState> {

        use GameState::*;
        let color = self.color;
        if self.get_game_state() == GameState::InProgress {
            if self.possible_move(&from).unwrap().contains(&to) {
                let from_index = self.find_position(&from);
                let to_index = self.find_position(&to);
                self.board[from_index.0 as usize][from_index.1 as usize] =
                    self.board[to_index.0 as usize][to_index.1 as usize];
            }
        }
        let king_position = self.king_position(color);
        match self.check_checker(&king_position, color) {
            InProgress => Some(InProgress),
            Check => Some(Check),
            GameOver => Some(GameOver),
        }
    }
    fn king_position(&self, color: Color) -> String {
        let mut king_position: String = String::new();
        use Piece::*;
        for rank in 0..=7 {
            for file in 0..=7 {
                if self.board[rank][file] == None {
                    continue;
                } else if self.board[rank][file].unwrap() == (King, color) {
                    king_position = self.index_to_string((rank, file));
                }
            }
        }
        king_position
    }


    /// Checking for checks
    /// We have to check if king is checked after every position
    /// and if the king can escape or has been checkmated
    /// the function gets the position of the king and loops through all the threats.

    fn check_checker(&self, position: &String, color: Color) -> GameState {
        use GameState::*;
        let mut output: GameState = InProgress;
        let king_moves: Vec<String> = self.king_moves(&position).unwrap();
        let mut hash_set: HashSet<String> = HashSet::new();
        for rank in 0..=7 {
            for file in 0..=7 {
                let piece = self.piece_position(&self
                    .index_to_string((rank, file)));
                let piece_position = self.index_to_string((rank, file));
                if piece != None {
                    if piece.unwrap().1 != color {
                        let position_moves: Vec<String> = self.possible_move(&piece_position).unwrap();
                        if position_moves.contains(&position) {
                            output = Check;
                        }
                    }
                }
                else {
                    hash_set.insert(piece_position);
                }
            }
        }
        if king_moves.iter().all(|king_move| hash_set
            .contains(king_move)) && hash_set.contains(position) {
            output = GameOver;
        }

        output
    }

    

    fn piece_position(&self, position: &String) -> Option<(Piece, Color)> {
        //Get index for position and return whatever is on that index
        let position = self.find_position(position);
        self.board[position.1 as usize][position.0 as usize]
    }
    fn find_position(&self, position: &String) -> (usize, usize) {
        //split string and turn into seperate variables based on color
        let rank: char = position[..1].parse().ok().unwrap();
        let file: usize = position[1..].parse().ok().unwrap();
        match self.color {
            Color::White => {
                let rank: usize = match rank {
                    'a' => 0,
                    'b' => 1,
                    'c' => 2,
                    'd' => 3,
                    'e' => 4,
                    'f' => 5,
                    'g' => 6,
                    'h' => 7,
                    _ => 0,
                };
                let file = file - 1;
                (rank, file)
            }
            Color::Black => {
                let rank: usize = match rank {
                    'a' => 7,
                    'b' => 6,
                    'c' => 5,
                    'd' => 4,
                    'e' => 3,
                    'f' => 2,
                    'g' => 1,
                    'h' => 0,
                    _ => 0,
                };
                let file: usize = match file {
                    1 => 7,
                    2 => 6,
                    3 => 5,
                    4 => 4,
                    5 => 3,
                    6 => 2,
                    7 => 1,
                    8 => 0,
                    _ => 0,
                };
                (rank, file)
                //Outside of scope is mapped to zero
            }
        }
    }


    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget to the rules for check.
    pub fn possible_move(&self, position: &String) -> Option<Vec<String>> {
        let piece = self.piece_position(&position);
        use Piece::*;
        if piece != None {
            match piece.unwrap().0 {
                King => self.king_moves(position),
                Queen => self.queen_moves(position),
                Bishop => self.bishop_moves(position),
                Knight => self.knight_moves(position),
                Rook => self.rook_moves(position),
                Pawn => self.pawn_moves(position),
            }
        } else {
            None
        }
    }
    fn relative_position(&self, position: &String, rank: i8, file: i8) -> Option<String> {
        let position = self.find_position(&position);
        let mut output: (usize, usize) = (
            (position.0 as i8 + file) as usize,
            (position.1 as i8 + rank) as usize,
        );
        if position.0 as i8 + file < 0 {
            output.0 = 0;
        }
        if position.0 as i8 + file > 7 {
            output.0 = 7;
        }
        if position.1 as i8 + rank < 0 {
            output.1 = 0;
        }
        if position.1 as i8 + rank > 7 {
            output.1 = 7;
        }

        Some(self.index_to_string(output))
    }

    fn index_to_string(&self, input: (usize, usize)) -> String {
        let mut output: String = String::with_capacity(2);
        output.push(match input.0 {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => ' ',
        });
        
        output.push(char::from_digit(input.1 as u32 + 1, 10).unwrap_or(' '));
        output

        //Panics if a piece is outside of board
    }




    ///In this part every potential legal move by every piece will be examined
    /// Would be interested to know if this part of the structure in Game can 
    /// be moved to lib_of_lib
    

    ///Pawn
    ///Pawn moves one move forward in normal movement
    /// It can move two positions in initial position 
    /// It can take other pieces by diagonal moves to left or right
    fn pawn_moves(&self, position: &String) -> Option<Vec<String>> {
        let mut output: Vec<String> = Vec::new();
        
        let basic_move = self.relative_position(position, 1, 0).unwrap();
        if self.piece_position(&basic_move) == None {
            output.push(basic_move);
        }

        let first_move_double = self.relative_position(position, 2, 0).unwrap();
        if self.find_position(position).0 == 7 && self.piece_position(&first_move_double) == None {
            output.push(first_move_double);
        };
        
        let take1 = self.relative_position(position, 1, 1).unwrap();
        if self.piece_position(&take1) != None {
            if self.piece_position(&take1).unwrap().1 != self.color {
                output.push(take1);
            }
        }

        let take2 = self.relative_position(position, 1, -1).unwrap();
        if self.piece_position(&take2) != None {
            if self.piece_position(&take2).unwrap().1 != self.color
                && &take2 != &self.relative_position(position, 1, -1).unwrap()
            {
                output.push(take2);
            }
        }

        Some(output)
    }

    ///For pieces which can make slighlty more complex manuvers we use loops to 
    /// check which legal positions are available

    ///Rook
    /// We check for all legal vertical and horizontal movements
    fn rook_moves(&self, _position: &String) -> Option<Vec<String>> {
        let pp = self.find_position(_position);
        let mut output: Vec<String> = Vec::new();

        for i in 1..(8 - pp.0 ) {
            let right = self.relative_position(_position, 0, i as i8).unwrap();
            if self.piece_position(&right) == None {
                output.push(right);
            } else {
                output.push(right);
                break;
            }
        }
        for i in 1..pp.1 {
            let left = self.relative_position(_position, 0, -(i as i8)).unwrap();
            if self.piece_position(&left) == None {
                output.push(left);
            } else {
                output.push(left);
                break;
            }
        }

        for i in 1..(8 - pp.1) {
            let down = self.relative_position(_position, i as i8, 0).unwrap();
            if self.piece_position(&down) == None {
                output.push(down);
            } else {
                output.push(down);
                break;
            }
        }

        for i in 1..pp.1 {
            let up = self.relative_position(_position, -(i as i8), 0).unwrap();
            if self.piece_position(&up) == None {
                output.push(up);
            } else {
                output.push(up);
                break;
            }
        }
        Some(output)
    }


    ///Knight 
    ///Quite complicated since it needs to move in three horizontal/diagonal and
    /// then two diagonal/horizontal and make a check for legality of the move
    /// Still unsatisfied with this method and recieved a lot of help in coding it 
    /// Inform me if you get any novel ideas
    
    fn knight_moves(&self, _position: &String) -> Option<Vec<String>> {
        let mut output: Vec<String> = Vec::new();
        let relative_rank: [i8; 8] = [1, 1, -1, -1, 2, 2, -2, -2];
        let relative_file: [i8; 8] = [2, -2, 2, -2, 1, -1, 1, -1];
        for i in 0..8 {
            let next_position = self
            .relative_position(_position, relative_rank[i], relative_file[i]).unwrap();
            if self.piece_position(&next_position) == None || self.piece_position(&next_position)
            .unwrap().1 != self.piece_position(&_position).unwrap().1
            {
                output.push(next_position);
            }
        }
        Some(output)
    }


    ///Bishop 
    /// Cross movements on the board
    fn bishop_moves(&self, _position: &String) -> Option<Vec<String>> {
        let position = self.find_position(_position);
        let mut output: Vec<String> = Vec::new();

        for i in 1..(8 - position.0) {
            let up_right = self.relative_position(_position, -(i as i8), i as i8).unwrap();
            if self.piece_position(&up_right) == None {
                output.push(up_right);
            } else {
                output.push(up_right);
                break;
            }
        }
        for i in 1..position.1 {
            let up_left = self.relative_position(_position, -(i as i8), -(i as i8)).unwrap();
            if self.piece_position(&up_left) == None {
                output.push(up_left);
            } else {
                output.push(up_left);
                break;
            }
        }

        for i in 1..(8 - position.1) {
            let down_right = self.relative_position(_position, i as i8, i as i8).unwrap();
            if self.piece_position(&down_right) == None {
                output.push(down_right);
            } else {
                output.push(down_right);
                break;
            }
        }
        for i in 1..position.1 {
            let down_left = self.relative_position(_position, i as i8, -(i as i8)).unwrap();
            if self.piece_position(&down_left) == None {
                output.push(down_left);
            } else {
                output.push(down_left);
                break;
            }
        }

        Some(output)
    }


    ///Queen
    /// We implement rook and bishop moves in queen
    fn queen_moves(&self, _position: &String) -> Option<Vec<String>> {
        let position = self.find_position(_position);
        let mut output: Vec<String> = Vec::new();
        for i in 1..(8 - position.0) {
            let up_right = self.relative_position(_position, -(i as i8), i as i8).unwrap();
            if self.piece_position(&up_right) == None {
                output.push(up_right);
            } else {
                output.push(up_right);
                break;
            }
        }
        for i in 1..position.1 {
            let up_left = self.relative_position(_position, -(i as i8), -(i as i8)).unwrap();
            if self.piece_position(&up_left) == None {
                output.push(up_left);
            } else {
                output.push(up_left);
                break;
            }
        }

        for i in 1..(8 - position.1) {
            let down_right = self.relative_position(_position, i as i8, i as i8).unwrap();
            if self.piece_position(&down_right) == None {
                output.push(down_right);
            } else {
                output.push(down_right);
                break;
            }
        }
        for i in 1..position.1 {
            let down_left = self.relative_position(_position, i as i8, -(i as i8)).unwrap();
            if self.piece_position(&down_left) == None {
                output.push(down_left);
            } else {
                output.push(down_left);
                break;
            }
        }
        for i in 1..(8 - position.0 ) {
            let right = self.relative_position(_position, 0, i as i8).unwrap();
            if self.piece_position(&right) == None {
                output.push(right);
            } else {
                output.push(right);
                break;
            }
        }
        for i in 1..position.1 {
            let left = self.relative_position(_position, 0, -(i as i8)).unwrap();
            if self.piece_position(&left) == None {
                output.push(left);
            } else {
                output.push(left);
                break;
            }
        }

        for i in 1..(8 - position.1) {
            let down = self.relative_position(_position, i as i8, 0).unwrap();
            if self.piece_position(&down) == None {
                output.push(down);
            } else {
                output.push(down);
                break;
            }
        }

        for i in 1..position.1 {
            let up = self.relative_position(_position, -(i as i8), 0).unwrap();
            if self.piece_position(&up) == None {
                output.push(up);
            } else {
                output.push(up);
                break;
            }
        }
        Some(output)
    }


    ///King 
    /// GameState and check should be taken into consideration
    /// surrounding positions of the king are checked with the 
    /// implementation of check checker

    fn king_moves(&self, _position: &String) -> Option<Vec<String>> {

        let mut output: Vec<String> = Vec::with_capacity(8);
        if self.piece_position(&_position) != None {
            let color: Color = self.piece_position(&_position).unwrap().1;
            for d in -1..=1 {
                for h in -1..=1 {
                    let possible_position = self.relative_position(_position, d, h).unwrap();
                    if _position != &possible_position
                        && self.piece_position(&possible_position) == None
                        && self.check_checker(&possible_position, color) == GameState::InProgress
                    {
                        output.push(possible_position);
                    }
                }
            }
        }
        Some(output)
    }





/// Implement print routine for Game.
///
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|

    pub fn print(&self) {
        let mut output: String = String::new();
        let board = self.board;
        output.push_str("|:----------------------:|");
        output.push_str(" A B C D E F G H \n");
        for rank in 0..7 {
            for file in 0..7 {
                output.push_str("|");
                if file == 7 {
                    output.push_str(&rank.to_string());
                    output.push_str("\n");
                }
                output += &self.symbol(board[2][2]);
                if rank == 7 {
                    output.push_str("")
                }
            }
            output.push_str("|:----------------------:|");
        }
    }

    fn symbol(&self, input: Option<(Piece, Color)>) -> String {
        use Color::*;
        use Piece::*;
        match input {
            Some((Pawn, White)) => format!("{}", "WP"),
            Some((Rook, White)) => format!("{}", "WR"),
            Some((Knight, White)) => format!("{}", "WKn"),
            Some((Bishop, White)) => format!("{}", "WB"),
            Some((Queen, White)) => format!("{}", "WQ"),
            Some((King, White)) => format!("{}", "WK"),

            Some((Pawn, Black)) => format!("{}", "BP"),
            Some((Rook, Black)) => format!("{}", "BR"),
            Some((Knight, Black)) => format!("{}", "BKn"),
            Some((Bishop, Black)) => format!("{}", "BB"),
            Some((Queen, Black)) => format!("{}", "BQ"),
            Some((King, Black)) => format!("{}", "BK"),
            None => format!("{}", "*"),
        }
    }

}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")}
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }

    