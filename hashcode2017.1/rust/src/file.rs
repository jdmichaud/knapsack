use std::fs::File;
use std::io::Write;
use std::io::Result;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

use optimize::CacheConfiguration;

pub struct Parameters {
  nb_video: i32,
  nb_endpoint: i32,
  nb_request: i32,
  nb_cache: i32,
  cache_size: i32,
}

pub struct Request {
  endpoint_id: i32,
  video_id: i32,
  nb_request: i32,
}

pub struct Endpoint {
  endpoint_id: i32,
  latency: i32,
  cache_latencies: HashMap<i32, i32>,
}

fn read_line<R: BufRead>(read: &mut R) -> Result<String> {
  let mut buf = String::new();
  // One of the few times I've had a use for Result::and()
  read.read_line(&mut buf).and(Ok(buf))
}

fn split_line(line: &str) -> Vec<i32> {
  line.split(' ')
    .collect::<Vec<&str>>().iter()
    .map(|s| s.trim().parse::<i32>().unwrap())
    .collect()
}

pub fn read_input_file(filename: &str) -> (Parameters, Vec<Endpoint>, Vec<Request>) {
  let mut file = BufReader::new(File::open(filename).expect("file not found"));
  let parameters = match split_line(&read_line(&mut file).unwrap()).as_slice() {
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
  let vsizes = split_line(&read_line(&mut file).unwrap());
  // Load endpoints
  let mut endpoints: Vec<Endpoint> = Vec::new();
  for endpoint_id in 0..parameters.nb_endpoint {
    endpoints.push(match split_line(&read_line(&mut file).unwrap()).as_slice() {
      &[latency, nb_cache] => {
        let mut cache_latencies = HashMap::new();
        for i in 0..nb_cache {
          match split_line(&read_line(&mut file).unwrap()).as_slice() {
            &[clatency, cache_id] => cache_latencies.insert(cache_id, clatency),
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
  (parameters, endpoints, requests)
}

pub fn write_output_file(filename: &str, cache_configuration: &Vec<CacheConfiguration>) -> () {
  let mut file = File::open(filename).expect("file not found");
  let non_empty = cache_configuration.iter()
    .filter(|cc| cc.videos.len() != 0).collect::<Vec<&CacheConfiguration>>();
  file.write_fmt(format_args!("{}", non_empty.len())).unwrap();
  non_empty.iter()
    .for_each(|cc|
      file.write_fmt(
        format_args!(
          "{} {}",
          cc.cache_id,
          cc.videos.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(" "))
      ).unwrap()
    );
  ()
}
