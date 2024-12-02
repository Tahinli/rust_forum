use extension::postgres::Type;
use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
struct Role;

#[derive(Iden, EnumIter)]
enum RoleType {
    #[iden = "Default"]
    Default,
    #[iden = "Admin"]
    Admin,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    Surname,
    Gender,
    BirthDate,
    Email,
    Role,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Role)
                    .values(RoleType::iter())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(string(User::Name))
                    .col(string(User::Surname))
                    .col(string(User::Email))
                    .col(date_time(User::BirthDate))
                    .col(boolean(User::Gender))
                    .col(enumeration(
                        User::Role,
                        Alias::new("role"),
                        RoleType::iter(),
                    ))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}
