import React, { useState, useEffect, useRef } from 'react';
import { marked } from 'marked';

const DOC_META = {
  'TRINITY_FANCY_BIBLE.md': {
    title: 'The Bible',
    emoji: '🔮',
    subtitle: 'Full Technical Specification — Every Line Explained',
    accent: '#CFB991', // Purdue gold
  },
  'PLAYERS_HANDBOOK.md': {
    title: "The Player's Handbook",
    emoji: '🎮',
    subtitle: 'A Guide to Conscious Learning in Trinity ID AI OS',
    accent: '#CFB991',
  },
  'ASK_PETE_FIELD_MANUAL.md': {
    title: 'The Field Manual',
    emoji: '🤝',
    subtitle: "How Pete Works — What to Expect",
    accent: '#CFB991',
  },
  'PROFESSOR.md': {
    title: 'Professor Programming',
    emoji: '🎓',
    subtitle: 'Sovereign Cognitive Engine · Institutional Scale & Theory',
    accent: '#CFB991',
  },
};

// Configure marked for GitHub-flavored markdown
marked.setOptions({
  gfm: true,
  breaks: false,
});

export default function ChariotViewer({ filename, onBack }) {
  const [html, setHtml] = useState('');
  const [loading, setLoading] = useState(true);
  const [scrollProgress, setScrollProgress] = useState(0);
  const contentRef = useRef(null);
  const meta = DOC_META[filename] || { title: filename, emoji: '📄', subtitle: '', accent: '#CFB991' };

  useEffect(() => {
    setLoading(true);
    fetch(`/docs/${filename}`)
      .then((r) => r.text())
      .then((md) => {
        setHtml(marked.parse(md));
        setLoading(false);
      })
      .catch(() => {
        setHtml('<p style="color:#ef4444">Failed to load document.</p>');
        setLoading(false);
      });
  }, [filename]);

  // Track scroll progress
  useEffect(() => {
    const el = contentRef.current;
    if (!el) return;
    const handleScroll = () => {
      const pct = el.scrollTop / (el.scrollHeight - el.clientHeight);
      setScrollProgress(Math.min(1, Math.max(0, pct)));
    };
    el.addEventListener('scroll', handleScroll);
    return () => el.removeEventListener('scroll', handleScroll);
  }, [html]);

  return (
    <div style={{
      gridColumn: '1 / -1', gridRow: 2,
      display: 'flex', flexDirection: 'column',
      overflow: 'hidden', background: '#0F0D0A',
    }}>
      {/* Progress bar */}
      <div style={{
        height: '2px', background: 'rgba(207, 185, 145, 0.1)',
        position: 'relative', flexShrink: 0,
      }}>
        <div style={{
          height: '100%', background: meta.accent,
          width: `${scrollProgress * 100}%`,
          transition: 'width 0.1s ease-out',
        }} />
      </div>

      {/* Document header */}
      <header style={{
        padding: '20px 40px', flexShrink: 0,
        display: 'flex', alignItems: 'center', gap: '16px',
        borderBottom: '1px solid rgba(207, 185, 145, 0.1)',
        background: 'rgba(15, 13, 10, 0.95)',
      }}>
        <button
          onClick={onBack}
          style={{
            padding: '6px 14px', borderRadius: '6px',
            background: 'transparent',
            border: '1px solid rgba(207, 185, 145, 0.2)',
            color: '#CFB991', cursor: 'pointer',
            fontFamily: "'Inter', sans-serif", fontSize: '13px',
            transition: 'all 0.15s',
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.background = 'rgba(207, 185, 145, 0.1)';
            e.currentTarget.style.borderColor = 'rgba(207, 185, 145, 0.4)';
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.background = 'transparent';
            e.currentTarget.style.borderColor = 'rgba(207, 185, 145, 0.2)';
          }}
        >
          ← Back
        </button>

        <div style={{ flex: 1 }}>
          <div style={{
            fontFamily: "'Cinzel', serif", fontSize: '20px',
            color: meta.accent, letterSpacing: '2px',
          }}>
            {meta.emoji} {meta.title}
          </div>
          <div style={{
            fontSize: '12px', color: '#6B7280', marginTop: '2px',
            fontFamily: "'Inter', sans-serif",
          }}>
            {meta.subtitle}
          </div>
        </div>

        <a
          href={`/docs/${filename}`}
          download={filename}
          style={{
            padding: '6px 14px', borderRadius: '6px',
            background: 'rgba(207, 185, 145, 0.08)',
            border: '1px solid rgba(207, 185, 145, 0.2)',
            color: '#CFB991', cursor: 'pointer',
            fontFamily: "'Inter', sans-serif", fontSize: '12px',
            textDecoration: 'none', transition: 'all 0.15s',
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.background = 'rgba(207, 185, 145, 0.15)';
            e.currentTarget.style.borderColor = 'rgba(207, 185, 145, 0.4)';
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.background = 'rgba(207, 185, 145, 0.08)';
            e.currentTarget.style.borderColor = 'rgba(207, 185, 145, 0.2)';
          }}
        >
          ⬇ Download .md
        </a>
      </header>

      {/* Document body */}
      <div
        ref={contentRef}
        className="chariot-content"
        style={{
          flex: 1, overflow: 'auto', padding: '40px',
        }}
      >
        {loading ? (
          <div style={{
            display: 'flex', justifyContent: 'center', alignItems: 'center',
            height: '200px', color: '#6B7280', fontSize: '14px',
          }}>
            Loading {meta.title}...
          </div>
        ) : (
          <article
            className="chariot-article"
            dangerouslySetInnerHTML={{ __html: html }}
          />
        )}
      </div>
    </div>
  );
}
