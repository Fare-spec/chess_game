#[cfg(test)]
mod test {
    use crate::{Board, Color, Coordinate, Piece};

    #[test]
    fn test_coordinate_valid() {
        let c = Coordinate::new(3, 5);
        assert!(c.is_valid());
    }

    #[test]
    fn test_coordinate_invalid() {
        let c = Coordinate::new(9, 2);
        assert!(!c.is_valid());
    }

    #[test]
    fn test_board_piece_color_positions() {
        let mut board = Board::new();
        board.fill();
        board.print_board();
        assert_eq!(board.get_fcolors(Color::White).len(), 16);
        assert_eq!(board.get_fcolors(Color::Black).len(), 16);
    }

    #[test]
    fn test_board_find_piece() {
        let mut board = Board::new();
        board.fill();
        let result = board.get_fpiece(Piece::King(Color::Black));
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Coordinate { x: 4, y: 7 });
    }
    #[test]
    fn test_fill_board() {
        let mut board = Board::new();
        board.fill();

        // verify width and height
        assert_eq!(board.squares.len(), 8);
        assert!(board.squares.iter().all(|row| row.len() == 8));

        // verify white pieces
        assert_eq!(board.squares[0][0], Some(Piece::Rook(Color::White)));
        assert_eq!(board.squares[0][4], Some(Piece::King(Color::White)));
        for x in 0..8 {
            assert_eq!(board.squares[1][x], Some(Piece::Pawn(Color::White)));
        }

        // verify black pieces
        assert_eq!(board.squares[7][0], Some(Piece::Rook(Color::Black)));
        assert_eq!(board.squares[7][4], Some(Piece::King(Color::Black)));
        for x in 0..8 {
            assert_eq!(board.squares[6][x], Some(Piece::Pawn(Color::Black)));
        }

        // verify empty cases
        for y in 2..6 {
            for x in 0..8 {
                assert_eq!(board.squares[y][x], None);
            }
        }
    }
}
