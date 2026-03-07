use surrealdb::{engine::local::Db, Surreal};

use crate::Error;

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Context {
    #[ts(type = "{ tb: string, id: { String: string }}", optional)]
    pub last_opened_account: Option<surrealdb::RecordId>,
}

pub async fn read(db: &Surreal<Db>) -> Result<Context, Error> {
    db.select(("context", "main"))
        .await?
        .ok_or(Error::RecordNotFound)
}

pub async fn update(db: &Surreal<Db>, context: Context) -> Result<(), Error> {
    let _: Option<crate::Record> = db.update(("context", "main")).merge(context).await?;

    Ok(())
}
