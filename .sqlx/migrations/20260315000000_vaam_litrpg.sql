-- VAAM-LitRPG Integration Schema
-- Trinity ID AI OS Database Extensions
-- March 15, 2026

-- ============================================================================
-- USER PROFILES
-- ============================================================================

-- Global user profiles (one user, multiple projects)
CREATE TABLE IF NOT EXISTS user_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE,
    display_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_active TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    metadata JSONB DEFAULT '{}'::jsonb
);

-- Index for quick user lookup
CREATE INDEX IF NOT EXISTS idx_user_profiles_user_id ON user_profiles(user_id);

-- ============================================================================
-- PROJECT PROFILES
-- ============================================================================

-- Project profiles (one project = one "player" with independent progress)
CREATE TABLE IF NOT EXISTS project_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_profile_id UUID NOT NULL REFERENCES user_profiles(id) ON DELETE CASCADE,
    project_id UUID NOT NULL UNIQUE,
    project_name VARCHAR(255) NOT NULL,
    workspace_path TEXT NOT NULL,
    genre VARCHAR(50) NOT NULL DEFAULT 'Cyberpunk',
    
    -- Character sheet fields
    current_coal REAL NOT NULL DEFAULT 100.0,
    total_coal_earned BIGINT NOT NULL DEFAULT 0,
    resonance_level INTEGER NOT NULL DEFAULT 1,
    user_class VARCHAR(100) NOT NULL DEFAULT 'SubjectMatterExpert',
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_active TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Settings
    settings JSONB DEFAULT '{}'::jsonb
);

-- Indexes for project lookups
CREATE INDEX IF NOT EXISTS idx_project_profiles_user ON project_profiles(user_profile_id);
CREATE INDEX IF NOT EXISTS idx_project_profiles_project_id ON project_profiles(project_id);

-- ============================================================================
-- VOCABULARY MASTERY
-- ============================================================================

-- Track discovered and mastered vocabulary per project
CREATE TABLE IF NOT EXISTS vocabulary_mastery (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_profile_id UUID NOT NULL REFERENCES project_profiles(id) ON DELETE CASCADE,
    word VARCHAR(255) NOT NULL,
    tier VARCHAR(50) NOT NULL,
    times_used INTEGER NOT NULL DEFAULT 0,
    is_mastered BOOLEAN NOT NULL DEFAULT FALSE,
    first_used_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ,
    total_coal_earned INTEGER NOT NULL DEFAULT 0,
    
    UNIQUE(project_profile_id, word)
);

-- Indexes for mastery queries
CREATE INDEX IF NOT EXISTS idx_vocab_mastery_project ON vocabulary_mastery(project_profile_id);
CREATE INDEX IF NOT EXISTS idx_vocab_mastery_mastered ON vocabulary_mastery(is_mastered) WHERE is_mastered = TRUE;

-- ============================================================================
-- WORD DETECTIONS (Audit Trail)
-- ============================================================================

-- Record each vocabulary detection for analytics
CREATE TABLE IF NOT EXISTS word_detections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_profile_id UUID NOT NULL REFERENCES project_profiles(id) ON DELETE CASCADE,
    word VARCHAR(255) NOT NULL,
    tier VARCHAR(50) NOT NULL,
    coal_earned INTEGER NOT NULL DEFAULT 0,
    is_correct_usage BOOLEAN NOT NULL DEFAULT TRUE,
    context TEXT,
    detected_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    session_id UUID
);

-- Index for session queries
CREATE INDEX IF NOT EXISTS idx_word_detections_project ON word_detections(project_profile_id);
CREATE INDEX IF NOT EXISTS idx_word_detections_session ON word_detections(session_id);
CREATE INDEX IF NOT EXISTS idx_word_detections_time ON word_detections(detected_at DESC);

-- ============================================================================
-- JOURNAL ENTRIES
-- ============================================================================

-- Journal entries (ADDIE git - the "why" train of thought)
CREATE TABLE IF NOT EXISTS journal_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_profile_id UUID NOT NULL REFERENCES project_profiles(id) ON DELETE CASCADE,
    entry_type VARCHAR(50) NOT NULL,
    addie_phase VARCHAR(50) NOT NULL DEFAULT 'Analysis',
    content TEXT NOT NULL,
    linked_quest_id VARCHAR(255),
    linked_files JSONB DEFAULT '[]'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Vocabulary moments
    vocabulary_word VARCHAR(255),
    is_mastery_moment BOOLEAN DEFAULT FALSE
);

