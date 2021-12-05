use std::fs;
use biblatex::Bibliography;

pub fn read_biblatex_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect(format!("Cannot read file {}", file_path).as_str());

    println!("{}", contents);

    contents
}

pub fn write_biblatex_file(file_path: &str, content: &String) -> std::io::Result<()> {
    fs::write(file_path, content)
}

pub fn read_bibliography_from_file(file_path: &str) -> Bibliography {
    let src = read_biblatex_file(file_path);
    let mut bibliography = Bibliography::parse(&src).unwrap();
    bibliography
}
