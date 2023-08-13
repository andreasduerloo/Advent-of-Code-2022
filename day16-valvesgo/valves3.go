package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

/*
What we're doing different this time:
- Instead of tracking the pressure per minute of a state, we will immediately add all the pressure a valve will release over the remaining time when we open it.
- Paths are explored until either:
  - All valves with a flow rate are open, or
  - We hit 30 minutes
- Once either of those conditions is met we report the pressure and don't create a new state (i.e. the branch ends)

That means we have to make a few changes:
- We have to identify which valves have a flowrate other than zero
- States no longer track ppm (pressure per minute), but they do track time
*/

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

	// Find the valves with a flow rate other than zero
	workingvalves := make([]string, 0)
	for _, v := range valves {
		if v.flowrate > 0 {
			workingvalves = append(workingvalves, v.name)
		}
	}

	// Create the initial state
	start := state{
		time:     1,
		location: valves["AA"],
		path:     []string{"AA"},
		open:     make([]string, 0),
		pressure: 0,
	}

	states := []*state{&start}
	highestpressure := 0

	for i := 1; i <= 30; i++ {
		states = pulse(states, &workingvalves, &highestpressure)
	}

	fmt.Println("Highest:", highestpressure)
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
	location *valve
	path     []string
	open     []string
	pressure int
}

func pulse(states []*state, wvalves *[]string, highest *int) []*state {
	out := make([]*state, 0)
	for _, s := range states {
		// Prune branches
		if s.time >= 15 && float64(s.pressure) <= float64(*highest)*float64(0.75) {
			continue
		}

		// Check if this branch can be stopped
		if s.time == 30 || len(s.open) == len(*wvalves) { // It can, do we have a new highest pressure?
			if s.pressure > *highest {
				*highest = s.pressure
			}
		} else { // Not yet, add more states
			// Is it worth opening this valve?
			if slices.Contains(*wvalves, s.location.name) && !slices.Contains(s.open, s.location.name) {
				newpath := make([]string, len(s.path))
				copy(newpath, s.path)

				newopen := make([]string, len(s.open))
				copy(newopen, s.open)

				newstate := state{
					time:     s.time + 1,
					location: s.location,
					path:     append(newpath, s.location.name+"*"),
					open:     append(newopen, s.location.name),
					pressure: s.pressure + (30-(s.time))*s.location.flowrate,
				}

				if newstate.pressure > *highest {
					*highest = newstate.pressure
				}
				out = append(out, &newstate)
			}

			// Add states for each neighbor, except the previous location
			for _, n := range s.location.neighbors {
				if s.time >= 2 {
					if n.name != s.path[len(s.path)-2] {
						newpath := make([]string, len(s.path))
						copy(newpath, s.path)

						newopen := make([]string, len(s.open))
						copy(newopen, s.open)

						newstate := state{
							time:     s.time + 1,
							location: n,
							path:     append(newpath, n.name),
							open:     newopen,
							pressure: s.pressure,
						}

						out = append(out, &newstate)
					}
				} else {
					if n.name != s.path[0] {
						newpath := make([]string, len(s.path))
						copy(newpath, s.path)

						newopen := make([]string, len(s.open))
						copy(newopen, s.open)

						newstate := state{
							time:     s.time + 1,
							location: n,
							path:     append(newpath, n.name),
							open:     newopen,
							pressure: s.pressure,
						}

						out = append(out, &newstate)
					}
				}
			}
		}
	}

	return out
}
