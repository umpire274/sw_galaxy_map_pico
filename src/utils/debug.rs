use rusqlite::{Connection, Result};

#[allow(dead_code)]
pub fn dump_meta(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT key, value, updated_utc FROM meta ORDER BY key")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;

    println!("--- META ---");
    for row in rows {
        let (k, v, t) = row?;
        println!("{k} = {v} ({t})");
    }

    Ok(())
}
