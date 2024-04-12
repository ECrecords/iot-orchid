#[allow(unused_imports)]
use crate::web::error::{Error, Result};

use axum::extract::Extension;
use axum::extract::State;
use axum_jrpc::JsonRpcResponse;
use axum_jrpc::{JrpcResult, JsonRpcExtractor};

use crate::context::UserContext;
use crate::model::ModelManager;

pub async fn rpc_handler(
    State(_mm): State<ModelManager>,
    Extension(_ctx): Extension<UserContext>,
    request: JsonRpcExtractor,
) -> JrpcResult {
    let req_id = request.id.clone();
    let method = request.method.as_str();

    // The function then checks what the method is.
    let response = match method {
        // If the method is "test", it returns a successful JSON-RPC response with the request ID and the string "test".
        "test" => JsonRpcResponse::success(req_id, Some("test".to_string())),

        // "cluster.ping_all" => {
        //     // let param: [String; 1] = request.parse_params()?;
        //     // let res = ClusterBMC::ping_all(&ctx, &mm, &param[0]).await;

        //     // JsonRpcResponse::success(req_id, Some(res.unwrap()))
        // }
        m => request.method_not_found(m),
    };

    Ok(response)
}
