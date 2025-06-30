use std::sync::Arc;

use async_openai::{
    config::OpenAIConfig, types::{ChatCompletionRequestMessage, ChatCompletionRequestUserMessageArgs, ChatCompletionToolArgs, ChatCompletionToolChoiceOption, CreateChatCompletionRequest, FunctionObject}, Client
};
use rpc_router::{router_builder, RpcParams};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let oa_client: Arc<Client<OpenAIConfig>> = Client::new().into(); // requires OPENAI_API_KEY env. var.
    let chat_client = oa_client.chat();
    let model = "gpt-4.1-mini";

    let question = "Give me all products for XXX from 2025/01/01 to 2025/05/01";

	let messages: Vec<ChatCompletionRequestMessage> = vec![
        ChatCompletionRequestUserMessageArgs::default().content(question).build()?.into()
    ];

    let tool_partner_products_report = ChatCompletionToolArgs::default()
        .function(FunctionObject {
            name: "get_products".to_string(),
            description: Some("get products for given partner".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "partner": {
                        "type": "string",
                        "description": "The name of a parnter, e.g. XXX, YYY"
                    },
                    "startDate": {
                        "type": "string",
                        "description": "Start date of the period to search"
                    },
                    "endDate": {
                        "type": "string",
                        "description": "Start date of the period to search"
                    },
                },
                "required": ["partner"],
            })),
            strict: None,
        }).build()?;

    let tools = Some(vec![
        tool_partner_products_report
    ]);

	let rpc_router = router_builder![get_products].build();

	let msg_req = CreateChatCompletionRequest {
		model: model.to_string(),
		messages,
        tools: tools.clone(),
        tool_choice: Some(ChatCompletionToolChoiceOption::Auto),
		..Default::default()
	};

	let chat_response = chat_client.create(msg_req).await?;

    println!("\nRAW RESPONSE: {chat_response:?}\n");

    let first_choice = chat_response
        .choices
        .into_iter()
        .next()
        .unwrap();

    if let Some(response_content) = first_choice.message.content {
        println!("\nResponse early (no tools):\n\n{response_content}");
        return Ok(());
    }

    if let Some(tool_calls) = first_choice.message.tool_calls {

		for tool_call in tool_calls {
			println!(r#"function: '{}'; arguments: {}"#, tool_call.function.name, tool_call.function.arguments);


            let _tool_call_id = tool_call.id.clone();
            let fn_name = tool_call.function.name.clone();
            let params: Value = serde_json::from_str(&tool_call.function.arguments)?;
    
            // Execute with rpc_router
            let call_result = rpc_router.call_route(None, fn_name, Some(params)).await?;
            let response = call_result.value;
    
            println!("Tool call result: {response:?}");
		}
	}

    Ok(())
}


#[allow(unused)] // Will be passthrough API
#[derive(Debug, Deserialize, RpcParams)]
struct ProductsArgs {
	partner: String,
	start_date: Option<String>,
	end_date: Option<String>,
}

#[derive(Serialize)]
struct ProductsResult {
    result_url: String,
}

async fn get_products(args: ProductsArgs) -> Result<ProductsResult, String> {
    println!("CALLED get_products: {} {:?} - {:?}", args.partner, args.start_date, args.end_date);

    Ok(ProductsResult{ result_url: "s3://results/result1".to_string() })
}
