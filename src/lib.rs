use tokio::fs;

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

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_connection() {
        let request = ureq::get("https://127.0.0.1:7070/");
        let response = request.call();
        match response {
            Ok(r) => {
                assert_eq!(r.status(), 200);
                assert_eq!(r.into_string().unwrap(), "hello");
            },
            Err(e) => {
                panic!("failed to connect to server: {}", e);
            }
        } 
    }
}
