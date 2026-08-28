use sea_orm_migration::{prelude::*, schema::*};

use crate::FK_CONTACT_FIRMA;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Firma::Table)
                    .add_column(string(Firma::Plzort).not_null().default(""))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Firma::Table)
                    .add_column(string(Firma::Stellenbezeichnung).not_null().default(""))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ContactNew::Table)
                    .col(pk_uuid(ContactNew::Id))
                    .col(date_time(ContactNew::Date))
                    .col(string(ContactNew::Type))
                    .col(uuid(ContactNew::FkFirma).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name(FK_CONTACT_FIRMA)
                            .from(ContactNew::Table, ContactNew::FkFirma)
                            .to(Firma::Table, Firma::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                INSERT INTO "contact_new" ("id", "date", "type", "fk_firma")
                SELECT "id",
                       CASE
                           WHEN instr("date", ' ') = 0 THEN "date" || ' 00:00:00'
                           ELSE "date"
                       END,
                       "type",
                       "fk_firma"
                FROM "contact"
                "#,
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Contact::Table).to_owned())
            .await?;
        manager
            .rename_table(
                Table::rename()
                    .table(ContactNew::Table, Contact::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ContactOld::Table)
                    .col(pk_uuid(ContactOld::Id))
                    .col(date(ContactOld::Date))
                    .col(string(ContactOld::Type))
                    .col(uuid(ContactOld::FkFirma).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name(FK_CONTACT_FIRMA)
                            .from(ContactOld::Table, ContactOld::FkFirma)
                            .to(Firma::Table, Firma::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                INSERT INTO "contact_old" ("id", "date", "type", "fk_firma")
                SELECT "id", substr("date", 1, 10), "type", "fk_firma"
                FROM "contact"
                "#,
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Contact::Table).to_owned())
            .await?;
        manager
            .rename_table(
                Table::rename()
                    .table(ContactOld::Table, Contact::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Firma::Table)
                    .drop_column(Firma::Plzort)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Firma::Table)
                    .drop_column(Firma::Stellenbezeichnung)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Firma {
    Table,
    Id,
    Plzort,
    Stellenbezeichnung,
}

#[derive(DeriveIden)]
enum Contact {
    Table,
}

#[derive(DeriveIden)]
enum ContactNew {
    Table,
    Id,
    Date,
    Type,
    FkFirma,
}

#[derive(DeriveIden)]
enum ContactOld {
    Table,
    Id,
    Date,
    Type,
    FkFirma,
}
