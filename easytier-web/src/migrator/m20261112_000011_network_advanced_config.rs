use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Network-level advanced settings inherited by every member at
        // compile time; stored as a NetworkConfig JSON document.
        manager
            .get_connection()
            .execute_unprepared("ALTER TABLE networks ADD COLUMN advanced_config TEXT")
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("ALTER TABLE networks DROP COLUMN advanced_config")
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{Database, Statement};

    #[tokio::test]
    async fn migration_adds_nullable_advanced_config_column() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        db.execute_unprepared("CREATE TABLE networks (id TEXT PRIMARY KEY, network_name TEXT)")
            .await
            .unwrap();
        let manager = SchemaManager::new(&db);
        Migration.up(&manager).await.unwrap();
        db.execute_unprepared("INSERT INTO networks (id, network_name) VALUES ('n1', 'net')")
            .await
            .unwrap();
        let row = db
            .query_one(Statement::from_string(
                db.get_database_backend(),
                "SELECT advanced_config FROM networks WHERE id = 'n1'".to_owned(),
            ))
            .await
            .unwrap()
            .unwrap();
        assert!(
            row.try_get::<Option<String>>("", "advanced_config")
                .unwrap()
                .is_none()
        );
        Migration.down(&manager).await.unwrap();
    }
}
