/**
 * PerspectiveSidebar.jsx — Ring 6: Perspective Engine UI
 *
 * Displays multi-perspective evaluations of Pete's responses as
 * collapsible marginalia in the PhaseWorkspace.
 *
 * Architecture:
 *   - Listens for 'perspective' SSE events via props
 *   - Shows collapsed badge: "🔮 3 perspectives"
 *   - Expands to show individual lens cards with icon, label, content
 *   - 👍/👎 feedback buttons (future: persist to DB for training)
 *
 * Ring Interactions:
 *   - Ring 6 (Perspective Engine) → this component
 *   - Perspectives evaluate Pete's output, never modify it
 */

import { useState, useEffect, useCallback, useRef } from 'react';

export default function PerspectiveSidebar({ sseEvents, onDismissEvent }) {
  const [perspectiveSets, setPerspectiveSets] = useState([]);
  const [expanded, setExpanded] = useState(false);
  const [feedback, setFeedback] = useState({}); // { lensId: 'up' | 'down' }
  const sidebarRef = useRef(null);

  // Collect perspective events from SSE
  useEffect(() => {
    if (!sseEvents?.length) return;

    const perspectiveEvents = sseEvents.filter(ev => ev.type === 'perspective');
    if (perspectiveEvents.length === 0) return;

    perspectiveEvents.forEach(ev => {
      if (ev.perspectives) {
        setPerspectiveSets(prev => {
          // Keep only the last 5 perspective sets
          const next = [...prev, {
            id: ev.id,
            perspectives: ev.perspectives,
            totalLatency: ev.total_latency_ms,
            timestamp: Date.now(),
          }];
          return next.slice(-5);
        });
      }
      if (onDismissEvent) onDismissEvent(ev.id);
    });
  }, [sseEvents, onDismissEvent]);

  // Auto-expand when new perspectives arrive
  useEffect(() => {
    if (perspectiveSets.length > 0) {
      setExpanded(true);
      // Auto-collapse after 15s
      const timer = setTimeout(() => setExpanded(false), 15000);
      return () => clearTimeout(timer);
    }
  }, [perspectiveSets.length]);

  const handleFeedback = useCallback((lensId, direction) => {
    setFeedback(prev => ({ ...prev, [lensId]: direction }));
    // Persist feedback for Ring 6 training data
    fetch('/api/perspective/feedback', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ lens_id: lensId, direction }),
    }).catch(() => {}); // Non-blocking, fire-and-forget
  }, []);

  const latestSet = perspectiveSets[perspectiveSets.length - 1];
  const perspectiveCount = latestSet?.perspectives?.length || 0;

  if (perspectiveCount === 0) return null;

  return (
    <div className={`perspective-sidebar ${expanded ? 'expanded' : 'collapsed'}`} ref={sidebarRef}>
      {/* Collapsed badge */}
      <button
        className="perspective-badge"
        onClick={() => setExpanded(!expanded)}
        title={expanded ? 'Collapse perspectives' : 'Expand perspectives'}
      >
        <span className="perspective-badge-icon">🔮</span>
        <span className="perspective-badge-count">{perspectiveCount}</span>
        <span className="perspective-badge-label">
          {expanded ? '▾' : '▸'} Perspectives
        </span>
        {latestSet?.totalLatency && (
          <span className="perspective-badge-latency">{latestSet.totalLatency}ms</span>
        )}
      </button>

      {/* Expanded lens cards */}
      {expanded && (
        <div className="perspective-cards">
          {latestSet.perspectives.map((p, i) => (
            <div
              key={`${p.lens_id}-${i}`}
              className={`perspective-card perspective-card-${p.lens_id}`}
              style={{ animationDelay: `${i * 0.1}s` }}
            >
              <div className="perspective-card-header">
                <span className="perspective-card-icon">{p.icon}</span>
                <span className="perspective-card-label">{p.label}</span>
                {p.relevance > 0.7 && (
                  <span className="perspective-relevance-high" title="High relevance">★</span>
                )}
              </div>
              <div className="perspective-card-content">
                {p.content}
              </div>
              <div className="perspective-card-footer">
                <div className="perspective-feedback">
                  <button
                    className={`perspective-fb-btn ${feedback[p.lens_id] === 'up' ? 'active' : ''}`}
                    onClick={() => handleFeedback(p.lens_id, 'up')}
                    title="Helpful"
                  >👍</button>
                  <button
                    className={`perspective-fb-btn ${feedback[p.lens_id] === 'down' ? 'active' : ''}`}
                    onClick={() => handleFeedback(p.lens_id, 'down')}
                    title="Not helpful"
                  >👎</button>
                </div>
                <span className="perspective-card-latency">{p.latency_ms}ms</span>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
