use duckdb::{params, Connection};
use crate::note::*;

pub fn save_note(note: Note) {
    let conn = Connection::open("notes.db").unwrap();

    let table_exists = table_exists(&conn, "notes");

    match table_exists {
        Ok(false) => create_table(&conn),
        Ok(true) => (),
        Err(_) => panic!("Could not check the database.")
    }

    save_note_to_db(&conn, note);
}

// TODO: should take an optional filter.
pub fn show(filter: Option<String>) -> Result<Vec<Note>, duckdb::Error> {
    let conn = Connection::open("notes.db")?;

    let query: String = if filter.is_some() {
        format!("SELECT * FROM notes WHERE content LIKE '%{}%'", filter.unwrap())
    }
    else {
        "SELECT * FROM notes".to_string()
    };

    let mut stmt = conn.prepare(&query)?;
    let notes_iter = stmt.query_map([], |row| {
        Ok(Note {
            id: Some(row.get::<_, i32>(0).unwrap()),
            content: row.get(1)?,
            due_date: row.get::<_, String>(2)?,
            inserted_at: Some(row.get::<_, String>(3).unwrap()),
        })
    })?;

    let notes: Result<Vec<_>, duckdb::Error> = notes_iter.collect();
    notes
}

fn save_note_to_db(conn: &Connection, note: Note) {
    let stmt = conn.prepare(
        "INSERT INTO notes(id, content,  due_date) VALUES (nextval('seq_notes_id'), ?, ?)"
    );

    let _ = stmt.expect("Could not insert into table").query(params![
        note.content,
        note.due_date,
    ]);
}

fn create_table(conn: &Connection) {
    let stmt = conn.prepare(
        "CREATE TABLE notes(id INTEGER PRIMARY KEY, content TEXT, due_date VARCHAR, inserted_at VARCHAR DEFAULT CAST(NOW() AS VARCHAR))"
    );

    let _ = stmt.expect("Could not create table").query(params![]);
    let create_sequence = conn.prepare(
        "CREATE SEQUENCE seq_notes_id START 1;"
    );

    let _ = create_sequence.expect("Could not create seq_notes_id").query(params![]);
}

fn table_exists(conn: &Connection, table_name: &str) -> Result<bool, duckdb::Error> {
    let mut stmt = conn.prepare(
        "SELECT COUNT(*) FROM information_schema.tables WHERE table_name = ?"
    )?;

    let mut rows = stmt.query([table_name])?;
    if let Some(row) = rows.next()? {
        let count: i64 = row.get(0)?;
        Ok(count > 0)
    } else {
        Ok(false)
    }

}