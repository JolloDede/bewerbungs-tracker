package database

type ContactType int

const (
	Bewerbung ContactType = iota
	Nachfrage
	Absage
)

var ContactTypeName = map[ContactType]string{
	Bewerbung: "bewerbung",
	Nachfrage: "nachfrage",
	Absage:    "absage",
}

type Contact struct {
	Date        string
	ContactType ContactType
}

func SaveContactDB(contact Contact) error {
	_, err := DB.Exec("INSERT INTO contact (date, type) VALUES (?, ?)", contact.Date, ContactTypeName[contact.ContactType])

	if err != nil {
		return err
	}

	return nil
}

func UpdateContactDB(id string, contact Contact) error {
	_, err := DB.Exec("UPDATE firma SET date = ?, type = ? WHERE id = ?", contact.Date, ContactTypeName[contact.ContactType], id)

	if err != nil {
		return err
	}

	return nil
}
