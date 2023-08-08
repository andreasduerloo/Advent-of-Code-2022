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

	// Parsing input
	valves := make(map[string]*valve)

	for _, line := range lines {
		if line != "" {
			valves[strings.Split(line, " ")[1]] = newvalve(line)
		}
	}

	// Populate neighbor pointers
	for _, v := range valves {
		for _, nb := range v.neighborsstr {
			v.neighbors = append(v.neighbors, valves[nb])
		}
	}

	for k, v := range valves {
		fmt.Println(k, v)
	}

	// Set the initial state
	start := state{
		time:     0,
		location: "AA",
		moving:   false,
		visited:  []string{"AA"},
		pressure: 0,
		parent:   nil,
	}

	var statetree []*state

	statetree = append(statetree, &start)

	// Keep running over the tree, creating a new one
}

type valve struct {
	name         string
	flowrate     int
	neighborsstr []string
	neighbors    []*valve
}

func newvalve(line string) *valve {
	elements := strings.Split(line, " ")
	flow, _ := strconv.Atoi(elements[4][5 : len(elements[4])-1])
	neighborscomma := elements[9:]

	neighbors := make([]string, 0)

	for _, nb := range neighborscomma {
		if rune(nb[len(nb)-1]) == ',' {
			neighbors = append(neighbors, string(nb[0:len(nb)-1]))
		} else {
			neighbors = append(neighbors, nb)
		}
	}

	out := valve{
		name:         elements[1],
		flowrate:     flow,
		neighborsstr: neighbors,
		neighbors:    make([]*valve, 0),
	}

	return &out
}

type state struct {
	time     int
	location string
	moving   bool
	visited  []string
	pressure int
	parent   *state
}

func pulse(states []*state) []*state {
	//
}
