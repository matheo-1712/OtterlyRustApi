use axum::{extract::State, Json};
use mysql::prelude::Queryable;
use mysql::Row;
use crate::{state::AppState, response::ApiResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct Server {
    pub id: u32,
    pub nom: String,
    pub jeu: String,
    pub version: String,
    pub modpack: String,
    pub modpack_url: String,
    pub nom_monde: String,
    pub embed_color: String,
    pub contenaire: String,
    pub description: String,
    pub actif: bool,
    pub global: bool,
    pub r#type: String,
    pub image: String,
}

pub async fn servers_handler(State(state): State<AppState>) -> Json<ApiResponse<Vec<Server>>> {
    // mysql crate est synchrone → exécuter dans un thread bloquant
    let servers = tokio::task::spawn_blocking(move || {
        let mut conn = state.get_conn();
        conn.query_map(
            "SELECT id, nom, jeu, version, modpack, modpack_url, nom_monde, embed_color, contenaire, description, actif, global, type, image FROM serveurs WHERE actif = 1",
            |row: Row| {
                Server {
                    id: row.get(0).unwrap(),
                    nom: row.get(1).unwrap(),
                    jeu: row.get(2).unwrap(),
                    version: row.get(3).unwrap(),
                    modpack: row.get(4).unwrap(),
                    modpack_url: row.get(5).unwrap(),
                    nom_monde: row.get(6).unwrap(),
                    embed_color: row.get(7).unwrap(),
                    contenaire: row.get(8).unwrap(),
                    description: row.get(9).unwrap(),
                    actif: row.get(10).unwrap(),
                    global: row.get(11).unwrap(),
                    r#type: row.get(12).unwrap(),
                    image: row.get(13).unwrap(),
                }
            }
        ).unwrap_or_default()
    }).await.unwrap_or_default();

    let count = servers.len();

    Json(ApiResponse::success_with_meta(servers, count, Some(1), Some(25)))
}
