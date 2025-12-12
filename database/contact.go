package database

import "time"

type Contact struct {
	Id          string
	Firma       string
	Date        string
	ContactType ContactType
}

func NewContact(firma string, date time.Time, contactType ContactType) Contact {
	return Contact{Firma: firma, ContactType: contactType, Date: date.Format(time.RFC3339)}
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