-- Indexes for journal queries
CREATE INDEX IF NOT EXISTS idx_journal_project ON journal_entries(project_profile_id);
CREATE INDEX IF NOT EXISTS idx_journal_phase ON journal_entries(addie_phase);
CREATE INDEX IF NOT EXISTS idx_journal_quest ON journal_entries(linked_quest_id);
CREATE INDEX IF NOT EXISTS idx_journal_time ON journal_entries(created_at DESC);

-- ============================================================================
-- QUEST BOARD STATE
-- ============================================================================

-- Quest summaries per project
CREATE TABLE IF NOT EXISTS quest_states (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_profile_id UUID NOT NULL REFERENCES project_profiles(id) ON DELETE CASCADE,
    quest_id VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status VARCHAR(50) NOT NULL DEFAULT 'Available',
    coal_cost INTEGER NOT NULL DEFAULT 0,
    steam_progress INTEGER NOT NULL DEFAULT 0,
    steam_required INTEGER NOT NULL DEFAULT 100,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    
    UNIQUE(project_profile_id, quest_id)
);

-- Indexes for quest queries
CREATE INDEX IF NOT EXISTS idx_quest_project ON quest_states(project_profile_id);
CREATE INDEX IF NOT EXISTS idx_quest_status ON quest_states(status);

-- ============================================================================
-- SESSION STATS
-- ============================================================================

-- Track per-session statistics
CREATE TABLE IF NOT EXISTS session_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_profile_id UUID NOT NULL REFERENCES project_profiles(id) ON DELETE CASCADE,
    session_id UUID NOT NULL,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ended_at TIMESTAMPTZ,
    
    -- Stats
    coal_earned INTEGER NOT NULL DEFAULT 0,
    steam_burned INTEGER NOT NULL DEFAULT 0,
    words_detected INTEGER NOT NULL DEFAULT 0,
    words_mastered INTEGER NOT NULL DEFAULT 0,
    quests_completed INTEGER NOT NULL DEFAULT 0,
    
    UNIQUE(project_profile_id, session_id)
);

-- Index for session lookups
CREATE INDEX IF NOT EXISTS idx_session_project ON session_stats(project_profile_id);

-- ============================================================================
-- HELPER FUNCTIONS
-- ============================================================================

-- Update last_active timestamp on project access
CREATE OR REPLACE FUNCTION touch_project(profile_id UUID)
RETURNS VOID AS $$
BEGIN
    UPDATE project_profiles 
    SET last_active = NOW() 
    WHERE id = profile_id;
END;
$$ LANGUAGE plpgsql;

-- Get vocabulary mastery progress by tier
CREATE OR REPLACE FUNCTION get_vocab_progress(profile_id UUID)
RETURNS TABLE(tier VARCHAR, total_words BIGINT, mastered_words BIGINT, percentage REAL) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        vm.tier,
        COUNT(*) as total_words,
        SUM(CASE WHEN vm.is_mastered THEN 1 ELSE 0 END) as mastered_words,
        CASE WHEN COUNT(*) > 0 
            THEN (SUM(CASE WHEN vm.is_mastered THEN 1 ELSE 0 END)::REAL / COUNT(*)::REAL) * 100.0
            ELSE 0.0 
        END as percentage
    FROM vocabulary_mastery vm
    WHERE vm.project_profile_id = profile_id
    GROUP BY vm.tier;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- SAMPLE DATA (for testing)
-- ============================================================================

-- Insert a test user and project
INSERT INTO user_profiles (user_id, display_name)
VALUES ('00000000-0000-0000-0000-000000000001', 'Test Conductor')
ON CONFLICT (user_id) DO NOTHING;

INSERT INTO project_profiles (user_profile_id, project_id, project_name, workspace_path, genre)
SELECT 
    id,
    '00000000-0000-0000-0000-000000000001',
    'Trinity Genesis',
    '/home/joshua/Workflow/desktop_trinity/trinity-genesis',
    'Cyberpunk'
FROM user_profiles 
WHERE user_id = '00000000-0000-0000-0000-000000000001'
ON CONFLICT (project_id) DO NOTHING;

-- ============================================================================
-- GRANTS (adjust as needed for your deployment)
-- ============================================================================

-- GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO trinity_user;
-- GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO trinity_user;
