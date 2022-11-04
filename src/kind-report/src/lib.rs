use yansi::Paint;

/// Data structures
pub mod data;
/// Render
pub mod report;

#[derive(Debug)]
pub struct Chars {
    pub vbar: char,
    pub hbar: char,
    pub dbar: char,
    pub trline: char,
    pub bxline: char,
    pub brline: char,
    pub ylline: char,
}

impl Chars {
    pub fn unicode() -> &'static Chars {
        &Chars {
            vbar: '│',
            hbar: '─',
            dbar: '┆',
            trline: '└',
            bxline: '┬',
            brline: '┌',
            ylline: '├',
        }
    }
    pub fn ascii() -> &'static Chars {
        &Chars {
            vbar: '|',
            hbar: '-',
            dbar: ':',
            trline: '\\',
            bxline: 'v',
            brline: '/',
            ylline: '-',
        }
    }
}

#[derive(Debug)]
pub struct RenderConfig<'a> {
    pub chars: &'a Chars,
    pub indent: usize,
}

impl<'a> RenderConfig<'a> {
    pub fn unicode(indent: usize) -> RenderConfig<'a> {
        RenderConfig {
            chars: Chars::unicode(),
            indent,
        }
    }
    pub fn ascii(indent: usize) -> RenderConfig<'a> {
        RenderConfig {
            chars: Chars::ascii(),
            indent,
        }
    }
}

pub fn check_if_colors_are_supported(disable: bool) {
    if cfg!(windows) && !Paint::enable_windows_ascii() || disable {
        Paint::disable();
    }
}