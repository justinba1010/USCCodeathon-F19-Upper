# City Building

The Country of Prim is asking it's brightest people how much road they need to build
so that every city is connected, but in the least amount of road possible. Note, they do
not want to build a road between every city. Instead they want to build a
network of roads, so that it is possible to get from any city to any other city.
The country is on a budget, so they previously hired a Kruskal-ese Engineer, but
he turned out to be a spy, and too complex. Instead they want the shorted
network that makes every city connected.

## Description

Given a network of cities and roads, pick the cheapest set of roads so that
every city is connected to each other.

## Input

The first line contains an integer $$n$$ and $$x$$, the number of cities, and
the number of city to city roadsites that are accounted for.  
Each line after will have two numbers denoting which two cities are being connected, and a length of road.

```
n x
a_1 a_2 cost
a_1 a_2 cost
...
```

## Constraints

$$0 \leq n \leq 10^3$$  
$$0 \leq x \leq 10^6$$  
$$0 \leq a_i \leq n$$  
$$0 \leq cost \leq 10^3$$

## Output

Print the cost of the shortest interconnected network.
