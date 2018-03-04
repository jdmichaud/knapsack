use file::Parameters;
use file::Endpoint;
use file::Request;

pub struct CacheConfiguration {
  pub cache_id: usize,
  pub videos: Vec<usize>,
}

fn requests_for_endpoint(
  requests: &Vec<Request>,
  endpoint: &Endpoint) -> Vec<Request> {
  Vec::new()
}

fn unsatisfied_requests(
  requests: &Vec<Request>,
  endpoint: &Endpoint,
  cached_videos: &Vec<usize>) -> Vec<Request> {
  Vec::new()
}

fn knapsack(
  requests: &Vec<Request>,
  endpoint: &Endpoint,
  parameters: &Parameters,
  cache_id: usize, capacity: usize) -> Vec<usize> {
  println!("knapsack!");
  Vec::new()
}

pub fn solve(
  parameters: Parameters,
  vsizes: Vec<usize>,
  endpoints: Vec<Endpoint>,
  requests: Vec<Request>) -> Vec<CacheConfiguration> {
  let cache_configurations: Vec<CacheConfiguration> = Vec::new();
  for endpoint in endpoints {
    for (&cache_id, _) in endpoint.cache_latencies.iter() {
      // Retrieve already cached video
      let cached_video = cache_configurations.iter()
        .fold(Vec::new(), |mut acc, cc| {
          acc.extend(cc.videos.iter().cloned());
          acc
        });
      // Get the remaining requests on this endpoint which could be cached
      let remaining_requests = requests_for_endpoint(
        &unsatisfied_requests(&requests, &endpoint, &cached_video),
        &endpoint
      );
      // Compute the remaining capacity of this cache
      let remaining_capacity: usize = parameters.cache_size.saturating_sub(
        cached_video.iter().map(|v| vsizes[*v]).sum());
      let videos = knapsack(&remaining_requests, &endpoint, &parameters, cache_id, remaining_capacity);
    }
  }
  return cache_configurations;
}