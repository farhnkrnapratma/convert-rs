use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{middleware::DefaultHeaders, App, HttpServer};
use std::time::Duration;

mod unit;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let governor_conf = GovernorConfigBuilder::default()
        .seconds_per_request(1)
        .burst_size(30)
        .finish()
        .unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET"]);

        App::new()
            .wrap(Governor::new(&governor_conf))
            .wrap(cors)
            .wrap(
                DefaultHeaders::new()
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("Cache-Control", "no-store")),
            )
            .service(unit::temperature::convert_temperature)
    })
    .keep_alive(Duration::from_secs(5))
    .client_request_timeout(Duration::from_secs(10))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
