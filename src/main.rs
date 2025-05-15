use colored::{ColoredString, Colorize};
use std::vec;

#[derive(Debug)]
struct Board {
    squares: Vec<Vec<Option<Piece>>>,
    side_to_move: Color,
}
#[derive(Clone, Debug)]

enum Piece {
    Pawn(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Queen(Color),
    King(Color),
}
#[derive(Clone, Debug)]
enum Color {
    White,
    Black,
}

fn main() {
    println!("Hello, world!");
    let mut chess_board = Board::new();
    chess_board.fill();
    chess_board.print_board();
}
impl Piece {
    fn to_char(&self) -> ColoredString {
        match *self {
            Piece::Pawn(Color::White) => '♙'.to_string().white().bold().on_black(),
            Piece::Pawn(Color::Black) => '♙'.to_string().bright_black().bold().on_black(),
            Piece::Knight(Color::White) => '♘'.to_string().white().bold().on_black(),
            Piece::Knight(Color::Black) => '♘'.to_string().bright_black().bold().on_black(),
            Piece::Bishop(Color::White) => '♗'.to_string().white().bold().on_black(),
            Piece::Bishop(Color::Black) => '♗'.to_string().bright_black().bold().on_black(),
            Piece::Rook(Color::White) => '♖'.to_string().white().bold().on_black(),
            Piece::Rook(Color::Black) => '♖'.to_string().bright_black().bold().on_black(),
            Piece::Queen(Color::White) => '♕'.to_string().white().bold().on_black(),
            Piece::Queen(Color::Black) => '♕'.to_string().bright_black().bold().on_black(),
            Piece::King(Color::White) => '♔'.to_string().white().bold().on_black(),
            Piece::King(Color::Black) => '♔'.to_string().bright_black().bold().on_black(),
        }
    }
}
impl Board {
    fn new() -> Board {
        Board {
            squares: vec![vec![None; 8]; 8],
            side_to_move: (Color::White),
        }
    }
    fn fill(&mut self) {
        // White pieces
        {
            self.squares[0][0] = Some(Piece::Rook(Color::White));
            self.squares[0][1] = Some(Piece::Knight(Color::White));
            self.squares[0][2] = Some(Piece::Bishop(Color::White));
            self.squares[0][3] = Some(Piece::Queen(Color::White));
            self.squares[0][4] = Some(Piece::King(Color::White));
            self.squares[0][5] = Some(Piece::Bishop(Color::White));
            self.squares[0][6] = Some(Piece::Knight(Color::White));
            self.squares[0][7] = Some(Piece::Rook(Color::White));
            //Pawn
            for i in 0..8 {
                self.squares[1][i] = Some(Piece::Pawn(Color::White));
            }
        }
        // Black pieces
        {
            self.squares[7][0] = Some(Piece::Rook(Color::Black));
            self.squares[7][1] = Some(Piece::Knight(Color::Black));
            self.squares[7][2] = Some(Piece::Bishop(Color::Black));
            self.squares[7][3] = Some(Piece::Queen(Color::Black));
            self.squares[7][4] = Some(Piece::King(Color::Black));
            self.squares[7][5] = Some(Piece::Bishop(Color::Black));
            self.squares[7][6] = Some(Piece::Knight(Color::Black));
            self.squares[7][7] = Some(Piece::Rook(Color::Black));
            for i in 0..8 {
                self.squares[6][i] = Some(Piece::Pawn(Color::Black));
            }
        }
    }

    fn print_board(&self) {
        for row in self.squares.iter().rev() {
            for cell in row {
                let symbol = match cell {
                    Some(piece) => piece.to_char(),
                    None => '🟥'.to_string().red(),
                };
                print!("{}\t", symbol);
            }
            println!();
        }
    }
    fn get_fcolors(&self, color: Color) -> Vec<Vec<u8>> {
        let pieces_d: Vec<Vec<u8>>;
        let it: u8 = 0;
        let yt: u8 = 0;
        for i in self.squares {
            it += 1;
            for j in i {
                yt += 1;
                if j(c) == color {
                    pieces_d.push([it, yt].to_vec());
                };
            }
        }
        pieces_d
    }
    fn get_fpiece(piece: Piece) -> Vec<Option<Vec<u8>>> {
        let coo: Vec<Option<Vec<u8>>>;
        let it: u8 = 0;
        let yt: u8 = 0;
        for i in self.squares {
            it += 1;
            for j in i {
                yt += 1;
                if j == piece {
                    coo.push(Option<[it, yt].to_vec()>);
                };
            }
        }
        pieces_d
    }
}
