pub async fn index() -> String {
    "Hello From the Server".to_owned()
}

pub async fn mirror_body_string(body: String) -> String {
    body
}
