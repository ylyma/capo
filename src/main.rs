use serde::{Deserialize, Serialize};
use std::{
    env::args,
    fs, io,
    sync::atomic::{AtomicUsize, Ordering},
};

static COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Deserialize, Serialize, Debug)]
struct Module {
    id: usize,
    name: String,
    grade: String,
    credits: u32,
    semester: String,
    real: bool,
}

impl Module {
    pub fn real_new(name: String, grade: String, credits: u32, semester: String) -> Module {
        Module {
            id: COUNTER.fetch_add(1, Ordering::SeqCst),
            name,
            grade,
            credits,
            semester,
            real: true,
        }
    }

    pub fn assume_new(name: String, grade: String, credits: u32, semester: String) -> Module {
        Module {
            id: COUNTER.fetch_add(1, Ordering::SeqCst),
            name,
            grade,
            credits,
            semester,
            real: false,
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    let command = &args[1];
    let params = &args[1..];
    println!("command: {}", command);
    match command.as_str() {
        "add" => execute_add(params.to_vec()),
        "edit" => execute_edit(params.to_vec()),
        "delete" => execute_delete(params.to_vec()),
        "assume" => execute_assume(params.to_vec()),
        "sort" => execute_sort(params.to_vec()),
        "show" => execute_show(),
        _ => println!("invalid command, please input any of the following commands: \n add NAME GRADE CREDITS \n edit ID \n delete ID \n assume NAME GRADE CREDITS \n sort name/semester/grade/credits \n show"),
    }
}

pub fn execute_add(params: Vec<String>) {}

pub fn execute_edit(params: Vec<String>) {}

pub fn execute_delete(params: Vec<String>) {}

pub fn execute_assume(params: Vec<String>) {}

pub fn execute_sort(params: Vec<String>) {}

pub fn execute_show() {}

pub fn save_file() -> Result<(), io::Error> {
    let output_path = "../data/data.json";
    let data = {
        let data = String::new();
        serde_json::from_str::<Module>(&data).unwrap();
    };
    fs::write(output_path, serde_json::to_string_pretty(&data).unwrap())?;
    Ok(())
}
