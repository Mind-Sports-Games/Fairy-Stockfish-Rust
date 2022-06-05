
#[cxx::bridge(namespace = "rsffish")]
pub mod ffi {

    struct PieceInfo {
        id: u32,
        name: String,
        betza: String,
    }

    struct TestWithGameResult {
        test: bool,
        gameResult: u32,
    }

    struct TestByPlayers {
        player1: bool,
        player2: bool,
    }

    enum Color {
        White,
        Black
    }

    struct Piece {
        color: Color,
        promoted: bool,
        pieceInfo: PieceInfo,
    }

    struct PieceOnBoard {
        piece: Piece,
        square: String
    }

    enum Notation {
        NOTATION_DEFAULT,
        NOTATION_SAN,
        NOTATION_LAN,
        NOTATION_SHOGI_HOSKING, // Examples: P76, S’34
        NOTATION_SHOGI_HODGES, // Examples: P-7f, S*3d
        NOTATION_SHOGI_HODGES_NUMBER, // Examples: P-76, S*34
        NOTATION_JANGGI,
        NOTATION_XIANGQI_WXF,
    }

    unsafe extern "C++" {
        include!("rsffish/src/fairystockfishrs.h");

        type Notation;

        fn init();

        /// # Examples
        /// ```
        /// assert_eq!("v0.0.11", rsffish::version());
        /// ```
        fn version() -> String;
        fn info();
        fn setUCIOption(name: &String, value: &String);
        fn loadVariantConfig(config: &String);

        /// # Examples
        /// ```
        /// rsffish::init();
        /// assert_eq!(
        ///     Some(&String::from("shogi")),
        ///     rsffish::availableVariants()
        ///         .iter()
        ///         .find(|&n| n.eq("shogi"))
        ///     );
        /// ```
        fn availableVariants() -> Vec<String>;

        /// # Examples
        /// ```
        /// rsffish::init();
        /// assert_eq!(
        ///     String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
        ///     rsffish::initialFen(&"chess".to_string())
        /// );
        /// assert_eq!(
        ///     String::from("rnbqkbnr/pppppppp/8/1PP2PP1/PPPPPPPP/PPPPPPPP/PPPPPPPP/PPPPPPPP w kq - 0 1"),
        ///     rsffish::initialFen(&"horde".to_string())
        /// );
        /// assert_eq!(
        ///     String::from("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL[-] w 0 1"),
        ///     rsffish::initialFen(&"shogi".to_string())
        /// );
        /// ```
        fn initialFen(variantName: &String) -> String;

        /// # Examples
        /// ```
        /// rsffish::init();
        /// assert_eq!(
        ///     Some(String::from("aiwok")),
        ///     rsffish::availablePieces()
        ///         .iter()
        ///         .skip(1)
        ///         .take(1)
        ///         .map(|pi| pi.name.clone())
        ///         .next()
        ///     );
        /// ```
        fn availablePieces() -> Vec<PieceInfo>;

        /// # Examples
        /// ```
        /// rsffish::init();
        /// assert_eq!(
        ///     String::from("ABCDEFGHJKLMNPQRSWabcdefghjklmnpqrsw"),
        ///     rsffish::availablePieceChars()
        ///     );
        /// ```
        fn availablePieceChars() -> String;

        /// # Examples
        /// ```
        /// rsffish::init();
        /// assert_eq!(
        ///     String::from("ABCEFGHJKLMNQRWabcefghjklmnqrw"),
        ///     rsffish::availablePromotablePieceChars()
        ///     );
        /// ```
        fn availablePromotablePieceChars() -> String;


        /// # Examples
        /// ```
        /// rsffish::init();
        /// assert!(
        ///     rsffish::validateFEN(
        ///         &"shogi".to_string(),
        ///         &String::from("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL[-] w 0 1"),
        ///         false
        ///     )
        /// );
        /// ```
        fn validateFEN(variantName: &String, fen: &String, isChess960: bool) -> bool;

        /// # Examples
        /// ```
        /// rsffish::init();
        /// let moves = vec![
        ///     "e2e4".to_string(),
        ///     "c7c5".to_string(),
        ///     "g1f3".to_string(),
        ///     "d7d6".to_string(),
        ///     "d2d4".to_string(),
        ///     "c5d4".to_string(),
        ///     "f3d4".to_string(),
        ///     "g8f6".to_string(),
        ///     "b1c3".to_string(),
        ///     "g7g6".to_string(),
        ///     "c1g5".to_string(),
        ///     "f8g7".to_string(),
        ///     "f2f4".to_string(),
        ///     "b8c6".to_string(),
        ///     "f1b5".to_string(),
        ///     "c8d7".to_string(),
        ///     "d4c6".to_string(),
        ///     "d7c6".to_string(),
        ///     "b5c6".to_string(),
        ///     "b7c6".to_string(),
        ///     "e1g1".to_string()
        /// ];
        /// let _960Moves = rsffish::to960Uci(
        ///         &"chess".to_string(),
        ///         &moves
        ///     );
        /// assert_eq!(moves.len(), _960Moves.len());
        /// assert_eq!(_960Moves[0], "e2e4".to_string());
        /// assert_eq!(_960Moves[1], "c7c5".to_string());
        /// assert_eq!(_960Moves[2], "g1f3".to_string());
        /// assert_eq!(_960Moves[3], "d7d6".to_string());
        /// assert_eq!(_960Moves[4], "d2d4".to_string());
        /// assert_eq!(_960Moves[5], "c5d4".to_string());
        /// assert_eq!(_960Moves[6], "f3d4".to_string());
        /// assert_eq!(_960Moves[7], "g8f6".to_string());
        /// assert_eq!(_960Moves[8], "b1c3".to_string());
        /// assert_eq!(_960Moves[9], "g7g6".to_string());
        /// assert_eq!(_960Moves[10], "c1g5".to_string());
        /// assert_eq!(_960Moves[11], "f8g7".to_string());
        /// assert_eq!(_960Moves[12], "f2f4".to_string());
        /// assert_eq!(_960Moves[13], "b8c6".to_string());
        /// assert_eq!(_960Moves[14], "f1b5".to_string());
        /// assert_eq!(_960Moves[15], "c8d7".to_string());
        /// assert_eq!(_960Moves[16], "d4c6".to_string());
        /// assert_eq!(_960Moves[17], "d7c6".to_string());
        /// assert_eq!(_960Moves[18], "b5c6".to_string());
        /// assert_eq!(_960Moves[19], "b7c6".to_string());
        /// assert_eq!(_960Moves[20], "e1h1".to_string());
        /// ```
        fn to960Uci(variantName: &String, moves: &Vec<String>) -> Vec<String>;

        type Position;

        fn startingPosition(variantName: &String, isChess960: bool) -> UniquePtr<Position>;
        fn positionFromFen(variantName: &String, fen: &String, isChess960: bool) -> UniquePtr<Position>;
        fn makeMoves(self: &Position, moves: &Vec<String>) -> UniquePtr<Position>;

        /// # Examples
        /// ```
        /// rsffish::init();
        /// let p = rsffish::startingPosition(&String::from("chess"), false);
        /// let p = p.makeMoves(&vec![String::from("e2e4")]);
        /// assert_eq!(
        ///     p.getFEN(),
        ///     String::from("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1")
        /// );
        /// ```
        fn getFEN(self: &Position) -> String;
        fn getFENWithArgs(self: &Position, sFen: bool, showPromoted: bool, countStarted: u32) -> String;

        /// # Examples
        /// ```
        /// rsffish::init();
        /// let p = rsffish::startingPosition(&String::from("chess"), false);
        /// assert_eq!(
        ///     Some(&String::from("e2e4")),
        ///     p.getLegalMoves()
        ///         .iter()
        ///         .find(|&n| n.eq("e2e4"))
        /// );
        /// let p = p.makeMoves(&vec![String::from("e2e4")]);
        /// assert_eq!(
        ///     Some(&String::from("e7e5")),
        ///     p.getLegalMoves()
        ///         .iter()
        ///         .find(|&n| n.eq("e7e5"))
        /// );
        /// ```
        fn getLegalMoves(self: &Position) -> Vec<String>;

        fn getSAN(self: &Position, uci: &String) -> String;
        fn getSANWithNotation(self: &Position, uci: &String, notation: Notation) -> String;
        fn getSANMoves(self: &Position, uci: &Vec<String>) -> Vec<String>;
        fn getSANMovesWithNotation(self: &Position, uci: &Vec<String>, notation: Notation) -> Vec<String>;

        /// # Examples
        /// ```
        /// rsffish::init();
        /// let p = rsffish::positionFromFen(
        ///     &String::from("chess"),
        ///     &String::from("rnb1kbnr/ppp1pppp/8/8/8/2N5/PPPB1PPP/R2QKBNR/QPpp b KQkq - 0 4"),
        ///     false
        /// );
        /// assert_eq!(
        ///     14,
        ///     p.piecesOnBoard()
        ///         .iter()
        ///         .filter(|&p| p.piece.color == rsffish::Color::White)
        ///         .count()
        /// );
        /// assert_eq!(
        ///     14,
        ///     p.piecesOnBoard()
        ///         .iter()
        ///         .filter(|&p| p.piece.color == rsffish::Color::Black)
        ///         .count()
        /// );
        /// assert_eq!(
        ///     1,
        ///     p.piecesOnBoard()
        ///         .iter()
        ///         .filter(|&p| p.piece.pieceInfo.name == String::from("queen"))
        ///         .count()
        /// );
        /// ```
        fn piecesOnBoard(self: &Position) -> Vec<PieceOnBoard>;

        /// # Examples
        /// ```
        /// rsffish::init();
        /// let p = rsffish::positionFromFen(
        ///     &String::from("chess"),
        ///     &String::from("rnb1kbnr/ppp1pppp/8/8/8/2N5/PPPB1PPP/R2QKBNR/QPpp b KQkq - 0 4"),
        ///     false
        /// );
        /// assert_eq!(
        ///     2,
        ///     p.piecesInHand()
        ///         .iter()
        ///         .filter(|&p| p.color == rsffish::Color::White)
        ///         .count()
        /// );
        /// assert_eq!(
        ///     2,
        ///     p.piecesInHand()
        ///         .iter()
        ///         .filter(|&p| p.color == rsffish::Color::Black)
        ///         .count()
        /// );
        /// assert_eq!(
        ///     1,
        ///     p.piecesInHand()
        ///         .iter()
        ///         .filter(|&p| p.pieceInfo.name == String::from("queen"))
        ///         .count()
        /// );
        /// ```
        fn piecesInHand(self: &Position) -> Vec<Piece>;

        fn givesCheck(self: &Position) -> bool;
        fn hasRepeated(self: &Position) -> bool;
        fn isDraw(self: &Position, ply: u32) -> bool;
        fn hasGameCycle(self: &Position, ply: u32) -> bool;
        /// # Examples
        /// ```
        /// rsffish::init();
        /// let p = rsffish::positionFromFen(
        ///     &String::from("chess"),
        ///     &String::from("8/1Q2b1k1/2p3p1/p2p2P1/8/5PB1/PP3RK1/3r3q w - - 2 37"),
        ///     false
        /// );
        /// assert_eq!(
        ///     -32000,
        ///     p.gameResult()
        /// );
        /// ```
        fn gameResult(self: &Position) -> i32;
        fn isImmediateGameEnd(self: &Position) -> TestWithGameResult;
        fn isOptionalGameEnd(self: &Position) -> TestWithGameResult;
        fn hasInsufficientMaterial(self: &Position) -> TestByPlayers;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_5check_game_continues_after_3_checks() {
        init();
        let moves: Vec<String> = vec![
            "e2e4", "c7c6", "d2d4", "d7d5",
            "e4d5", "c6c5", "d4c5", "c8g4",
            "f1b5", "g4d7", "b5d7", "d8d7",
            "c5c6", "b7c6", "d5c6", "d7d1",
            "e1d1", "b8c6", "b2b3", "e8c8",
            "c1d2", "d8d2"
        ].iter().map(|s| s.to_string()).collect();
        let mut pos = startingPosition(&"5check".to_string(), false);
        pos = pos.makeMoves(&moves);
        assert!(pos.getLegalMoves().len() > 0);
    }
}

// Re-export at the top level.
pub use ffi::{
    init,
    version,
    info,
    setUCIOption,
    loadVariantConfig,
    availableVariants,
    initialFen,
    availablePieces,
    availablePieceChars,
    availablePromotablePieceChars,
    validateFEN,
    to960Uci,
    startingPosition,
    positionFromFen,
    Color,
    Piece,
};

