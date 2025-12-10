package main

import (
	"net/http"

	"github.com/jollodede/bewerbungs_tol/database"
	"github.com/jollodede/bewerbungs_tol/handler"
	_ "github.com/ncruces/go-sqlite3/driver"
	_ "github.com/ncruces/go-sqlite3/embed"
)

func init() {
	err := database.Init()

	if err != nil {
		panic("Failed to load DB: " + err.Error())
	}
}

func main() {
	defer database.DB.Close()

	http.HandleFunc("/", handler.IndexHandler)
	http.Handle("/firma/", handler.FirmaHandler{})
	http.HandleFunc("/contact/add", handler.ContactHandler)

	http.Handle("/css/", http.StripPrefix("/css/", http.FileServer(http.Dir("assets/css"))))

	println("Running on http://localhost:8080")
	http.ListenAndServe(":8080", nil)
}
