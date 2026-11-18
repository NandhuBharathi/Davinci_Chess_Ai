
use davinci_chess::ChessBoard;

fn main() {
    let board = ChessBoard::initial_position();

    println!("{}", board);
    println!("Total Pieces: {}", board.total_pieces());
    println!("White Material: {}", board.material_points(
        davinci_chess::Color::White
    ));
    println!("Black Material: {}", board.material_points(
        davinci_chess::Color::Black
    ));
    println!("Board Valid: {}", board.validate());
}
