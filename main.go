package main

import (
	"spendlite-api/cmd"
)

func main() {
	// Initialize the API server
	cmd.Execute() // This will start the Cobra command line interface
}
