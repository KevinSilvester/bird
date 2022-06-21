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

#[macro_export]
macro_rules! colour {
    (amber, $($arg:tt)*) => {{
        use $crate::utils::colour;
        format!("{}", colour::warn(&format!($($arg)*)))
    }};

    (red, $($arg:tt)*) => {{
        use $crate::utils::colour;
        format!("{}", colour::error(&format!($($arg)*)))
    }};

    (green, $($arg:tt)*) => {{
        use $crate::utils::colour;
        format!("{}", colour::success(&format!($($arg)*)))
    }};

   (blue, $($arg:tt)*) => {{
        use $crate::utils::colour;
        format!("{}", colour::info(&format!($($arg)*)))
    }};
}

#[macro_export]
macro_rules! outln {
    (error, $($arg:tt)*) => {{
        print!("{}: ", $crate::colour!(red, "ERROR"));
        println!($($arg)*);
    }};

    (info, $($arg:tt)*) => {{
        print!("{}: ", $crate::colour!(blue, "INFO"));
        println!($($arg)*);
    }};

    (warn, $($arg:tt)*) => {{
        print!("{}: ", $crate::colour!(amber, "WARNING"));
        println!($($arg)*);
    }};

    ($($arg:tt)*) => {{
        println!($($arg)*);
    }}
}
