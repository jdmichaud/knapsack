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

fn usage() {
  println!("usage: ./solve inputfile");
}

fn main() {
  match env::args().nth(1) {
    Some(filename) => {
      let (parameters, endpoints, requests) = read_input_file(&filename);
      let output_filename = format!("{}.out",
        filename.split(".in").collect::<Vec<&str>>()[0]);
      write_output_file(&output_filename, &solve(parameters, endpoints, requests));
      ()
    }
    None => {
      println!("error: missing argument");
      usage();
    },
  };  
}
