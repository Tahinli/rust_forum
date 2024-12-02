use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum InteractionType {
    Like,
    Dislike,
    Heart,
    Confetti,
    Plus,
    Minus,
    Rocket,
    Smile,
    Laugh,
    Sad,
    Shrug,
}
#[derive(Debug, Serialize, Deserialize)]
struct Interaction {
    post_id: String,
    user_id: String,
    interaction_type: InteractionType,
}
