package handler

import (
	"html/template"
	"net/http"
	"strings"

	"github.com/jollodede/bewerbungs_tol/database"
)

type Contacthandler struct{}

func (h Contacthandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	path := strings.TrimPrefix(r.URL.Path, "/contact/")

	switch path {
	case "":
		h.contactListHandler(w, r)
	case "add":
		h.contactAddHandler(w, r)
	}
}

func (h *Contacthandler) contactAddHandler(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case "POST":
		err := r.ParseForm()
		if err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}
		name := r.FormValue("")
		if name == "" {
			http.Error(w, "Name must be present", http.StatusBadRequest)
			return
		}
		urls := r.FormValue("urls")
		if urls == "" {
			http.Error(w, "atleast one url must be supplied", http.StatusBadRequest)
			return
		}
		err = database.SaveFirmaToDB(database.Firma{Name: name, Urls: urls})

		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		http.Redirect(w, r, "/firma", http.StatusFound)
	case "GET":
		templates, err := template.ParseFiles("templates/base.html", "templates/contact_form.html")

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
			Types  []database.KeyValue
		}{
			Firmas: firmas,
			Types:  database.ContactTypeList(),
		}

		err = templates.ExecuteTemplate(w, "base", data)

		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
		}
	}
}

func (h *Contacthandler) contactListHandler(w http.ResponseWriter, r *http.Request) {

}
