/*
The plan:

1. Parse the input into a 'monkey' struct, put them all in a hashmap ([string]*monkey).
Fields:
- Number (0 if not known)
- Two children (strings or "")
- An operation (+, -, *, or /)
- Pointers to the children (see below)
- Pointer to the parent, assume one parent for every monkey /!\

2. Build a tree of monkeys (put them in a big array/slice?)

3. "Resolve" the tree from bottom to top.
- Grab the two last leaves of the tree
- Resolve their parent (set the number and )
- Remove the leaves from the tree (can also be done by tracking an index)

Children of monkey n are at 2n and 2n+1, parent of monkey n is at floor(n/2)
*/

package main
