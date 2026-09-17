package main

import (
	"fmt"
	"net/http"

	"github.com/go-chi/chi"
	"github.com/jabbott-iii/golang-practice/goapi-sqlite/internal"
	log "github.com/sirupsen/logrus"
)

func main() {

	fmt.Println("Starting server...")
	_, err := internal.NewDatabase()
	if err != nil {
		log.Fatal(err)
	}

	log.SetReportCaller(true)
	var r *chi.Mux = chi.NewRouter()
	internal.Handler(r)

	fmt.Println("Starting Go API service.........")

	var err2 error = http.ListenAndServe("localhost:8000", r)
	if err2 != nil {
		log.Error(err2)
	}
}
