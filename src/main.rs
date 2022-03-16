use serde_json::Value;
use std::fs;
use std::io::{BufReader, Read};
use walkdir::WalkDir;

fn main() {
    let path = "";
    let contents = get_contents(&path);

    let deps = get_dependencies(&path);

    for dep in deps {
        if !contents.contains(&dep) {
            println!("({}) dep NOT found", dep);
        }
    }
}

fn get_contents(path: &str) -> String {
    let mut contents = String::from("");

    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();

        if f_name.contains(".ts") || f_name.contains(".tsx") {
            let file = fs::File::open(entry.path()).expect("Failed to open file");

            let mut file = BufReader::new(file);

            let mut lines = String::new();

            file.read_to_string(&mut lines)
                .expect("Failed to read file to string");

            contents += lines.as_str();
        }
    }

    contents
}

fn get_dependencies(path: &str) -> Vec<String> {
    let mut deps = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let f_name = entry.file_name().to_string_lossy();

        if f_name.contains("package.json") {
            let file = fs::File::open(entry.path()).expect("Failed to open file");
            let mut data = String::new();

            let mut file = BufReader::new(file);

            file.read_to_string(&mut data)
                .expect("Failed to read to string");

            let v: Value = serde_json::from_str(data.as_str()).expect("Failed to parse json");

            for (key, _) in v["dependencies"]
                .as_object()
                .expect("Failed to get as object")
            {
                deps.push(key.to_owned());
            }
        }
    }

    deps
}
