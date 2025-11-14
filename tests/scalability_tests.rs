use rust_template::patterns::event_sourcing::{InMemoryEventStore, StoredEvent, EventStore};
use rust_template::patterns::cqrs::{CommandBus, QueryBus};
use rust_template::gameserver::{MatchmakingQueue, MatchmakingRequest, Leaderboard, GameSessionManager};
use chrono::Utc;

#[cfg(test)]
mod event_sourcing_tests {
    use super::*;

    #[test]
    fn test_append_and_get_events() {
        let store = InMemoryEventStore::new();

        let event1 = StoredEvent {
            id: "1".to_string(),
            aggregate_id: "user-123".to_string(),
            event_type: "UserCreated".to_string(),
            payload: serde_json::json!({"name": "John"}),
            version: 1,
            timestamp: Utc::now(),
        };

        let event2 = StoredEvent {
            id: "2".to_string(),
            aggregate_id: "user-123".to_string(),
            event_type: "UserUpdated".to_string(),
            payload: serde_json::json!({"name": "John Doe"}),
            version: 2,
            timestamp: Utc::now(),
        };

        store.append(event1.clone()).unwrap();
        store.append(event2.clone()).unwrap();

        let events = store.get_events("user-123").unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_type, "UserCreated");
        assert_eq!(events[1].event_type, "UserUpdated");
    }

    #[test]
    fn test_get_events_from_version() {
        let store = InMemoryEventStore::new();

        for i in 1..=5 {
            let event = StoredEvent {
                id: i.to_string(),
                aggregate_id: "user-123".to_string(),
                event_type: format!("Event{}", i),
                payload: serde_json::json!({}),
                version: i,
                timestamp: Utc::now(),
            };
            store.append(event).unwrap();
        }

        let events = store.get_events_since("user-123", 2).unwrap();
        assert_eq!(events.len(), 3); // versions 3, 4, 5 (> 2)
        assert_eq!(events[0].version, 3);
    }

    #[test]
    fn test_different_aggregates_isolated() {
        let store = InMemoryEventStore::new();

        let event1 = StoredEvent {
            id: "1".to_string(),
            aggregate_id: "user-1".to_string(),
            event_type: "UserCreated".to_string(),
            payload: serde_json::json!({}),
            version: 1,
            timestamp: Utc::now(),
        };

        let event2 = StoredEvent {
            id: "2".to_string(),
            aggregate_id: "user-2".to_string(),
            event_type: "UserCreated".to_string(),
            payload: serde_json::json!({}),
            version: 1,
            timestamp: Utc::now(),
        };

        store.append(event1).unwrap();
        store.append(event2).unwrap();

        assert_eq!(store.get_events("user-1").unwrap().len(), 1);
        assert_eq!(store.get_events("user-2").unwrap().len(), 1);
    }
}

#[cfg(test)]
mod cqrs_tests {
    use super::*;

    #[test]
    fn test_command_bus_creation() {
        let _command_bus = CommandBus::new();
        // Just test creation works
    }

    #[test]
    fn test_query_bus_creation() {
        let _query_bus = QueryBus::new();
        // Just test creation works
    }
}

#[cfg(test)]
mod matchmaking_tests {
    use super::*;

    #[test]
    fn test_add_player_to_queue() {
        let queue = MatchmakingQueue::new(100);
        
        let request = MatchmakingRequest {
            player_id: "player1".to_string(),
            skill_rating: 1500,
            requested_at: Utc::now(),
        };
        
        queue.add_player(request);
        assert_eq!(queue.queue_size(), 1);
    }

    #[test]
    fn test_find_match_insufficient_players() {
        let queue = MatchmakingQueue::new(100);
        
        queue.add_player(MatchmakingRequest {
            player_id: "player1".to_string(),
            skill_rating: 1500,
            requested_at: Utc::now(),
        });
        
        // Need 4 players, only have 1
        let result = queue.find_match(4);
        assert!(result.is_none());
    }

    #[test]
    fn test_find_match_success() {
        let queue = MatchmakingQueue::new(100);
        
        // Add 4 players with similar skill
        for i in 0..4 {
            queue.add_player(MatchmakingRequest {
                player_id: format!("player{}", i),
                skill_rating: 1500 + i * 10,
                requested_at: Utc::now(),
            });
        }
        
        let result = queue.find_match(4);
        assert!(result.is_some());
        
        let match_result = result.unwrap();
        assert_eq!(match_result.players.len(), 4);
        
        // Queue should be empty after match
        assert_eq!(queue.queue_size(), 0);
    }

