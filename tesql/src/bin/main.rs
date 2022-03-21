use tesql::{gen_inserts_from_file, GenInsertOptions};

fn main() {
    match gen_inserts_from_file(
        "./testdata/data-test.json",
        GenInsertOptions {
            split_data_files: false,
            sql_out_dir_path: "./out",
            sql_out_file_path: "./out/out.sql",
            json_out_dir_path: "./out",
            json_out_file_path: "./out/out.json",
        },
    ) {
        Ok(_) => {
            println!("Succesfully generated sql");
        }
        Err(e) => println!("{:?}", e),
    }
}
