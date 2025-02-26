use actix_web::{get, HttpResponse, Responder};

#[get("/health_check")]
async fn service() -> impl Responder {
    HttpResponse::Ok()
}

#[cfg(test)]
mod health_tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn health_check_works() {
        let app = test::init_service(App::new().service(service)).await;
        let req = test::TestRequest::get().uri("/health_check").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
