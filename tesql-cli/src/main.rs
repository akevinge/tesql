use structopt::StructOpt;
use tesql;

mod error;
use crate::error::print_error;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(name = "in_path", long = "in", short = "i", default_value = ".")]
    in_path: String,
    #[structopt(
        name = "json_out_path",
        long = "json-out",
        default_value = "./tesql_out/out.json"
    )]
    json_out_path: String,
    #[structopt(
        name = "sql_out_path",
        long = "sql-out",
        default_value = "./tesql_out/out.sql"
    )]
    sql_out_path: String,
    #[structopt(
        name = "json_out_dir",
        long = "json-dir",
        default_value = "./tesql_json"
    )]
    json_dir_path: String,
    #[structopt(name = "sql_out_dir", long = "sql-dir", default_value = "./tesql_sql")]
    sql_dir_path: String,
    #[structopt(name = "split_files", long = "split", short = "sp")]
    split_files: bool,
}

#[cfg(windows)]
fn main() {
    let args = Cli::from_args();
    let gen = tesql::gen_inserts_from_file(
        args.in_path.as_str(),
        tesql::GenInsertOptions {
            split_data_files: args.split_files,
            json_out_dir_path: args.json_dir_path.as_str(),
            json_out_file_path: args.json_out_path.as_str(),
            sql_out_dir_path: args.sql_dir_path.as_str(),
            sql_out_file_path: args.sql_out_path.as_str(),
        },
    );

    match gen {
        Ok(_) => {
            println!("[tesql]: successfully generated sql and json files");
        }
        Err(err) => print_error(format!("{:?}", err)),
    }
}
