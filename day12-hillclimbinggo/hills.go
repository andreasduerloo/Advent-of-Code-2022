package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	input, _ := os.ReadFile("./input.txt")
	lines := strings.Split(string(input), "\n")

	grid, start, end := buildGrid(lines) // We only need the grid for the second star

	// First star
	fmt.Println(bfs(start, end))

	// Second star
	reset(grid)
	fmt.Println(descend(end))
}

type node struct {
	value     rune
	visited   bool
	neighbors []*node
	steps     int
}

func newNode(value rune) *node {
	out := node{
		value:     value,
		visited:   false,
		neighbors: make([]*node, 0),
		steps:     0,
	}

	return &out
}

func buildGrid(lines []string) ([4674]*node, *node, *node) {
	var graph [4674]*node
	var start *node
	var end *node

	for i, l := range lines {
		if l != "" {
			for j, r := range l {
				graph[114*i+j] = newNode(r)

				if r == 'S' {
					start = graph[114*i+j]
					start.value = 'a'
				} else if r == 'E' {
					end = graph[114*i+j]
					end.value = 'z'
				}
			}
		}
	}

	// Assign neighbor pointers
	for i, n := range graph {
		if i%114 != 0 { // Left edge
			n.neighbors = append(n.neighbors, graph[i-1])
		}
		if i%114 != 113 { // Right edge
			n.neighbors = append(n.neighbors, graph[i+1])
		}
		if i >= 114 { // Top edge
			n.neighbors = append(n.neighbors, graph[i-114])
		}
		if i < 4560 { // Bottom edge
			n.neighbors = append(n.neighbors, graph[i+114])
		}
	}

	return graph, start, end
}

func bfs(start *node, end *node) int {
	// Create a queue
	var queue []*node
	current := start
	current.visited = true

	for end.steps == 0 {
		for _, n := range current.neighbors {
			if n.value <= current.value+1 && !n.visited {
				n.visited = true
				n.steps = current.steps + 1
				queue = append(queue, n)
			}
		}

		if len(queue) > 0 {
			current, queue = dequeue(queue)
		}
	}

	return end.steps
}

func dequeue(queue []*node) (*node, []*node) {
	out := queue[0]
	var outqueue []*node

	if len(queue) == 1 {
		outqueue = make([]*node, 0)
	} else {
		outqueue = queue[1:]
	}

	return out, outqueue
}

func reset(grid [4674]*node) {
	for _, n := range grid {
		n.visited = false
		n.steps = 0
	}
}

func descend(start *node) int {
	// Create a queue
	var queue []*node
	current := start
	current.visited = true

	for current.value != 'a' {
		for _, n := range current.neighbors {
			if n.value >= current.value-1 && !n.visited {
				n.visited = true
				n.steps = current.steps + 1
				queue = append(queue, n)
			}
		}

		if len(queue) > 0 {
			current, queue = dequeue(queue)
		}
	}

	return current.steps
}
