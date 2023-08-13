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
	monkeyslice := make([]*monkey)

	for _, c := range monkeymap[root].cstr {
		monkeymap[root].children = append(monkeymap[root].children, monkeymap[c])

		monkeymap[c].parent = "root"
	}
	delete(monkeys, "root")

	monkeyslice = monkeyslice.append[monkeymap["root"]]

	for len(monkeys) != 0 {
		//
	}

}

type monkey struct {
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
				value: val,
			}

			(*m)[elems[0][:len(elems[0])-1]] = &newm
		case 4:
			newm := monkey{
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
