#include "fairystockfishrs.h"

#include "fairystockfish/src/lib.rs.h"

#include <algorithm>

namespace fairystockfish::rustffi {

    rust::Vec<rust::String> toRust(std::vector<std::string> const &vals) {
        rust::Vec<rust::String> retVal;
        std::copy(vals.begin(), vals.end(), std::back_inserter(retVal));
        return retVal;
    }

    std::vector<std::string> toCpp(rust::Vec<rust::String> const &vals) {
        std::vector<std::string> retVal;
        std::transform(
            vals.begin(),
            vals.end(),
            std::back_inserter(retVal),
            [](auto const &s) {
                return std::string(s);
            }
        );
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

    void setUCIOption(rust::String const &name, rust::String const &value) {
        fairystockfish::setUCIOption(std::string(name), std::string(value));
    }

    void loadVariantConfig(rust::String const &config) {
        fairystockfish::loadVariantConfig(std::string(config));
    }

    rust::Vec<rust::String> availableVariants() {
        return toRust(fairystockfish::availableVariants());
    }

    rust::Vec<Piece> toRust(std::vector<fairystockfish::Piece> const &vals) {
        rust::Vec<Piece> retVal;
        std::transform(
            vals.begin(),
            vals.end(),
            std::back_inserter(retVal),
            [](auto const &p) {
                return toRust(p);
            }
        );
        return retVal;
    }

    fairystockfish::rustffi::PieceInfo toRust(fairystockfish::PieceInfo const &p) {
        return fairystockfish::rustffi::PieceInfo{
            std::uint32_t(p.id()),
            p.name(),
            p.betza()
        };
    }

    fairystockfish::rustffi::Piece toRust(fairystockfish::Piece const &p) {
        return fairystockfish::rustffi::Piece{
            p.isWhite() ? Color::White : Color::Black,
            p.promoted(),
            toRust(p.pieceInfo())
        };
    }

    fairystockfish::rustffi::PieceOnBoard toRust(std::string square, fairystockfish::Piece const &p) {
        return fairystockfish::rustffi::PieceOnBoard{
            toRust(p),
            square
        };

    }

    rust::String initialFen(rust::String const &variantName) { return fairystockfish::initialFen(std::string(variantName)); }

    rust::Vec<fairystockfish::rustffi::PieceInfo> availablePieces() {
        auto pieceMap = fairystockfish::availablePieces();
        rust::Vec<PieceInfo> retVal;
        for (auto const &[name, piece] : pieceMap) {
            retVal.push_back(toRust(piece));
        }
        return retVal;
    }

    bool validateFEN(rust::String const &variantName, rust::String const &fen, bool isChess960) {
        return fairystockfish::validateFEN(std::string(variantName), std::string(fen), isChess960);
    }

    std::unique_ptr<Position> startingPosition(rust::String const &variantName, bool isChess960) {
        return std::make_unique<Position>(
            fairystockfish::Position{std::string(variantName), isChess960}
        );
    }

    std::unique_ptr<Position> positionFromFen(rust::String const &variantName, rust::String const &fen, bool isChess960) {
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
    rust::String Position::getFENWithArgs(bool sFen, bool showPromoted, std::uint32_t countStarted) const {
        return impl->getFEN(sFen, showPromoted, countStarted);
    }
    rust::String Position::getSAN(rust::String const &uci) const { return impl->getSAN(std::string(uci)); }
    rust::String Position::getSANWithNotation(rust::String const &uci, Notation notation) const {
        return impl->getSAN(std::string(uci), notation);
    }

    rust::Vec<rust::String> Position::getSANMoves(rust::Vec<rust::String> const &uci) const {
        return toRust(impl->getSANMoves(toCpp(uci)));
    }
    rust::Vec<rust::String> Position::getSANMovesWithNotation(rust::Vec<rust::String> const &uci, Notation notation) const {
        return toRust(impl->getSANMoves(toCpp(uci), notation));
    }

    rust::Vec<rust::String> Position::getLegalMoves() const {
        return toRust(impl->getLegalMoves());
    }

    rust::Vec<fairystockfish::rustffi::Piece> Position::piecesInHand() const {
        return toRust(impl->piecesInHand());
    }
    rust::Vec<fairystockfish::rustffi::PieceOnBoard> Position::piecesOnBoard() const {
        auto pieceMap = impl->piecesOnBoard();
        rust::Vec<PieceOnBoard> retVal;
        for (auto const &[board, piece] : pieceMap) {
            retVal.push_back(toRust(board, piece));
        }
        return retVal;
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
