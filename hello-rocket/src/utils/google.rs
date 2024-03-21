extern crate percent_encoding; 

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
            .add(b'<').add(b'>').add(b'`');

pub fn construct_google_search_from_query(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT);
    format!("https://google.com/search?q={}", query);
    let google_search_url = format!("https://google.com/search?q={}", encoded_query);

    google_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_google_search_url() {
        let fake_query = "hello";
        assert_eq!(
            construct_google_search_from_query(fake_query),
            "https://google.com/search?q=hello"
        );
    }

    #[test]
    fn test_construct_google_search_url_with_encoding() {
        let fake_query = "hello world";
        assert_eq!(
            construct_google_search_from_query(fake_query),
            "https://google.com/search?q=hello%20world"
        );
    }
}

