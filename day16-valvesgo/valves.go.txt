package main

import (
	"fmt"
	"os"
	"slices"
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

	// Set the initial state
	start := state{
		location: valves["AA"],
		open:     make([]string, 0),
		pressure: 0,
		ppm:      0,
		parent:   "",
		path:     make([]string, 0),
	}

	states := []*state{&start}

	// Keep running over the states, creating a new slice with every pulse
	for i := 1; i <= 30; i++ {
		if i >= 25 { // PRUNE THE TREE
			states = pulse(prune(states))
		} else {
			states = pulse(states)
		}
	}

	fmt.Println("Highest:", highest(states))
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
	location *valve
	open     []string
	pressure int
	ppm      int
	parent   string
	path     []string
}

func pulse(states []*state) []*state {
	out := make([]*state, 0)

	for _, st := range states {
		for _, n := range st.location.neighbors { // Move to all neighbors, except right back to the previous one without opening the valve
			if st.parent != n.name {
				newst := state{
					location: n,
					open:     st.open,
					pressure: (st.pressure + st.ppm),
					ppm:      st.ppm,
					parent:   st.location.name,
					path:     append(st.path, n.name),
				}
				out = append(out, &newst)
			}
		}
		if st.location.flowrate > 0 && !slices.Contains(st.open, st.location.name) { // Is the valve closed and non-zero?
			newst := state{ // Open it up
				location: st.location,
				open:     append(st.open, st.location.name),
				pressure: (st.pressure + st.ppm),
				ppm:      (st.ppm + st.location.flowrate),
				parent:   st.location.name,
				path:     append(st.path, st.location.name+"*"),
			}

			out = append(out, &newst)
		}
	}
	return out
}

func highest(st []*state) int {
	max := 0
	open := make([]string, 0)
	path := make([]string, 0)
	ppm := 0

	for _, s := range st {
		if s.pressure >= max {
			max = s.pressure
			open = s.open
			ppm = s.ppm
			path = s.path
		}
	}

	fmt.Println(open, ppm, path)
	return max
}

func prune(st []*state) []*state {
	maxppm := 0
	out := make([]*state, 0)

	for _, s := range st {
		if s.ppm > maxppm {
			maxppm = s.ppm
		}
	}

	for _, s := range st {
		if s.ppm >= maxppm/2 { // Anything more than twice as fast halfway through is not likely to be overtaken
			out = append(out, s)
		}
	}

	return out
}
