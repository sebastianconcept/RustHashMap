

// Return a hello world string to the caller.
fn get_hello_world() -> String {
    String::from("Hello world!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_library_function_returns_correct_string() {
        let result = get_hello_world();

        assert_eq!(result, String::from("Hello world!"));
    }
}
