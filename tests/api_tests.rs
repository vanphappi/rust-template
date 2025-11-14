// Integration tests cho API
// Chạy tests: cargo test

#[cfg(test)]
mod tests {
    // TODO: Thêm integration tests cho từng endpoint
    // Ví dụ:

    /*
    use rust_template::routes::{configure_health_routes, configure_user_routes};
    use rust_template::state::AppState;
    use actix_web::web;

    #[actix_web::test]
    async fn test_health_endpoint() {
        let app = test::init_service(
            App::new()
                .configure(configure_health_routes)
        ).await;

        let req = test::TestRequest::get()
            .uri("/health")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_users() {
        let app_state = web::Data::new(AppState::new());

        let app = test::init_service(
            App::new()
                .app_data(app_state)
                .configure(configure_user_routes)
        ).await;

        let req = test::TestRequest::get()
            .uri("/users")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
    */
}
