pub enum TextStyle {
    BOLD,
    ITALIC,
    UNDERLINE,

    GRAY,
    RED,
    GREEN,
    YELLOW,
    CYAN,
    VIOLET,
    BLUE,
}

impl TextStyle {
    /// Get the prefix text for the color
    pub fn get_prefix(&self) -> &str {
        match &self {
            Self::BOLD => "\x1b[1m",
            Self::ITALIC => "\x1b[3m",
            Self::UNDERLINE => "\x1b[4m",

            Self::GRAY => "\x1b[90m",
            Self::RED => "\x1b[91m",
            Self::GREEN => "\x1b[92m",
            Self::YELLOW => "\x1b[93m",
            Self::CYAN => "\x1b[94m",
            Self::VIOLET => "\x1b[95m",
            Self::BLUE => "\x1b[96m",

            _ => "",
        }
    }
}

impl TextStyle {
    pub fn postfix() -> String {
        "\x1b[0m".to_string()
    }

    /// Return an String colored with the TextStyle color!
    /// ### Example
    /// ```
    /// use highlight_search::*;
    /// 
    /// let cyan_string = TextStyle::to_string("Some text...", TextStyle::CYAN);
    /// println!("{} << yey", cyan_string);
    ///
    /// assert_eq!(cyan_string, "\x1b[94mSome text...\x1b[0m");
    /// ```
    pub fn to_string(text: &str, style: TextStyle) -> String {
        format!("{}{}{}", style.get_prefix(), text, TextStyle::postfix())
    }
}
