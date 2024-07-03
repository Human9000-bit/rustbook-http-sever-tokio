pub struct Page {
    pub contents: String,
    pub length: usize
}

impl Page {
    pub async fn new(filename: &str) -> Page {
        let conts = tokio::fs::read_to_string(filename).await.unwrap();
        let len = conts.len();
        Page {
            contents: conts,
            length: len
        }
    }
}