package database

import "github.com/google/uuid"

type Firma struct {
	Id         string
	Name       string
	Urls       string
	Text       string
	Created_at string
}

func SaveFirmaToDB(firma Firma) (string, error) {
	firma.Id = uuid.NewString()
	_, err := DB.Exec("INSERT INTO firma (id, name, urls, text, created_at) VALUES (?, ?, ?, ?, DATE())", firma.Id, firma.Name, firma.Urls, firma.Text)

	if err != nil {
		return "", err
	}

	return firma.Id, nil
}

func DeleteFirmaFromDB(id string) error {
	_, err := DB.Exec("DELETE FROM firma WHERE id = ?", id)

	if err != nil {
		return err
	}

	return nil
}

func UpdateFirmaDB(id string, firma Firma) error {
	_, err := DB.Exec("UPDATE firma SET name = ?, urls = ? WHERE id = ?", firma.Name, firma.Urls, id)

	if err != nil {
		return err
	}

	return nil
}

func LoadFirmasDB() ([]Firma, error) {
	rows, err := DB.Query("SELECT * FROM firma")

	if err != nil {
		return nil, err
	}
	firmas := make([]Firma, 0)

	for rows.Next() {
		var f Firma
		if err := rows.Scan(&f.Id, &f.Name, &f.Urls, &f.Text, &f.Created_at); err != nil {
			return nil, err
		}
		firmas = append(firmas, f)
	}

	return firmas, nil
}
