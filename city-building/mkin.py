import random
n = random.randint(10, 500)
E = []
for i in range(n):
    for j in range(i+1, n):
        E.append([i, j, random.randint(10,100)])
random.shuffle(E)

print(str(n) + " " + str(len(E)))
for edge in E:
    print(str(edge[0]) + " " + str(edge[1]) + " " + str(edge[2]))
