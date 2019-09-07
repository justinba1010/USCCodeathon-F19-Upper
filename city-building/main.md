# City Building

The Country of Prim is asking it's brightest people how much road they need to build
so that every city is connected, but in the least amount of road possible. Note, they do
not want to build a road between every city. Instead they want to build a
network of roads, so that it is possible to get from any city to any other city.
The country is on a budget, so they previously hired a Kruskal-ese Engineer, but
his road system was too complex. Instead they want the shortest
network that makes every city connected. It is assumed the cost to go from a
city to itself is 0.

## Description

Given a network of cities and roads, pick the cheapest set of roads so that
every city is connected to each other.

## Input

The first line contains an integer $$n$$ and $$x$$, the number of cities, and
the number of city to city roadsites that are accounted for.  
Each line after will have two numbers denoting which two cities are being
connected, and the length of the road connecting those two cities.


```
n x
a_1 a_2 cost
a_1 a_2 cost
...
```

## Constraints

$$0 \leq n \leq 500$$  
$$0 \leq x \leq 250000$$  
$$0 \leq a_i \leq n$$  
$$0 \leq cost \leq 10^3$$

## Output

Print the cost of the shortest interconnected network.
