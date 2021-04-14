use actix_web::{Json, Query};
use log::debug;
use failure::Error;
use crate::Server;

// POST / csv のハンドラ
pub fn handle_post_csv(server: State<Server>) ->
    Result<HttpResponse, Error>{
        let logs = Default::default();

        Ok(HttpResponse::Ok().json(api::csv::post::Response(logs)))
    }

// POST /logs のハンドラ
pub fn handle_post_logs(server: State<Server>,
    log: Json<api::logs::post::Request>,) ->
    // POSTのボディは　Json<T>を引数に書くと自動的に弟子リアライズされて渡される
    Result<HttpResponse, Error>{
        debug!("{:?}", log);
        // レスポンスはAccepted
        Ok(HttpResponse::Accepted().finish())
    }

// GET /logs のハンドラ
pub fn handle_get_logs(server: State<Server>,
        range: Query<api::logs::get::Query>,) ->
    Result<HttpResponse, Error>{
        debug!("{:?}", range);

        let logs = Default::default();

        Ok(HttpResponse::Ok().json(api::logs::get::Response(logs)))
    }


// GET / csv のハンドラ
pub fn handle_get_csv(
    sever: State<Server>,
    range: Query<api::csv::get::Query>,
) -> Result<HttpResponse, Error>{
    debug!("{:?}", range);
    
    // csv ファイルはバイナリデータにして返す
    let csv: Vec<u8> = vec![];
    Ok(HttpResponse::Ok()
    .header("Content-Type". "text/csv")
    .body(csv))
}