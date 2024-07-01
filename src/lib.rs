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
        Black,
    }

    struct Piece {
        color: Color,
        promoted: bool,
        pieceInfo: PieceInfo,
    }

    struct PieceOnBoard {
        piece: Piece,
        square: Square,
    }

    enum Notation {
        NOTATION_DEFAULT,
        NOTATION_SAN,
        NOTATION_LAN,
        NOTATION_SHOGI_HOSKING,       // Examples: P76, S’34
        NOTATION_SHOGI_HODGES,        // Examples: P-7f, S*3d
        NOTATION_SHOGI_HODGES_NUMBER, // Examples: P-76, S*34
        NOTATION_JANGGI,
        NOTATION_XIANGQI_WXF,
    }

    enum Square {
        SQ_A1,
        SQ_B1,
        SQ_C1,
        SQ_D1,
        SQ_E1,
        SQ_F1,
        SQ_G1,
        SQ_H1,
        SQ_I1,
        SQ_J1,
        SQ_K1,
        SQ_L1,
        SQ_A2,
        SQ_B2,
        SQ_C2,
        SQ_D2,
        SQ_E2,
        SQ_F2,
        SQ_G2,
        SQ_H2,
        SQ_I2,
        SQ_J2,
        SQ_K2,
        SQ_L2,
        SQ_A3,
        SQ_B3,
        SQ_C3,
        SQ_D3,
        SQ_E3,
        SQ_F3,
        SQ_G3,
        SQ_H3,
        SQ_I3,
        SQ_J3,
        SQ_K3,
        SQ_L3,
        SQ_A4,
        SQ_B4,
        SQ_C4,
        SQ_D4,
        SQ_E4,
        SQ_F4,
        SQ_G4,
        SQ_H4,
        SQ_I4,
        SQ_J4,
        SQ_K4,
        SQ_L4,
        SQ_A5,
        SQ_B5,
        SQ_C5,
        SQ_D5,
        SQ_E5,
        SQ_F5,
        SQ_G5,
        SQ_H5,
        SQ_I5,
        SQ_J5,
        SQ_K5,
        SQ_L5,
        SQ_A6,
        SQ_B6,
        SQ_C6,
        SQ_D6,
        SQ_E6,
        SQ_F6,
        SQ_G6,
        SQ_H6,
        SQ_I6,
        SQ_J6,
        SQ_K6,
        SQ_L6,
        SQ_A7,
        SQ_B7,
        SQ_C7,
        SQ_D7,
        SQ_E7,
        SQ_F7,
        SQ_G7,
        SQ_H7,
        SQ_I7,
        SQ_J7,
        SQ_K7,
        SQ_L7,
        SQ_A8,
        SQ_B8,
        SQ_C8,
        SQ_D8,
        SQ_E8,
        SQ_F8,
        SQ_G8,
        SQ_H8,
        SQ_I8,
        SQ_J8,
        SQ_K8,
        SQ_L8,
        SQ_A9,
        SQ_B9,
        SQ_C9,
        SQ_D9,
        SQ_E9,
        SQ_F9,
        SQ_G9,
        SQ_H9,
        SQ_I9,
        SQ_J9,
        SQ_K9,
        SQ_L9,
        SQ_A10,
        SQ_B10,
        SQ_C10,
        SQ_D10,
        SQ_E10,
        SQ_F10,
        SQ_G10,
        SQ_H10,
        SQ_I10,
        SQ_J10,
        SQ_K10,
        SQ_L10,
    }

    unsafe extern "C++" {
        include!("rsffish/src/fairystockfishrs.h");

        type Notation;
        type Square;

        fn init();

        /// # Examples
        /// ```
        /// assert_eq!("v0.0.20", rsffish::version());
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
        ///     String::from("ABCDEFGHJKLMNQRWabcdefghjklmnqrw"),
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
        fn positionFromFen(
            variantName: &String,
            fen: &String,
            isChess960: bool,
        ) -> UniquePtr<Position>;
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
        fn getFENWithArgs(
            self: &Position,
            sFen: bool,
            showPromoted: bool,
            countStarted: u32,
        ) -> String;

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
        fn getSANMovesWithNotation(
            self: &Position,
            uci: &Vec<String>,
            notation: Notation,
        ) -> Vec<String>;

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
            "e2e4", "c7c6", "d2d4", "d7d5", "e4d5", "c6c5", "d4c5", "c8g4", "f1b5", "g4d7", "b5d7",
            "d8d7", "c5c6", "b7c6", "d5c6", "d7d1", "e1d1", "b8c6", "b2b3", "e8c8", "c1d2", "d8d2",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let mut pos = startingPosition(&"5check".to_string(), false);
        pos = pos.makeMoves(&moves);
        assert!(pos.getLegalMoves().len() > 0);
    }

    #[test]
    fn test_5check_castling_white_queenside_notation() {
        init();
        let moves: Vec<String> = vec![
            "e2e4", "b8c6", "b2b3", "e7e6", "c1b2", "d8h4", "b1c3", "h4e7", "d1f3", "c6d4", "f1b5",
            "d4f3", "g1f3", "a7a6", "e1g1",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let nine_sixty_moves = to960Uci(&"5check".to_string(), &moves);
        assert_eq!(moves.len(), nine_sixty_moves.len());
        assert_eq!(nine_sixty_moves[0], "e2e4".to_string());
        assert_eq!(nine_sixty_moves[1], "b8c6".to_string());
        assert_eq!(nine_sixty_moves[2], "b2b3".to_string());
        assert_eq!(nine_sixty_moves[3], "e7e6".to_string());
        assert_eq!(nine_sixty_moves[4], "c1b2".to_string());
        assert_eq!(nine_sixty_moves[5], "d8h4".to_string());
        assert_eq!(nine_sixty_moves[6], "b1c3".to_string());
        assert_eq!(nine_sixty_moves[7], "h4e7".to_string());
        assert_eq!(nine_sixty_moves[8], "d1f3".to_string());
        assert_eq!(nine_sixty_moves[9], "c6d4".to_string());
        assert_eq!(nine_sixty_moves[10], "f1b5".to_string());
        assert_eq!(nine_sixty_moves[11], "d4f3".to_string());
        assert_eq!(nine_sixty_moves[12], "g1f3".to_string());
        assert_eq!(nine_sixty_moves[13], "a7a6".to_string());
        assert_eq!(nine_sixty_moves[14], "e1h1".to_string());
    }

    #[test]
    fn test_5check_castling_white_kingside_notation() {
        init();
        let moves: Vec<String> = vec![
            "e2e4", "b8c6", "b2b3", "e7e6", "c1b2", "d8h4", "b1c3", "h4e7", "d1f3", "c6d4", "f1b5",
            "d4f3", "g1f3", "a7a6", "e1c1",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let nine_sixty_moves = to960Uci(&"5check".to_string(), &moves);
        assert_eq!(moves.len(), nine_sixty_moves.len());
        assert_eq!(nine_sixty_moves[0], "e2e4".to_string());
        assert_eq!(nine_sixty_moves[1], "b8c6".to_string());
        assert_eq!(nine_sixty_moves[2], "b2b3".to_string());
        assert_eq!(nine_sixty_moves[3], "e7e6".to_string());
        assert_eq!(nine_sixty_moves[4], "c1b2".to_string());
        assert_eq!(nine_sixty_moves[5], "d8h4".to_string());
        assert_eq!(nine_sixty_moves[6], "b1c3".to_string());
        assert_eq!(nine_sixty_moves[7], "h4e7".to_string());
        assert_eq!(nine_sixty_moves[8], "d1f3".to_string());
        assert_eq!(nine_sixty_moves[9], "c6d4".to_string());
        assert_eq!(nine_sixty_moves[10], "f1b5".to_string());
        assert_eq!(nine_sixty_moves[11], "d4f3".to_string());
        assert_eq!(nine_sixty_moves[12], "g1f3".to_string());
        assert_eq!(nine_sixty_moves[13], "a7a6".to_string());
        assert_eq!(nine_sixty_moves[14], "e1a1".to_string());
    }

    #[test]
    fn test_5check_castling_black_kingside_notation() {
        init();
        let moves: Vec<String> = vec![
            "e2e4", "b7b6", "b1c3", "c8b7", "d2d4", "g7g6", "g1f3", "f8g7", "f3g5", "g8f6", "e4e5",
            "b8c6", "b2b3", "d7d6", "e5f6", "g7f6", "f1c4", "e7e6", "c3e4", "d8e7", "e4f6", "e7f6",
            "c4b5", "e8g8",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let nine_sixty_moves = to960Uci(&"5check".to_string(), &moves);
        assert_eq!(moves.len(), nine_sixty_moves.len());
        assert_eq!(nine_sixty_moves[0], "e2e4".to_string());
        assert_eq!(nine_sixty_moves[1], "b7b6".to_string());
        assert_eq!(nine_sixty_moves[2], "b1c3".to_string());
        assert_eq!(nine_sixty_moves[3], "c8b7".to_string());
        assert_eq!(nine_sixty_moves[4], "d2d4".to_string());
        assert_eq!(nine_sixty_moves[5], "g7g6".to_string());
        assert_eq!(nine_sixty_moves[6], "g1f3".to_string());
        assert_eq!(nine_sixty_moves[7], "f8g7".to_string());
        assert_eq!(nine_sixty_moves[8], "f3g5".to_string());
        assert_eq!(nine_sixty_moves[9], "g8f6".to_string());
        assert_eq!(nine_sixty_moves[10], "e4e5".to_string());
        assert_eq!(nine_sixty_moves[11], "b8c6".to_string());
        assert_eq!(nine_sixty_moves[12], "b2b3".to_string());
        assert_eq!(nine_sixty_moves[13], "d7d6".to_string());
        assert_eq!(nine_sixty_moves[14], "e5f6".to_string());
        assert_eq!(nine_sixty_moves[15], "g7f6".to_string());
        assert_eq!(nine_sixty_moves[16], "f1c4".to_string());
        assert_eq!(nine_sixty_moves[17], "e7e6".to_string());
        assert_eq!(nine_sixty_moves[18], "c3e4".to_string());
        assert_eq!(nine_sixty_moves[19], "d8e7".to_string());
        assert_eq!(nine_sixty_moves[20], "e4f6".to_string());
        assert_eq!(nine_sixty_moves[21], "e7f6".to_string());
        assert_eq!(nine_sixty_moves[22], "c4b5".to_string());
        assert_eq!(nine_sixty_moves[23], "e8h8".to_string());
    }

    #[test]
    fn test_5check_castling_black_queenside_notation() {
        init();
        let moves: Vec<String> = vec![
            "e2e4", "b7b6", "b1c3", "c8b7", "d2d4", "g7g6", "g1f3", "f8g7", "f3g5", "g8f6", "e4e5",
            "b8c6", "b2b3", "d7d6", "e5f6", "g7f6", "f1c4", "e7e6", "c3e4", "d8e7", "e4f6", "e7f6",
            "c4b5", "e8c8",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let nine_sixty_moves = to960Uci(&"5check".to_string(), &moves);
        assert_eq!(moves.len(), nine_sixty_moves.len());
        assert_eq!(nine_sixty_moves[0], "e2e4".to_string());
        assert_eq!(nine_sixty_moves[1], "b7b6".to_string());
        assert_eq!(nine_sixty_moves[2], "b1c3".to_string());
        assert_eq!(nine_sixty_moves[3], "c8b7".to_string());
        assert_eq!(nine_sixty_moves[4], "d2d4".to_string());
        assert_eq!(nine_sixty_moves[5], "g7g6".to_string());
        assert_eq!(nine_sixty_moves[6], "g1f3".to_string());
        assert_eq!(nine_sixty_moves[7], "f8g7".to_string());
        assert_eq!(nine_sixty_moves[8], "f3g5".to_string());
        assert_eq!(nine_sixty_moves[9], "g8f6".to_string());
        assert_eq!(nine_sixty_moves[10], "e4e5".to_string());
        assert_eq!(nine_sixty_moves[11], "b8c6".to_string());
        assert_eq!(nine_sixty_moves[12], "b2b3".to_string());
        assert_eq!(nine_sixty_moves[13], "d7d6".to_string());
        assert_eq!(nine_sixty_moves[14], "e5f6".to_string());
        assert_eq!(nine_sixty_moves[15], "g7f6".to_string());
        assert_eq!(nine_sixty_moves[16], "f1c4".to_string());
        assert_eq!(nine_sixty_moves[17], "e7e6".to_string());
        assert_eq!(nine_sixty_moves[18], "c3e4".to_string());
        assert_eq!(nine_sixty_moves[19], "d8e7".to_string());
        assert_eq!(nine_sixty_moves[20], "e4f6".to_string());
        assert_eq!(nine_sixty_moves[21], "e7f6".to_string());
        assert_eq!(nine_sixty_moves[22], "c4b5".to_string());
        assert_eq!(nine_sixty_moves[23], "e8a8".to_string());
    }

    #[test]
    fn test_valid_rook_moves_not_translated() {
        init();
        let moves: Vec<String> = vec![
            "e2e4", "b8a6", "g1f3", "a6b8", "f1d3", "b8a6", "e1e2", "a6b8", "h1e1", "b8a6", "e1g1",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let nine_sixty_moves = to960Uci(&"5check".to_string(), &moves);
        assert_eq!(moves.len(), nine_sixty_moves.len());
        assert_eq!(nine_sixty_moves[0], "e2e4".to_string());
        assert_eq!(nine_sixty_moves[1], "b8a6".to_string());
        assert_eq!(nine_sixty_moves[2], "g1f3".to_string());
        assert_eq!(nine_sixty_moves[3], "a6b8".to_string());
        assert_eq!(nine_sixty_moves[4], "f1d3".to_string());
        assert_eq!(nine_sixty_moves[5], "b8a6".to_string());
        assert_eq!(nine_sixty_moves[6], "e1e2".to_string());
        assert_eq!(nine_sixty_moves[7], "a6b8".to_string());
        assert_eq!(nine_sixty_moves[8], "h1e1".to_string());
        assert_eq!(nine_sixty_moves[9], "b8a6".to_string());
        assert_eq!(nine_sixty_moves[10], "e1g1".to_string());
    }
}

// Re-export at the top level.
pub use ffi::{
    availablePieceChars, availablePieces, availablePromotablePieceChars, availableVariants, info,
    init, initialFen, loadVariantConfig, positionFromFen, setUCIOption, startingPosition, to960Uci,
    validateFEN, version, Color, Piece,
};
