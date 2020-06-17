pub mod store
{
    use std::cmp::max;
    use self::unqlite::{UnQLite, KV, Error};
    use std::convert::TryInto;

    extern crate unqlite;

    pub struct Store
    {
        // scores: std::collections::HashMap<String, DirtyArea>,
        database : UnQLite,
    }

    fn convert_to(dirtiness : u32) -> [u8; 4]
    {
        dirtiness.to_le_bytes()
    }

    fn convert_from(stored_data : &Vec<u8>) -> u32
    {
        if stored_data.len() != 4
        {
            panic!("Database corrupted");
        }
        let (int_bytes, _) = stored_data.split_at(std::mem::size_of::<u32>());
        u32::from_le_bytes(int_bytes.try_into().unwrap())
    }

    impl Store
    {
        pub fn declare_area(&self, area_name: &str) -> Result<String, String>
        {
            if self.database.kv_contains(&String::from(area_name))
            {
                return Err("Area already exists ".to_owned() + area_name);
            }

            self.database.kv_store(&String::from(area_name), convert_to(0));
            Ok(String::from(area_name))
        }
        pub fn score_of(&self, area_name: &str) -> std::option::Option<u32>
        {
            match self.database.kv_fetch(area_name)
            {
                Ok(score) => {
                    Some(convert_from(&score))
                },
                Err(_) => { None },
            }
        }

        pub fn clean_area(&self, area_name: &str) -> Result<String, String>
        {
            if !self.database.kv_contains(&String::from(area_name))
            {
                return Err("Area does not exist: ".to_owned() + area_name);
            }

            self.database.kv_store(&String::from(area_name), convert_to(0));
            return Ok(String::from(area_name));
        }

        pub fn adjust_score(&self, area_name: &str, increment_size: i32) -> Result<u32, String>
        {
            let current_score = self.score_of(area_name);
            match current_score
            {
                None => { Err("Could not find ".to_owned() + area_name) },
                Some(current_score) => {
                    let new_score = max(current_score as i32 + increment_size, 0) as u32;
                    self.database.kv_store(area_name, convert_to(new_score));
                    Ok(new_score)
                },
            }
        }

        pub fn initialize() -> Store
        {
            Store { database: UnQLite::create_temp() }
        }

        pub fn initialize_from(database_str : &str) -> Store
        {
            Store {
                database : UnQLite::create(database_str),
            }
        }
    }

    #[cfg(test)]
    mod store_tests {
        use crate::Store;
        use super::unqlite::UnQLite;
        use std::fs;

        #[test]
        fn add_an_area()
        {
            let mut store = Store::initialize();
            store.declare_area("bathroom sink");
            assert_eq!(store.score_of("bathroom sink").unwrap(), 0);
        }

        #[test]
        fn add_duplicate_area()
        {
            let mut store = Store::initialize();
            assert!(store.declare_area("bathroom sink").is_ok());
            assert!(store.declare_area("bathroom sink").is_err());
        }

        #[test]
        fn get_nonexistant_area()
        {
            let mut store = Store::initialize();
            assert!(store.score_of("bla").is_none());
        }

        #[test]
        fn increment_score()
        {
            let mut store = Store::initialize();
            let area_name = "bathroom sink";
            store.declare_area(area_name);
            assert_eq!(store.score_of(area_name).unwrap(), 0);
            assert_eq!(store.adjust_score(area_name, 1).unwrap(), 1);
            assert_eq!(store.score_of(area_name).unwrap(), 1);
            assert_eq!(store.adjust_score(area_name, -1).unwrap(), 0);
            assert_eq!(store.score_of(area_name).unwrap(), 0);
            assert_eq!(store.adjust_score(area_name, -1).unwrap(), 0);
            assert_eq!(store.score_of(area_name).unwrap(), 0);
        }

        #[test]
        fn increment_score_on_invalid_room()
        {
            let mut store = Store::initialize();
            assert!(store.adjust_score("boo", 1).is_err());
        }

        #[test]
        fn clean_area()
        {
            let mut store = Store::initialize();
            let area_name = "bathroom sink";
            store.declare_area(area_name);
            assert_eq!(store.score_of(area_name).unwrap(), 0);
            assert_eq!(store.adjust_score(area_name, 5).unwrap(), 5);
            assert!(store.clean_area(area_name).is_ok());
            assert_eq!(store.score_of(area_name).unwrap(), 0);
            assert!(store.clean_area(area_name).is_ok());
            assert_eq!(store.score_of(area_name).unwrap(), 0);
        }

        #[test]
        fn clean_nonexistant_area()
        {
            let mut store = Store::initialize();
            assert!(store.clean_area("boo").is_err());
        }

        #[test]
        fn test_database_persistance()
        {
            let database_title = "test_database";
            let area_name = "bathroom sink";
            {
                let mut store = Store::initialize_from(&database_title);
                store.declare_area(area_name);
                assert_eq!(store.adjust_score(area_name, 5).unwrap(), 5);
            }
            {
                let mut store = Store::initialize_from(&database_title);
                assert_eq!(store.score_of(area_name).unwrap(), 5);
            }

            fs::remove_file(database_title);
        }
    }
}