macro_rules! exit {
    ($message: expr, $exit_code: expr) => {{
        println!("{}", $message);
        exit($exit_code)
    }};
}

pub(crate) use exit;
