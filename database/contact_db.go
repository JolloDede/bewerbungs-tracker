package database

type KeyValue struct {
	Key   int
	Value string
}

type ContactType int

const (
	Erfasst ContactType = iota
	Bewerbung
	Nachfrage
	Absage
)

var contactTypeName = map[ContactType]string{
	Erfasst:   "erfasst",
	Bewerbung: "bewerbung",
	Nachfrage: "nachfrage",
	Absage:    "absage",
}

func (ct ContactType) String() string {
	return contactTypeName[ct]
}

func ContactTypeList() []KeyValue {
	list := make([]KeyValue, 0)

	for id, name := range contactTypeName {
		list = append(list, KeyValue{Key: int(id), Value: name})
	}

	return list
}

type Contact struct {
	Date        string
	ContactType ContactType
}

func SaveContactDB(contact Contact) error {
	_, err := DB.Exec("INSERT INTO contact (date, type) VALUES (?, ?)", contact.Date, contactTypeName[contact.ContactType])

	if err != nil {
		return err
	}

	return nil
}

func UpdateContactDB(id string, contact Contact) error {
	_, err := DB.Exec("UPDATE firma SET date = ?, type = ? WHERE id = ?", contact.Date, contactTypeName[contact.ContactType], id)

	if err != nil {
		return err
	}

	return nil
}
