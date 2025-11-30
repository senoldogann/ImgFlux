use axum::{
    extract::{Multipart, Query, Request},
    http::{StatusCode, header},
    middleware::{self, Next}, // Middleware modülleri
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use image::ImageFormat;
use std::io::Cursor;
use serde::Deserialize;
use tower_http::limit::RequestBodyLimitLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Router Kurulumu
    let app = Router::new()
        .route("/", get(|| async { "ImgFlux: Secure Image API 🔒" }))
        .route("/process", post(process_image))
        // 1. Önce Boyut Limiti (DDoS Koruması)
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024))
        // 2. Sonra Güvenlik Katmanı (API Key Kontrolü)
        // Her istek önce bu fonksiyonun süzgecinden geçer.
        .layer(middleware::from_fn(auth_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 ImgFlux (Secured) 0.0.0.0:3000 üzerinde çalışıyor...");
    axum::serve(listener, app).await.unwrap();
}

// --- MIDDLEWARE (GÜVENLİK GÖREVLİSİ) ---
async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    // 1. Header'dan "x-api-key" değerini okumaya çalış
    let api_key = req
        .headers()
        .get("x-api-key")
        .and_then(|header| header.to_str().ok());

    // Ortam değişkeninden geçerli anahtarları al (Virgülle ayrılmış)
    // Örn: API_KEYS="rust_is_fast_123,demo_user_007"
    let env_keys = std::env::var("API_KEYS").unwrap_or_else(|_| "rust_is_fast_123,demo_user_007".to_string());
    let valid_keys: Vec<&str> = env_keys.split(',').collect();

    // 2. Anahtarı kontrol et
    match api_key {
        Some(key) if valid_keys.contains(&key) => {
            // Anahtar geçerli, geçiş izni ver (Handler'a git)
            Ok(next.run(req).await)
        }
        _ => {
            // Anahtar yok veya geçersiz -> 401 Unauthorized
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

// --- AŞAĞISI AYNI (HANDLER) ---

#[derive(Deserialize)]
struct ResizeParams {
    w: Option<u32>,
    h: Option<u32>,
}

async fn process_image(
    Query(params): Query<ResizeParams>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        if name == "image" {
            let data = field.bytes().await.unwrap();
            
            let processed_image = tokio::task::spawn_blocking(move || {
                let img = image::load_from_memory(&data).map_err(|_| "Resim formatı geçersiz")?;
                let width = params.w.unwrap_or(300);
                let height = params.h.unwrap_or(300);
                let resized = img.resize(width, height, image::imageops::FilterType::Lanczos3);
                let mut buffer = Cursor::new(Vec::new());
                resized.write_to(&mut buffer, ImageFormat::Png).map_err(|_| "Encode hatası")?;
                Ok::<Vec<u8>, &'static str>(buffer.into_inner())
            }).await.unwrap();

            return match processed_image {
                Ok(bytes) => (StatusCode::OK, [(header::CONTENT_TYPE, "image/png")], bytes).into_response(),
                Err(err) => (StatusCode::BAD_REQUEST, err).into_response(),
            }
        }
    }
    (StatusCode::BAD_REQUEST, "Dosya bulunamadı").into_response()
}
