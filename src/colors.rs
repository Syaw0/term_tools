pub struct BasicColor {
    pub fg: u8,
    pub bg: u8,
}

pub const BLACK: BasicColor = BasicColor { fg: 30, bg: 40 };
pub const RED: BasicColor = BasicColor { fg: 31, bg: 41 };
pub const GREEN: BasicColor = BasicColor { fg: 32, bg: 42 };
pub const YELLOW: BasicColor = BasicColor { fg: 33, bg: 43 };
pub const BLUE: BasicColor = BasicColor { fg: 34, bg: 44 };
pub const MAGENTA: BasicColor = BasicColor { fg: 35, bg: 45 };
pub const CYAN: BasicColor = BasicColor { fg: 36, bg: 46 };
pub const WHITE: BasicColor = BasicColor { fg: 37, bg: 47 };
pub const GRAY: BasicColor = BasicColor { fg: 90, bg: 10 };
pub const BRIGHT_RED: BasicColor = BasicColor { fg: 91, bg: 101 };
pub const BRIGHT_GREEN: BasicColor = BasicColor { fg: 92, bg: 102 };
pub const BRIGHT_YELLOW: BasicColor = BasicColor { fg: 93, bg: 103 };
pub const BRIGHT_BLUE: BasicColor = BasicColor { fg: 94, bg: 104 };
pub const BRIGHT_MAGENTA: BasicColor = BasicColor { fg: 95, bg: 105 };
pub const BRIGHT_CYAN: BasicColor = BasicColor { fg: 96, bg: 106 };
pub const BRIGHT_WHITE: BasicColor = BasicColor { fg: 97, bg: 107 };

// =======================================================================

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn black_colors() {
        assert_eq!(30, BLACK.fg);
        assert_eq!(40, BLACK.bg)
    }

    #[test]
    fn cyan_colors() {
        assert_eq!(36, CYAN.fg);
        assert_eq!(46, CYAN.bg)
    }
}