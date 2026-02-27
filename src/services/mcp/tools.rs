use axum::{Json, extract::Path};
use schemars::{JsonSchema, schema_for};
use serde_json::{Value, json};

use crate::handlers::{
    credits::{get_credits_api, topup_credits_api},
    donate::donate_api,
    payment::{create_payment_request_api, get_payment_status_api},
    wallet::{get_balance_api, send_nano_api},
};
use crate::structs::{
    api::ApiResponse,
    credit::TopupCreditsRequestApi,
    donate::DonateRequestApi,
    payment::{CreatePaymentRequestApi, PaymentStatusRequestApi},
    wallet::SendNanoRequestApi,
};

pub fn list_tools() -> Vec<Value> {
    vec![
        json!({
            "name": "wallet.balance",
            "description": "Get wallet balance and pending amount.",
            "inputSchema": empty_input_schema()
        }),
        json!({
            "name": "wallet.send",
            "description": "Send Nano to a recipient address.",
            "inputSchema": input_schema_for::<SendNanoRequestApi>()
        }),
        json!({
            "name": "payment.request",
            "description": "Create a payment request for a receive address and amount.",
            "inputSchema": input_schema_for::<CreatePaymentRequestApi>()
        }),
        json!({
            "name": "payment.status",
            "description": "Get payment status by transaction_id.",
            "inputSchema": input_schema_for::<PaymentStatusRequestApi>()
        }),
        json!({
            "name": "credits.get",
            "description": "Get available credits and current credit prices.",
            "inputSchema": empty_input_schema()
        }),
        json!({
            "name": "credits.topup",
            "description": "Top up credits by predefined credits_amount tier.",
            "inputSchema": topup_input_schema()
        }),
        json!({
            "name": "donate.send",
            "description": "Send a donation in Nano.",
            "inputSchema": input_schema_for::<DonateRequestApi>()
        }),
    ]
}

pub async fn handle_tool_call(name: &str, arguments: Value) -> Result<Value, Value> {
    match name {
        "wallet.balance" => map_api_result(get_balance_api().await),
        "wallet.send" => {
            let payload: SendNanoRequestApi = serde_json::from_value(arguments)
                .map_err(|error| invalid_args_error(error.to_string()))?;
            map_api_result(send_nano_api(Json(payload)).await)
        }
        "payment.request" => {
            let payload: CreatePaymentRequestApi = serde_json::from_value(arguments)
                .map_err(|error| invalid_args_error(error.to_string()))?;
            map_api_result(create_payment_request_api(Json(payload)).await)
        }
        "payment.status" => {
            let payload: PaymentStatusRequestApi = serde_json::from_value(arguments)
                .map_err(|error| invalid_args_error(error.to_string()))?;
            map_api_result(get_payment_status_api(Path(payload.transaction_id)).await)
        }
        "credits.get" => map_api_result(get_credits_api().await),
        "credits.topup" => {
            let payload: TopupCreditsRequestApi = serde_json::from_value(arguments)
                .map_err(|error| invalid_args_error(error.to_string()))?;
            map_api_result(topup_credits_api(Path(payload.credits_amount)).await)
        }
        "donate.send" => {
            let payload: DonateRequestApi = serde_json::from_value(arguments)
                .map_err(|error| invalid_args_error(error.to_string()))?;
            map_api_result(donate_api(Path(payload.amount)).await)
        }
        _ => Err(json!({
            "error": "TOOL_NOT_FOUND",
            "message": format!("Tool '{}' not found", name)
        })),
    }
}

fn input_schema_for<T: JsonSchema>() -> Value {
    serde_json::to_value(schema_for!(T)).unwrap_or_else(|_| empty_input_schema())
}

fn empty_input_schema() -> Value {
    json!({
        "type": "object",
        "properties": {}
    })
}

fn topup_input_schema() -> Value {
    let mut schema = input_schema_for::<TopupCreditsRequestApi>();

    if let Some(field_schema) = schema.pointer_mut("/properties/credits_amount") {
        *field_schema = json!({
            "type": "integer",
            "description": "Allowed credits top-up tiers",
            "enum": [10, 50, 100, 500, 1000, 5000, 10000, 50000, 100000]
        });
    }

    schema
}

fn map_api_result<T: serde::Serialize>(
    result: Result<
        Json<ApiResponse<T>>,
        (
            reqwest::StatusCode,
            Json<ApiResponse<crate::structs::api::ApiError>>,
        ),
    >,
) -> Result<Value, Value> {
    match result {
        Ok(Json(response)) => serde_json::to_value(response).map_err(|error| {
            json!({
                "error": "SERIALIZE_ERROR",
                "message": error.to_string(),
            })
        }),
        Err((status, Json(response))) => {
            let mut out = serde_json::to_value(response).unwrap_or_else(|_| {
                json!({
                    "success": false,
                    "data": Value::Null,
                    "error": {
                        "error": "UNKNOWN_ERROR",
                        "message": "Failed to serialize API error"
                    }
                })
            });

            if let Value::Object(ref mut map) = out {
                map.insert("status".to_string(), json!(status.as_u16()));
            }

            Err(out)
        }
    }
}

fn invalid_args_error(message: String) -> Value {
    json!({
        "error": "INVALID_ARGUMENTS",
        "message": message,
    })
}
