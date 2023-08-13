/*
	STATUS: tree data structure works for the example input, but not for the full input.
	This probably means the tree is extremely unbalanced.

	Root was a ruse.

	New idea:
	- Work with pointers
	- Put all the pointers in a big old slice (which will have a length of len(monkeys))
*/

package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	var lines []string
	input, err := os.ReadFile("./exampleinput.txt")
	if err != nil {
		fmt.Println("Error:", err)
	} else {
		lines = strings.Split(string(input), "\n")
	}

	monkeymap := make(map[string]*monkey) // Is indirection necessary here? Might be slower.

	for _, line := range lines {
		parse(line, &monkeymap)
	}

	// Put all the monkeys in a big ol' slice (from the top down) and populate the pointers
	monkeyslice := make([]*monkey, 0)

	for i, c := range monkeymap["root"].cstr {
		monkeymap["root"].children[i] = monkeymap[c]

		monkeymap[c].parent = monkeymap["root"]
	}
	monkeyslice = append(monkeyslice, monkeymap["root"])
	delete(monkeymap, "root")

	// This part needs work
	index := 0
	for len(monkeymap) != 0 {
		if monkeyslice[index].value == 0 { // This monkey has children
			for _, c := range monkeyslice[index].cstr { // For each child...
				if monkeymap[c].value == 0 {
					for i, g := range monkeymap[c].cstr { // Set the grandchildren's children and parent
						monkeymap[c].children[i] = monkeymap[g]
						monkeymap[g].parent = monkeymap[c]
					}
					monkeyslice = append(monkeyslice, monkeymap[c])
					delete(monkeymap, c)

					index += 1 // This doesn't go here
				}
			}
		}
	}

}

type monkey struct {
	//name      string
	value     int
	cstr      [2]string
	children  [2]*monkey
	parent    *monkey
	operation string
}

func parse(l string, m *map[string]*monkey) { // Map is a reference type, we don't actually have to pass a pointer.
	if l != "" {
		elems := strings.Split(l, " ")

		switch len(elems) {
		case 2:
			val, _ := strconv.Atoi(elems[1])
			newm := monkey{
				//name: elems[0][:4],
				value: val,
			}

			(*m)[elems[0][:len(elems[0])-1]] = &newm
		case 4:
			newm := monkey{
				//name: elems[0][:4],
				cstr:      [2]string{elems[1], elems[3]},
				operation: elems[2],
			}

			(*m)[elems[0][:len(elems[0])-1]] = &newm
		}
	}
}

func resolve(t []*monkey) int {
	index := len(t) - 1

	for t[1].value == 0 {
		if t[index] == nil {
			index -= 1
		} else { // We found a value
			switch t[index/2].operation {
			case "+":
				t[index/2].value = t[index].value + t[index-1].value
			case "-":
				t[index/2].value = t[index-1].value - t[index].value
			case "*":
				t[index/2].value = t[index].value * t[index-1].value
			case "/":
				t[index/2].value = t[index-1].value / t[index].value
			}

			index = index - 2
		}

	}

	return t[1].value
}
