#include "fairystockfishrs.h"
#include "fairystockfish.h"

#include "fairystockfish/src/lib.rs.h"

#include <algorithm>

namespace fairystockfish::rustffi {
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
        rust::Vec<rust::String> retVal;
        auto variants = fairystockfish::availableVariants();
        std::copy(variants.begin(), variants.end(), std::back_inserter(retVal));
        return retVal;
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
}
