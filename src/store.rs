pub mod store
{
    use std::cmp::max;
    use self::unqlite::{UnQLite, KV, Transaction};
    use std::convert::TryInto;

    extern crate unqlite;

    pub struct Store
    {
        // scores: std::collections::HashMap<String, DirtyArea>,
        database_address : String,
    }

    fn convert_to(dirtiness: u32) -> [u8; 4]
    {
        dirtiness.to_le_bytes()
    }

    fn convert_from(stored_data: &Vec<u8>) -> u32
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
        fn database(&self) -> UnQLite {
            UnQLite::create(&self.database_address)
        }

        pub fn declare_area(&self, area_name: &str) -> Result<String, String>
        {
            if self.database().kv_contains(&String::from(area_name))
            {
                return Err("Area already exists ".to_owned() + area_name);
            }

            match self.database().kv_store(&String::from(area_name), convert_to(0))
            {
                Ok(_) => {Ok(String::from(area_name))},
                Err(_) => {Err(String::from("Error writing to database"))},
            }
        }
        pub fn score_of(&self, area_name: &str) -> std::option::Option<u32>
        {
            match self.database().kv_fetch(area_name)
            {
                Ok(score) => {
                    Some(convert_from(&score))
                }
                Err(_) => { None }
            }
        }

        pub fn clean_area(&self, area_name: &str) -> Result<String, String>
        {
            if !self.database().kv_contains(&String::from(area_name))
            {
                return Err("Area does not exist: ".to_owned() + area_name);
            }

            let result = match self.database().kv_store(&String::from(area_name), convert_to(0)) {
                Ok(_) => {Ok(String::from(area_name))},
                Err(_) => {Err(String::from("Error writing to database"))},
            };

            return result;
        }

        pub fn adjust_score(&self, area_name: &str, increment_size: i32) -> Result<u32, String>
        {
            let current_score = self.score_of(area_name);
            match current_score
            {
                None => { Err("Could not find ".to_owned() + area_name) }
                Some(current_score) => {
                    let new_score = max(current_score as i32 + increment_size, 0) as u32;

                    let result = match self.database().kv_store(area_name, convert_to(new_score)) {
                        Ok(_) => {Ok(new_score)},
                        Err(_) => {Err(String::from("Error writing to datbase"))},
                    };

                    return result;
                }
            }
        }

        pub fn initialize() -> Store
        {
            Store::initialize_from("temp")
        }

        pub fn initialize_from(database_address: &str) -> Store
        {
            Store {
                database_address: String::from(database_address),
            }
        }
    }

    #[cfg(test)]
    mod store_tests {
        use crate::Store;
        use std::fs;

        #[test]
        fn add_an_area()
        {
            let store = Store::initialize();
            assert!(store.declare_area("bathroom sink").is_ok());
            assert_eq!(store.score_of("bathroom sink").unwrap(), 0);
            fs::remove_file("temp").expect("Deleting test db failed");
        }

        #[test]
        fn add_duplicate_area()
        {
            let store = Store::initialize();
            assert!(store.declare_area("bathroom sink").is_ok());
            assert!(store.declare_area("bathroom sink").is_err());
            fs::remove_file("temp").expect("Deleting test db failed");
        }

        #[test]
        fn get_nonexistant_area()
        {
            let store = Store::initialize();
            assert!(store.score_of("bla").is_none());
            fs::remove_file("temp").expect("Deleting test db failed");
        }

        #[test]
        fn increment_score()
        {
            let store = Store::initialize();
            let area_name = "bathroom sink";
            assert!(store.declare_area(area_name).is_ok());
            assert_eq!(store.score_of(area_name).unwrap(), 0);
            assert_eq!(store.adjust_score(area_name, 1).unwrap(), 1);
            assert_eq!(store.score_of(area_name).unwrap(), 1);
            assert_eq!(store.adjust_score(area_name, -1).unwrap(), 0);
            assert_eq!(store.score_of(area_name).unwrap(), 0);
            assert_eq!(store.adjust_score(area_name, -1).unwrap(), 0);
            assert_eq!(store.score_of(area_name).unwrap(), 0);
            fs::remove_file("temp").expect("Deleting test db failed");
        }

        #[test]
        fn increment_score_on_invalid_room()
        {
            let store = Store::initialize();
            assert!(store.adjust_score("boo", 1).is_err());
            fs::remove_file("temp").expect("Deleting test db failed");
        }

        #[test]
        fn clean_area()
        {
            let store = Store::initialize();
            let area_name = "bathroom sink";
            assert!(store.declare_area(area_name).is_ok());
            assert_eq!(store.score_of(area_name).unwrap(), 0);
            assert_eq!(store.adjust_score(area_name, 5).unwrap(), 5);
            assert!(store.clean_area(area_name).is_ok());
            assert_eq!(store.score_of(area_name).unwrap(), 0);
            assert!(store.clean_area(area_name).is_ok());
            assert_eq!(store.score_of(area_name).unwrap(), 0);
            fs::remove_file("temp").expect("Deleting test db failed");
        }

        #[test]
        fn clean_nonexistant_area()
        {
            let store = Store::initialize();
            assert!(store.clean_area("boo").is_err());
            fs::remove_file("temp").expect("Deleting test db failed");
        }

        #[test]
        fn test_database_persistance()
        {
            let database_title = "test_database";
            let area_name = "bathroom sink";
            {
                let store = Store::initialize_from(&database_title);
                assert!(store.declare_area(area_name).is_ok());
                assert_eq!(store.adjust_score(area_name, 5).unwrap(), 5);
            }
            {
                let store = Store::initialize_from(&database_title);
                assert_eq!(store.score_of(area_name).unwrap(), 5);
            }

            fs::remove_file(database_title).expect("Deleting test db failed");
        }
    }
}