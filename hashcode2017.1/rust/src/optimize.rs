use file::Parameters;
use file::Endpoint;
use file::Request;

pub struct CacheConfiguration {
  pub cache_id: i32,
  pub videos: Vec<i32>,
}

fn knapsack(
  endpoint: Endpoint,
  requests: Vec<Request>,
  parameters: Parameters,
  cache_id: i32) -> CacheConfiguration {
  CacheConfiguration { cache_id: 0, videos: vec![] }
}

pub fn solve(
  parameters: Parameters,
  endpoints: Vec<Endpoint>,
  requests: Vec<Request>) -> Vec<CacheConfiguration> {
  let cache_configurations: Vec<CacheConfiguration> = Vec::new();



  return cache_configurations;
}