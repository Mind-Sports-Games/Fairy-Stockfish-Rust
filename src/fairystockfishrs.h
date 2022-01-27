#pragma once

//----------------------------------------------------------------------------
// Unfortunately the `cxx` crate is not directly compatible with by-value
// strings. This means that we have to convert strings to/from it's wrapped
// version of a rust-compatible string. 
//
// TODO: evaluate the impact on copying data tha this has. It may not show up
//       in our use case, but it probably is impactful.
//----------------------------------------------------------------------------
#include "rust/cxx.h"

#include "fairystockfish.h"

#include <memory>

namespace fairystockfish::rustffi {
    using Notation = fairystockfish::Notation;
    struct PieceInfo;
    struct TestWithGameResult;
    struct TestByPlayers;
    struct Piece;
    struct PieceOnBoard;

    struct Position {
        // Member implementation
        std::unique_ptr<fairystockfish::Position> impl;

        // Construct from a moved in unique_ptr
        Position(std::unique_ptr<fairystockfish::Position> &&p) 
            : impl{std::move(p)} {}

        // Construct from a position by value. (Makes a copy)
        Position(fairystockfish::Position const &p)
            : impl{new fairystockfish::Position(p)} {}

        // Public API
        std::unique_ptr<Position> makeMoves(rust::Vec<rust::String> const &moves) const;
        rust::String getFEN() const;
        rust::String getFENWithArgs(bool sFen, bool showPromoted, std::uint32_t countStarted) const;
        rust::String getSAN(rust::String const &uci) const;
        rust::String getSANWithNotation(rust::String const &uci, Notation notation) const;
        rust::Vec<rust::String> getSANMoves(rust::Vec<rust::String> const &uci) const;
        rust::Vec<rust::String> getSANMovesWithNotation(rust::Vec<rust::String> const &uci, Notation notation) const;
        rust::Vec<rust::String> getLegalMoves() const;
        rust::Vec<fairystockfish::rustffi::Piece> piecesInHand() const;
        rust::Vec<fairystockfish::rustffi::PieceOnBoard> piecesOnBoard() const;
        bool givesCheck() const;
        bool hasRepeated() const;
        bool isDraw(std::uint32_t ply) const;
        bool hasGameCycle(std::uint32_t ply) const;
        std::uint32_t gameResult() const;
        fairystockfish::rustffi::TestWithGameResult isImmediateGameEnd() const;
        fairystockfish::rustffi::TestWithGameResult isOptionalGameEnd() const;
        fairystockfish::rustffi::TestByPlayers hasInsufficientMaterial() const;

    };

    std::vector<std::string> toCpp(rust::Vec<rust::String> const &vals);

    rust::Vec<rust::String> toRust(std::vector<std::string> const &vals);
    rust::Vec<Piece> toRust(std::vector<fairystockfish::Piece> const &vals);
    TestWithGameResult toRust(std::tuple<bool, int> res);
    fairystockfish::rustffi::PieceInfo toRust(fairystockfish::PieceInfo const &p);
    fairystockfish::rustffi::Piece toRust(fairystockfish::Piece const &p);
    fairystockfish::rustffi::PieceOnBoard toRust(std::string square, fairystockfish::Piece const &p);

    void init();
    rust::String version();
    void info();
    void setUCIOption(rust::String const &name, rust::String const &value);
    void loadVariantConfig(rust::String const &config);
    rust::Vec<rust::String> availableVariants();
    rust::String initialFen(rust::String const &variantName);
    rust::Vec<fairystockfish::rustffi::PieceInfo> availablePieces();
    bool validateFEN(rust::String const &variantName, rust::String const &fen, bool isChess960);
    std::unique_ptr<Position> startingPosition(rust::String const &variantName, bool isChess960);
    std::unique_ptr<Position> positionFromFen(rust::String const &variantName, rust::String const &fen, bool isChess960);
}
