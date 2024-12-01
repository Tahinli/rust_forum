pub struct Message {
    pub sender_email: String,
    pub receiver_email: String,
    pub message: String,
}

impl Message {
    pub async fn new(sender_email: String, receiver_email: String, message: String) -> Self{
        Message {
            sender_email,
            receiver_email,
            message,
        }
    }
}