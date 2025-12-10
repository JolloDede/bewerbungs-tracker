package database

import "database/sql"

var Db *sql.DB

func InitDb() error {
	var err error
	Db, err = sql.Open("sqlite3", "file:test.db")
	if err != nil {
		// panic("failed to open db: " + err.Error())
		return err
	}

	err = setupDb()

	if err != nil {
		// panic("Failed to setup db: " + err.Error())
		return err
	}

	return nil
}

func setupTables() error {

}
