import React, { useState, useRef, useEffect } from 'react';

const IS_HOSTED = window.location.pathname.startsWith('/trinity');

export default function NavBar({ quest, activeTab, onTabChange, onNewJourney }) {
  const tabs = [
    { id: 'voice',     label: '📖 Story Mode' },
    { id: 'ironroad',  label: '🚂 Iron Road' },
    { id: 'art',       label: '🎨 ART Studio' },
    { id: 'character', label: '🎭 Character' },
    { id: 'yard',      label: '⚙️ Yardmaster' },
  ];

  // Help Menu — The Four Chariots
  const [helpOpen, setHelpOpen] = useState(false);
  const helpRef = useRef(null);

  useEffect(() => {
    const handleClickOutside = (e) => {
      if (helpRef.current && !helpRef.current.contains(e.target)) {
        setHelpOpen(false);
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const chariots = [
    { emoji: '🔮', label: 'The Bible',            desc: 'Technical spec — every line explained',        file: 'TRINITY_FANCY_BIBLE.md' },
    { emoji: '🎮', label: "Player's Handbook",    desc: 'The philosophy behind the game',               file: 'PLAYERS_HANDBOOK.md' },
    { emoji: '🤝', label: 'Field Manual',          desc: 'How Pete works — what to expect',              file: 'ASK_PETE_FIELD_MANUAL.md' },
    { emoji: '🎓', label: 'Professor Programming', desc: 'Standards, privacy, institutional eval',  file: 'PROFESSOR.md' },
  ];

  return (
    <nav className="nav">
      {IS_HOSTED && (
        <a
          href="/"
          style={{
            display: 'flex', alignItems: 'center', gap: '6px',
            padding: '4px 12px', borderRadius: '6px', marginRight: '8px',
            fontSize: '11px', fontFamily: "'Cinzel', serif",
            letterSpacing: '1px', textTransform: 'uppercase',
            color: '#CFB991', textDecoration: 'none',
            background: 'rgba(207,185,145,0.06)',
            border: '1px solid rgba(207,185,145,0.12)',
            transition: 'all 0.2s',
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.background = 'rgba(207,185,145,0.12)';
            e.currentTarget.style.borderColor = 'rgba(207,185,145,0.3)';
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.background = 'rgba(207,185,145,0.06)';
            e.currentTarget.style.borderColor = 'rgba(207,185,145,0.12)';
          }}
        >
          ← Portfolio
        </a>
      )}
      <div className="nav-brand">TRINITY</div>
      <div className="nav-links">
        {tabs.map((t) => (
          <button
            key={t.id}
            id={`nav-${t.id}`}
            className={`nav-link ${activeTab === t.id ? 'active' : ''}`}
            onClick={() => onTabChange(t.id)}
          >
            {t.label}
          </button>
        ))}
      </div>
      <div className="nav-status">
        {/* Help Menu */}
        <div ref={helpRef} style={{ position: 'relative', display: 'inline-block' }}>
          <button
            id="nav-help-btn"
            className="nav-link"
            onClick={() => setHelpOpen(!helpOpen)}
            style={{ fontSize: '16px', cursor: 'pointer', padding: '4px 8px' }}
            title="The Four Chariots — Documentation"
          >
            📚
          </button>
          <button
            id="nav-author-btn"
            className={`nav-link ${activeTab === 'portfolio' ? 'active' : ''}`}
            onClick={() => onTabChange('portfolio')}
            style={{ fontSize: '14px', cursor: 'pointer', padding: '4px 10px', marginLeft: '4px' }}
            title="Author Portfolio — Joshua Atkinson, LDT @ Purdue"
          >
            👤
          </button>
          {helpOpen && (
            <div style={{
              position: 'absolute', right: 0, top: '100%', marginTop: '8px',
              width: '320px', background: 'rgba(24, 22, 18, 0.96)',
              border: '1px solid rgba(207, 185, 145, 0.2)',
              borderRadius: '10px', padding: '12px', zIndex: 1000,
              backdropFilter: 'blur(16px)',
              boxShadow: '0 8px 32px rgba(0,0,0,0.5)',
            }}>
              <div style={{
                fontFamily: "'Cinzel', serif", fontSize: '11px',
                color: '#CFB991', letterSpacing: '2px', textTransform: 'uppercase',
                marginBottom: '10px', paddingBottom: '8px',
                borderBottom: '1px solid rgba(207, 185, 145, 0.1)',
              }}>
                The Four Chariots
              </div>
              {chariots.map((c) => (
                <button
                  key={c.file}
                  onClick={() => {
                    onTabChange(`chariot:${c.file}`);
                    setHelpOpen(false);
                  }}
                  style={{
                    display: 'flex', gap: '10px', alignItems: 'flex-start',
                    padding: '8px', borderRadius: '6px', textDecoration: 'none',
                    color: '#E2E8F0', marginBottom: '4px',
                    transition: 'background 0.15s',
                    background: 'transparent', border: 'none',
                    cursor: 'pointer', width: '100%', textAlign: 'left',
                  }}
                  onMouseEnter={(e) => e.currentTarget.style.background = 'rgba(207, 185, 145, 0.08)'}
                  onMouseLeave={(e) => e.currentTarget.style.background = 'transparent'}
                >
                  <span style={{ fontSize: '18px', lineHeight: 1 }}>{c.emoji}</span>
                  <div>
                    <div style={{
                      fontFamily: "'Inter', sans-serif", fontSize: '13px',
                      fontWeight: 600, color: '#E2E8F0',
                    }}>
                      {c.label}
                    </div>
                    <div style={{
                      fontFamily: "'Inter', sans-serif", fontSize: '11px',
                      color: '#6B7280', marginTop: '2px',
                    }}>
                      {c.desc}
                    </div>
                  </div>
                </button>
              ))}
              {/* Divider + Hook Book — separate from the metaphysical Four Chariots */}
              <div style={{
                marginTop: '8px', paddingTop: '8px',
                borderTop: '1px solid rgba(207, 185, 145, 0.1)',
              }}>
                <div style={{
                  fontFamily: "'Cinzel', serif", fontSize: '10px',
                  color: '#CFB991', letterSpacing: '2px', textTransform: 'uppercase',
                  marginBottom: '6px', opacity: 0.7,
                }}>
                  Spell Book
                </div>
                <button
                  onClick={() => {
                    onTabChange('chariot:HOOK_BOOK.md');
                    setHelpOpen(false);
                  }}
                  style={{
                    display: 'flex', gap: '10px', alignItems: 'flex-start',
                    padding: '8px', borderRadius: '6px',
                    color: '#E2E8F0', background: 'transparent', border: 'none',
                    cursor: 'pointer', width: '100%', textAlign: 'left',
                    transition: 'background 0.15s',
                  }}
                  onMouseEnter={(e) => e.currentTarget.style.background = 'rgba(207, 185, 145, 0.08)'}
                  onMouseLeave={(e) => e.currentTarget.style.background = 'transparent'}
                >
                  <span style={{ fontSize: '18px', lineHeight: 1 }}>📖</span>
                  <div>
                    <div style={{
                      fontFamily: "'Inter', sans-serif", fontSize: '13px',
                      fontWeight: 600, color: '#E2E8F0',
                    }}>
                      The Hook Book
                    </div>
                    <div style={{
                      fontFamily: "'Inter', sans-serif", fontSize: '11px',
                      color: '#6B7280', marginTop: '2px',
                    }}>
                      Every capability — the spell book
                    </div>
                  </div>
                </button>
              </div>
            </div>
          )}
        </div>

        <span className="sep">|</span>
        <span className={`status-dot ${quest?.phase ? 'connected' : ''}`}></span>
        <span>{quest?.phase || 'awaiting…'}</span>
        <span className="sep">|</span>
        <span>Ch {quest?.chapter || '—'}</span>
        {quest?.subject && onNewJourney && (
          <>
            <span className="sep">|</span>
            <button
              id="nav-new-journey-btn"
              className="nav-new-journey"
              onClick={onNewJourney}
            >
              ↺ NEW JOURNEY
            </button>
          </>
        )}
      </div>
    </nav>
  );
}
