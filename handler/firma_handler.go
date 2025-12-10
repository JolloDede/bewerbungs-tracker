package handler

import (
	"html/template"
	"net/http"
	"strings"
)

type FirmaHandler struct {
}

func (h FirmaHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	path := strings.TrimPrefix(r.URL.Path, "/firma/")

	switch path {
	case "":
		http.Redirect(w, r, "/home", http.StatusMovedPermanently)
	case "add":
		h.FirmaAddHandler(w, r)
	}
}

func (h FirmaHandler) FirmaAddHandler(w http.ResponseWriter, r *http.Request) {
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
		if urls == "" {
			http.Error(w, "atleast one url must be supplied", http.StatusBadRequest)
			return
		}
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
