package main

import (
	"fmt"
	"os"
	"strings"
)

/*
What we're doing different this time:
- Instead of tracking the pressure per minute of a state, we will immediately add all the pressure a valve will release over the remaining time when we open it.
- Paths are explored until either:
  - All valves with a flow rate are open, or
  - We hit 30 minutes

That means we have to make a few changes:
- We have to identify which valves have a flowrate other than zero
- States no longer track ppm (pressure per minute), but they do track time
*/

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
}
