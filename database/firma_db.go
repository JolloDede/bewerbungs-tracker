package database

type Firma struct {
	name string
	urls string
}

func SaveFirmaToDB(firma Firma) error {
	_, err := DB.Exec("INSERT INTO firma (name, urls) VALUES (?, ?)", firma.name, firma.urls)

	if err != nil {
		return err
	}

	return nil
}

func DeleteFirmaFromDB(id string) error {
	_, err := DB.Exec("DELETE FROM firma WHERE id = ?", id)

	if err != nil {
		return err
	}

	return nil
}

func UpdateFirmaDB(id string, firma Firma) error {
	_, err := DB.Exec("UPDATE firma SET name = ?, urls = ? WHERE id = ?", firma.name, firma.urls, id)

	if err != nil {
		return err
	}

	return nil
}
