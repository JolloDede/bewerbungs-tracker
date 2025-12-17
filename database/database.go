package database

import (
	"database/sql"
	"os"
)

var DB *sql.DB

func Init() error {
	dbExists := true
	const dbFile = "tmp/test.db"
	_, err := os.Stat(dbFile)

	if err != nil {
		dbExists = false
	}

	DB, err = sql.Open("sqlite3", "file:"+dbFile)
	if err != nil {
		return err
	}

	if !dbExists {
		err = setupTables()

		if err != nil {
			return err
		}
	}

	return nil
}

func setupTables() error {
	_, err := DB.Exec(`
	DROP TABLE IF EXISTS contact;
	DROP TABLE IF EXISTS firma;
	`)

	if err != nil {
		return err
	}

	_, err = DB.Exec(`
	CREATE TABLE firma (
		id			TEXT PRIMARY KEY NOT NULL, -- UUID
		name		TEXT NOT NULL,
		urls		TEXT,
		text		TEXT,
		created_at	TEXT NOT NULL
	);

	CREATE TABLE contact (
		id			TEXT PRIMARY KEY NOT NULL, -- UUID
		date		TEXT NOT NULL,
		type		TEXT NOT NULL,
		fk_firma	TEXT NOT NULL,
		FOREIGN KEY(fk_firma) REFERENCES firma(id)
	);
	`)

	if err != nil {
		return err
	}

	return nil
}
