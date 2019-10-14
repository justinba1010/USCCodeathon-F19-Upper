# Description

Hackers are trying to disrupt the USC computer network! Each computer in the network has a bidirectional link to one or more other computers in the network. Any two computers in the network may communicate if there is some path of links that connect the first computer to the second (this path can go through other computers in the network as well).

The hackers are capable of completely disabling a single computer in the network. Your goal is to identify weak points in the network in order to deploy countermeasures to minimize the risk of disruption. Let $x$, $a$, and $b$ be any three computers in the network. $x$ is a weak point of the network if all communication from $a$ to $b$ must pass through $x$. In other words, a weak point is some computer that, if disabled, would prevent one or more pairs of computers from being able to communicate.

# Input Format

The first line of input is the number of test cases, $T$.
$T$ test cases follow, each following this format:

The first line of a test case is $N$, the number of computers in the network.

The next $N$ lines are a space-delineated series of numbers. The first number in the line, $n$, is the numeric ID of some computer in the network, and the remaining numbers on the line indicate the IDs of the computers to which $n$ has a direct link.

There are $N$ computers in the network, and each computer has a numeric ID in the range $[0, N)$. These are always given in increasing order.

# Constraints

$2 \leq T \leq 10$

$3 \leq N \leq 2^{16}$

# Output Format

$T$ lines, each line containing a space delineated list of numbers giving the indices of the weak points in the network for that test case. If a network has no weak points, this should be a blank line.
