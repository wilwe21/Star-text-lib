use std::{fmt::Alignment};

use regex::Regex;
use unicode_width::UnicodeWidthStr;

use crate::{ansi_comapct::*, future::*};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FONT {
    AnsiCompat,
    Future
}

#[derive(Clone, Debug)]
pub struct Text {
    pub content: String,
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
    pub font: FONT,
}
#[derive(Clone, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Text {
    pub fn new(string: &str) -> Self {
        let fg = Color { red: 255, green: 255, blue: 255 };
        let font = FONT::Future;
        return Self {
            content: string.to_string(),
            fg_color: Some(fg),
            bg_color: None,
            font: font,
        }
    }
    pub fn set_fg(&mut self, color: Option<Color>) {
        self.fg_color = color;
    }
    pub fn set_bg(&mut self, color: Option<Color>) {
        self.bg_color = color;
    }
    pub fn set_text(&mut self, text: &str) {
        self.content = text.to_string();
    }
    pub fn set_font(&mut self, font: FONT) {
        self.font = font;
    }
}
impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        return Self {
            red,
            green,
            blue
        };
    }
    pub fn reset() -> String {
        return "\x1B[39m\x1B[49m".to_string();
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            f.write_str(&format!("\x1B[48;2;{};{};{}m", self.red, self.green, self.blue))
        } else {
            f.write_str(&format!("\x1B[38;2;{};{};{}m", self.red, self.green, self.blue))
        }
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vec = self.content.chars().collect::<Vec<char>>();
        if f.sign_plus() {
            let mut tester = vec!();
            for i in vec {
                let big = get_big_char(i, self.font.clone());
                let big2 = repalce_forward(&big);
                let bbb = big2.split('\n').collect::<Vec<&str>>();
                if i == ' ' && self.font.clone() == FONT::AnsiCompat {
                    tester.push(" ".to_string());
                    tester.push(" ".to_string());
                    tester.push(" ".to_string());
                } else {
                    for b in bbb {
                        tester.push(format!("{}", b));
                    }
                }
            }
            let size = tester.clone().into_iter().map(|x| x.width()).max().unwrap_or(3) + 2;
            let finfin;
            if let Some(bg) = self.bg_color.clone() {
                finfin = tester.clone().into_iter().map(|x|
                    x.split("\n").map(|z| {
                        let finz = repalce_forward(z);
                        let mut fin = String::new();
                        if let Some(fg) = self.fg_color.clone() {
                            fin.push_str(&format!("{}", fg));
                        }
                        format!("{}{:#}{: ^size$}{}\n", fin, bg, finz, Color::reset())
                    }).collect::<Vec<String>>().join("\n")
                ).collect::<Vec<String>>();
            } else {
                if let Some(fg) = self.fg_color.clone() {
                    finfin = tester.clone().into_iter().map(|x| 
                        x.split("\n").map(|z| format!("{}{}{}\n", fg, z, Color::reset())).collect::<Vec<String>>().join("\n")).collect::<Vec<String>>();
                } else {
                    finfin = tester
                }
            }
            let fin_size = size.checked_sub(2).unwrap_or(1);
            if let Some(bg) = self.bg_color.clone() {
                if f.alternate() {
                    f.write_str(&format!("{}▄{}▄{}\n", bg, "█".repeat(fin_size), Color::reset()));
                }
            }
            for fi in finfin {
                f.write_str(&fi);
            }
            if let Some(bg) = self.bg_color.clone() {
                if f.alternate() {
                    f.write_str(&format!("{}▀{}▀{}", bg, "█".repeat(fin_size), Color::reset()));
                }
            }
            return Ok(());
        }
        if f.sign_minus() {
            let reversed = vec.iter().rev().map(|x| x.to_owned()).collect::<Vec<char>>();
            let mut tester = vec!();
            for i in reversed {
                let big = get_big_char(i, self.font.clone());
                let big2 = repalce_forward(&big);
                let bbb = big2.split('\n').collect::<Vec<&str>>();
                if i == ' ' && self.font.clone() == FONT::AnsiCompat {
                    tester.push(" ".to_string());
                    tester.push(" ".to_string());
                    tester.push(" ".to_string());
                } else {
                    for b in bbb {
                        tester.push(format!("{}", b));
                    }
                }
            }
            let size = tester.clone().into_iter().map(|x| x.width()).max().unwrap_or(3) + 2;
            let finfin;
            if let Some(bg) = self.bg_color.clone() {
                finfin = tester.clone().into_iter().map(|x|
                    x.split("\n").map(|z| {
                        let finz = repalce_forward(z);
                        let mut fin = String::new();
                        if let Some(fg) = self.fg_color.clone() {
                            fin.push_str(&format!("{}", fg));
                        }
                        format!("{}{:#}{: ^size$}{}\n", fin, bg, finz, Color::reset())
                    }).collect::<Vec<String>>().join("\n")
                ).collect::<Vec<String>>();
            } else {
                if let Some(fg) = self.fg_color.clone() {
                    finfin = tester.clone().into_iter().map(|x|
                        x.split("\n").map(|z| format!("{}{}{}\n", fg, z, Color::reset())).collect::<Vec<String>>().join("\n")).collect::<Vec<String>>();
                } else {
                    finfin = tester
                }
            }
            let fin_size = size.checked_sub(2).unwrap_or(1);
            if let Some(bg) = self.bg_color.clone() {
                if f.alternate() {
                    f.write_str(&format!("{}▄{}▄{}\n", bg, "█".repeat(fin_size), Color::reset()));
                }
            }
            for fi in finfin {
                f.write_str(&fi);
            }
            if let Some(bg) = self.bg_color.clone() {
                if f.alternate() {
                    f.write_str(&format!("{}▀{}▀{}", bg, "█".repeat(fin_size), Color::reset()));
                }
            }
            return Ok(());
        }
        let mut top = String::new();
        let mut middle = String::new();
        let mut bottom = String::new();
        let mut optional = String::new();
        let mut optional_top = String::new();
        if f.alternate() {
            if let Some(bg) = self.bg_color.clone() {
                if self.font == FONT::AnsiCompat {
                    optional_top.push_str(&format!("{}▄{}", bg, Color::reset()));
                    top.push_str(&format!("{}█{}", bg, Color::reset()));
                    middle.push_str(&format!("{}█{}", bg, Color::reset()));
                    bottom.push_str(&format!("{}█{}", bg, Color::reset()));
                    optional.push_str(&format!("{}▀{}", bg, Color::reset()));
                } else if self.font == FONT::Future {
                    top.push_str(&format!("{}▄{}", bg, Color::reset()));
                    middle.push_str(&format!("{}█{}", bg, Color::reset()));
                    bottom.push_str(&format!("{}▀{}", bg, Color::reset()));
                }
            }
        }
        if let Some(fg) = self.fg_color.clone() {
            top.push_str(&format!("{}", fg));
            middle.push_str(&format!("{}", fg));
            bottom.push_str(&format!("{}", fg));
        }
        if let Some(bg) = self.bg_color.clone() {
            top.push_str(&format!("{:#} ", bg));
            middle.push_str(&format!("{:#} ", bg));
            bottom.push_str(&format!("{:#} ", bg));
            optional.push_str(&format!("{}{:#} ", bg, bg));
            optional_top.push_str(&format!("{}{:#} ", bg, bg));
        }
        let mut not = String::new();
        for c in vec {
            let mut bb = get_big_char(c, self.font.clone());
            if self.bg_color.is_some() {
                bb = repalce_forward(&bb);
            }
            let big = bb.split("\n").collect::<Vec<&str>>();
            top.push_str(big[0]);
            top.push(' ');
            middle.push_str(big[1]);
            middle.push(' ');
            bottom.push_str(big[2]);
            bottom.push(' ');
            optional.push_str(big[2]);
            optional.push(' ');
            optional_top.push_str(big[0]);
            optional_top.push(' ');
            not.push_str(big[0]);
            not.push(' ');
        }
        top.push_str(&Color::reset());
        middle.push_str(&Color::reset());
        bottom.push_str(&Color::reset());
        optional.push_str(&Color::reset());
        optional_top.push_str(&Color::reset());
        if f.alternate() {
            if let Some(bg) = self.bg_color.clone() {
                if self.font == FONT::AnsiCompat {
                    optional_top.push_str(&format!("{}▄{}", bg, Color::reset()));
                    top.push_str(&format!("{}█{}", bg, Color::reset()));
                    middle.push_str(&format!("{}█{}", bg, Color::reset()));
                    bottom.push_str(&format!("{}█{}", bg, Color::reset()));
                    optional.push_str(&format!("{}▀{}", bg, Color::reset()));
                } else if self.font == FONT::Future {
                    top.push_str(&format!("{}▄{}", bg, Color::reset()));
                    middle.push_str(&format!("{}█{}", bg, Color::reset()));
                    bottom.push_str(&format!("{}▀{}", bg, Color::reset()));
                }
            }
        }
        let mut top_aligned = String::new();
        let mut middle_aligned = String::new();
        let mut bottom_aligned = String::new();
        let mut optional_aligned = String::new();
        let mut optional_top_aligned = String::new();
        if let Some(align) = f.align() {
            if let Some(width) = f.width() {
                let padding: usize;
                if self.bg_color.is_some() {
                    padding = width.checked_sub(not.width()+3).unwrap_or(0).checked_div(2).unwrap_or(0);
                } else {
                    padding = width.checked_sub(not.width()).unwrap_or(0).checked_div(2).unwrap_or(0);
                }
                if align == Alignment::Center {
                    top_aligned.push_str(&format!("\x1B[{}C", padding));
                    middle_aligned.push_str(&format!("\x1B[{}C", padding));
                    bottom_aligned.push_str(&format!("\x1B[{}C", padding));
                    optional_aligned.push_str(&format!("\x1B[{}C", padding));
                    optional_top_aligned.push_str(&format!("\x1B[{}C", padding));
                }
                if align == Alignment::Right {
                    top_aligned.push_str(&format!("\x1B[{}C", width));
                    middle_aligned.push_str(&format!("\x1B[{}C", width));
                    bottom_aligned.push_str(&format!("\x1B[{}C", width));
                    optional_aligned.push_str(&format!("\x1B[{}C", width));
                    optional_top_aligned.push_str(&format!("\x1B[{}C", padding));
                }
                optional_top_aligned.push_str(&optional_top);
                top_aligned.push_str(&top);
                middle_aligned.push_str(&middle);
                bottom_aligned.push_str(&bottom);
                optional_aligned.push_str(&optional);
                if align == Alignment::Left {
                    top_aligned.push_str(&format!("\x1B[{}C", width));
                    middle_aligned.push_str(&format!("\x1B[{}C", width));
                    bottom_aligned.push_str(&format!("\x1B[{}C", width));
                    optional_aligned.push_str(&format!("\x1B[{}C", width));
                    optional_top_aligned.push_str(&format!("\x1B[{}C", padding));
                }
                if align == Alignment::Center {
                    optional_aligned.push_str(&format!("\x1B[{}C", padding));
                    top_aligned.push_str(&format!("\x1B[{}C", padding));
                    middle_aligned.push_str(&format!("\x1B[{}C", padding));
                    bottom_aligned.push_str(&format!("\x1B[{}C", padding));
                    optional_aligned.push_str(&format!("\x1B[{}C", padding));
                    optional_top_aligned.push_str(&format!("\x1B[{}C", padding));
                }
            } else {
                top_aligned.push_str(&top);
                middle_aligned.push_str(&middle);
                bottom_aligned.push_str(&bottom);
                optional_aligned.push_str(&optional);
                optional_top_aligned.push_str(&optional_top);
            }
        } else {
            top_aligned.push_str(&top);
            middle_aligned.push_str(&middle);
            bottom_aligned.push_str(&bottom);
            optional_aligned.push_str(&optional);
            optional_top_aligned.push_str(&optional_top);
        }
        optional_top_aligned.push('\n');
        top_aligned.push('\n');
        middle_aligned.push('\n');
        bottom_aligned.push('\n');
        if self.bg_color.is_some() && self.font == FONT::AnsiCompat {
            f.write_str(&optional_top_aligned);
        }
        f.write_str(&top_aligned);
        f.write_str(&middle_aligned);
        f.write_str(&bottom_aligned);
        if self.bg_color.is_some() && self.font == FONT::AnsiCompat {
            f.write_str(&optional_aligned);
        }
        Ok(())
    }
}

