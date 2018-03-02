import sys

def solve():
  pass

class Endpoint:
  def __init__(self, dclatency):
    self.dclatency = 0
    self.conn = []

if __name__ == '__main__':
  if (len(sys.argv) != 2):
    print('usage: python knapsack.py inputfile')
    sys.exit(1)
  with open(sys.argv[1], 'r') as ifile:
    V, E, R, C, Csize = [int(x) for x in ifile.readline().split(' ')]
    vsizes = [int(x) for x in ifile.readline().split(' ')]
    caches = {}
    endpoints = {}
    for i in range(E):
      endpoints[i] = []
      dclatency, ncacheconn = [int(x) for x in ifile.readline().split(' ')]
      endpoints[i] = dclatency
      for j in range(ncacheconn):
        cid, clatency = [int(x) for x in ifile.readline().split(' ')]
        if cid not in caches: caches[cid] = []
        caches[cid].append((i, clatency))
    requests = []
    for i in range(R):
      vid, eid, nbr = [int(x) for x in ifile.readline().split(' ')]
      requests.append((vid, eid, nbr))
