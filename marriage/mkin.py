import random
n = random.randint(400, 500)

print(n)
for i in range(2*n):
    for j in range(n):
        print(random.randint(0, 100), end=" ")
    print("")