pub fn repalce_forward(text: &str) -> String {
    let re = Regex::new(r"\x1B\[(\d*)C").unwrap();
    re.replace_all(text, |caps: &regex::Captures| {
        let n_str = caps.get(1).map_or("", |m| m.as_str());
        let n = n_str.parse::<usize>().unwrap_or(1);
        " ".repeat(n)
    }).to_string()
}

fn get_big_char(c: char, font: FONT) -> String {
    let s = c.to_string().to_lowercase().chars().collect::<Vec<char>>().first().unwrap_or(&' ').to_owned();
    match font {
        FONT::AnsiCompat => {    
            match s {
                'a' => COMPACT_A.to_string(),
                'b' => COMPACT_B.to_string(),
                'c' => COMPACT_C.to_string(),
                'd' => COMPACT_D.to_string(),
                'e' => COMPACT_E.to_string(),
                'f' => COMPACT_F.to_string(),
                'g' => COMPACT_G.to_string(),
                'h' => COMPACT_H.to_string(),
                'i' => COMPACT_I.to_string(),
                'j' => COMPACT_J.to_string(),
                'k' => COMPACT_K.to_string(),
                'l' => COMPACT_L.to_string(),
                'm' => COMPACT_M.to_string(),
                'n' => COMPACT_N.to_string(),
                'o' => COMPACT_O.to_string(),
                'p' => COMPACT_P.to_string(),
                'q' => COMPACT_Q.to_string(),
                'r' => COMPACT_R.to_string(),
                's' => COMPACT_S.to_string(),
                't' => COMPACT_T.to_string(),
                'u' => COMPACT_U.to_string(),
                'v' => COMPACT_V.to_string(),
                'w' => COMPACT_W.to_string(),
                'x' => COMPACT_X.to_string(),
                'y' => COMPACT_Y.to_string(),
                'z' => COMPACT_Z.to_string(),
                '0' => COMPACT_DIGIT_0.to_string(),
                '1' => COMPACT_DIGIT_1.to_string(),
                '2' => COMPACT_DIGIT_2.to_string(),
                '3' => COMPACT_DIGIT_3.to_string(),
                '4' => COMPACT_DIGIT_4.to_string(),
                '5' => COMPACT_DIGIT_5.to_string(),
                '6' => COMPACT_DIGIT_6.to_string(),
                '7' => COMPACT_DIGIT_7.to_string(),
                '8' => COMPACT_DIGIT_8.to_string(),
                '9' => COMPACT_DIGIT_9.to_string(),
                '~' => COMPACT_TILDE.to_string(),
                '`' => COMPACT_BACKTICK.to_string(),
                '!' => COMPACT_EXCLAMATION.to_string(),
                '@' => COMPACT_AT.to_string(),
                '#' => COMPACT_HASH.to_string(),
                '$' => COMPACT_DOLLAR.to_string(),
                '%' => COMPACT_PERCENT.to_string(),
                '^' => COMPACT_CARET.to_string(),
                '&' => COMPACT_AMPERSAND.to_string(),
                '*' => COMPACT_ASTERISK.to_string(),
                '(' => COMPACT_PAREN_OPEN.to_string(),
                ')' => COMPACT_PAREN_CLOSE.to_string(),
                '_' => COMPACT_UNDERSCORE.to_string(),
                '-' => COMPACT_HYPHEN.to_string(),
                '+' => COMPACT_PLUS.to_string(),
                '=' => COMPACT_EQUALS.to_string(),
                '/' => COMPACT_SLASH.to_string(),
                '\\' => COMPACT_BACKSLASH.to_string(),
                ',' => COMPACT_COMMA.to_string(),
                '.' => COMPACT_PERIOD.to_string(),
                '<' => COMPACT_LESS_THAN.to_string(),
                '>' => COMPACT_GREATER_THAN.to_string(),
                ':' => COMPACT_COLON.to_string(),
                ';' => COMPACT_SEMICOLON.to_string(),
                '"' => COMPACT_DOUBLE_QUOTE.to_string(),
                '\'' => COMPACT_SINGLE_QUOTE.to_string(),
                '[' => COMPACT_BRACKET_OPEN.to_string(),
                ']' => COMPACT_BRACKET_CLOSE.to_string(),
                '{' => COMPACT_CURLY_OPEN.to_string(),
                '}' => COMPACT_CURLY_CLOSE.to_string(),
                '|' => COMPACT_PIPE.to_string(),
                '?' => COMPACT_QUESTION.to_string(),
                _ => "　　\n　　\n　　".to_string()
            } 
        },
        FONT::Future => {
            match s {
                'a' => FUTURE_A.to_string(),
                'b' => FUTURE_B.to_string(),
                'c' => FUTURE_C.to_string(),
                'd' => FUTURE_D.to_string(),
                'e' => FUTURE_E.to_string(),
                'f' => FUTURE_F.to_string(),
                'g' => FUTURE_G.to_string(),
                'h' => FUTURE_H.to_string(),
                'i' => FUTURE_I.to_string(),
                'j' => FUTURE_J.to_string(),
                'k' => FUTURE_K.to_string(),
                'l' => FUTURE_L.to_string(),
                'm' => FUTURE_M.to_string(),
                'n' => FUTURE_N.to_string(),
                'o' => FUTURE_O.to_string(),
                'p' => FUTURE_P.to_string(),
                'q' => FUTURE_Q.to_string(),
                'r' => FUTURE_R.to_string(),
                's' => FUTURE_S.to_string(),
                't' => FUTURE_T.to_string(),
                'u' => FUTURE_U.to_string(),
                'v' => FUTURE_V.to_string(),
                'w' => FUTURE_W.to_string(),
                'x' => FUTURE_X.to_string(),
                'y' => FUTURE_Y.to_string(),
                'z' => FUTURE_Z.to_string(),
                '1' => FUTURE_ONE.to_string(),
                '2' => FUTURE_TWO.to_string(),
                '3' => FUTURE_THREE.to_string(),
                '4' => FUTURE_FOUR.to_string(),
                '5' => FUTURE_FIVE.to_string(),
                '6' => FUTURE_SIX.to_string(),
                '7' => FUTURE_SEVEN.to_string(),
                '8' => FUTURE_EIGHT.to_string(),
                '9' => FUTURE_NINE.to_string(),
                '0' => FUTURE_ZERO.to_string(),
                '`' => FUTURE_BACKTICK.to_string(),
                '~' => FUTURE_TILDE.to_string(),
                '!' => FUTURE_EXCLAMATION.to_string(),
                '@' => FUTURE_AT.to_string(),
                '#' => FUTURE_HASH.to_string(),
                '$' => FUTURE_DOLAR.to_string(),
                '%' => FUTURE_PERCENT.to_string(),
                '^' => FUTURE_CARET.to_string(),
                '&' => FUTURE_AND.to_string(),
                '*' => FUTURE_ASTERISK.to_string(),
                '(' => FUTURE_OPENING.to_string(),
                ')' => FUTURE_CLOSING.to_string(),
                '_' => FUTURE_UNDERSCORE.to_string(),
                '-' => FUTURE_MINUS.to_string(),
                '+' => FUTURE_PLUS.to_string(),
                '=' => FUTURE_EQUAL.to_string(),
                '/' => FUTURE_SLASH.to_string(),
                '\\'=> FUTURE_BACKSLASH.to_string(),
                ',' => FUTURE_COMA.to_string(),
                '.' => FUTURE_DOT.to_string(),
                '<' => FUTURE_LOWER.to_string(),
                '>' => FUTURE_GREATER.to_string(),
                ':' => FUTURE_COLON.to_string(),
                ';' => FUTURE_SEMICOLON.to_string(),
                '"' => FUTURE_QUOTE.to_string(),
                '\'' => FUTURE_SINGLEQUOTE.to_string(),
                '{' => FUTURE_CURLY_OPENING.to_string(),
                '}' => FUTURE_CURLY_CLOSING.to_string(),
                '[' => FUTURE_SQUERE_OPENING.to_string(),
                ']' => FUTURE_SQUERE_CLOSING.to_string(),
                '?' => FUTURE_QUESTION.to_string(),
                _ => "  \n  \n  ".to_string()
            } 
        }
    }
}
