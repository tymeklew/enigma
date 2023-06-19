pub enum Modifier {
    Bold,
    Faint,
    Italic,
    Underline,
    Strikethrough,
}
pub enum Colour {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}
pub trait Style {
    fn style(&self, colour: Option<Colour>, modifier: Option<Modifier>) -> String;
}
impl Style for String {
    fn style(&self, colour: Option<Colour>, modifier: Option<Modifier>) -> String {
        style(self.to_string(), colour, modifier)
    }
}
impl Style for str {
    fn style(&self, colour: Option<Colour>, modifier: Option<Modifier>) -> String {
        style(self.to_string(), colour, modifier)
    }
}

fn style(text: String, colour: Option<Colour>, modifier: Option<Modifier>) -> String {
    format!(
        "\x1b[{}{}m{}\x1b[0,",
        match modifier {
            Some(modifier) => {
                let m = match modifier {
                    Modifier::Bold => 1,
                    Modifier::Italic => 3,
                    Modifier::Underline => 4,
                    Modifier::Strikethrough => 9,
                    Modifier::Faint => 2,
                }
                .to_string();
                if let Some(_) = colour {
                    m + ";"
                } else {
                    m
                }
            }
            None => "".to_string(),
        },
        match colour {
            Some(colour) => match colour {
                Colour::Black => 30,
                Colour::Red => 31,
                Colour::Green => 32,
                Colour::Yellow => 33,
                Colour::Blue => 34,
                Colour::Magenta => 35,
                Colour::Cyan => 36,
                Colour::White => 37,
            }
            .to_string(),
            None => "".to_string(),
        },
        text
    )
}
