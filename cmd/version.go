package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var (
	// Version is the version of the application.
	Version = "1.0.0"

	// BuildTime is the time when the application was built.
	BuildTime = "unknown"

	// Commit is the commit hash of the application.
	// This should be set using -ldflags during the build process.
	// For example: go build -ldflags "-X 'spendliteapi/cmd.Commit=$(git rev-parse HEAD)'" ./cmd/
	// This will set the Commit variable to the current git commit hash.
	Commit = "unknown"

	// Branch is the branch name of the application.
	// This should be set using -ldflags during the build process.
	// For example: go build -ldflags "-X 'spendliteapi/cmd.Branch=$(git rev-parse --abbrev-ref HEAD)'" ./cmd/
	// This will set the Branch variable to the current git branch name.
	Branch = "unknown"
)
var VersionCmd = &cobra.Command{
	Use:   "version",
	Short: "Print the version number of storage",
	Long:  `All software has versions. This is storage's`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Printf("Storage Version: %s\n", Version)
		fmt.Printf("Build Time: %s\n", BuildTime)
		fmt.Printf("Commit: %s\n", Commit)
		fmt.Printf("Branch: %s\n", Branch)
	},
}
