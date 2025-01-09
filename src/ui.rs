use clap::builder::styling::{AnsiColor, Effects, Styles};

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::BrightYellow.on_default() | Effects::BOLD)
        .usage(AnsiColor::BrightYellow.on_default() | Effects::BOLD)
        .literal(AnsiColor::BrightGreen.on_default() | Effects::BOLD)
        .error(AnsiColor::BrightRed.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::BrightGreen.on_default())
}
