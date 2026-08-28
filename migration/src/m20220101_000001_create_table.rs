use sea_orm_migration::{prelude::*, schema::*};

use crate::FK_CONTACT_FIRMA;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Firma::Table)
                    .if_not_exists()
                    .col(pk_uuid(Firma::Id))
                    .col(string(Firma::Name))
                    .col(string(Firma::Urls))
                    .col(string(Firma::Text))
                    .col(date_time(Firma::CreateAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Contact::Table)
                    .if_not_exists()
                    .col(pk_uuid(Contact::Id))
                    .col(date(Contact::Date))
                    .col(string(Contact::Type))
                    .col(uuid(Contact::FkFirma).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name(FK_CONTACT_FIRMA)
                            .from(Contact::Table, Contact::FkFirma)
                            .to(Firma::Table, Firma::Id)
                            .on_delete(ForeignKeyAction::Cascade), // .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Contact::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Firma::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Firma {
    Table,
    Id,
    Name,
    Urls,
    Text,
    CreateAt,
}

#[derive(DeriveIden)]
enum Contact {
    Table,
    Id,
    Date,
    Type,
    FkFirma,
}
