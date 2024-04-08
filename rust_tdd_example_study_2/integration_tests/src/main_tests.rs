extern crate process;

mod common;

#[cfg(test)]
mod tests {
    use process::execute;
    use super::common;

    #[test]
    fn test_main_add() {
        common::setup();

        // setup environment args
        let app_name: String = "".to_string();
        let args: Vec<String> = vec![
            app_name,
            "add".into(),
            "1".into(),
            "1".into()
        ];
        // call main
        let result = execute(args);

        assert_eq!(result, 2)
    }
}