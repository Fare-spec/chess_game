use colored::{ColoredString, Colorize};
use std::vec;
mod tests;
#[derive(Clone, Debug, PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize,
}
#[derive(Debug)]
struct Board {
    squares: Vec<Vec<Option<Piece>>>,
    side_to_move: Color,
}
#[derive(Clone, Debug, PartialEq, Eq)]

enum Piece {
    Pawn(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Queen(Color),
    King(Color),
}
#[derive(Clone, Debug, PartialEq, Eq)]
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
impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    fn is_valid(&self) -> bool {
        return self.x < 8 && self.y < 8;
    }
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

    fn get_color(&self) -> &Color {
        match self {
            Piece::Pawn(color) => color,
            Piece::Rook(color) => color,
            Piece::Knight(color) => color,
            Piece::Bishop(color) => color,
            Piece::Queen(color) => color,
            Piece::King(color) => color,
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
    fn get_fcolors(&self, color: Color) -> Vec<Coordinate> {
        let mut result = Vec::new();
        for (y, row) in self.squares.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Some(piece) = cell {
                    if piece.get_color() == &color {
                        result.push(Coordinate { x, y });
                    }
                }
            }
        }
        result
    }
    fn get_fpiece(&self, piece: Piece) -> Vec<Coordinate> {
        let mut positions = Vec::new();
        for (y, row) in self.squares.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Some(p) = cell {
                    if *p == piece {
                        positions.push(Coordinate { x, y });
                    }
                }
            }
        }
        positions
    }
    fn get_fcoordinates(&self, coo: &Coordinate) -> &Option<Piece> {
        &self.squares[coo.y][coo.x]
    }
    fn verify_pawn(&self, departure: Coordinate, destination: Coordinate, color: Color) -> bool {
        // out of the board
        if !destination.is_valid() || !departure.is_valid() {
            return false;
        }

        let dx = destination.x as isize - departure.x as isize;
        let dy = destination.y as isize - departure.y as isize;

        match color {
            Color::White => {
                // simple forward
                if dx == 0 && dy == 1 {
                    return self.squares[destination.y][destination.x].is_none();
                }

                // first move  (2 steps forward max)
                if dx == 0 && dy == 2 && departure.y == 1 {
                    return self.squares[departure.y + 1][departure.x].is_none()
                        && self.squares[destination.y][destination.x].is_none();
                }

                // Diagonal capture
                if dy == 1 && dx.abs() == 1 {
                    if let Some(p) = &self.squares[destination.y][destination.x] {
                        return p.get_color() != &color;
                    }
                }

                false
            }

            Color::Black => {
                if dx == 0 && dy == -1 {
                    return self.squares[destination.y][destination.x].is_none();
                }

                if dx == 0 && dy == -2 && departure.y == 6 {
                    return self.squares[departure.y - 1][departure.x].is_none()
                        && self.squares[destination.y][destination.x].is_none();
                }

                if dy == -1 && dx.abs() == 1 {
                    if let Some(p) = &self.squares[destination.y][destination.x] {
                        return p.get_color() != &color;
                    }
                }

                false
            }
        }
    }

    fn verify_move(&self, departure: Coordinate, destination: Coordinate) -> bool {
        let piece_opt = self.get_fcoordinates(&departure);
        if let Some(piece) = piece_opt {
            match piece {
                Piece::Pawn(color) => self.verify_pawn(departure, destination, color.clone()),
                Piece::Rook(_) => todo!(),
                Piece::Knight(_) => todo!(),
                Piece::Bishop(_) => todo!(),
                Piece::King(_) => todo!(),
                Piece::Queen(_) => todo!(),
            }
        } else {
            false
        }
    }

    fn move_piece(&mut self, departure: Coordinate, destination: Coordinate) {
        todo!()
    }
}
