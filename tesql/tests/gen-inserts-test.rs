#[cfg(test)]
mod integration {
    use std::fs::{self};

    use tesql::{gen_inserts_from_file, GenInsertOptions};

    #[test]
    fn test_gen_inserts_from_file_single_file() {
        let opts = GenInsertOptions {
            split_data_files: false,
            sql_out_file_path: "./testgen-out/data-test.sql",
            ..Default::default()
        };
        match gen_inserts_from_file("./testdata/data-test.json", opts) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }

        let str = fs::read_to_string("./testgen-out/data-test.sql").unwrap();
        assert_eq!(true, str.starts_with("INSERT INTO \"user\""));
    }
}
