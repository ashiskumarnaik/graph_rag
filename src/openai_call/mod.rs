use async_openai::{Client, types::{CreateCompletionRequestArgs}};

pub async fn test_embedding_from_openai() -> String {

    let client = Client::new();


    let request = CreateCompletionRequestArgs::default()
        .model("gpt-3.5-turbo-instruct")
        .prompt("Tell me the recipe of alfredo pasta")
        .max_tokens(40_u32)
        .build()
        .unwrap();


    let response = client
        .completions()      // Get the API "group" (completions, images, etc.) from the client
        .create(request)    // Make the API call in that "group"
        .await
        .unwrap();

    let response_str = response.choices.first().unwrap().text.clone();
    response_str
}
