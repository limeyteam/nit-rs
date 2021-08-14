use colored::*;
use std::process::{Command, Stdio};
use std::io::{BufRead,BufReader, BufWriter};
#[derive(Debug)]
pub enum NitEvent {
    Info,
    Warning,
    Error,
    Success,
}

#[derive(Debug)]
pub struct Commit<T> {
    author: String,
    message: String,
    files: Vec<T>,
}

pub fn nit_info(msg: String) {
    nit_output(NitEvent::Info, msg);
}
pub fn nit_warn(msg: String) {
    nit_output(NitEvent::Warning, msg);
}
pub fn nit_error(msg: String) {
    nit_output(NitEvent::Error, msg);
}
pub fn nit_success(msg: String) {
    nit_output(NitEvent::Success, msg);
}

fn nit_output(event: NitEvent, msg: String) {
    match event {
        NitEvent::Error => println!("[{0}] {1}", "ERR".red(), msg),
        NitEvent::Warning => println!("[{0}] {1}", "WARN".yellow(), msg),
        NitEvent::Info => println!("[{0}] {1}", "MSG".blue(), msg),
        NitEvent::Success => println!("[{0}] {1}", "SUCCESS".green(), msg),
        _ => {
            println!("Event borken please submit issue on https://github.com/limeyteam/nit/issues")
        }
    }
}

// pub fn commit_test() {
//     let mut array = Vec::with_capacity(2);
//     array = ["/var/testfile.txt", "/home/user"].to_vec();
//     let commit = Commit {
//         author: String::from("Exeon"),
//         message: String::from("Test"),
//         files: array,
//     };
// }

pub fn nit_commit<T: std::fmt::Debug>(
    author: String,
    message: String,
    files: Vec<T>,
    itrue: String,
) {
    if itrue == "-m" {
        let commit = Commit {
            author: author,
            message: message,
            files: files,
        };
        nit_info(format!(
            "New commit from {0}: \"{1}\", adding {2:?}",
            commit.author, commit.message, commit.files
        ));
    } else {
        nit_error("You did not specify a message!".to_string());
    }
}

pub fn nit_add(files: String) {
    if files == ".".to_string() {
        let mut pwd =
        Command::new("cmd")
        .args(&["/C", "cd"])
        .output()
        .expect("Le pwd has failed to do ze thing, please submit an issue on https://github.com/limeyteam/nit/issues");

        let directory = String::from_utf8_lossy(&pwd.stdout);
        println!("{}", directory);
        let mut fullpath = format!("{}\\7z.exe", &directory);
        let mut child =
        Command::new("cmd")
        .args(&["/C", "{}", fullpath.as_str()])
        .output()
        .expect("Le 7z has failed to do ze thing, please submit an issue on https://github.com/limeyteam/nit/issues");

        let up = String::from_utf8_lossy(&child.stdout);
        println!("{}", fullpath);
    }
}
