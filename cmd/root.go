package cmd

import "github.com/spf13/cobra"

var rootCmd = &cobra.Command{
	Use:   "spendliteapi",
	Short: "Spendlite API",
	Long:  `Spendlite API is a RESTful API for managing personal finances.`,
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		panic(err)
	}
}

func init() {
	// Add subcommands to the root command
	rootCmd.AddCommand(VersionCmd)
	rootCmd.AddCommand(startCmd)

	// You can add more commands here as needed
	// For example: rootCmd.AddCommand(stopCmd), restartCmd, etc.
}
