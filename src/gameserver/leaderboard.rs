use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

/// Leaderboard entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub player_id: String,
    pub score: i64,
    pub rank: usize,
}

/// Leaderboard
#[derive(Clone)]
pub struct Leaderboard {
    name: String,
    scores: Arc<RwLock<BTreeMap<i64, Vec<String>>>>,
}

impl Leaderboard {
    pub fn new(name: String) -> Self {
        Self {
            name,
            scores: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    pub fn update_score(&self, player_id: String, score: i64) {
        if let Ok(mut scores) = self.scores.write() {
            // Remove player from old score
            for players in scores.values_mut() {
                players.retain(|p| p != &player_id);
            }

            // Add player to new score
            scores.entry(score).or_insert_with(Vec::new).push(player_id);
        }
    }

    pub fn get_top(&self, limit: usize) -> Vec<LeaderboardEntry> {
        if let Ok(scores) = self.scores.read() {
            let mut entries = Vec::new();
            let mut rank = 1;

            for (score, players) in scores.iter().rev() {
                for player_id in players {
                    if entries.len() >= limit {
                        return entries;
                    }

                    entries.push(LeaderboardEntry {
                        player_id: player_id.clone(),
                        score: *score,
                        rank,
                    });
                    rank += 1;
                }
            }

            entries
        } else {
            Vec::new()
        }
    }

    pub fn get_player_rank(&self, player_id: &str) -> Option<LeaderboardEntry> {
        if let Ok(scores) = self.scores.read() {
            let mut rank = 1;

            for (score, players) in scores.iter().rev() {
                for pid in players {
                    if pid == player_id {
                        return Some(LeaderboardEntry {
                            player_id: player_id.to_string(),
                            score: *score,
                            rank,
                        });
                    }
                    rank += 1;
                }
            }

            None
        } else {
            None
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

