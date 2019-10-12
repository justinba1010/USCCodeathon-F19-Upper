import random
import numpy
n = random.randint(4, 5)

print(n)
totalmatrix = []
for i in range(2*n):
    g1pairs = []
    for j in range(n):
        x = random.randint(0,10000)
        # Transpose the matrix so we can check if any other group 1 has the same rating for a group 2 member
        transposetmatrix = numpy.transpose(totalmatrix)
        while x in g1pairs or len(totalmatrix) == 0 or x in transposetmatrix[j]:
            x = random.randint(0,10000)
        g1pairs.append(x)
    print(" ".join(map(str, g1pairs)))
