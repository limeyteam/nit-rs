pub mod nitc;
pub mod configure;
use std::env;
use std::fs;

fn main() {
    nitc::nit_warn("Nit Started, Garbage collection done!".to_string());

    let args: Vec<String> = env::args().collect();

    nitc::nit_info("Collected Arguments".to_string());

    let author = "Exeon";

    let mut vec = Vec::with_capacity(2);
    vec = ["file1", "file2", "file3"].to_vec();

    match args[1].as_str() {
        "commit" => nitc::nit_commit(author.to_string(), args[3].to_string(), vec, args[2].to_string()),
        "add" => nitc::nit_add(args[2].to_string()),
        _ => nitc::nit_error("Unknown Nit command.".to_string()),
    }
}