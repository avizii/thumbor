pub mod engine;
pub mod pb;

use crate::engine::{Engine, Photon};
use anyhow::Result;
use axum::extract::{Extension, Path};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::{routing::get, AddExtensionLayer, Router, Server};
use bytes::Bytes;
use image::ImageOutputFormat;
use lru::LruCache;
use pb::*;
use percent_encoding::{percent_decode_str, percent_encode, NON_ALPHANUMERIC};
use serde::Deserialize;
use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use time::macros::format_description;
use time::UtcOffset;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tracing::info;
use tracing::instrument;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::OffsetTime;

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

#[instrument(level = "info", skip(cache))]
async fn retrieve_image(url: &str, cache: Cache) -> Result<Bytes> {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let key = hasher.finish();

    let g = &mut cache.lock().await;

    let data = match g.get(&key) {
        Some(v) => {
            info!("match cache {}", key);
            v.to_owned() // clone the data
        }
        None => {
            info!("retrieve url");
            let response = reqwest::get(url).await?;
            let data = response.bytes().await?;
            g.put(key, data.clone());
            data
        }
    };

    Ok(data)
}

async fn generate(
    Path(Params { spec, url }): Path<Params>,
    Extension(cache): Extension<Cache>,
) -> Result<(HeaderMap, Vec<u8>), StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();

    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let data = retrieve_image(&url, cache)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut engine: Photon = data
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    engine.apply(&spec.spec);

    let image = engine.generate(ImageOutputFormat::Jpeg(85));

    info!("finished processing: image size {}", image.len());

    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("image/jpeg"));

    Ok((headers, image))
}

#[tokio::main]
async fn main() {
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0,0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]")
    );

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(local_time)
        .init();

    let cache: Cache = Arc::new(Mutex::new(LruCache::new(1024)));

    let app = Router::new()
        .route("/image/:spec/:url", get(generate))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(cache))
                .into_inner(),
        );

    let address = "127.0.0.1:3000".parse().unwrap();

    print_test_url("https://images.pexels.com/photos/1562477/pexels-photo-1562477.jpeg?auto=compress&cs=tinysrgb&dpr=3&h=750&w=1260");

    info!("listening on {}", address);

    Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn print_test_url(url: &str) {
    let image_spec = ImageSpec::new(vec![
        Spec::new_resize(500, 800, resize::SampleFilter::CatmullRom),
        Spec::new_watermark(20, 20),
        Spec::new_filter(filter::Filter::Marine),
    ]);

    let s: String = image_spec.borrow().into();
    let test_image = percent_encode(url.as_bytes(), NON_ALPHANUMERIC).to_string();
    println!("test url: http://localhost:3000/image/{}/{}", s, test_image);
}

