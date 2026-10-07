use crate::settings::app_setting;
use lunaria_core::settings::error::SettingError;
use sea_orm::sea_query::OnConflict;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use serde::de::DeserializeOwned;

pub mod language;
pub mod shortcuts;

async fn load_setting<T>(
    database: &DatabaseConnection,
    key: &str,
) -> Result<Option<T>, SettingError>
where
    T: DeserializeOwned,
{
    let record = app_setting::Entity::find_by_id(key)
        .one(database)
        .await
        .map_err(SettingError::storage)?;

    match record {
        Some(record) => serde_json::from_str(&record.value)
            .map(Some)
            .map_err(SettingError::storage),
        None => Ok(None),
    }
}

async fn save_setting(
    database: &DatabaseConnection,
    key: &str,
    value: String,
) -> Result<(), SettingError> {
    app_setting::Entity::insert(app_setting::ActiveModel {
        key: Set(key.to_owned()),
        value: Set(value),
    })
    .on_conflict(
        OnConflict::column(app_setting::Column::Key)
            .update_column(app_setting::Column::Value)
            .to_owned(),
    )
    .exec_without_returning(database)
    .await
    .map_err(SettingError::storage)?;

    Ok(())
}
