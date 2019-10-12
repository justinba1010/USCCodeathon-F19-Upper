import random
n = random.randint(400, 500)

print(n)
for i in range(2*n):
    g1pairs = []
    for j in range(n):
        x = random.randint(0,10000)
        while x in g1pairs:
            x = random.randint(0,10000)
        g1pairs.append(x)
    print(" ".join(map(str, g1pairs)))
