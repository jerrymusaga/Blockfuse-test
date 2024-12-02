use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Event::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Event::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(Event::Type).string().not_null())
                .col(ColumnDef::new(Event::Data).json().not_null())
                .col(ColumnDef::new(Event::Timestamp).timestamp().not_null())
                .to_owned(),
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Event::Table).to_owned()).await
    }
}

#[derive(Iden)]
enum Event {
    Table,
    Id,
    Type,
    Data,
    Timestamp,
}
