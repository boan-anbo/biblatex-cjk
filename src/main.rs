pub mod title;
mod models;
mod fsutil;
mod jieba;
mod name;

use std::borrow::BorrowMut;
use std::fmt::Debug;
use biblatex::{Bibliography, Chunk, ChunksExt, Entry};
use std::fs;
use crate::fsutil::read_write::{read_biblatex_file, write_biblatex_file};
use crate::title::switch_title::{read_entry_titles, switch_title_titleaddon, write_entry_titles};

fn main() {
    let input_path = r"test-latex/test.bib";
    let output_path = r"test-latex/test_output.bib";
    let src = read_biblatex_file(input_path);
    let mut bibliography = Bibliography::parse(&src).unwrap();
    let entry = bibliography.iter_mut().into_slice().first_mut().unwrap();


    let result_entry = switch_title_titleaddon(entry);
    let result_string = &result_entry.to_biblatex_string();

    println!("{}", result_string);

    write_biblatex_file(output_path, result_string);
}




