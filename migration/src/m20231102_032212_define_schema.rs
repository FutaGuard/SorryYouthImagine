use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::IsAdmin).boolean().not_null())
                    .col(ColumnDef::new(Users::Active).boolean().not_null())
                    .col(ColumnDef::new(Users::UserName).string().not_null())
                    .col(ColumnDef::new(Users::PassWord).string().not_null())
                    .col(ColumnDef::new(Settings::Id).uuid().not_null())
                    .col(ColumnDef::new(Settings::Guest).boolean().not_null())
                    .col(ColumnDef::new(Settings::SiteName).string().not_null())
                    .col(ColumnDef::new(Images::Id).uuid().not_null())
                    .col(ColumnDef::new(Images::Date).timestamp().not_null())
                    .col(ColumnDef::new(Images::UploadBy).uuid().not_null())
                    .col(ColumnDef::new(Images::Hash).string().not_null())
                    .col(ColumnDef::new(Images::OriginFileName).text().not_null())
                    .col(ColumnDef::new(Images::Slug).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    IsAdmin,
    Active,
    UserName,
    PassWord,
}

#[derive(DeriveIden)]
enum Images {
    Table,
    Id,
    Date,
    UploadBy,
    Hash,
    OriginFileName,
    Slug,
}

#[derive(DeriveIden)]
enum Settings {
    Table,
    Id,
    Guest,
    SiteName,
}