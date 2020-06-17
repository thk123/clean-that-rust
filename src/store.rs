pub mod store
{
    use std::cmp::max;

    pub struct DirtyArea
    {
        area_name: String,
        dirtieness_score: u32,
    }

    pub struct Store
    {
        scores: std::collections::HashMap<String, DirtyArea>,
    }

    impl Store
    {
        pub fn declare_area(&mut self, area_name: &str) -> Result<String, String>
        {
            if self.scores.contains_key(&String::from(area_name))
            {
                return Err("Area already exists ".to_owned() + area_name);
            }

            let new_area = DirtyArea {
                area_name: String::from(area_name),
                dirtieness_score: 0,
            };
            self.scores.insert(String::from(area_name), new_area);
            Ok(String::from(area_name))
        }
        pub fn score_of(&self, area_name: &str) -> std::option::Option<u32>
        {
            match self.scores.get(&String::from(area_name))
            {
                None => { None }
                Some(area) => { Some(area.dirtieness_score) }
            }
        }

        pub fn clean_area(&mut self, area_name: &str) -> Result<String, String>
        {
            match self.scores.get_mut(&String::from(area_name))
            {
                None => { Err("Could not find area: ".to_owned() + area_name) }
                Some(area) => {
                    area.dirtieness_score = 0;
                    Ok(String::from(area_name))
                }
            }
        }

        pub fn adjust_score(&mut self, area_name: &str, increment_size: i32) -> Result<u32, String>
        {
            match self.scores.get_mut(&String::from(area_name))
            {
                None => { Err("Could not find ".to_owned() + area_name) }
                Some(area) =>
                    {
                        let new_score = area.dirtieness_score as i32 + increment_size;
                        area.dirtieness_score = max(new_score, 0) as u32;
                        Ok(area.dirtieness_score)
                    }
            }
        }

        pub fn initialize() -> Store
        {
            Store { scores: std::collections::HashMap::new() }
        }
    }

    #[cfg(test)]
    mod store_tests {
        use crate::Store;

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
    }
}