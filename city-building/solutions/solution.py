# Justin Baum
# Kruskals Algorithm for Codeathon 2019 Fall
# 240/350 Problem

def find(i, dj):
  while dj[i] != i:
    i = dj[i]
  return i

def union(i, j, dj):
  a = find(i, dj)
  b = find(j, dj)
  dj[a] = b

def kruskals(n, E):
  E.sort(key = lambda x: x[2])
  dist = 0
  dj = list(range(n))
  walk = []
  while len(walk) <= n - 2:
    leg = E[0]
    E = E[1:]
    if find(leg[0], dj) != find(leg[1], dj):
      union(leg[0], leg[1], dj)
      walk.append(leg)
  return sum(leg[2] for leg in walk)

x = input().split(" ")
n = int(x[0])
x = int(x[1])
E = []
for i in range(x):
  row = input().split(" ")
  E.append([int(row[0]), int(row[1]), int(row[2])])
print(kruskals(n, E))
