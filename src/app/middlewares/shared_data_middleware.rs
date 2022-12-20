#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn shared_data() -> SharedData {
    let shared_data = SharedData {
        message: "Shared Data Message".to_string(),
    };
    shared_data
}
