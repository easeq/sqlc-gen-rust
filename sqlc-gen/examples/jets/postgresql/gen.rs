/// @generated by the sqlc-gen-rust on sqlc-generate using sqlc.yaml
/// DO NOT EDIT.
use postgres::{Error, Row};
const COUNT_PILOTS: &str = r#"SELECT COUNT(*) FROM pilots"#;
const LIST_PILOTS: &str = r#"SELECT id, name FROM pilots LIMIT 5"#;
const DELETE_PILOT: &str = r#"DELETE FROM pilots WHERE id = $1"#;
#[derive(Clone, Debug, sqlc_derive::FromPostgresRow, PartialEq)]
pub(crate) struct Jet {
    pub id: i32,
    pub pilot_id: i32,
    pub age: i32,
    pub name: String,
    pub color: String,
}
#[derive(Clone, Debug, sqlc_derive::FromPostgresRow, PartialEq)]
pub(crate) struct Language {
    pub id: i32,
    pub language: String,
}
#[derive(Clone, Debug, sqlc_derive::FromPostgresRow, PartialEq)]
pub(crate) struct Pilot {
    pub id: i32,
    pub name: String,
}
#[derive(Clone, Debug, sqlc_derive::FromPostgresRow, PartialEq)]
pub(crate) struct PilotLanguage {
    pub pilot_id: i32,
    pub language_id: i32,
}
pub(crate) fn count_pilots(&mut self, arg: CountPilotsParams) -> anyhow::Result<i64> {
    let row = self.client.query_one(COUNT_PILOTS, &[])?;
    Ok(sqlc_core::FromPostgresRow::from_row(&row)?)
}
pub(crate) fn delete_pilot(&mut self, id: i32) -> anyhow::Result<()> {
    self.client.execute(DELETE_PILOT, &[&"id"])?;
    Ok(())
}
pub(crate) fn list_pilots(
    &mut self,
    arg: ListPilotsParams,
) -> anyhow::Result<Vec<ListPilotsRow>> {
    let rows = self.client.query(LIST_PILOTS, &[])?;
    let mut result: Vec<ListPilotsRow> = vec![];
    for row in rows {
        result.push(sqlc_core::FromPostgresRow::from_row(&row)?);
    }
    Ok(result)
}
