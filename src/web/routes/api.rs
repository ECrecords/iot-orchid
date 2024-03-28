use crate::web::error::{Error, Result};

use axum::extract::Extension;
use axum::extract::State;
use axum_jrpc::error::JsonRpcError;
use axum_jrpc::JsonRpcResponse;
use axum_jrpc::{JrpcResult, JsonRpcExtractor, JsonRpcRequest, error::JsonRpcErrorReason };

use crate::context::Ctx;
use crate::model::ModelManager;
use crate::model::cluster::ClusterBMC;


pub async fn rpc_handler(
    State(mm): State<ModelManager>,
    Extension(ctx): Extension<Ctx>,
    request: JsonRpcExtractor,
) -> JrpcResult {
    // The `method` variable is set to the method of the request.
    let req_id = request.id.clone();
    let method = request.method.as_str();
    
    // The function then checks what the method is.
    let response = match method {
        // If the method is "test", it returns a successful JSON-RPC response with the request ID and the string "test".
        "test" => JsonRpcResponse::success(req_id, Some("test".to_string())),
        
        m => {
            request.method_not_found(m)
        }
    };

    Ok(response)
}
