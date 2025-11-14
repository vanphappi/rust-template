-- Create events table for Event Sourcing pattern
CREATE TABLE IF NOT EXISTS events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    aggregate_id VARCHAR(255) NOT NULL,
    event_type VARCHAR(255) NOT NULL,
    payload JSONB NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    version BIGINT NOT NULL,
    
    -- Ensure unique version per aggregate (optimistic locking)
    CONSTRAINT unique_aggregate_version UNIQUE (aggregate_id, version)
);

-- Index for fast aggregate event retrieval
CREATE INDEX IF NOT EXISTS idx_events_aggregate_id ON events(aggregate_id);

-- Index for version-based queries
CREATE INDEX IF NOT EXISTS idx_events_aggregate_version ON events(aggregate_id, version);

-- Index for event type queries (useful for projections)
CREATE INDEX IF NOT EXISTS idx_events_event_type ON events(event_type);

-- Index for timestamp-based queries (temporal queries)
CREATE INDEX IF NOT EXISTS idx_events_timestamp ON events(timestamp);

-- Composite index for efficient event replay
CREATE INDEX IF NOT EXISTS idx_events_aggregate_timestamp ON events(aggregate_id, timestamp);

