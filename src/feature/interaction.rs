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
}
struct Interaction {
    post_id:String,
    user_id:String,
    interaction_type: InteractionType,
}