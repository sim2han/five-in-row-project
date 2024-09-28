use super::game_queue;
use std::collections::VecDeque;

/// user information for matching
struct UserInfo {
    ip_address: String,
    user_name: String,
    rating: i32,
}

struct MatchQueue {
    queue: VecDeque<UserInfo>,
}

impl MatchQueue {
    /// Create a new match queue
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}

pub fn start_match_queue_thread() {}
