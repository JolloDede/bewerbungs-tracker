package handler

import (
	"html/template"
	"net/http"
	"strings"
	"time"

	"github.com/jollodede/bewerbungs_tol/database"
)

type FirmaHandler struct {
}

func (h FirmaHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	path := strings.TrimPrefix(r.URL.Path, "/firma/")

	switch path {
	case "":
		h.firmenListHandler(w, r)
	case "add":
		h.firmaAddHandler(w, r)
	}
}

func (h FirmaHandler) firmaAddHandler(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case "POST":
		err := r.ParseForm()
		if err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}
		name := r.FormValue("name")
		if name == "" {
			http.Error(w, "Name must be present", http.StatusBadRequest)
			return
		}
		urls := r.FormValue("urls")
		text := r.FormValue("text")
		id, err := database.SaveFirmaToDB(database.Firma{Name: name, Urls: urls, Text: text})

		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		err = database.SaveContactDB(database.NewContact(id, time.Now(), database.Erfasst))

		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		http.Redirect(w, r, "/firma", http.StatusFound)
	case "GET":
		templates, err := template.ParseFiles("templates/base.html", "templates/firma_form.html")

		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		err = templates.ExecuteTemplate(w, "base", nil)

		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
		}
	}
}

func (h *FirmaHandler) firmenListHandler(w http.ResponseWriter, r *http.Request) {
	templates, err := template.ParseFiles("templates/base.html", "templates/firma_list.html")

	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	firmas, err := database.LoadFirmasDB()

	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	data := struct {
		Firmas []database.Firma
	}{
		Firmas: firmas,
	}

	err = templates.ExecuteTemplate(w, "base", data)

	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}