    #[test]
    fn test_skill_based_matchmaking() {
        let queue = MatchmakingQueue::new(50); // skill range: 50
        
        // Add players with different skills
        queue.add_player(MatchmakingRequest {
            player_id: "player1".to_string(),
            skill_rating: 1000,
            requested_at: Utc::now(),
        });
        
        queue.add_player(MatchmakingRequest {
            player_id: "player2".to_string(),
            skill_rating: 1020,
            requested_at: Utc::now(),
        });
        
        queue.add_player(MatchmakingRequest {
            player_id: "player3".to_string(),
            skill_rating: 2000, // Too far from others
            requested_at: Utc::now(),
        });
        
        // Should not match because player3 is too far
        let result = queue.find_match(3);
        assert!(result.is_none());
    }
}

#[cfg(test)]
mod leaderboard_tests {
    use super::*;

    #[test]
    fn test_update_score() {
        let leaderboard = Leaderboard::new("global".to_string());
        
        leaderboard.update_score("player1".to_string(), 1000);
        leaderboard.update_score("player2".to_string(), 2000);
        
        let top = leaderboard.get_top(10);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].player_id, "player2"); // Higher score first
        assert_eq!(top[0].score, 2000);
        assert_eq!(top[0].rank, 1);
    }

    #[test]
    fn test_get_player_rank() {
        let leaderboard = Leaderboard::new("global".to_string());
        
        leaderboard.update_score("player1".to_string(), 1000);
        leaderboard.update_score("player2".to_string(), 2000);
        leaderboard.update_score("player3".to_string(), 1500);
        
        let rank = leaderboard.get_player_rank("player3");
        assert!(rank.is_some());
        assert_eq!(rank.unwrap().rank, 2); // 2nd place
    }

    #[test]
    fn test_score_update_changes_rank() {
        let leaderboard = Leaderboard::new("global".to_string());
        
        leaderboard.update_score("player1".to_string(), 1000);
        leaderboard.update_score("player2".to_string(), 2000);
        
        // player1 improves score
        leaderboard.update_score("player1".to_string(), 3000);
        
        let rank = leaderboard.get_player_rank("player1");
        assert_eq!(rank.unwrap().rank, 1); // Now first place
    }

    #[test]
    fn test_get_top_limit() {
        let leaderboard = Leaderboard::new("global".to_string());
        
        for i in 0..20 {
            leaderboard.update_score(format!("player{}", i), i * 100);
        }
        
        let top_5 = leaderboard.get_top(5);
        assert_eq!(top_5.len(), 5);
    }
}

#[cfg(test)]
mod game_session_tests {
    use super::*;

    #[test]
    fn test_create_session() {
        let manager = GameSessionManager::new();
        
        let session_id = manager.create_session(vec![
            "player1".to_string(),
            "player2".to_string(),
        ]);
        
        assert!(!session_id.is_empty());
        
        let session = manager.get_session(&session_id);
        assert!(session.is_some());
    }

    #[test]
    fn test_start_session() {
        let manager = GameSessionManager::new();
        
        let session_id = manager.create_session(vec!["player1".to_string()]);
        
        let success = manager.start_session(&session_id);
        assert!(success);
        
        let session = manager.get_session(&session_id).unwrap();
        assert_eq!(session.status, rust_template::gameserver::session::SessionStatus::InProgress);
        assert!(session.started_at.is_some());
    }

    #[test]
    fn test_end_session() {
        let manager = GameSessionManager::new();
        
        let session_id = manager.create_session(vec!["player1".to_string()]);
        manager.start_session(&session_id);
        
        let success = manager.end_session(&session_id);
        assert!(success);
        
        let session = manager.get_session(&session_id).unwrap();
        assert_eq!(session.status, rust_template::gameserver::session::SessionStatus::Completed);
        assert!(session.ended_at.is_some());
    }

    #[test]
    fn test_list_active_sessions() {
        let manager = GameSessionManager::new();
        
        let session1 = manager.create_session(vec!["player1".to_string()]);
        let session2 = manager.create_session(vec!["player2".to_string()]);
        
        manager.start_session(&session1);
        manager.start_session(&session2);
        manager.end_session(&session2);
        
        let active = manager.list_active_sessions();
        assert_eq!(active.len(), 1); // Only session1 is active
    }
}

