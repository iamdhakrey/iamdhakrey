use sea_orm_migration::{prelude::*, schema::*};

use crate::m20220101_000001_create_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Spend::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Spend::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Spend::UserId)
                            .uuid()
                            .not_null()
                            .to_owned(),
                    )
                    .col(
                        ColumnDef::new(Spend::Amount)
                            .decimal()
                            .not_null()
                            .to_owned(),
                    )
                    .col(
                        ColumnDef::new(Spend::Description)
                            .string()
                            .to_owned(),
                    )
                    .col(
                        ColumnDef::new(Spend::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .to_owned(),
                    )
                    .col(
                        ColumnDef::new(Spend::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .to_owned(),
                    )
                    .col(
                        ColumnDef::new(Spend::Category)
                            .string()
                            .not_null()
                            .to_owned(),
                    )
                    .col(
                        ColumnDef::new(Spend::Date)
                            .date()
                            .not_null()
                            .to_owned(),
                    )
                    .col(
                        ColumnDef::new(Spend::PaymentMethod)
                            .string()
                            .to_owned(),
                    )
                    .col(
                        ColumnDef::new(Spend::IsRecurring)
                            .boolean()
                            .default(false)
                            .to_owned(),
                    )
                    .col(ColumnDef::new(Spend::Tags).string().to_owned())
                    .col(
                        ColumnDef::new(Spend::Location)
                            .string()
                            .to_owned(),
                    )
                    .col(ColumnDef::new(Spend::Person).string().to_owned())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_spend_user_id")
                            .from(Spend::Table, Spend::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Spend::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Spend {
    Table,
    Id,
    UserId,
    Amount,
    Description,
    CreatedAt,
    UpdatedAt,
    Category,
    Date,
    PaymentMethod,
    IsRecurring,
    Tags,
    Location,
    Person,
}
