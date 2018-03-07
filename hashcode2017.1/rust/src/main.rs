#![feature(slice_patterns)]
#[allow(unused_variables)]
#[allow(dead_code)]
mod file;

#[allow(unused_variables)]
#[allow(dead_code)]
mod optimize;

use std::env;
use file::read_input_file;
use file::write_output_file;
use optimize::solve;

fn main() {
  env::args().skip(1).for_each(|filename| {
    if env::args().len() > 2 { println!("{}", filename) }
    let (parameters, vsizes, endpoints, requests) = read_input_file(&filename);
    let output_filename = format!("{}.out",
      filename.split(".in").collect::<Vec<&str>>()[0]);
    write_output_file(&output_filename, &solve(parameters, vsizes, endpoints, requests));
  });
}
