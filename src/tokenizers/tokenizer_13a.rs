use lazy_static::lazy_static;
use regex::Regex;

fn tokenize(input: &str) -> Vec<String> {
    lazy_static! {
        static ref DELETE: Regex = Regex::new(r"<skipped>|-\n").unwrap();
    }
    let res = DELETE.replace_all(input, "");
    super::tokenizer_re::tokenize(
        res.replace("\n", " ")
            .replace("&quot;", "\"")
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .as_str(),
    )
}
