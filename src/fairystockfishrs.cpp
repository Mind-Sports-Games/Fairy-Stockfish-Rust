#include "fairystockfishrs.h"

#include "fairystockfish/src/lib.rs.h"

#include <algorithm>

namespace fairystockfish::rustffi {

    rust::Vec<rust::String> toRust(std::vector<std::string> const &vals) {
        rust::Vec<rust::String> retVal;
        std::copy(vals.begin(), vals.end(), std::back_inserter(retVal));
        return retVal;
    }
    TestWithGameResult toRust(std::tuple<bool, int> res) {
        return fairystockfish::rustffi::TestWithGameResult{
            std::get<0>(res),  
            std::uint32_t(std::get<1>(res))
        };
    }
    TestByPlayers toRust(std::tuple<bool, bool> res) {
        return fairystockfish::rustffi::TestByPlayers{
            std::get<0>(res),  
            std::get<1>(res)
        };
    }

    void init() { fairystockfish::init(); }

    rust::String version() { return fairystockfish::version(); }

    void info() { fairystockfish::info(); }

    void setUCIOption(rust::String name, rust::String value) {
        fairystockfish::setUCIOption(std::string(name), std::string(value));
    }

    void loadVariantConfig(rust::String config) {
        fairystockfish::loadVariantConfig(std::string(config));
    }

    rust::Vec<rust::String> availableVariants() {
        return toRust(fairystockfish::availableVariants());
    }

    rust::String initialFen(rust::String variantName) { return fairystockfish::initialFen(std::string(variantName)); }

    rust::Vec<fairystockfish::rustffi::PieceInfo> availablePieces() {
        auto pieceMap = fairystockfish::availablePieces();
        rust::Vec<PieceInfo> retVal;
        for (auto const &[name, piece] : pieceMap) {
            retVal.push_back(fairystockfish::rustffi::PieceInfo{
                std::uint32_t(piece.id()),
                piece.name(),
                piece.betza()
            });
        }
        return retVal;
    }

    bool validateFEN(rust::String variantName, rust::String fen, bool isChess960) {
        return fairystockfish::validateFEN(std::string(variantName), std::string(fen), isChess960);
    }

    std::unique_ptr<Position> startingPosition(rust::String variantName, bool isChess960) {
        return std::make_unique<Position>(
            fairystockfish::Position{std::string(variantName), isChess960}
        );
    }

    std::unique_ptr<Position> positionFromFen(rust::String variantName, rust::String fen, bool isChess960) {
        return std::make_unique<Position>(
            fairystockfish::Position{std::string(variantName), std::string(fen), isChess960}
        );
    }

    std::unique_ptr<Position> Position::makeMoves(rust::Vec<rust::String> const &moves) const {
        std::vector<std::string> uciMoves;
        for (auto const &m : moves) {
            uciMoves.push_back(std::string(m));
        }
        return std::make_unique<Position>(impl->makeMoves(uciMoves));
    }

    rust::String Position::getFEN() const { return impl->getFEN(); }

    rust::Vec<rust::String> Position::getLegalMoves() const {
        return toRust(impl->getLegalMoves());
    }

    bool Position::givesCheck() const { return impl->givesCheck(); }
    bool Position::hasRepeated() const { return impl->hasRepeated(); }
    bool Position::isDraw(std::uint32_t ply) const { return impl->isDraw(int(ply)); }
    bool Position::hasGameCycle(std::uint32_t ply) const { return impl->hasGameCycle(int(ply)); }
    std::uint32_t Position::gameResult() const {
        return std::uint32_t(impl->gameResult());
    }

    fairystockfish::rustffi::TestWithGameResult Position::isImmediateGameEnd() const {
        return toRust(impl->isImmediateGameEnd());
    }

    fairystockfish::rustffi::TestWithGameResult Position::isOptionalGameEnd() const {
        return toRust(impl->isOptionalGameEnd());
    }
    fairystockfish::rustffi::TestByPlayers Position::hasInsufficientMaterial() const {
        return toRust(impl->hasInsufficientMaterial());
    }
}
