-- 008_lessons.sql — Lesson persistence for Trinity ID
-- Stores generated lesson plans, specs, HTML content, and SCORM export paths.

CREATE TABLE IF NOT EXISTS trinity_lessons (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    subject TEXT NOT NULL DEFAULT '',
    grade_band TEXT NOT NULL DEFAULT '',
    lesson_spec TEXT NOT NULL DEFAULT '{}',
    html_content TEXT NOT NULL DEFAULT '',
    scorm_path TEXT NOT NULL DEFAULT '',
    standards_aligned TEXT NOT NULL DEFAULT '[]',
    status TEXT NOT NULL DEFAULT 'draft',
    session_id TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_lessons_subject ON trinity_lessons (subject);
CREATE INDEX IF NOT EXISTS idx_lessons_grade ON trinity_lessons (grade_band);
CREATE INDEX IF NOT EXISTS idx_lessons_status ON trinity_lessons (status);
CREATE INDEX IF NOT EXISTS idx_lessons_created ON trinity_lessons (created_at DESC);
