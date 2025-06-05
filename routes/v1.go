package routes

import (
	"spendlite-api/handlers"

	"github.com/labstack/echo/v4"
	"gorm.io/gorm"
)

func SetupV1Routes(e *echo.Echo, db *gorm.DB) {
	// e.POST("/auth/signup", handlers.UserCreationHandler)
	e.POST("/auth/login", handlers.UserLoginHandler)

	// e.Use()
	// e.GET("/auth/google/login", user.GoogleLoginRedirect)
	// e.GET("/auth/google/callback", user.GoogleCallbackHandler)

	// Protected
	// r := e.Group("/user")
	// r.Use(middleware.JWTMiddleware)
	// r.GET("/profile", controllers.UserProfile)
}
