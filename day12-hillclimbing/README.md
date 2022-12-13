# Breadth-first search

For this implementation I have decoupled the "grid" (nested arrays of tuples, each containing a char and a bool - the latter indicating if the node has been visited) from the co-ordinates (tuples of usizes). I keep track of the nodes' parents through a HashMap with both the keys and the values being tuples of usizes (coordinates).

I tried to do an implementation with a "Point" Struct, which contains:
- The row
- The column
- The value (char)
- Whether it's been visited (bool)
- The parent (pointer to a Point -> Option\<Box\<Point\>\>)

This poses an issue: we (or at least I) can't derive or implement 'Copy' for anything containing a box, which means I can't initialize my arrays in the beginning. It would be possible to work with a Vec\<Vec\<Point\>\> and then initialize that in a big loop, but that doesn't feel to clean (and would mean everything lives on the heap).

There has to be a clean way to do this, so I will come back to this challenge. I should probably look into using Rc rather than Box, for starters.