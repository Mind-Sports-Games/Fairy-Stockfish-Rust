
#[cxx::bridge(namespace = "fairystockfish::rustffi")]
pub mod ffi {

    // TODO: Not sure if this is the right representation in Rust.
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
        include!("fairystockfish/src/fairystockfishrs.h");

        type Notation;

        fn init();

        // TODO: switch to references for all string  parameters, probably.

        /// # Examples
        /// ```
        /// assert_eq!("v0.0.5", fairystockfish::version());
        /// ```
        fn version() -> String;
        fn info();
        fn setUCIOption(name: String, value: String);
        fn loadVariantConfig(config: String);

        /// # Examples
        /// ```
        /// fairystockfish::init();
        /// assert_eq!(
        ///     Some(&String::from("shogi")),
        ///     fairystockfish::availableVariants()
        ///         .iter()
        ///         .find(|&n| n.eq("shogi"))
        ///     );
        /// ```
        fn availableVariants() -> Vec<String>;

        /// # Examples
        /// ```
        /// fairystockfish::init();
        /// assert_eq!(
        ///     String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
        ///     fairystockfish::initialFen("chess".to_string())
        /// );
        /// assert_eq!(
        ///     String::from("rnbqkbnr/pppppppp/8/1PP2PP1/PPPPPPPP/PPPPPPPP/PPPPPPPP/PPPPPPPP w kq - 0 1"),
        ///     fairystockfish::initialFen("horde".to_string())
        /// );
        /// assert_eq!(
        ///     String::from("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL[-] w 0 1"),
        ///     fairystockfish::initialFen("shogi".to_string())
        /// );
        /// ```
        fn initialFen(variantName: String) -> String;

        /// # Examples
        /// ```
        /// fairystockfish::init();
        /// assert_eq!(
        ///     Some(String::from("aiwok")),
        ///     fairystockfish::availablePieces()
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
        /// fairystockfish::init();
        /// assert!(
        ///     fairystockfish::validateFEN(
        ///         "shogi".to_string(),
        ///         String::from("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL[-] w 0 1"),
        ///         false
        ///     )
        /// );
        /// ```
        fn validateFEN(variantName: String, fen: String, isChess960: bool) -> bool;

        type Position;

        fn startingPosition(variantName: String, isChess960: bool) -> UniquePtr<Position>;
        fn positionFromFen(variantName: String, fen: String, isChess960: bool) -> UniquePtr<Position>;
        fn makeMoves(self: &Position, moves: &Vec<String>) -> UniquePtr<Position>;

        /// # Examples
        /// ```
        /// fairystockfish::init();
        /// let p = fairystockfish::startingPosition(String::from("chess"), false);
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
        /// fairystockfish::init();
        /// let p = fairystockfish::startingPosition(String::from("chess"), false);
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

        fn getSAN(self: &Position, uci: String) -> String;
        fn getSANWithNotation(self: &Position, uci: String, notation: Notation) -> String;
        fn getSANMoves(self: &Position, uci: &Vec<String>) -> Vec<String>;
        fn getSANMovesWithNotation(self: &Position, uci: &Vec<String>, notation: Notation) -> Vec<String>;

        /// # Examples
        /// ```
        /// fairystockfish::init();
        /// let p = fairystockfish::positionFromFen(
        ///     String::from("chess"),
        ///     String::from("rnb1kbnr/ppp1pppp/8/8/8/2N5/PPPB1PPP/R2QKBNR/QPpp b KQkq - 0 4"),
        ///     false
        /// );
        /// assert_eq!(
        ///     14,
        ///     p.piecesOnBoard()
        ///         .iter()
        ///         .filter(|&p| p.piece.color == fairystockfish::Color::White)
        ///         .count()
        /// );
        /// assert_eq!(
        ///     14,
        ///     p.piecesOnBoard()
        ///         .iter()
        ///         .filter(|&p| p.piece.color == fairystockfish::Color::Black)
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
        /// fairystockfish::init();
        /// let p = fairystockfish::positionFromFen(
        ///     String::from("chess"),
        ///     String::from("rnb1kbnr/ppp1pppp/8/8/8/2N5/PPPB1PPP/R2QKBNR/QPpp b KQkq - 0 4"),
        ///     false
        /// );
        /// assert_eq!(
        ///     2,
        ///     p.piecesInHand()
        ///         .iter()
        ///         .filter(|&p| p.color == fairystockfish::Color::White)
        ///         .count()
        /// );
        /// assert_eq!(
        ///     2,
        ///     p.piecesInHand()
        ///         .iter()
        ///         .filter(|&p| p.color == fairystockfish::Color::Black)
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
        fn gameResult(self: &Position) -> u32;
        fn isImmediateGameEnd(self: &Position) -> TestWithGameResult;
        fn isOptionalGameEnd(self: &Position) -> TestWithGameResult;
        fn hasInsufficientMaterial(self: &Position) -> TestByPlayers;
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
    validateFEN,
    startingPosition,
    positionFromFen,
    Color,
    Piece,
};
