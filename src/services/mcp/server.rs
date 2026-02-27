use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::thread;

use serde_json::{Value, json};
use tokio::runtime::Handle;

use super::tools::{handle_tool_call, list_tools};
use crate::structs::mcp::JsonRpcRequest;

pub fn initialize_mcp_stdio_server(handle: Handle) {
    eprintln!("✅ MCP stdio server started");
    thread::spawn(move || {
        if let Err(error) = run_server_loop(handle) {
            if is_expected_stdio_disconnect(&error) {
                eprintln!("ℹ️ MCP stdio server stopped: {}", error);
            } else {
                eprintln!("❌ MCP server failed: {}", error);
            }
        }
    });
}

fn is_expected_stdio_disconnect(error: &str) -> bool {
    let lowered = error.to_ascii_lowercase();
    lowered.contains("broken pipe")
        || lowered.contains("stream did not contain valid utf-8")
        || lowered.contains("the pipe is being closed")
}

fn run_server_loop(handle: Handle) -> Result<(), String> {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let stdout = std::io::stdout();
    let mut writer = stdout.lock();

    loop {
        let payload = match read_framed_message(&mut reader) {
            Ok(Some(message)) => message,
            Ok(None) => return Ok(()),
            Err(error) => return Err(error),
        };

        let parsed: Result<JsonRpcRequest, _> = serde_json::from_slice(&payload);
        let request = match parsed {
            Ok(request) => request,
            Err(error) => {
                let response = json!({
                    "jsonrpc": "2.0",
                    "id": Value::Null,
                    "error": {
                        "code": -32700,
                        "message": format!("Parse error: {}", error),
                    }
                });
                write_framed_message(&mut writer, &response)?;
                continue;
            }
        };

        if request.id.is_none() {
            continue;
        }

        let id = request.id.clone().unwrap_or(Value::Null);

        if request.jsonrpc != "2.0" {
            let response = json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32600,
                    "message": "Invalid Request: jsonrpc must be 2.0"
                }
            });
            write_framed_message(&mut writer, &response)?;
            continue;
        }

        let response = dispatch_request(&handle, request, id);
        write_framed_message(&mut writer, &response)?;
    }
}

fn dispatch_request(handle: &Handle, request: JsonRpcRequest, id: Value) -> Value {
    match request.method.as_str() {
        "initialize" => json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": "2024-11-05",
                "serverInfo": {
                    "name": "ifenpay-nano-agent-plugin",
                    "version": "0.1.0"
                },
                "capabilities": {
                    "tools": {
                        "listChanged": false
                    }
                }
            }
        }),
        "tools/list" => json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "tools": list_tools()
            }
        }),
        "tools/call" => {
            let params = request.params.unwrap_or_else(|| json!({}));
            let name = params
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();
            let arguments = params
                .get("arguments")
                .cloned()
                .unwrap_or_else(|| json!({}));

            let call_result = handle.block_on(async move { handle_tool_call(&name, arguments).await });
            match call_result {
                Ok(data) => json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "content": [{
                            "type": "text",
                            "text": serde_json::to_string_pretty(&data).unwrap_or_else(|_| "{}".to_string())
                        }],
                        "structuredContent": data,
                        "isError": false
                    }
                }),
                Err(error) => json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "content": [{
                            "type": "text",
                            "text": serde_json::to_string_pretty(&error).unwrap_or_else(|_| "{}".to_string())
                        }],
                        "structuredContent": error,
                        "isError": true
                    }
                }),
            }
        }
        _ => json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {
                "code": -32601,
                "message": "Method not found"
            }
        }),
    }
}

fn read_framed_message(
    reader: &mut BufReader<std::io::StdinLock<'_>>,
) -> Result<Option<Vec<u8>>, String> {
    loop {
        let mut headers = HashMap::new();

        loop {
            let mut line = String::new();
            let read = reader.read_line(&mut line).map_err(|error| error.to_string())?;

            if read == 0 {
                return Ok(None);
            }

            if line == "\r\n" || line == "\n" {
                if headers.is_empty() {
                    continue;
                }
                break;
            }

            if let Some((name, value)) = line.split_once(':') {
                headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_string());
            }
        }

        let length = match headers.get("content-length") {
            Some(value) => match value.parse::<usize>() {
                Ok(length) => length,
                Err(_) => continue,
            },
            None => continue,
        };

        let mut payload = vec![0_u8; length];
        reader
            .read_exact(&mut payload)
            .map_err(|error| error.to_string())?;

        return Ok(Some(payload));
    }
}

fn write_framed_message(writer: &mut std::io::StdoutLock<'_>, value: &Value) -> Result<(), String> {
    let payload = serde_json::to_vec(value).map_err(|error| error.to_string())?;
    let headers = format!("Content-Length: {}\r\n\r\n", payload.len());

    writer
        .write_all(headers.as_bytes())
        .map_err(|error| error.to_string())?;
    writer.write_all(&payload).map_err(|error| error.to_string())?;
    writer.flush().map_err(|error| error.to_string())?;

    Ok(())
}
