use serde::{Deserialize, Serialize};
use std::env::args;

#[derive(Deserialize, Serialize, Debug)]
struct Module {
    id: u32,
    name: String,
    grade: Grade,
    credits: u32,
    real: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Grade {
    Aplus,
    A,
    Bplus,
    B,
    Bminus,
    Cplus,
    C,
    Dplus,
    D,
    F,
}

fn main() {
    let args: Vec<String> = args().collect();

    let command = &args[1];
    let params = &args[1..];
    println!("command: {}", command);
    match command.as_str() {
        "add" => {
            execute_add(params.to_vec());
        }
        "edit" => {
            execute_edit(params.to_vec());
        }
        "delete" => {
            execute_delete(params.to_vec());
        }
        "assume" => {
            execute_assume(params.to_vec());
        }
        "sort" => {
            execute_sort(params.to_vec());
        }
        "show" => {
            execute_show();
        }
        _ => {
            println!("invalid command, please input any of the following commands: \n add NAME GRADE CREDITS \n edit ID \n delete ID \n assume NAME GRADE CREDITS \n sort name/semester/grade/credits \n show");
        }
    }
}

pub fn parse_grade(grade: String) -> Grade {
    Grade::A
}

pub fn execute_add(params: Vec<String>) {}

pub fn execute_edit(params: Vec<String>) {}

pub fn execute_delete(params: Vec<String>) {}

pub fn execute_assume(params: Vec<String>) {}

pub fn execute_sort(params: Vec<String>) {}

pub fn execute_show() {}

pub fn save_file() {}
