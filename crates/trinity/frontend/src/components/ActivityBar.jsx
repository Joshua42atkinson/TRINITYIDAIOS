import React, { useState, useRef, useEffect } from 'react';
import { activityBus } from '../hooks/activityBus';

/**
 * ActivityBar — The Yard's Kitchen Window
 *
 * A persistent bottom dock visible on every tab, showing live engine activity
 * (tool calls, d20 rolls, coal/steam/XP, VAAM, cognitive load) in real-time.
 * Subscribes to the global activityBus for events emitted by useYardmaster.
 */
export default function ActivityBar() {
  const [events, setEvents] = useState([]);
  const [active, setActive] = useState(false);
  const [expanded, setExpanded] = useState(false);
  const scrollRef = useRef(null);

  // Subscribe to the global activity bus
  useEffect(() => {
    const unsub = activityBus.subscribe((evts) => {
      setEvents(evts);
      setActive(activityBus.getActive());
    });
    return unsub;
  }, []);

  // Auto-scroll when new events arrive and panel is expanded
  useEffect(() => {
    if (expanded && scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [events, expanded]);

  const recentEvents = events.slice(-50);
  const lastEvent = recentEvents[recentEvents.length - 1];

  return (
    <div className={`activity-bar ${expanded ? 'activity-bar--expanded' : ''}`}>
      {/* Collapsed Header — always visible */}
      <div
        className="activity-bar__header"
        onClick={() => setExpanded(!expanded)}
      >
        <div className="activity-bar__left">
          <span className={`activity-bar__dot ${active ? 'activity-bar__dot--active' : ''}`} />
          <span className="activity-bar__title">⚒️ THE YARD</span>
          <span className="activity-bar__count">{recentEvents.length} events</span>
        </div>

        <div className="activity-bar__right">
          {/* Show last event preview when collapsed */}
          {!expanded && lastEvent && (
            <span className="activity-bar__preview">
              {lastEvent.text?.substring(0, 80)}{lastEvent.text?.length > 80 ? '…' : ''}
            </span>
          )}
          <span className="activity-bar__toggle">{expanded ? '▾' : '▸'}</span>
        </div>
      </div>

      {/* Expanded Terminal — event log */}
      {expanded && (
        <div className="activity-bar__terminal" ref={scrollRef}>
          {recentEvents.length === 0 ? (
            <div className="activity-bar__empty">
              No events yet — interact with Pete to see the engine at work.
            </div>
          ) : (
            recentEvents.map((event, i) => (
              <div
                key={event.ts || i}
                className={`activity-bar__line ${event.type || ''}`}
              >
                {event.text}
              </div>
            ))
          )}
          <div className="activity-bar__line">
            <span className="activity-bar__cursor" />
          </div>
        </div>
      )}
    </div>
  );
}
