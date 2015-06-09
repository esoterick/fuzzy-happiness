#![allow(dead_code)]

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

    fn exec_or_die(&mut self, sql: &str) {
        match self.dbh.exec(sql) {
            Ok(true)  => (),
            Ok(false) => panic!("exec: {}", self.dbh.get_errmsg()),
            Err(msg)  => panic!("exec: {:?}, {:?}\nIn sql: '{}'\n",
                                msg, self.dbh.get_errmsg(), sql)
        }
    }

    fn initialize(&mut self) {
        self.exec_or_die("CREATE TABLE IF NOT EXISTS
                          shows (id   INTEGER PRIMARY KEY,
                                 name VARCHAR )");
        self.exec_or_die("CREATE UNIQUE INDEX IF NOT EXISTS
                          Shows_UniqueName ON shows(name)");
        self.exec_or_die("BEGIN");
    }

    fn prepare_or_die<'a>(&'a self, sql: &str) -> Cursor<'a> {
        match self.dbh.prepare(sql, &None) {
            Ok(s)  => s,
            Err(x) => panic!("sqlite error: {:?} ({:?})",
                             self.dbh.get_errmsg(), x),
        }
    }

    fn select_one<'a>(&'a mut self, sql: &str) -> Option<Cursor<'a>> {
        let mut cursor = self.prepare_or_die(sql);
        if cursor.step() == SQLITE_ROW {
            Some(cursor)
        } else {
            None
        }
    }
}
