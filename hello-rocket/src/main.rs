#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> &'static str {
    print!("You typed this command: {}", cmd);
    "Hello mello from the search page!"
}

fn main() {
    rocket::ignite().mount("/", routes![index, search]).launch();
}

fn get_command_from_query_string(query_string: &str) -> &str {
    let test = "hello";
    return &test;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_from_query_string_no_whitespace() {
          // Test with command only
        let actual = get_command_from_query_string("tw");
        let expected = "tw";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_command_from_query_string_with_whitespace() {
        let actual = get_command_from_query_string
        ("tw @fbOpenSource");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
}
