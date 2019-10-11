# Copyright 2019
# Justin Baum
n = int(input())
group1 = [list(map(int, input().split(" ")[:n])) for _ in range(n)]
group2 = [list(map(int, input().split(" ")[:n])) for _ in range(n)]
group1choice = [-1 for _ in range(n)]
while -1 in group1choice:
    g1 = group1choice.index(-1) # Get unmatched group 1
    g2 = group1[g1].index(max(group1[g1])) # Get highest ranking
    if g2 in group1choice:
        g1_other = group1choice.index(g2)
        if group2[g2][g1] > group2[g2][g1_other]: # Group 2 person likes this person more
            group1choice[g1_other] = -1
            group1[g1_other][g2] = -1
            group1choice[g1] = g2
        else:
            group1[g1][g2] = -1
        continue
    group1choice[g1] = g2
summy = 0
for (g1, g2) in enumerate(group1choice):
    summy += group1[g1][g2] + group2[g2][g1]
print(summy)
