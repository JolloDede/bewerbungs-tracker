package database

type Firma struct {
	Name string
	Urls string
}

func SaveFirmaToDB(firma Firma) error {
	_, err := DB.Exec("INSERT INTO firma (name, urls) VALUES (?, ?)", firma.Name, firma.Urls)

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
	_, err := DB.Exec("UPDATE firma SET name = ?, urls = ? WHERE id = ?", firma.Name, firma.Urls, id)

	if err != nil {
		return err
	}

	return nil
}
