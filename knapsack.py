import sys

def knapsack(entries, capacity, indent=0):
  max_weight = 0
  max_content = []
  for entry in entries:
    weight, content = knapsack([x for x in entries if x != entry], capacity - entry, indent + 1)
    total_weight = entry + weight
    if total_weight <= capacity and max_weight < total_weight:
      max_weight = total_weight
      max_content = content + [entry]
  return max_weight, max_content


if __name__ == '__main__':
  if (len(sys.argv) != 2):
    print('usage: python knapsack.py inputfile')
    sys.exit(1)
  with open(sys.argv[1], 'r') as ifile:
    capacity, *entries = [int(x) for x in ifile.readline().split(' ')]
    print(knapsack([x for x in entries if x <= capacity], capacity))
