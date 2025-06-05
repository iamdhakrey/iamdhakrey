package logger

import (
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

func InitLogger(e *echo.Echo) {
	// Initialize the logger middleware
	e.Use(middleware.LoggerWithConfig(middleware.LoggerConfig{
		Format: "${time_rfc3339} ${status} ${method} ${remote_ip} ${uri} ${latency_human} ${user_agent}\n",
	}))

	// Optionally, you can add more configurations or middlewares here
	// e.Use(middleware.Recover())

}
