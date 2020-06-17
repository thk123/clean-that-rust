pub mod store
{
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
        pub fn declare_area(&mut self, area_name: &str)
        {
            let new_area = DirtyArea {
                area_name: String::from(area_name),
                dirtieness_score: 0,
            };
            self.scores.insert(String::from(area_name), new_area);
        }
        pub fn score_of(&self, area_name: &str) -> std::option::Option<u32>
        {
            match self.scores.get(&String::from(area_name))
            {
                None => { None },
                Some(area) => { Some(area.dirtieness_score) },
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
            assert_eq!(store.score_of("bathroom sink").expect("Room not found"), 0);
        }

        fn get_nonexistant_area()
        {
            let mut store = Store::initialize();
            assert!(store.score_of("bla").is_none());
        }
    }
}