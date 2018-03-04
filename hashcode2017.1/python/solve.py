#!/usr/bin/env python
import sys
from functools import reduce
from collections import namedtuple
from unittest import TestCase

M = namedtuple('M', ['score', 'result'])
Request = namedtuple('Request', ['video_id', 'endpoint_id', 'nb'])
Endpoint = namedtuple('Endpoint', ['id', 'dclatency', 'clatencies'])
CacheConfiguration = namedtuple('CacheConfiguration', ['cache_id', 'videos'])

def assertEqual(lhs, rhs): TestCase().assertEqual(lhs, rhs)

# Return the requests for that endpoint
def requests_for_endpoint(requests, endpoint):
  return [Request(vid, eid, nbr) for (vid, eid, nbr) in requests if eid == endpoint.id]

# Filter out requests that are already satisfied (video not already cached)
def unsatisfied_requests(requests, cached_videos):
  return [Request(vid, eid, nbr) for (vid, eid, nbr) in requests if not vid in cached_videos]

def test_unsatisfied_requests():
  r = unsatisfied_requests(
    [Request(0, 0, 10), Request(1, 0, 10), Request(2, 1, 10)],
    [1]
  )
  assertEqual(r, [Request(0, 0, 10), Request(2, 1, 10)])

# Score a particular endpoint
# If videos are seen from this endpoint and the videos are in the cache
# the score is time saved by streaming from the caches rather than the datacenter
def score(endpoint, requests, caches):
  score = 0
  # Filter on the requests from this endpoint
  for (video, eid, nbr) in requests_for_endpoint(requests, endpoint):
    # Filter on the cache which stores the requested video
    caches_with_video = [cid for (cid, videos) in caches if video in videos]
    # Get the latencies of those caches and sort them
    video_latencies = sorted([endpoint.clatencies[cid] for cid in endpoint.clatencies.keys() if cid in caches_with_video])
    if video_latencies:
      # If the video is stored in a cache attached to this point, compute the score
      score += (endpoint.dclatency - video_latencies[0]) * nbr
  return score

def test_score():
  s = score(Endpoint(0, 1000, { 0: 500 }), [Request(3, 0, 10)], [(0, [3])])
  assertEqual(s, (1000 - 500) * 10)

def valid(Csize, vsizes, caches):
  def add(vsizes, videos):
    return reduce(lambda x, video: x + vsizes[video], videos, 0)
  return not [size for size in [add(vsizes, videos) for (cid, videos) in caches] if size > Csize]

def knapsack(endpoint, requests, C, Csize, vsizes, cid):
  if (Csize <= 0 or not requests): return M(0, [])
  # Return the size of the video from request i - 1
  def w(i): return vsizes[requests[i - 1].video_id]
  # Return the score if we store video from request i - 1 in the cache
  def v(i): 
    if not i in v.score:
      v.score[i] = score(endpoint, requests, [(cid, [requests[i - 1].video_id])])
    return v.score[i]
  v.score = {}

  m = [[M(0, []) for j in range(Csize)] for i in range(2)]
  total = len(requests) * Csize
  count = 0
  for i in range(1, len(requests) + 1):
    for j in range(Csize):
      count += 1
      if (total / 100 == count):
        print('.', end='', flush=True)
        count = 0
      if (w(i) > j):
        m[1][j] = m[0][j]
      else:
        # m[1][j] = max(m[0][j].score, m[0][j - 1].score + v(i))
        score_with_video_i = m[0][j - 1].score + v(i)
        if (score_with_video_i > m[0][j].score):
          m[1][j] = M(score_with_video_i, m[0][j - 1].result + [requests[i - 1].video_id])
        else:
          m[1][j] = m[0][j]
    m[0] = m[1]
    m[1] = [M(0, []) for j in range(Csize)]

  flat_m = [item for sublist in m for item in sublist]
  print('')
  return max(flat_m, key=lambda x: x.score)

def solve(endpoints, requests, C, Csize, vsizes):
  # For each endpoint
  cache_configurations = {}
  for endpoint in endpoints:
    # Order cache by latency to the endpoint
    caches = sorted([(cid, endpoint.clatencies[cid]) for cid in endpoint.clatencies.keys()],
                    key=lambda clatency: clatency[1])
    for cache in caches:
      if not cache[0] in cache_configurations: cache_configurations[cache[0]] = []
      # Get already cached video by flattening the configuration
      cached_videos = [videos for conf in list(cache_configurations.values()) for videos in conf]
      remaining_requests = unsatisfied_requests(requests_for_endpoint(requests, endpoint), cached_videos)
      remanining_capacity = Csize - sum([vsizes[video] for video in cache_configurations[cache[0]]])
      # Solve the knapsack starting with the quickest cache
      m = knapsack(endpoint, remaining_requests, C, remanining_capacity, vsizes, cache[0])
      cache_configurations[cache[0]] += m.result
  return cache_configurations

def print_cache_config(cache_configurations, file):
  nb_used_cache = len([cache for cache in cache_configurations.keys() if cache_configurations[cache]])
  file.write(f'{nb_used_cache}')
  for cid in sorted(cache_configurations.keys()):
    videos = ' '.join([str(v) for v in cache_configurations[cid]])
    if (videos):
      file.write(f'\n{cid} {videos}')

def read_file(ifile):
  V, E, R, C, Csize = [int(x) for x in ifile.readline().split(' ')]
  vsizes = [int(x) for x in ifile.readline().split(' ')]
  endpoints = []
  for i in range(E):
    clatencies = {}
    dclatency, ncacheconn = [int(x) for x in ifile.readline().split(' ')]
    for j in range(ncacheconn):
      cid, clatency = [int(x) for x in ifile.readline().split(' ')]
      if cid not in clatencies: clatencies[cid] = []
      clatencies[cid] = clatency
    endpoints.append(Endpoint(i, dclatency, clatencies))
  requests = []
  for i in range(R):
    vid, eid, nbr = [int(x) for x in ifile.readline().split(' ')]
    requests.append(Request(vid, eid, nbr))
  return ((V, E, R, C, Csize), vsizes, endpoints, requests, C, Csize)

if __name__ == '__main__':
  if (len(sys.argv) != 2):
    print('usage: ./solve.py inputfile')
    sys.exit(1)
  with open(sys.argv[1], 'r') as ifile:
    ((V, E, R, C, Csize), vsizes, endpoints, requests, C, Csize) = read_file(ifile)

    # print(knapsack(endpoints[0], requests_for_endpoint(requests, endpoints[0]), C, Csize, vsizes, 0))
    with open(f'{sys.argv[1].split(".in")[0]}.out', 'w') as ofile:
      print_cache_config(solve(endpoints, requests, C, Csize, vsizes), ofile)
    # print(solve(endpoints, requests, C, Csize, vsizes))