package internal

import (
	"github.com/go-chi/chi"
	chimiddle "github.com/go-chi/chi/middleware"
)

// capital H identifies it can be used in other packages
func Handler(r *chi.Mux) {
	// global middleware
	r.Use(chimiddle.StripSlashes)

	r.Route("/account", func(router chi.Router) {
		// middleware for /account route
		//router.Use(Authorization)

		router.Get("/coins", GetCoinBalance)
		router.Put("/coins", UpdateCoinBalance)

		router.Post("/user", CreateUserAccount)
		router.Get("/user", GetUserDetails)
	})
}
