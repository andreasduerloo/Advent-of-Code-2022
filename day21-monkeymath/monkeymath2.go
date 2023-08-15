package main

import (
	"fmt"
	"os"
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

	for monkeymap["root"].value == 0 {
		knownvals = resolve(knownvals)
	}

	fmt.Println(monkeymap["root"].value)
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

func resolve(known []*monkey) []*monkey {
	m, kvals := dequeue(known)

	if m.parent.value == 0 { // Try to resolve the parent so we can add it to the queue
		if m.parent.children[0].value != 0 && m.parent.children[1].value != 0 { // Are both children resolved?
			switch m.parent.operation {
			case "+":
				m.parent.value = m.parent.children[0].value + m.parent.children[1].value
			case "-":
				m.parent.value = m.parent.children[0].value - m.parent.children[1].value
			case "*":
				m.parent.value = m.parent.children[0].value * m.parent.children[1].value
			case "/":
				m.parent.value = m.parent.children[0].value / m.parent.children[1].value
			}

			kvals = append(kvals, m.parent)
		} else { // The other child is not resolved yet, back to the queue.
			kvals = append(kvals, m)
		}
	}
	return kvals
}

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
