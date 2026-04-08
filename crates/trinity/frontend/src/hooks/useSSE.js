import { useState, useEffect, useCallback } from 'react';

/** SSE hook for /api/book/stream events */
export function useSSE() {
  const [events, setEvents] = useState([]);

  useEffect(() => {
    const source = new EventSource('/api/book/stream');

    source.addEventListener('CreepTameable', (e) => {
      try {
        const data = JSON.parse(e.data);
        setEvents((prev) => [...prev, { type: 'scope', ...data, id: Date.now() }]);
      } catch { /* skip malformed */ }
    });

    source.addEventListener('CreepDiscovered', (e) => {
      try {
        const data = JSON.parse(e.data);
        setEvents((prev) => [...prev, { type: 'discovered', ...data, id: Date.now() }]);
      } catch { /* skip malformed */ }
    });

    source.addEventListener('LessonCompleted', (e) => {
      try {
        const data = JSON.parse(e.data);
        setEvents((prev) => [...prev, { type: 'lesson', ...data, id: Date.now() }]);
      } catch { /* skip malformed */ }
    });

    source.addEventListener('update', (e) => {
      try {
        const data = JSON.parse(e.data);
        setEvents((prev) => [...prev, { type: 'update', ...data, id: Date.now() }]);
      } catch { /* skip malformed */ }
    });

    source.addEventListener('perspective', (e) => {
      try {
        const data = JSON.parse(e.data);
        setEvents((prev) => [...prev, { type: 'perspective', ...data, id: Date.now() }]);
      } catch { /* skip malformed */ }
    });

    source.addEventListener('character_update', (e) => {
      try {
        const data = JSON.parse(e.data);
        setEvents((prev) => [...prev, { type: 'character_update', ...data, id: Date.now() }]);
      } catch { /* skip malformed */ }
    });

    source.onerror = () => { /* auto-reconnect is built into SSE */ };

    return () => source.close();
  }, []);

  const dismissEvent = useCallback((id) => {
    setEvents((prev) => prev.filter((e) => e.id !== id));
  }, []);

  return { events, dismissEvent };
}
