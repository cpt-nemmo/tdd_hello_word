use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    println!("Hello, world");

    let app = get_router();
    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
    "Hello, world!"
}

fn get_router() -> Router {
    Router::new().route("/hello", get(handler))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_router() {
        //Given
        let app = get_router();

        //When
        let response = app
            .oneshot(
                http::Request::builder()
                    .uri("/hello")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), 200);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello, world!")
    }
}
