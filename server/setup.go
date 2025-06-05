package server

import (
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"

	"spendlite-api/database"
	"spendlite-api/logger"
	"spendlite-api/routes"
)

func Configure() *echo.Echo {
	e := echo.New()
	logger.InitLogger(e)
	e.Use(middleware.Logger())
	// e.Use(middleware.Recover())
	db := database.InitDB()
	routes.SetupRoutes(e, db)
	return e
}
