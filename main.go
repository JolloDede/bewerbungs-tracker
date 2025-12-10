package main

import (
	"database/sql"
	"net/http"

	"github.com/jollodede/bewerbungs_tol/handler"
	_ "github.com/ncruces/go-sqlite3/driver"
	_ "github.com/ncruces/go-sqlite3/embed"
)

var db *sql.DB

func init() {
	var err error
	db, err = sql.Open("sqlite3", "file:tmp/test.db")
	if err != nil {
		panic("failed to open db: " + err.Error())
	}

	err = setupDb()

	if err != nil {
		panic("Failed to setup db: " + err.Error())
	}
}

func main() {
	defer db.Close()

	http.HandleFunc("/", handler.IndexHandler)
	http.Handle("/firma/", handler.FirmaHandler{})
	http.HandleFunc("/contact/add", handler.ContactHandler)

	http.Handle("/css/", http.StripPrefix("/css/", http.FileServer(http.Dir("assets/css"))))

	println("Running on http://localhost:8080")
	http.ListenAndServe(":8080", nil)
}

func setupDb() error {
	_, err := db.Exec(`
	DROP TABLE IF EXISTS firma;
	CREATE TABLE firma (
		id			TEXT PRIMARY KEY, -- UUID
		name		TEXT NOT NULL,	
		urls		TEXT,
		created_at	TEXT NOT NULL
	);	

	DROP TABLE IF EXISTS contact;
	CREATE TABLE contact (
		id			INTEGER PRIMARY KEY AUTOINCREMENT,
		date		TEXT NOT NULL, 
		type		TEXT NOT NULL
	);
	`)

	if err != nil {
		return err
	}

	return nil
}
