
#[cxx::bridge(namespace = "fairystockfish::rustffi")]
pub mod ffi {

    // TODO: Not sure if this is the right representation in Rust.
    struct PieceInfo {
        id: u32,
        name: String,
        betza: String,
    }

    unsafe extern "C++" {
        include!("fairystockfish/src/fairystockfishrs.h");

        fn init();

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
    validateFEN
};
