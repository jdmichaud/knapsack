#!/usr/bin/env python
import sys
from collections import namedtuple
from solve import read_file, score

CacheConfiguration = namedtuple('CacheConfiguration', ['cache_id', 'videos'])

def score_endpoints(endpoints, requests, cache_configuration):
  return int(sum([score(endpoint, requests, cache_configuration) for endpoint in endpoints]) / sum([nbr for (_, _, nbr) in requests]) * 1000)

if __name__ == '__main__':
  if (len(sys.argv) != 3):
    print('usage: ./score.py inputfile outputfile')
    sys.exit(1)
  with open(sys.argv[1], 'r') as ifile:
    ((V, E, R, C, Csize), vsizes, endpoints, requests, C, Csize) = read_file(ifile)
    with open(sys.argv[2], 'r') as ofile:
      nbcache = int(ofile.readline())
      cache_configuration = []
      for i in range(nbcache):
        l = ofile.readline().split(' ')
        if (len(l) > 1):
          cid, *videos = [int(x) for x in l]
          cache_configuration.append((cid, videos))
      print(score_endpoints(endpoints, requests, cache_configuration))
