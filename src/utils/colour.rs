use ansi_term::Color;

pub fn red(msg: &str) -> String {
   format!("{}", Color::RGB(235, 66, 66).paint(msg))
}

pub fn amber(msg: &str) -> String {
   format!("{}", Color::RGB(245, 181, 61).paint(msg))
}

pub fn green(msg: &str) -> String {
   format!("{}", Color::RGB(57, 219, 57).paint(msg))
}

pub fn blue(msg: &str) -> String {
   format!("{}", Color::RGB(2, 149, 235).paint(msg))
}

#[macro_export]
macro_rules! colour {
    (amber, $($arg:tt)*) => {{
        use $crate::utils::colour;
        format!("{}", colour::amber(&format!($($arg)*)))
    }};

    (red, $($arg:tt)*) => {{
        use $crate::utils::colour;
        format!("{}", colour::red(&format!($($arg)*)))
    }};

    (green, $($arg:tt)*) => {{
        use $crate::utils::colour;
        format!("{}", colour::green(&format!($($arg)*)))
    }};

   (blue, $($arg:tt)*) => {{
        use $crate::utils::colour;
        format!("{}", colour::blue(&format!($($arg)*)))
    }};
}

#[macro_export]
macro_rules! outln {
    (error, $($arg:tt)*) => {{
        print!("\n{}: ", $crate::colour!(red, "ERROR"));
        println!($($arg)*);
    }};

    (info, $($arg:tt)*) => {{
        print!("\n{}: ", $crate::colour!(blue, "INFO"));
        println!($($arg)*);
    }};

    (warn, $($arg:tt)*) => {{
        print!("\n{}: ", $crate::colour!(amber, "WARNING"));
        println!($($arg)*);
    }};

    ($($arg:tt)*) => {{
        println!($($arg)*);
    }}
}
