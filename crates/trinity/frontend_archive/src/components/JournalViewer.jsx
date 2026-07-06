/**
 * JournalViewer.jsx — Journal States UI Component
 *
 * Displays journal entries (phase milestones, weekly reflections)
 * in a scrollable timeline view with export capability.
 *
 * Architecture:
 *   - Fetches entries from GET /api/journal
 *   - Shows timeline of snapshots with icons by type
 *   - Click entry → expand to show full snapshot
 *   - Export button → opens HTML export in new tab
 *   - "New Reflection" button → POST /api/journal with user text
 */

import { useState, useEffect, useCallback } from 'react';

export default function JournalViewer({ onClose }) {
  const [entries, setEntries] = useState([]);
  const [expanded, setExpanded] = useState(null);
  const [loading, setLoading] = useState(true);
  const [showReflection, setShowReflection] = useState(false);
  const [reflectionText, setReflectionText] = useState('');
  const [saving, setSaving] = useState(false);

  // Load entries
  const loadEntries = useCallback(async () => {
    setLoading(true);
    try {
      const res = await fetch('/api/journal');
      if (res.ok) {
        const data = await res.json();
        setEntries(data);
      }
    } catch { /* offline */ }
    setLoading(false);
  }, []);

  useEffect(() => { loadEntries(); }, [loadEntries]);

  // Create weekly reflection
  const submitReflection = async () => {
    if (!reflectionText.trim()) return;
    setSaving(true);
    try {
      const res = await fetch('/api/journal', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          entry_type: 'weekly_reflection',
          reflection: reflectionText,
          tags: ['weekly'],
        }),
      });
      if (res.ok) {
        setReflectionText('');
        setShowReflection(false);
        loadEntries();
      }
    } catch { /* offline */ }
    setSaving(false);
  };

  // Create demo bookmark
  const createBookmark = async () => {
    try {
      await fetch('/api/journal', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ entry_type: 'demo_bookmark', tags: ['demo'] }),
      });
      loadEntries();
    } catch { /* offline */ }
  };

  const typeIcons = {
    PhaseComplete: '🚉',
    ChapterComplete: '🏆',
    WeeklyReflection: '📓',
    ManualCheckpoint: '📌',
    DemoBookmark: '🎬',
  };

  const formatTime = (ts) => {
    try {
      return new Date(ts).toLocaleDateString('en-US', {
        month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit',
      });
    } catch { return ts; }
  };

  return (
    <div className="journal-viewer">
      <div className="journal-header">
        <h3 className="journal-title">📜 Journal</h3>
        <div className="journal-actions">
          <button
            className="journal-btn journal-btn-reflect"
            onClick={() => setShowReflection(!showReflection)}
          >📓 Reflect</button>
          <button
            className="journal-btn journal-btn-bookmark"
            onClick={createBookmark}
          >🎬 Bookmark</button>
          {onClose && (
            <button className="journal-btn journal-btn-close" onClick={onClose}>✕</button>
          )}
        </div>
      </div>

      {/* Reflection input */}
      {showReflection && (
        <div className="journal-reflection-input">
          <textarea
            className="journal-reflection-textarea"
            placeholder="What did you learn this week? What challenges did you overcome?"
            value={reflectionText}
            onChange={e => setReflectionText(e.target.value)}
            rows={4}
          />
          <button
            className="journal-btn journal-btn-save"
            onClick={submitReflection}
            disabled={saving || !reflectionText.trim()}
          >{saving ? '...' : 'Save Reflection'}</button>
        </div>
      )}

      {/* Timeline */}
      <div className="journal-timeline">
        {loading && <div className="journal-loading">Loading journal...</div>}
        {!loading && entries.length === 0 && (
          <div className="journal-empty">
            No journal entries yet. Complete a phase or write a reflection to begin!
          </div>
        )}
        {entries.map((entry, i) => (
          <div
            key={entry.id}
            className={`journal-entry ${expanded === entry.id ? 'expanded' : ''}`}
            onClick={() => setExpanded(expanded === entry.id ? null : entry.id)}
          >
            <div className="journal-entry-header">
              <span className="journal-entry-icon">
                {typeIcons[entry.entry_type] || '📌'}
              </span>
              <span className="journal-entry-summary">
                {entry.summary.length > 80 ? entry.summary.slice(0, 80) + '...' : entry.summary}
              </span>
              <span className="journal-entry-time">{formatTime(entry.timestamp)}</span>
            </div>

            {expanded === entry.id && (
              <div className="journal-entry-detail">
                <div className="journal-detail-grid">
                  <div className="journal-stat">
                    <span className="journal-stat-value">{entry.quest?.phase || '—'}</span>
                    <span className="journal-stat-label">Phase</span>
                  </div>
                  <div className="journal-stat">
                    <span className="journal-stat-value">{entry.character?.resonance || 0}</span>
                    <span className="journal-stat-label">Resonance</span>
                  </div>
                  <div className="journal-stat">
                    <span className="journal-stat-value">{entry.quest?.xp || 0}</span>
                    <span className="journal-stat-label">XP</span>
                  </div>
                  <div className="journal-stat">
                    <span className="journal-stat-value">{entry.quest?.completed_phases?.length || 0}</span>
                    <span className="journal-stat-label">Phases Done</span>
                  </div>
                </div>

                {entry.reflection && (
                  <div className="journal-reflection-text">
                    <strong>Reflection:</strong> {entry.reflection}
                  </div>
                )}

                {entry.tags?.length > 0 && (
                  <div className="journal-tags">
                    {entry.tags.map(t => (
                      <span key={t} className="journal-tag">{t}</span>
                    ))}
                  </div>
                )}

                <div className="journal-entry-actions">
                  <a
                    href={`/api/journal/export/${entry.id}`}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="journal-btn journal-btn-export"
                    onClick={e => e.stopPropagation()}
                  >📄 Export HTML</a>
                </div>
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
