package handler

import (
	"html/template"
	"net/http"

	"github.com/jollodede/bewerbungs_tol/database"
)

func IndexHandler(w http.ResponseWriter, r *http.Request) {
	templates, err := template.ParseFiles("templates/base.html", "templates/index.html")

	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	contacts, err := database.GetLatestContactByFirma()

	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	data := struct {
		Contacts []database.DisplayContact
	}{
		Contacts: contacts,
	}

	err = templates.ExecuteTemplate(w, "base", data)

	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}
