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
	http.Handle("/contact/", handler.Contacthandler{})

	http.Handle("/css/", http.StripPrefix("/css/", http.FileServer(http.Dir("assets/css"))))
	http.Handle("/js/", http.StripPrefix("/js/", http.FileServer(http.Dir("assets/js"))))

	println("Running on http://localhost:8080")
	err := http.ListenAndServe(":8080", nil)

	if err != nil {
		println(err.Error())
	}
}
