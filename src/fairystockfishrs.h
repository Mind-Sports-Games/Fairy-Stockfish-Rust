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
    struct PieceInfo;
    struct TestWithGameResult;

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
        rust::Vec<rust::String> getLegalMoves() const;
        bool givesCheck() const;
        std::uint32_t gameResult() const;
        fairystockfish::rustffi::TestWithGameResult isImmediateGameEnd() const;

    };

    rust::Vec<rust::String> toRust(std::vector<std::string> const &vals);

    void init();
    rust::String version();
    void info();
    void setUCIOption(rust::String name, rust::String value);
    void loadVariantConfig(rust::String config);
    rust::Vec<rust::String> availableVariants();
    rust::String initialFen(rust::String variantName);
    rust::Vec<fairystockfish::rustffi::PieceInfo> availablePieces();
    bool validateFEN(rust::String variantName, rust::String fen, bool isChess960);
    std::unique_ptr<Position> startingPosition(rust::String variantName, bool isChess960);
    std::unique_ptr<Position> positionFromFen(rust::String variantName, rust::String fen, bool isChess960);
}
