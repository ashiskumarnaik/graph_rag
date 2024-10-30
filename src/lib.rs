pub mod openai_call;

async fn main() {
    let response= openai_call::test_embedding_from_openai().await;
    println!("{0}", response);
}



#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_embedding() {

        main().await;

    }


}
