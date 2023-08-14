/*
	STATUS: tree data structure works for the example input, but not for the full input.
	This probably means the tree is extremely unbalanced.

	Root was a ruse.

	New idea:
	- Work with pointers
	- Put all the pointers in a map
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

	// Populate the pointers and store the monkeys with a known value
	knownvals := make([]*monkey, 0)
	for _, v := range monkeymap {
		if v.value != 0 { // This monkey has no children
			knownvals = append(knownvals, v)
		} else {
			v.children[0] = monkeymap[v.cstr[0]]
			monkeymap[v.cstr[0]].parent = v

			v.children[1] = monkeymap[v.cstr[1]]
			monkeymap[v.cstr[1]].parent = v
		}
	}

	for k, v := range monkeymap {
		fmt.Println(k, v)
	}

	fmt.Println(knownvals)

	for monkeymap["root"].value == 0 {
		fmt.Println(knownvals) // This doesn't replicate to the next iteration of the loop -> Turn this into a function and return the queue TODO
		m, knownvals := dequeue(knownvals)
		fmt.Println(knownvals)
		fmt.Println(m.value)

		if m.parent.value == 0 { // Try to resolve this monkey (the other child might not be resolved yet)
			if m.parent.children[0].value != 0 && m.parent.children[1].value != 0 { // Resolve parent and add to queue
				fmt.Println(m.parent.children[0].value, m.parent.children[1].value)
				switch m.parent.operation {
				case "+":
					m.parent.value = m.parent.children[0].value + m.parent.children[1].value
				case "-":
					m.parent.value = m.parent.children[0].value - m.parent.children[1].value
				case "*":
					m.parent.value = m.parent.children[0].value + m.parent.children[1].value
				case "/":
					m.parent.value = m.parent.children[0].value / m.parent.children[1].value
				}

				knownvals = append(knownvals, m.parent)
			} else { // The other child is not resolved yet, put this one at the back of the queue
				knownvals = append(knownvals, m)
			}
		} else { // This monkey has already been resolved, move on.
			fmt.Println("Here")
			continue
		}
	}

	fmt.Println(monkeymap["Root"].value)
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

/*
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
*/

// We'll use a slice to implement a queue, we add by appending, so we just need a dequeue function
func dequeue(queue []*monkey) (*monkey, []*monkey) {
	out := queue[0]
	var outqueue []*monkey

	if len(queue) == 1 {
		outqueue = make([]*monkey, 0)
	} else {
		outqueue = queue[1:]
	}

	return out, outqueue
}
