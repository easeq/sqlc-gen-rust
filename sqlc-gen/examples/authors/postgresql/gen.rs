/// @generated by the sqlc-gen-rust on sqlc-generate using sqlc.yaml
/// DO NOT EDIT.
use postgres::{Error, Row};
const GET_AUTHOR: &str = r#"
SELECT id, name, bio FROM authors
WHERE id = $1 LIMIT 1
"#;
const LIST_AUTHORS: &str = r#"
SELECT id, name, bio FROM authors
ORDER BY name
"#;
const CREATE_AUTHOR: &str = r#"
INSERT INTO authors (
          name, bio
) VALUES (
  $1, $2
)
RETURNING id, name, bio
"#;
const DELETE_AUTHOR: &str = r#"
DELETE FROM authors
WHERE id = $1
"#;
#[derive(Clone, Debug, sqlc_derive::FromPostgresRow, PartialEq)]
pub(crate) struct Author {
    pub id: i64,
    pub name: String,
    pub bio: Option<String>,
}
pub(crate) fn create_author(
    &mut self,
    arg: CreateAuthorParams,
) -> anyhow::Result<CreateAuthorRow> {
    let row = self.client.query_one(CREATE_AUTHOR, &[&"arg".name, &"arg".bio])?;
    Ok(sqlc_core::FromPostgresRow::from_row(&row)?)
}
pub(crate) fn delete_author(&mut self, id: i64) -> anyhow::Result<()> {
    self.client.execute(DELETE_AUTHOR, &[&"id"])?;
    Ok(())
}
pub(crate) fn get_author(&mut self, id: i64) -> anyhow::Result<GetAuthorRow> {
    let row = self.client.query_one(GET_AUTHOR, &[&"id"])?;
    Ok(sqlc_core::FromPostgresRow::from_row(&row)?)
}
pub(crate) fn list_authors(
    &mut self,
    arg: ListAuthorsParams,
) -> anyhow::Result<Vec<ListAuthorsRow>> {
    let rows = self.client.query(LIST_AUTHORS, &[])?;
    let mut result: Vec<ListAuthorsRow> = vec![];
    for row in rows {
        result.push(sqlc_core::FromPostgresRow::from_row(&row)?);
    }
    Ok(result)
}
