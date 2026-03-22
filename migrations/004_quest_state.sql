-- Quest State Persistence
-- Stores player progress through the Iron Road tutorial and quest system

CREATE TABLE IF NOT EXISTS quest_state (
    id SERIAL PRIMARY KEY,
    player_id TEXT NOT NULL DEFAULT 'default',
    chapter INT NOT NULL DEFAULT 1,
    phase TEXT NOT NULL DEFAULT 'analysis',
    xp INT NOT NULL DEFAULT 0,
    coal INT NOT NULL DEFAULT 100,
    steam INT NOT NULL DEFAULT 0,
    resonance INT NOT NULL DEFAULT 0,
    stats JSONB NOT NULL DEFAULT '{"strength":5,"agility":5,"wisdom":5,"charisma":5}'::jsonb,
    inventory JSONB NOT NULL DEFAULT '[]'::jsonb,
    completed_objectives JSONB NOT NULL DEFAULT '[]'::jsonb,
    active_quests JSONB NOT NULL DEFAULT '[]'::jsonb,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(player_id)
);

-- Index for fast player lookups
CREATE INDEX IF NOT EXISTS idx_quest_state_player ON quest_state(player_id);

-- Quest completion history
CREATE TABLE IF NOT EXISTS quest_history (
    id SERIAL PRIMARY KEY,
    player_id TEXT NOT NULL DEFAULT 'default',
    quest_id TEXT NOT NULL,
    quest_title TEXT NOT NULL,
    status TEXT NOT NULL, -- 'completed', 'failed', 'abandoned'
    xp_earned INT NOT NULL DEFAULT 0,
    duration_secs INT,
    completed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    results JSONB
);

-- Index for player quest history
CREATE INDEX IF NOT EXISTS idx_quest_history_player ON quest_history(player_id);
CREATE INDEX IF NOT EXISTS idx_quest_history_quest ON quest_history(quest_id);

-- Insert default player state if not exists
INSERT INTO quest_state (player_id, chapter, phase, xp, coal, steam, resonance, stats, inventory)
VALUES ('default', 1, 'analysis', 0, 100, 0, 0, 
    '{"strength":5,"agility":5,"wisdom":5,"charisma":5}'::jsonb,
    '[]'::jsonb)
ON CONFLICT (player_id) DO NOTHING;
