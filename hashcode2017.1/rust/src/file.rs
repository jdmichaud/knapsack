use std::fs::File;
use std::io::Write;
use std::io::Result;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use std::fmt::Debug;
use std::collections::HashMap;

use optimize::CacheConfiguration;

#[derive(Debug, Clone, Copy)]
pub struct Parameters {
  pub nb_video: usize,
  pub nb_endpoint: usize,
  pub nb_request: usize,
  pub nb_cache: usize,
  pub cache_size: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Request {
  pub endpoint_id: usize,
  pub video_id: usize,
  pub nb_request: usize,
}

#[derive(Debug)]
pub struct Endpoint {
  pub endpoint_id: usize,
  pub latency: usize,
  pub cache_latencies: HashMap<usize, usize>,
}

fn read_line<R: BufRead>(read: &mut R) -> Result<String> {
  let mut buf = String::new();
  read.read_line(&mut buf).and(Ok(buf))
}

fn split_line<T>(line: &str) -> Vec<T> where T: FromStr, T::Err: Debug {
  line.split(' ')
    .collect::<Vec<&str>>().iter()
    .map(|s| s.trim().parse::<T>().unwrap())
    .collect()
}

pub fn read_input_file(filename: &str) -> (Parameters, Vec<usize>, Vec<Endpoint>, Vec<Request>) {
  let mut file = BufReader::new(File::open(filename).expect("file not found"));
  // Reading the first line (the parameters)
  let parameters = match split_line::<usize>(&read_line(&mut file).unwrap()).as_slice() {
    &[nb_video, nb_endpoint, nb_request, nb_cache, cache_size] =>
      Parameters { 
        nb_video: nb_video,
        nb_endpoint: nb_endpoint,
        nb_request: nb_request,
        nb_cache: nb_cache,
        cache_size: cache_size,
      },
    _ => unreachable!(),
  };
  // The list of video sizes
  let vsizes = split_line::<usize>(&read_line(&mut file).unwrap());
  // Load endpoints
  let mut endpoints: Vec<Endpoint> = Vec::new();
  for endpoint_id in 0..parameters.nb_endpoint {
    endpoints.push(match split_line(&read_line(&mut file).unwrap()).as_slice() {
      &[latency, nb_cache] => {
        let mut cache_latencies = HashMap::new();
        for i in 0..nb_cache {
          match split_line(&read_line(&mut file).unwrap()).as_slice() {
            &[cache_id, clatency] => cache_latencies.insert(cache_id, clatency),
            _ => unreachable!(),
          };
        }
        Endpoint { endpoint_id, latency, cache_latencies }
      }
      _ => unreachable!(),
    });
  }
  // Load requests
  let mut requests: Vec<Request> = Vec::new();
  for i in 0..parameters.nb_request {
    requests.push(match split_line(&read_line(&mut file).unwrap()).as_slice() {
      &[video_id, endpoint_id, nb_request] => Request { endpoint_id, video_id, nb_request },
      _ => unreachable!(),
    });
  }
  (parameters, vsizes, endpoints, requests)
}

pub fn write_output_file(filename: &str, cache_configuration: &Vec<CacheConfiguration>) -> () {
  let mut file = File::create(filename).expect("file not found");
  let non_empty = cache_configuration.iter()
    .filter(|cc| cc.videos.len() != 0).collect::<Vec<&CacheConfiguration>>();
  file.write_fmt(format_args!("{}\n", non_empty.len())).unwrap();
  non_empty.iter()
    .for_each(|cc|
      file.write_fmt(
        format_args!(
          "{} {}\n",
          cc.cache_id,
          cc.videos.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(" "))
      ).unwrap()
    );
  file.write(b"\n").unwrap();
  ()
}
