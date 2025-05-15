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
            self.squares[0][1] = Some(Piece::Rook(Color::White));
            self.squares[0][2] = Some(Piece::Rook(Color::White));
            self.squares[0][3] = Some(Piece::Rook(Color::White));
            self.squares[0][4] = Some(Piece::Rook(Color::White));
            self.squares[0][5] = Some(Piece::Rook(Color::White));
            self.squares[0][6] = Some(Piece::Rook(Color::White));
            self.squares[0][7] = Some(Piece::Rook(Color::White));
            //Pawn
            let i: usize;
            for i in 0..8 {
                self.squares[1][i] = Some(Piece::Pawn(Color::White));
            }
        }
        // Black pieces
        {
            self.squares[7][0] = Some(Piece::Rook(Color::Black));
            self.squares[7][1] = Some(Piece::Rook(Color::Black));
            self.squares[7][2] = Some(Piece::Rook(Color::Black));
            self.squares[7][3] = Some(Piece::Rook(Color::Black));
            self.squares[7][4] = Some(Piece::Rook(Color::Black));
            self.squares[7][5] = Some(Piece::Rook(Color::Black));
            self.squares[7][6] = Some(Piece::Rook(Color::Black));
            self.squares[7][7] = Some(Piece::Rook(Color::Black));
            let i: usize;
            for i in 0..8 {
                self.squares[6][i] = Some(Piece::Pawn(Color::Black));
            }
        }
    }
    pub fn print_board(&self) {
        for row in self.squares.iter().rev() {
            println!();
            println!();
            println!();

            for cell in row {
                print!("{:?}\t", cell);
            }
        }
        println!()
    }
}
