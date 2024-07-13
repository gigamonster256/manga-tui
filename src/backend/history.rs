// save what mangas the user is reading and which chapters where read
// need a file to store that data,
// need to update it
//

use std::sync::Mutex;

use once_cell::sync::Lazy;
use rusqlite::{params, Connection};

pub static DBCONN: Lazy<Mutex<Option<Connection>>> = Lazy::new(|| {
    let conn = Connection::open("./db_test.db");

    if conn.is_err() {
        return Mutex::new(None);
    }
    let conn = conn.unwrap();
    conn.execute(
        "CREATE TABLE if not exists mangas (
                id    TEXT  PRIMARY KEY,
                title TEXT  NOT NULL
             )",
        (),
    )
    .unwrap();

    conn.execute(
        "CREATE TABLE if not exists chapters (
                id    TEXT  PRIMARY KEY,
                title TEXT  NOT NULL,
                manga_id TEXT  NOT NULL,
                FOREIGN KEY (manga_id) REFERENCES mangas (id)

            )",
        (),
    )
    .unwrap();

    Mutex::new(Some(conn))
});

// Create sqlite file if it does not exist and its tables
pub fn create_history() {}

pub struct MangaReadingHistorySave<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub chapter_id: &'a str,
    pub chapter_title: &'a str,
}

pub struct Manga {
    id: String,
}

// if it's the first time the user is reading a manga then save it to mangas table and save the
// current chapter that is read, else just save the chapter and associate the manga,
pub fn save_history(manga_read: MangaReadingHistorySave<'_>) -> rusqlite::Result<()> {
    let binding = DBCONN.lock().unwrap();

    let conn = binding.as_ref().unwrap();

    let mut manga_exists_statement = conn.prepare("SELECT id FROM mangas WHERE id = ?1")?;

    let mut manga_exists = manga_exists_statement
        .query_map(params![manga_read.id], |row| Ok(Manga { id: row.get(0)? }))?;

    if let Some(manga) = manga_exists.next() {
        let manga = manga.unwrap();
        conn.execute(
            "INSERT INTO chapters VALUES (?1, ?2, ?3)",
            (manga_read.chapter_id, manga_read.chapter_title, manga.id),
        )?;
        return Ok(());
    }

    conn.execute(
        "INSERT INTO mangas VALUES (?1, ?2)",
        (manga_read.id, manga_read.title),
    )?;

    conn.execute(
        "INSERT INTO chapters VALUES (?1, ?2, ?3)",
        (
            manga_read.chapter_id,
            manga_read.chapter_title,
            manga_read.id,
        ),
    )?;

    Ok(())
}

pub struct MangaReadingHistoryRetrieve<'a> {
    pub chapters_read: Vec<&'a str>,
}

pub fn get_manga_history(id: &str) -> MangaReadingHistoryRetrieve<'_> {
    let db_connection = Connection::open("./db_test.db").unwrap();

    let mut result = db_connection.prepare("SELECT id from mangas ").unwrap();

    MangaReadingHistoryRetrieve {
        chapters_read: vec![],
    }
}
