package database

import (
	"time"

	"github.com/google/uuid"
)

type KeyValue struct {
	Key   int
	Value string
}

func SaveContactDB(contact Contact) error {
	contact.Id = uuid.NewString()
	_, err := DB.Exec("INSERT INTO contact (id, fk_firma, date, type) VALUES (?, ?, ?, ?)", contact.Id, contact.Firma, contact.Date, contactTypeName[contact.ContactType])

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

func DeleteContactFromDB(id string) error {
	_, err := DB.Exec("DELETE FROM contact WHERE id = ?", id)

	if err != nil {
		return err
	}

	return nil
}

func ContactList() ([]DisplayContact, error) {
	rows, err := DB.Query("SELECT c.id, c.date, c.type, f.Name FROM contact c INNER JOIN firma f ON f.id = c.fk_firma ORDER BY f.name, c.date")

	if err != nil {
		return nil, err
	}

	contacts := make([]DisplayContact, 0)

	for rows.Next() {
		var c DisplayContact
		if err := rows.Scan(&c.Id, &c.Date, &c.Status, &c.Firma); err != nil {
			return nil, err
		}
		tempDate, err := time.Parse(time.RFC3339, c.Date)
		if err != nil {
			return nil, err
		}
		c.Date = tempDate.Format("15:04 02.01.06")

		contacts = append(contacts, c)
	}

	return contacts, nil
}

type DisplayContact struct {
	Id     string
	Firma  string
	Date   string
	Status string
}

func GetLatestContactByFirma() ([]DisplayContact, error) {
	rows, err := DB.Query(`
	SELECT f.id, f.name, c.date, c.type FROM firma f LEFT JOIN (
		SELECT fk_firma, MAX(date) as max_date FROM contact GROUP BY fk_firma	
	) latest_contact ON f.id = latest_contact.fk_firma
	 INNER JOIN contact c ON c.fk_firma = latest_contact.fk_firma AND c.date = latest_contact.max_date
	 ORDER BY c.date ASC;
	`)

	if err != nil {
		return nil, err
	}

	contacts := make([]DisplayContact, 0)

	for rows.Next() {
		var c DisplayContact
		if err := rows.Scan(&c.Id, &c.Firma, &c.Date, &c.Status); err != nil {
			return nil, err
		}
		tempDate, err := time.Parse(time.RFC3339, c.Date)
		if err != nil {
			return nil, err
		}
		c.Date = tempDate.Format("15:04 02.01.06")

		contacts = append(contacts, c)
	}

	return contacts, nil
}
