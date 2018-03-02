import sys
from functools import reduce

class Endpoint:
  def __init__(self, id, dclatency, clatencies):
    self.id = id
    self.dclatency = dclatency
    self.clatencies = clatencies

# Score a particular endpoint
# If videos are seen from this endpoint and the videos are in the cache
# the score is time saved by streaming from the caches rather than the datacenter
def score(endpoint, requests, caches):
  score = 0
  # FIlter on the requests from this endpoint
  for (video, eid, nbr) in [(vid, eid, nbr) for (vid, eid, nbr) in requests if eid == endpoint.id]:
    # Filter on the cache which stores the requested video
    caches_with_video = [cid for (cid, videos) in caches if video in videos]
    # Get the latencies of those caches and sort them
    video_latencies = sorted([latency for (cid, latency) in enumerate(endpoint.clatencies) if cid in caches_with_video])
    if (len(video_latencies) != 0):
      # If the video is stored in a cache attached to this point, compute the score
      score += (endpoint.dclatency - video_latencies[0]) * nbr
  return score

def valid(Csize, vsizes, caches):
  def add(vsizes, videos):
    return reduce(lambda x, video: x + vsizes[video], videos, 0)
  return not [size for size in [add(vsizes, videos) for (cid, videos) in caches] if size > Csize]

def score_endpoints(endpoints, requests, caches):
  return reduce(lambda x, y: x + y, [score(endpoint, requests, caches) for endpoint in endpoints])

def solve(endpoints, requests, C, Csize):
  caches = [(i, []) for i in range(C)]
  return caches

if __name__ == '__main__':
  if (len(sys.argv) != 2):
    print('usage: python knapsack.py inputfile')
    sys.exit(1)
  with open(sys.argv[1], 'r') as ifile:
    V, E, R, C, Csize = [int(x) for x in ifile.readline().split(' ')]
    vsizes = [int(x) for x in ifile.readline().split(' ')]
    clatencies = {}
    endpoints = []
    for i in range(E):
      dclatency, ncacheconn = [int(x) for x in ifile.readline().split(' ')]
      for j in range(ncacheconn):
        cid, clatency = [int(x) for x in ifile.readline().split(' ')]
        if cid not in clatencies: clatencies[cid] = []
        clatencies[cid] = clatency
      endpoints.append(Endpoint(i, dclatency, clatencies))
    requests = []
    for i in range(R):
      vid, eid, nbr = [int(x) for x in ifile.readline().split(' ')]
      requests.append((vid, eid, nbr))

    caches = solve(endpoints, requests, C, Csize)
    print(score_endpoints(endpoints, requests, caches))
