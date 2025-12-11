use sea_orm_migration::{prelude::*, schema::*};
use tokio::try_join;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let create_received_good = manager.create_table(
            Table::create()
                .table("received_good")
                .if_not_exists()
                .col(pk_uuid("id").default(Expr::cust("gen_random_uuid()"))) // Генерируется UUIDv4, всегда заполнять вручную UUIDv7!!!
                .col(timestamp_with_time_zone("received").default(Expr::current_timestamp()))
                .to_owned(),
        );

        let create_bin = manager.create_table(
            Table::create()
                .table("bin")
                .if_not_exists()
                .col(pk_auto("id"))
                .col(integer("rack"))
                .col(uuid_null("good"))
                .to_owned(),
        );

        try_join!(create_received_good, create_bin)?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let drop_received_good = manager.drop_table(Table::drop().table("post").to_owned());
        let drop_bin = manager.drop_table(Table::drop().table("bin").to_owned());

        try_join!(drop_received_good, drop_bin)?;
        Ok(())
    }
}
