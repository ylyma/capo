use serde::{Deserialize, Serialize};
use std::{
    env::args,
    fs, io,
    io::Write,
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
};

static COUNTER: AtomicUsize = AtomicUsize::new(1);
const DATAFILEPATH: &str = "data.json";

#[derive(Deserialize, Serialize, Debug)]
struct Module {
    id: usize,
    name: String,
    grade: String,
    credits: String,
    semester: String,
    real: bool,
}

impl Module {
    pub fn real_new(name: String, grade: String, credits: String, semester: String) -> Module {
        Module {
            id: COUNTER.fetch_add(1, Ordering::SeqCst),
            name,
            grade,
            credits,
            semester,
            real: true,
        }
    }

    pub fn assume_new(name: String, grade: String, credits: String, semester: String) -> Module {
        Module {
            id: COUNTER.fetch_add(1, Ordering::SeqCst),
            name,
            grade,
            credits,
            semester,
            real: false,
        }
    }

    pub fn get_modules() -> Vec<Module> {
        let mut data = fs::read_to_string(DATAFILEPATH).expect("cannot read file");
        let last_char = data.pop();
        let second_last_char = data.pop();
        let data_array = format!("[ {} ]", data);
        let modules: Vec<Module> = serde_json::from_str(&data_array).expect("cannot get modules");
        modules
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    let command = &args[1];
    let params = &args[1..];
    println!("command: {}", command);
    match command.as_str() {
        "add" => execute_add(&params.to_vec()),
        "edit" => execute_edit(params.to_vec()),
        "delete" => execute_delete(params.to_vec()),
        "assume" => execute_assume(params.to_vec()),
        "sort" => execute_sort(params.to_vec()),
        "show" => execute_show(),
        _ => println!("invalid command, please input any of the following commands: \n add NAME GRADE CREDITS \n edit ID \n delete ID \n assume NAME GRADE CREDITS \n sort name/semester/grade/credits \n show"),
    }
}

pub fn execute_add(params: &Vec<String>) {
    let name: String = params[1].to_string();
    let grade: String = params[2].to_string();
    let credits: String = params[3].to_string();
    let semester: String = params[4].to_string();
    let module = Module::real_new(name, grade, credits, semester);
    let ending_comma: &str = ", ";
    let mut data = serde_json::to_string(&module).expect("cannot parse module");
    data.push_str(ending_comma);
    let mut f = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .append(true)
        .open(DATAFILEPATH)
        .unwrap();

    f.write_all(data.as_bytes()).expect("cannot write to file");
}

pub fn execute_edit(params: Vec<String>) {
    let modules: Vec<Module> = Module::get_modules();
    println!("module {}", modules[0].name);
}

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
