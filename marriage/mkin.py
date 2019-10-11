import random
n = random.randint(0, 1000)

print(n)
for i in range(2*n):
    for j in range(n):
        print(random.randint(0, 100), end=" ")
    print("")
