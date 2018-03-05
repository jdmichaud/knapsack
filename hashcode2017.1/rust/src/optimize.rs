use std::cmp::max;
use file::Parameters;
use file::Endpoint;
use file::Request;
use std::iter::FromIterator;
use std::collections::HashMap;

// Everywhere you see 'HEURISTIC !', a decision is made which affects the
// performance of the algorithm.

#[derive(Debug, Clone)]
pub struct CacheConfiguration {
  pub cache_id: usize,
  pub videos: Vec<usize>,
}

fn requests_for_endpoint(
  requests: &Vec<Request>,
  endpoint: &Endpoint) -> Vec<Request> {
  requests.iter()
    .filter(|r| r.endpoint_id == endpoint.endpoint_id)
    // TODO: I'd rather handle reference than cloning requests
    .cloned()
    .collect::<Vec<Request>>()
}

#[test]
fn test_requests_for_endpoint() {
  let requests = vec![
    Request { endpoint_id: 0, video_id: 1, nb_request: 10 },
    Request { endpoint_id: 2, video_id: 3, nb_request: 20 },
    Request { endpoint_id: 4, video_id: 5, nb_request: 30 },
  ];
  let endpoint = Endpoint { endpoint_id: 2, latency: 500, cache_latencies: HashMap::new() };
  assert_eq!(requests_for_endpoint(&requests, &endpoint), vec![
    Request { endpoint_id: 2, video_id: 3, nb_request: 20 },
  ]);
}

fn unsatisfied_requests(
  requests: &Vec<Request>,
  cached_videos: &Vec<usize>) -> Vec<Request> {
  requests.iter()
    .filter(|&r| cached_videos.len() == 0 || cached_videos.iter().any(|&vid| vid != r.video_id))
    // TODO: I'd rather handle reference than cloning requests
    .cloned()
    .collect::<Vec<Request>>()
}

#[test]
fn test_unsatisfied_requests() {
  {
    let requests = vec![
      Request { endpoint_id: 0, video_id: 1, nb_request: 10 },
      Request { endpoint_id: 2, video_id: 3, nb_request: 20 },
      Request { endpoint_id: 2, video_id: 5, nb_request: 30 },
    ];
    let cached_videos = vec![5];
    assert_eq!(unsatisfied_requests(&requests, &cached_videos), vec![
      Request { endpoint_id: 0, video_id: 1, nb_request: 10 },
      Request { endpoint_id: 2, video_id: 3, nb_request: 20 },
    ]);
  }
  {
    let requests = vec![
      Request { endpoint_id: 0, video_id: 3, nb_request: 1500 },
      Request { endpoint_id: 1, video_id: 0, nb_request: 1000 },
      Request { endpoint_id: 0, video_id: 4, nb_request: 500 },
      Request { endpoint_id: 0, video_id: 1, nb_request: 1000 }
    ];
    let cached_videos = vec![];
    assert_eq!(unsatisfied_requests(&requests, &cached_videos), requests);
  }
}

pub fn score(
  requests: &Vec<Request>,
  endpoint: &Endpoint,
  caches: &Vec<CacheConfiguration>) -> usize {
  let mut score = 0;
  // Filter on the requests from this endpoint
  for request in requests.iter().filter(|r| r.endpoint_id == endpoint.endpoint_id) {
    // Filter on the cache which stores the requested video
    let mut lowest_latencies = caches.iter()
      .filter(|c| c.videos.iter().any(|&vid| vid == request.video_id))
      // map a cache with its latencies versus the endpoinf
      .map(|c| endpoint.cache_latencies.get(&c.cache_id).unwrap())
      .collect::<Vec<&usize>>();
      // Get the lowest latency
    lowest_latencies.sort_unstable();  
    if lowest_latencies.len() != 0 {
      // If the video is stored in a cache attached to this point, compute the score
      score += (endpoint.latency - lowest_latencies[0]) * request.nb_request;
    }
  }
  score
}

fn knapsack(
  requests: &Vec<Request>,
  endpoint: &Endpoint,
  parameters: &Parameters,
  vsizes: &Vec<usize>,
  cache_id: usize, capacity: usize) -> Vec<usize> {
  // TODO: Restrict the video size check only to requested video from this endpoind
  if capacity == 0 || requests.len() == 0 || vsizes.iter().filter(|&&s| s <= capacity).sum::<usize>() == 0 {
    return vec![];
  }
  // Return the weigh of video i - 1
  let w = |i: usize| -> usize { vsizes[requests[i - 1].video_id] };
  // Return what would be the score if we were caching video i - 1
  let v = |i: usize| -> usize {
    score(requests, endpoint,
      &vec![CacheConfiguration { cache_id, videos: vec![requests[i - 1].video_id] }])
  };

  let mut m = vec![vec![(0 as usize, vec![]); capacity]; 2];
  for i in 1..requests.len() + 1 {
    for j in 0..capacity {
      if w(i) > j {
        m[1][j].0 = m[0][j].0;
        m[1][j].1 = m[0][j].1.iter().cloned().collect();
      } else {
        let score_with_video = m[0][j - 1].0 + v(i);
        m[1][j].0 = max(m[0][j].0, score_with_video);
        m[1][j].1 = [&m[0][j - 1].1[..], &vec![requests[i - 1].video_id][..]].concat();
      }
    }
    m[0] = Vec::from_iter(m[1].iter().cloned());
    m[1] = vec![(0 as usize, vec![]); capacity];
  }

  m[0].iter().max_by_key(|x| x.0).unwrap().1.iter().cloned().collect()
}

pub fn solve(
  parameters: Parameters,
  vsizes: Vec<usize>,
  endpoints: Vec<Endpoint>,
  requests: Vec<Request>) -> Vec<CacheConfiguration> {
  let mut cache_configurations: Vec<CacheConfiguration> = Vec::new();
  // HEURISTIC !
  for endpoint in endpoints {
    // HEURISTIC !
    for (&cache_id, _) in endpoint.cache_latencies.iter() {
      // Retrieve already cached video
      let cached_video = cache_configurations.iter()
        .fold(Vec::new(), |mut acc, cc| {
          acc.extend(cc.videos.iter().cloned());
          acc
        });
      // Get the remaining requests on this endpoint which could be cached
      let remaining_requests = requests_for_endpoint(
        &unsatisfied_requests(&requests, &cached_video),
        &endpoint
      );
      // Compute the remaining capacity of this cache
      let remaining_capacity: usize = parameters.cache_size.saturating_sub(
        cached_video.iter().map(|v| vsizes[*v]).sum());
      let videos = knapsack(&remaining_requests, &endpoint, &parameters, &vsizes, cache_id, remaining_capacity);
      cache_configurations.push(CacheConfiguration { cache_id, videos });
    }
  }
  return cache_configurations;
}