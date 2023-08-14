/*
The plan:

1. Parse the input into a 'monkey' struct, put them all in a hashmap ([string]*monkey).
Fields:
- Number (0 if not known)
- Two children (strings or "")
- An operation (+, -, *, or /)
- Pointers to the children (see below)
- Pointer to the parent, assume one parent for every monkey /!\

We don't actually need pointers, just build the tree in an array/slice

2. Build a tree of monkeys (put them in a big array/slice?)

3. "Resolve" the tree from bottom to top.
- Grab the two last leaves of the tree
- Resolve their parent (set the number and )
- Remove the leaves from the tree (can also be done by tracking an index)

Children of monkey n are at 2n and 2n+1, parent of monkey n is at floor(n/2)
*/

/*
	STATUS: tree data structure works for the example input, but not for the full input.
	This probably means the tree is extremely unbalanced.

	Root was a ruse.

	New idea:
	- Work with pointers
	- Put all the pointers in a big old slice (which will have a length of len(monkeys))
*/

/*
- Tree in a slice doesn't work because the slice gets too big.
- Tree in a map doesn't work because the indexes get so big they wrap around.
- Back to pointers. Queue with items for which we know the value? Add to the back as we discover values.
- Grab the front item, if the parent's value is known, remove this item.
  - If the parent's value is not known, resolve it and add the parent to the back of the queue. Remove the item.
*/

package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	var lines []string
	input, err := os.ReadFile("./input.txt")
	if err != nil {
		fmt.Println("Error:", err)
	} else {
		lines = strings.Split(string(input), "\n")
	}

	monkeys := make(map[string]*monkey) // Is indirection necessary here? Might be slower.

	for _, line := range lines {
		parse(line, &monkeys)
	}

	// Build the tree from the top down

	fmt.Println("Building tree...")

	// tree := make([]*monkey, len(monkeys)*5) // We won't use index 1 so the tree math works. Grossly overprovision to avoid index out of bounds. Worst case: the right child always has two children.
	tree := make(map[int]*monkey)

	/*
		tree[1] = monkeys["root"] // Get rid of the index thing: array of used indexes, position to move through it
		delete(monkeys, "root")
		index := 1

		for len(monkeys) != 0 { // No guarantee that the slice will be 'packed'
			if tree[index] == nil {
				index += 1
			} else if tree[index].value == 0 {
				tree[2*index] = monkeys[tree[index].children[0]]
				delete(monkeys, tree[index].children[0])

				tree[2*index+1] = monkeys[tree[index].children[1]]
				delete(monkeys, tree[index].children[1])

				index += 1
			} else {
				index += 1
			}
		}
	*/

	indices := make([]int, 0)
	tree[1] = monkeys["root"]
	pos := 0
	indices = append(indices, 1)
	// delete(monkeys, "root")

	for len(monkeys) != 0 {
		val := indices[pos]
		// fmt.Println(indices)
		// fmt.Println("Val is now:", val)
		if tree[val].value == 0 { // Monkey with no known value
			tree[2*val] = monkeys[tree[val].children[0]]
			// delete(monkeys, tree[val].children[0])
			indices = append(indices, 2*val)

			tree[2*val+1] = monkeys[tree[val].children[1]]
			// delete(monkeys, tree[val].children[1])
			indices = append(indices, 2*val+1)

			delete(monkeys, tree[val].name)
			pos += 1
		} else { // Monkey with a known value
			delete(monkeys, tree[val].name)
			pos += 1
		}
	}

	fmt.Println("Tree built!\nResolving...")

	// Resolve the tree from the bottom up
	fmt.Println(resolve(tree))
}

type monkey struct {
	name      string
	value     int
	children  [2]string
	operation string
}

/*
type tree struct {
	lastindex: int
	content: []*monkey
}
*/

func parse(l string, m *map[string]*monkey) { // Map is a reference type, we don't actually have to pass a pointer.
	if l != "" {
		elems := strings.Split(l, " ")

		switch len(elems) {
		case 2:
			val, _ := strconv.Atoi(elems[1])
			newm := monkey{
				name:  elems[0][:4],
				value: val,
			}

			(*m)[elems[0][:len(elems[0])-1]] = &newm
		case 4:
			newm := monkey{
				name:      elems[0][:4],
				children:  [2]string{elems[1], elems[3]},
				operation: elems[2],
			}

			(*m)[elems[0][:len(elems[0])-1]] = &newm
		}
	}
}

// func resolve(t []*monkey) int {
func resolve(t map[int]*monkey) int { // The numbers get too big and wrap around
	// Put all indices in a slice
	indices := make([]int, 0)

	for k, _ := range t {
		indices = append(indices, k)
	}

	// Sort that slice - we could also build a max heap if this is too slow
	sort.Ints(indices)

	pos := len(indices) - 1

	for t[1].value == 0 {
		val := indices[pos]

		switch t[val/2].operation {
		case "+":
			t[val/2].value = t[val].value + t[val-1].value
		case "-":
			t[val/2].value = t[val-1].value - t[val].value
		case "*":
			t[val/2].value = t[val].value * t[val-1].value
		case "/":
			t[val/2].value = t[val-1].value / t[val].value
		}

		pos = pos - 2
	}

	return t[1].value
}
