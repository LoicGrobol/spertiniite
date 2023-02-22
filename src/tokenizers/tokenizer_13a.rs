use lazy_static::lazy_static;
use regex::Regex;

use tokenizer_re::tokenize;

use crate::tokenizers::tokenizer_re;

fn tokenize(input: &str) -> Vec<&str> {
    lazy_static! {
        static ref delete: Regex = Regex::new(r"<skipped>|-\n").unwrap();
    }
    let res = delete.replace_all(input, "");
    tokenizer_re::tokenize(
        res.replace("\n", " ")
            .replace("&quot;", '"')
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .as_str(),
    )
}
