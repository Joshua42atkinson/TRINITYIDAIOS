-- Character Sheet VAAM Integration
-- Extends character data with genre, vocabulary packs, and party configuration

-- Vocabulary Packs - User-created vocabulary for VAAM system
CREATE TABLE IF NOT EXISTS vocabulary_packs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    genre TEXT NOT NULL DEFAULT 'Cyberpunk',
    name TEXT NOT NULL,
    description TEXT,
    words JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for user's vocabulary packs
CREATE INDEX IF NOT EXISTS idx_vocabulary_packs_user ON vocabulary_packs(user_id);
CREATE INDEX IF NOT EXISTS idx_vocabulary_packs_genre ON vocabulary_packs(genre);

-- Party Configurations - AI party setup for concurrent model execution
CREATE TABLE IF NOT EXISTS party_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    character_sheet_id UUID,
    memory_budget_gb INT NOT NULL DEFAULT 24,
    is_customized BOOLEAN NOT NULL DEFAULT FALSE,
    roles JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- One active config per user
    UNIQUE(user_id)
);

-- Index for party config lookups
CREATE INDEX IF NOT EXISTS idx_party_configs_user ON party_configs(user_id);

-- Model Registry - Available AI models for party assignment
CREATE TABLE IF NOT EXISTS model_registry (
    id TEXT PRIMARY KEY, -- e.g., 'gpt-oss-20b', 'reap-25b'
    display_name TEXT NOT NULL,
    model_path TEXT NOT NULL,
    memory_gb INT NOT NULL,
    context_length INT NOT NULL DEFAULT 4096,
    is_downloaded BOOLEAN NOT NULL DEFAULT FALSE,
    download_url TEXT,
    model_type TEXT NOT NULL DEFAULT 'gguf', -- 'gguf', 'onnx', 'safetensors'
    capabilities JSONB NOT NULL DEFAULT '[]'::jsonb, -- ['code', 'chat', 'vision', etc.]
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed the model registry with known models
INSERT INTO model_registry (id, display_name, model_path, memory_gb, context_length, is_downloaded, model_type, capabilities)
VALUES 
    ('gpt-oss-20b', 'GPT-OSS 20B (Conductor)', 'models/yardmaster/', 12, 8192, true, 'gguf', '["chat", "orchestration"]'::jsonb),
    ('reap-25b', 'REAP 25B (Engineer)', 'models/engineer/', 15, 16384, true, 'gguf', '["code", "analysis"]'::jsonb),
    ('opus-27b', 'Opus 27B (Quality)', 'models/opus/', 21, 65536, true, 'gguf', '["reasoning", "evaluation"]'::jsonb),
    ('qwen-35b', 'Qwen 35B (Vision)', 'models/visionary/', 20, 32768, true, 'gguf', '["vision", "chat"]'::jsonb),
    ('minimax-50b', 'MiniMax 50B (Colossus)', 'models/colossus/', 30, 32768, false, 'gguf', '["reasoning", "analysis"]'::jsonb),
    ('personaplex-7b', 'PersonaPlex 7B (Voice)', 'models/personaplex/', 14, 4096, true, 'onnx', '["voice", "audio"]'::jsonb)
ON CONFLICT (id) DO UPDATE SET
    display_name = EXCLUDED.display_name,
    model_path = EXCLUDED.model_path,
    memory_gb = EXCLUDED.memory_gb,
    context_length = EXCLUDED.context_length,
    updated_at = NOW();

-- Character Sheet Extensions - Add VAAM fields
-- Note: Character sheets are stored as JSON files in ~/.trinity/
-- This table provides a database mirror for server-side operations
CREATE TABLE IF NOT EXISTS character_sheets (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    alias TEXT NOT NULL,
    user_class TEXT NOT NULL DEFAULT 'Architect',
    genre TEXT NOT NULL DEFAULT 'Cyberpunk',
    vocabulary_pack_id UUID REFERENCES vocabulary_packs(id),
    party_config_id UUID REFERENCES party_configs(id),
    resonance_level INT NOT NULL DEFAULT 1,
    total_xp BIGINT NOT NULL DEFAULT 0,
    current_coal REAL NOT NULL DEFAULT 100.0,
    mana_pool_vram INT NOT NULL DEFAULT 24,
    stamina_ram INT NOT NULL DEFAULT 64,
    agility_compute INT NOT NULL DEFAULT 8,
    concurrency_mode TEXT NOT NULL DEFAULT 'LoneWolf',
    skills JSONB NOT NULL DEFAULT '{}'::jsonb,
    completed_contracts JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(user_id)
);

-- Index for character sheet lookups
CREATE INDEX IF NOT EXISTS idx_character_sheets_user ON character_sheets(user_id);
CREATE INDEX IF NOT EXISTS idx_character_sheets_vocabulary ON character_sheets(vocabulary_pack_id);
CREATE INDEX IF NOT EXISTS idx_character_sheets_party ON character_sheets(party_config_id);

-- Vocabulary Mastery Tracking - Track which words user has mastered
CREATE TABLE IF NOT EXISTS vocabulary_mastery (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL,
    word TEXT NOT NULL,
    tier TEXT NOT NULL,
    times_used INT NOT NULL DEFAULT 0,
    is_mastered BOOLEAN NOT NULL DEFAULT FALSE,
    first_seen_at TIMESTAMPTZ,
    mastered_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ,
    
    UNIQUE(user_id, word)
);

-- Index for mastery lookups
CREATE INDEX IF NOT EXISTS idx_vocabulary_mastery_user ON vocabulary_mastery(user_id);
CREATE INDEX IF NOT EXISTS idx_vocabulary_mastery_mastered ON vocabulary_mastery(user_id, is_mastered);

-- Function to update timestamps on update
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply timestamp triggers
CREATE TRIGGER update_vocabulary_packs_updated_at
    BEFORE UPDATE ON vocabulary_packs
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_party_configs_updated_at
    BEFORE UPDATE ON party_configs
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_character_sheets_updated_at
    BEFORE UPDATE ON character_sheets
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_model_registry_updated_at
    BEFORE UPDATE ON model_registry
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
