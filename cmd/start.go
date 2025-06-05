package cmd

import (
	"fmt"

	"github.com/spf13/cobra"

	"spendlite-api/server"
)

var startCmd = &cobra.Command{
	Use:   "start",
	Short: "Start the Spendlite API server",
	Long:  `Start the Spendlite API server to handle requests for managing personal finances.`,
	Run: func(cmd *cobra.Command, args []string) {
		// Here you would typically start your server.
		// For example:
		srv := &server.Server{
			Host:       "localhost",
			Port:       "8080",
			LogLevel:   "info",
			LogFile:    "server.log",
			ConfigFile: "config.yaml",
		}
		if err := srv.Start(); err != nil {
			fmt.Println("Error starting server:", err)
		}
	},
}
