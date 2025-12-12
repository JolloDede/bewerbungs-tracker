package handler

import (
	"html/template"
	"net/http"
	"strconv"
	"strings"
	"time"

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
	default:
		h.contactDeleteHandler(w, r)
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
		firma := r.FormValue("firma")
		if firma == "" {
			http.Error(w, "Firma must be present", http.StatusBadRequest)
			return
		}
		typ := r.FormValue("typ")
		if typ == "" {
			http.Error(w, "Typ must be supplied", http.StatusBadRequest)
			return
		}
		typId, err := strconv.Atoi(typ)
		if err != nil {
			http.Error(w, "Failed to convert typ to int", http.StatusBadRequest)
		}
		err = database.SaveContactDB(database.NewContact(firma, time.Now(), database.ContactType(typId)))

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
	switch r.Method {
	case "GET":
		templates, err := template.ParseFiles("templates/base.html", "templates/contact_list.html")

		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		contacts, err := database.ContactList()

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
}

func (h *Contacthandler) contactDeleteHandler(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case "DELETE":
		fullPath := strings.Split(r.URL.Path, "/")
		id := fullPath[len(fullPath)-1]

		err := database.DeleteContactFromDB(id)

		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		w.WriteHeader(http.StatusOK)
	}
}
