use sqlite3::database::{Database};
use sqlite3::cursor::{Cursor};
use sqlite3::types::ResultCode::{SQLITE_ROW};
use sqlite3::{open};

#[derive(Debug)]
pub struct Show {
    dbh: Database,
    id: i64,
    name: String,
}

impl Show {
    pub fn new(path: String) -> Show {
        let mut shows = match open(&path) {
            Ok(dbh) => Show {
                dbh: dbh,
                id: -1,
                name: "bleh".to_string(),
            },
            Err(err) => panic!("{:?}", err),
        };
        shows.initialize();
        shows
    }

    fn initialize(&mut self) {
        self.exec_or_die("CREATE TABLE IF NOT EXISTS
                          shows (id        INTEGER PRIMARY KEY,
                                 name      VARCHAR )");
        self.exec_or_die("CREATE UNIQUE INDEX IF NOT EXISTS
                          Shows_UniqueName ON shows(name)");
        self.exec_or_die("BEGIN");
    }
}
