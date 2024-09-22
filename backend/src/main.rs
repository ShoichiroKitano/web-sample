use anyhow::Context;

#[derive(Clone)]
struct State {
    pool: sqlx::Pool<sqlx::mysql::MySql>,
}

#[derive(serde::Deserialize)]
struct CreateSampleJson {
    name: String,
    status: i32,
}

async fn create_sample(
    data: actix_web::web::Data<State>,
    req: actix_web::web::Json<CreateSampleJson>,
) -> actix_web::HttpResponse {
    let pool = data.pool.clone();
    let result = sqlx::query!(
        "INSERT INTO samples (name, status) VALUES (?, ?);",
        req.0.name,
        req.0.status
    )
    .execute(&pool)
    .await;

    // 自動採番されたidを取得
    let id = match result {
        Ok(r) => r.last_insert_id(),
        Err(e) => {
            println!("{:?}", e);
            return actix_web::HttpResponse::InternalServerError().finish();
        }
    };

    actix_web::HttpResponse::Created().json(serde_json::json!({
        "id": id,
        "name": req.0.name,
        "status": req.0.status,
    }))
}

struct Sample {
    id: i32,
    name: String,
    status: Option<i32>, // テーブルのカラムnot null制約を入れてないのでnull or 32bit整数が入ることをしめす
}

async fn index_samples(data: actix_web::web::Data<State>) -> actix_web::HttpResponse {
    let result = sqlx::query_as!(Sample, "SELECT * FROM samples;")
        .fetch_all(&data.pool)
        .await;
    let samples = match result {
        Ok(samples) => samples,
        Err(e) => {
            println!("{:?}", e);
            return actix_web::HttpResponse::InternalServerError().finish();
        }
    };

    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "samples": samples.iter().map(|s| {
            serde_json::json!({
                "id": s.id,
                "name": s.name,
                "status": s.status,
            })
        }).collect::<Vec<serde_json::Value>>()
    }))
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let pool = sqlx::mysql::MySqlPool::connect_with(
        "mysql://root:password@127.0.0.1:3306/sample_web"
            .parse()
            .context("connection parse failed")?,
    )
    .await
    .context("database connection failes")?; // anyhow::Contextをuseすると使えるようになるメソッド
    let _ = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(State { pool: pool.clone() }))
            .service(
                actix_web::web::resource("/samples")
                    .route(actix_web::web::post().to(create_sample))
                    .route(actix_web::web::get().to(index_samples)),
            )
    })
    .bind(("127.0.0.1", 8080))
    .context("bind failed")?
    .run()
    .await;
    Ok(())
}
