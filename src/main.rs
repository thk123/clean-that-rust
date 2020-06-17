use actix_web::{web, App, HttpResponse, HttpServer, Responder};
struct DirtyArea
{
    area_name : String,
    dirtieness_score : u32,
}

struct StoreTK
{
    scores : std::collections::HashMap<String, DirtyArea>,
}

impl StoreTK
{
    fn declare_area(&mut self, area_name : &str)
    {
        let new_area = DirtyArea {
            area_name: String::from(area_name),
            dirtieness_score: 0,
        };
        self.scores.insert(String::from(area_name), new_area);
    }
    fn score_of(&self, area_name : &str) -> std::option::Option<u32>
    {
        match self.scores.get(&String::from(area_name))
        {
            None => {None},
            Some(area) => {Some(area.dirtieness_score)},
        }
    }

}

#[cfg(test)]
mod store_tests {
    use crate::StoreTK;

    #[test]
    fn add_an_area()
    {
        let mut store = StoreTK
        {
            scores:std::collections::HashMap::new()
        };
        store.declare_area("bathroom sink");
        assert_eq!(store.score_of("bathroom sink").expect("Room not found"), 0);
    }
}
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
