pub mod repositories {
    use crate::domain::entities::Keep;
    use crate::domain::repositories::repositories::KeepRepository;
    use diesel::sql_types::Text;
    use diesel::SqliteConnection;
    use diesel::*;

    pub struct DBKeepRepository {
        pub db_conn: SqliteConnection,
    }

    #[derive(QueryableByName, Debug, Clone)]
    struct DataKeep {
        #[sql_type = "Text"]
        url: String,
    }

    impl KeepRepository for DBKeepRepository {
        fn save(&self, keep: Keep) {
            let qr_str = format!("INSERT INTO keeps(url) VALUES(\"{url}\")", url = keep.url);
            self.db_conn.execute(qr_str.as_str()).unwrap();
        }
        fn keeps(&self) -> std::vec::Vec<Keep> {
            let qr_str = format!("SELECT * FROM keeps;");
            let result: Vec<DataKeep> = sql_query(qr_str).load::<DataKeep>(&self.db_conn).unwrap();
            return result.iter().map(map).collect();
        }
        fn delete(&self, keep: Keep) {
            let qr_str = format!("DELETE FROM keeps WHERE url = \"{url}\";", url = keep.url);
            self.db_conn.execute(qr_str.as_str()).unwrap();
        }
    }

    fn map(data_keep: &DataKeep) -> Keep {
        Keep {
            url: data_keep.url.clone(),
        }
    }
}
