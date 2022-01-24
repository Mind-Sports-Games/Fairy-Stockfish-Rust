//----------------------------------------------------------------------------
// Unfortunately the `cxx` crate is not directly compatible with by-value
// strings. This means that we have to convert strings to/from it's wrapped
// version of a rust-compatible string. 
//
// TODO: evaluate the impact on copying data tha this has. It may not show up
//       in our use case, but it probably is impactful.
//----------------------------------------------------------------------------
#include "rust/cxx.h"


namespace fairystockfish::rustffi {
    struct PieceInfo;

    void init();
    rust::String version();
    void info();
    void setUCIOption(rust::String name, rust::String value);
    void loadVariantConfig(rust::String config);
    rust::Vec<rust::String> availableVariants();
    rust::String initialFen(rust::String variantName);
    rust::Vec<fairystockfish::rustffi::PieceInfo> availablePieces();
    bool validateFEN(rust::String variantName, rust::String fen, bool isChess960);
}
