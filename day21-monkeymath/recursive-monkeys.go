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

	// Populate the pointers
	for _, v := range monkeymap {
		if v.value == 0 { // Check not needed for first star
			v.children[0] = monkeymap[v.cstr[0]]
			v.children[1] = monkeymap[v.cstr[1]]

			// Second star
			monkeymap[v.cstr[0]].parent = v
			monkeymap[v.cstr[1]].parent = v
		}
	}

	fmt.Println(solve(monkeymap["root"]))

	// Second star
	// Mark all monkeys that depend on the value of "humn"

	m := monkeymap["humn"]
	m.humnbranch = true
	for !monkeymap["root"].humnbranch {
		m = m.parent
		m.humnbranch = true
	}

	makeEqual(monkeymap["root"])
}

type monkey struct {
	value     int
	cstr      [2]string
	children  [2]*monkey
	operation string

	// Second star
	humnbranch bool
	parent     *monkey
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

func solve(m *monkey) int { // RECURSION! Parallelize?
	if m.value != 0 {
		return m.value
	} else {
		switch m.operation {
		case "+":
			m.value = solve(m.children[0]) + solve(m.children[1])
		case "-":
			m.value = solve(m.children[0]) - solve(m.children[1])
		case "*":
			m.value = solve(m.children[0]) * solve(m.children[1])
		case "/":
			m.value = solve(m.children[0]) / solve(m.children[1])
		}

		return m.value
	}
}

func makeEqual(m *monkey) { // We have to go down, at each step we know what the value of m should be
	if m.parent == nil { // This is "root". Set the value of the "humn" branch to that of the other branch
		if m.children[0].humnbranch { // The left child is an ancestor of "humn"
			m.children[0].value = m.children[1].value
			makeEqual(m.children[0])
		} else { // The right one is
			m.children[0].value = m.children[1].value
			makeEqual(m.children[1])
		}

	} else if m.children[0] == nil { // This is "humn"
		fmt.Println(m.value)

	} else { // Work out what the value of the "humn" child should be, so that this monkey's value is correct
		var ancestor *monkey
		var othermonkey *monkey
		if m.children[0].humnbranch { // The left child is an ancestor of "humn"
			ancestor = m.children[0]
			othermonkey = m.children[1]
		} else { // The right one is
			ancestor = m.children[1]
			othermonkey = m.children[0]
		}

		switch m.operation {
		case "+":
			ancestor.value = m.value - othermonkey.value
		case "-": // Order matters
			if m.value > othermonkey.value {
				ancestor.value = m.value + othermonkey.value
			} else {
				ancestor.value = othermonkey.value - m.value
			}
		case "*":
			ancestor.value = m.value / othermonkey.value
		case "/": // Order matters
			if m.value > othermonkey.value {
				ancestor.value = m.value * othermonkey.value
			} else {
				ancestor.value = othermonkey.value / m.value
			}
		}

		makeEqual(ancestor)
	}
}
