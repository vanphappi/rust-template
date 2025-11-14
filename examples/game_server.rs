/// Game server example with matchmaking and leaderboards
/// 
/// Run with: cargo run --example game_server

use actix_web::{web, App, HttpResponse, HttpServer, middleware};
use rust_template::{
    gameserver::{MatchmakingQueue, MatchmakingRequest, Leaderboard, GameSessionManager},
    errors::ApiError,
};
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Deserialize)]
struct JoinQueueRequest {
    player_id: String,
    skill_rating: u32,
}

#[derive(Deserialize)]
struct UpdateScoreRequest {
    player_id: String,
    score: i64,
}

#[derive(Serialize)]
struct MatchResponse {
    match_id: String,
    players: Vec<String>,
}

async fn join_queue(
    queue: web::Data<MatchmakingQueue>,
    req: web::Json<JoinQueueRequest>,
) -> Result<HttpResponse, ApiError> {
    let request = MatchmakingRequest {
        player_id: req.player_id.clone(),
        skill_rating: req.skill_rating,
        requested_at: Utc::now(),
    };

    queue.add_player(request);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Added to matchmaking queue",
        "queue_size": queue.queue_size()
    })))
}

async fn find_match(
    queue: web::Data<MatchmakingQueue>,
    session_manager: web::Data<GameSessionManager>,
) -> Result<HttpResponse, ApiError> {
    if let Some(match_result) = queue.find_match(4) {
        // Create game session
        let session_id = session_manager.create_session(match_result.players.clone());
        session_manager.start_session(&session_id);

        Ok(HttpResponse::Ok().json(MatchResponse {
            match_id: session_id,
            players: match_result.players,
        }))
    } else {
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Not enough players",
            "queue_size": queue.queue_size()
        })))
    }
}

async fn update_score(
    leaderboard: web::Data<Leaderboard>,
    req: web::Json<UpdateScoreRequest>,
) -> Result<HttpResponse, ApiError> {
    leaderboard.update_score(req.player_id.clone(), req.score);

    let rank = leaderboard.get_player_rank(&req.player_id);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Score updated",
        "rank": rank.map(|r| r.rank)
    })))
}

async fn get_leaderboard(
    leaderboard: web::Data<Leaderboard>,
) -> Result<HttpResponse, ApiError> {
    let top_10 = leaderboard.get_top(10);

    Ok(HttpResponse::Ok().json(top_10))
}

async fn get_queue_status(
    queue: web::Data<MatchmakingQueue>,
) -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "queue_size": queue.queue_size()
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = env_logger::try_init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Initialize game server components
    let matchmaking_queue = MatchmakingQueue::new(100); // skill range: 100
    let leaderboard = Leaderboard::new("global".to_string());
    let session_manager = GameSessionManager::new();

    println!("🎮 Starting Game Server on http://127.0.0.1:8080");
    println!("📝 Endpoints:");
    println!("   POST /matchmaking/join - Join matchmaking queue");
    println!("   POST /matchmaking/find - Find a match");
    println!("   GET  /matchmaking/status - Get queue status");
    println!("   POST /leaderboard/score - Update player score");
    println!("   GET  /leaderboard - Get top 10 players");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(matchmaking_queue.clone()))
            .app_data(web::Data::new(leaderboard.clone()))
            .app_data(web::Data::new(session_manager.clone()))
            .wrap(middleware::Logger::default())
            .route("/matchmaking/join", web::post().to(join_queue))
            .route("/matchmaking/find", web::post().to(find_match))
            .route("/matchmaking/status", web::get().to(get_queue_status))
            .route("/leaderboard/score", web::post().to(update_score))
            .route("/leaderboard", web::get().to(get_leaderboard))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

