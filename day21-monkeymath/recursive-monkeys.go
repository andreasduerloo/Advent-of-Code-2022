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
	for _, v := range monkeymap {
		v.children[0] = monkeymap[v.cstr[0]]
		// monkeymap[v.cstr[0]].parent = v

		v.children[1] = monkeymap[v.cstr[1]]
		// monkeymap[v.cstr[1]].parent = v
	}

	fmt.Println(solve(monkeymap["root"]))
}

type monkey struct {
	value    int
	cstr     [2]string
	children [2]*monkey
	// parent    *monkey
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

func solve(m *monkey) int {
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
