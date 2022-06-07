use ansi_term::Color;

pub fn error(msg: &str) -> String {
   format!("{}", Color::RGB(235, 66, 66).paint(msg))
}

pub fn warn(msg: &str) -> String {
   format!("{}", Color::RGB(245, 181, 61).paint(msg))
}

pub fn success(msg: &str) -> String {
   format!("{}", Color::RGB(57, 219, 57).paint(msg))
}

pub fn info(msg: &str) -> String {
   format!("{}", Color::RGB(2, 149, 235).paint(msg))
}
