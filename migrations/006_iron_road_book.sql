-- Iron Road Book Schema
-- WHY: The Iron Road is the single source of truth for Trinity
-- HOW: PostgreSQL stores structured entries for queries, Great Recycler updates continuously

-- Book entries table - each entry is a piece of the narrative
CREATE TABLE IF NOT EXISTS iron_road_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Entry classification
    entry_type TEXT NOT NULL,  -- 'narrative_update', 'player_action', 'quest_completion', 'system_event'
    
    -- Content
    content TEXT NOT NULL,
    
    -- Structured metadata for queries
    metadata JSONB NOT NULL DEFAULT '{}',
    
    -- Chapter and resonance tracking
    chapter_number INTEGER NOT NULL DEFAULT 1,
    resonance_level INTEGER NOT NULL DEFAULT 1,
    
    -- Quest linkage (if applicable)
    quest_id UUID REFERENCES quest_states(id) ON DELETE SET NULL,
    
    -- Source tracking
    source TEXT NOT NULL DEFAULT 'great_recycler_npu',  -- 'great_recycler_npu', 'player', 'conductor', 'system'
    model_used TEXT,  -- 'granite-1b', 'gpt-oss-20b', etc.
    
    -- Full-text search
    search_vector TSVECTOR GENERATED ALWAYS AS (to_tsvector('english', content)) STORED
);

-- Index for full-text search
CREATE INDEX IF NOT EXISTS idx_iron_road_search ON iron_road_entries USING GIN(search_vector);

-- Index for chronological queries
CREATE INDEX IF NOT EXISTS idx_iron_road_chronological ON iron_road_entries(created_at DESC);

-- Index for entry type filtering
CREATE INDEX IF NOT EXISTS idx_iron_road_entry_type ON iron_road_entries(entry_type);

-- Index for chapter/resonance progression
CREATE INDEX IF NOT EXISTS idx_iron_road_progression ON iron_road_entries(chapter_number, resonance_level);

-- Index for quest linkage
CREATE INDEX IF NOT EXISTS idx_iron_road_quest ON iron_road_entries(quest_id) WHERE quest_id IS NOT NULL;

-- Book state table - tracks overall book progress
CREATE TABLE IF NOT EXISTS iron_road_state (
    id INTEGER PRIMARY KEY DEFAULT 1 CHECK (id = 1),  -- Singleton row
    
    -- Current progress
    current_chapter INTEGER NOT NULL DEFAULT 1,
    resonance_level INTEGER NOT NULL DEFAULT 1,
    total_entries INTEGER NOT NULL DEFAULT 0,
    
    -- Active quests
    active_quest_ids UUID[] DEFAULT '{}',
    
    -- Last update tracking
    last_recycler_update TIMESTAMPTZ,
    last_player_action TIMESTAMPTZ,
    
    -- NPU status
    npu_backend TEXT,  -- 'OnnxVitisAI', 'Cpu', 'None'
    npu_model_loaded TEXT,  -- Model name if loaded
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Initialize singleton state
INSERT INTO iron_road_state (id) VALUES (1) ON CONFLICT DO NOTHING;

-- Function to update state when entries are added
CREATE OR REPLACE FUNCTION update_iron_road_state()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE iron_road_state SET
        current_chapter = NEW.chapter_number,
        resonance_level = GREATEST(resonance_level, NEW.resonance_level),
        total_entries = total_entries + 1,
        updated_at = NOW();
    
    -- Update last_recycler_update or last_player_action based on source
    IF NEW.source = 'great_recycler_npu' THEN
        UPDATE iron_road_state SET last_recycler_update = NOW();
    ELSIF NEW.source = 'player' THEN
        UPDATE iron_road_state SET last_player_action = NOW();
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to auto-update state
CREATE TRIGGER trigger_iron_road_entry_insert
    AFTER INSERT ON iron_road_entries
    FOR EACH ROW
    EXECUTE FUNCTION update_iron_road_state();

-- View for recent entries (last 100)
CREATE OR REPLACE VIEW iron_road_recent AS
SELECT * FROM iron_road_entries
ORDER BY created_at DESC
LIMIT 100;

-- View for chapter summaries
CREATE OR REPLACE VIEW iron_road_chapters AS
SELECT 
    chapter_number,
    COUNT(*) as entry_count,
    MIN(created_at) as chapter_start,
    MAX(created_at) as chapter_end,
    array_agg(DISTINCT entry_type) as entry_types
FROM iron_road_entries
GROUP BY chapter_number
ORDER BY chapter_number;

-- View for resonance progression
CREATE OR REPLACE VIEW iron_road_progression AS
SELECT 
    resonance_level,
    COUNT(*) as events_at_level,
    MIN(created_at) as reached_at,
    array_agg(DISTINCT quest_id) as quests_completed
FROM iron_road_entries
WHERE entry_type = 'quest_completion' OR entry_type = 'player_action'
GROUP BY resonance_level
ORDER BY resonance_level;

-- Grant permissions (adjust as needed for your setup)
-- GRANT ALL ON iron_road_entries TO trinity;
-- GRANT ALL ON iron_road_state TO trinity;
-- GRANT ALL ON iron_road_recent TO trinity;
-- GRANT ALL ON iron_road_chapters TO trinity;
-- GRANT ALL ON iron_road_progression TO trinity;

COMMENT ON TABLE iron_road_entries IS 'The Iron Road book entries - single source of truth for Trinity';
COMMENT ON TABLE iron_road_state IS 'Current state of the Iron Road book (singleton)';
COMMENT ON VIEW iron_road_recent IS 'Most recent 100 entries for quick access';
COMMENT ON VIEW iron_road_chapters IS 'Chapter summaries with entry counts';
COMMENT ON VIEW iron_road_progression IS 'Player resonance level progression tracking';
