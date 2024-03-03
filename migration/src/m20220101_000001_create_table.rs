use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .col(
                        ColumnDef::new(Post::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Post::Title)
                            .string()
                            .char_len(500)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Post::Body).text().not_null())
                    .col(ColumnDef::new(Post::DateCreated).date_time().not_null())
                    .col(ColumnDef::new(Post::DateModified).date_time().null())
                    .col(ColumnDef::new(Post::DatePublished).date_time().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Body,
    DatePublished,
    DateModified,
    DateCreated,
}
