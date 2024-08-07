use rusqlite::{params, Error as SqlError, Params};
use serde::{Deserialize, Serialize};

use crate::{utils::common_utils::time_stamp, CONN};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BookMark {
    pub book_id: String,
    mark_id: usize,
    start_position: Position,
    end_position: Position,
    pub create_time: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Position {
    chapter: usize,
    paragraph: usize,
    offset: usize,
}

impl BookMark {
    pub fn new(book_id: &str) -> Self {
        BookMark {
            book_id: book_id.to_string(),
            mark_id: BookMark::next_id(),
            create_time: time_stamp(),
            ..Default::default()
        }
    }

    pub fn start_pos(mut self, chapter: usize, paragraph: usize, offset: usize) -> Self {
        self.start_position = Position {
            chapter,
            paragraph,
            offset,
        };
        self
    }

    pub fn end_pos(mut self, chapter: usize, paragraph: usize, offset: usize) -> Self {
        self.end_position = Position {
            chapter,
            paragraph,
            offset,
        };
        self
    }

    pub fn update(mut self) -> Self {
        self.mark_id = BookMark::next_id();
        self.create_time = time_stamp();
        self
    }

    pub fn is_legle(&self) -> bool {
        self.end_position.chapter >= self.start_position.chapter
            && self.end_position.paragraph >= self.start_position.paragraph
            && self.end_position.offset >= self.start_position.offset
    }

    pub fn get_mark_list(id: &str) -> Result<Vec<BookMark>, SqlError> {
        let sql = "SELECT * FROM book_mark WHERE book_id = ?1;";

        BookMark::query_mark_list(sql, [id])
    }

    pub fn get_mark_list_by_chapter(id: &str, chapter: usize) -> Result<Vec<BookMark>, SqlError> {
        let sql = "SELECT * FROM book_mark WHERE book_id = ?1 AND start_chapter = ?2;";

        BookMark::query_mark_list(sql, params![id, chapter])
    }

    pub fn get_mark_list_by_paragraph(
        id: &str,
        chapter: usize,
        paragraph: usize,
    ) -> Result<Vec<BookMark>, SqlError> {
        let sql = "SELECT * FROM book_mark WHERE book_id = ?1 AND start_chapter = ?2 AND start_paragraph = ?3;";

        BookMark::query_mark_list(sql, params![id, chapter, paragraph])
    }

    fn query_mark_list<P>(sql: &str, params: P) -> Result<Vec<BookMark>, SqlError>
    where
        P: Params,
    {
        let conn = CONN.lock().unwrap();

        let mut stmt = conn.prepare(sql).unwrap();
        let rows = stmt
            .query_map(params, |row| {
                Ok(BookMark {
                    book_id: row.get(0).unwrap(),
                    mark_id: row.get(1).unwrap(),
                    start_position: Position {
                        chapter: row.get(2).unwrap(),
                        paragraph: row.get(3).unwrap(),
                        offset: row.get(4).unwrap(),
                    },
                    end_position: Position {
                        chapter: row.get(5).unwrap(),
                        paragraph: row.get(6).unwrap(),
                        offset: row.get(7).unwrap(),
                    },
                    create_time: row.get(8).unwrap(),
                })
            })
            .unwrap(); // 只有当参数不匹配时才会返回 Err, 此处断言不会错误, 依赖于开发者的开发水平

        let mut list = Vec::new();
        for row in rows {
            match row {
                Ok(mark) => list.push(mark),
                Err(err) => return Err(err),
            }
        }

        Ok(list)
    }

    pub fn insert_mark(mark: &Self) -> bool {
        let conn = CONN.lock().unwrap();

        let sql = "INSERT INTO book_mark (
                            book_id,
                            mark_id,
                            start_chapter,
                            start_paragraph,
                            start_offset,
                            end_chapter,
                            end_paragraph,
                            end_offset,
                            create_time
                        ) VALUES (
                            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9
                        );";
        let params = params![
            mark.book_id,
            mark.mark_id,
            mark.start_position.chapter,
            mark.start_position.paragraph,
            mark.start_position.offset,
            mark.end_position.chapter,
            mark.end_position.paragraph,
            mark.end_position.offset,
            mark.create_time,
        ];

        match conn.execute(sql, params) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn remove_mark(id: usize) -> bool {
        let conn = CONN.lock().unwrap();

        let sql = "DELETE FROM book_mark WHERE mark_id = ?1;";
        match conn.execute(sql, [id]) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn next_id() -> usize {
        let conn = CONN.lock().unwrap();

        let sql = "SELECT MAX(mark_id) FROM book_mark;";
        let result = conn.query_row(sql, [], |row| row.get::<usize, usize>(0));
        println!("max id: {:?}", result);

        match result {
            Ok(data) => data + 1,
            Err(_) => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::BookMark;

    #[test]
    fn db_test() {
        let mark1 = BookMark::new("id_1").start_pos(0, 0, 0).end_pos(1, 1, 1);
        BookMark::insert_mark(&mark1);

        let mark2 = BookMark::new("id_1").start_pos(1, 0, 0).end_pos(2, 2, 2);
        BookMark::insert_mark(&mark2);

        println!("{:?}\n", BookMark::get_mark_list("id_1"));
        println!("{:?}\n", BookMark::get_mark_list_by_chapter("id_1", 0));
        println!("{:?}\n", BookMark::get_mark_list_by_paragraph("id_1", 1, 0));

        for id in 0..BookMark::next_id() {
            println!("{}", BookMark::remove_mark(id));
        }
    }

    #[test]
    fn legle_test() {
        let mark1 = BookMark::new("id_1").start_pos(0, 0, 0).end_pos(1, 1, 1);
        assert_eq!(mark1.is_legle(), true);

        let mark2 = BookMark::new("id_1").start_pos(1, 0, 0).end_pos(0, 1, 1);
        assert_eq!(mark2.is_legle(), false);
    }
}
