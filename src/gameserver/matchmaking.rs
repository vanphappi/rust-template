use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use chrono::{DateTime, Utc};

/// Matchmaking request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchmakingRequest {
    pub player_id: String,
    pub skill_rating: u32,
    pub requested_at: DateTime<Utc>,
}

/// Match result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    pub id: String,
    pub players: Vec<String>,
    pub created_at: DateTime<Utc>,
}

/// Matchmaking queue
#[derive(Clone)]
pub struct MatchmakingQueue {
    queue: Arc<RwLock<VecDeque<MatchmakingRequest>>>,
    skill_range: u32,
}

impl MatchmakingQueue {
    pub fn new(skill_range: u32) -> Self {
        Self {
            queue: Arc::new(RwLock::new(VecDeque::new())),
            skill_range,
        }
    }

    pub fn add_player(&self, request: MatchmakingRequest) {
        if let Ok(mut queue) = self.queue.write() {
            queue.push_back(request);
        }
    }

    pub fn find_match(&self, players_per_match: usize) -> Option<Match> {
        if let Ok(mut queue) = self.queue.write() {
            if queue.len() < players_per_match {
                return None;
            }

            // Simple matchmaking: take first N players with similar skill
            let first = queue.front()?;
            let mut matched_players = vec![first.player_id.clone()];
            let mut indices_to_remove = vec![0];

            for (i, req) in queue.iter().enumerate().skip(1) {
                if matched_players.len() >= players_per_match {
                    break;
                }

                let skill_diff = if req.skill_rating > first.skill_rating {
                    req.skill_rating - first.skill_rating
                } else {
                    first.skill_rating - req.skill_rating
                };

                if skill_diff <= self.skill_range {
                    matched_players.push(req.player_id.clone());
                    indices_to_remove.push(i);
                }
            }

            if matched_players.len() >= players_per_match {
                // Remove matched players from queue
                for &i in indices_to_remove.iter().rev() {
                    queue.remove(i);
                }

                Some(Match {
                    id: uuid::Uuid::new_v4().to_string(),
                    players: matched_players,
                    created_at: Utc::now(),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn queue_size(&self) -> usize {
        if let Ok(queue) = self.queue.read() {
            queue.len()
        } else {
            0
        }
    }
}

impl Default for MatchmakingQueue {
    fn default() -> Self {
        Self::new(100)
    }
}

