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
	Firma       string
	Date        string
	ContactType ContactType
}

func SaveContactDB(contact Contact) error {
	_, err := DB.Exec("INSERT INTO contact (fk_firma, date, type) VALUES (?, ?, ?)", contact.Firma, contact.Date, contactTypeName[contact.ContactType])

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

type LatestContact struct {
	id     string
	Firma  string
	Date   string
	Status string
}

func GetLatestContactByFirma() ([]LatestContact, error) {
	rows, err := DB.Query(`
	SELECT f.id, f.name, c.date, c.type FROM firma f LEFT JOIN (
		SELECT fk_firma, MAX(date) as max_date FROM contact GROUP BY fk_firma	
	) latest_contact ON f.id = latest_contact.fk_firma
	 LEFT JOIN contact c ON c.fk_firma = latest_contact.fk_firma AND c.date = latest_contact.max_date;
	`)

	if err != nil {
		return nil, err
	}

	contacts := make([]LatestContact, 0)

	for rows.Next() {
		var c LatestContact
		if err := rows.Scan(&c.id, &c.Firma, &c.Date, &c.Status); err != nil {
			return nil, err
		}
		contacts = append(contacts, c)
	}

	return contacts, nil
}
