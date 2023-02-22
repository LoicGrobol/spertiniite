use lazy_static::lazy_static;
use regex::Regex;

pub fn tokenize(input: &str) -> Vec<&str> {
    lazy_static! {
        static ref western: Regex = Regex::new(r"([\{-\~\[-\` -\&\(-\+\:-\@\/])").unwrap();
        static ref digit_period_comma: Regex = Regex::new(r"([^0-9])([\.,])").unwrap();
        static ref period_comma_digit: Regex = Regex::new(r"([\.,])([^0-9]").unwrap();
        static ref digit_dash: Regex = Regex::new(r"([0-9])(-)").unwrap();
    }
    // TODO: maybe this could be made faster using `regex::Regex.split`?
    let res = western.replace_all(input, "$1 ");
    res = digit_period_comma.replace_all(res.as_ref(), "$1 $2 ");
    res = period_comma_digit.replace_all(res.as_ref(), " $1 $2");
    res = digit_dash.replace_all(res.as_ref(), "$1 $2 ");
    res.as_ref().split_whitespace()
}
