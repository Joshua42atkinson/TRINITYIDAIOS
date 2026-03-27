/**
 * activityBus — Global event bus for the Yard Activity Bar
 *
 * The Yardmaster hook (useYardmaster.js) emits events into this bus,
 * and ActivityBar.jsx subscribes to them. This allows the activity
 * feed to persist across tab switches without duplicating SSE parsing.
 *
 * Pattern: simple pub/sub with a circular buffer (last 100 events).
 */

const MAX_EVENTS = 100;
let events = [];
let listeners = new Set();
let isActive = false;

export const activityBus = {
  /** Push a new event */
  emit(text, type = '') {
    const event = { text, type, ts: Date.now() };
    events = [...events.slice(-(MAX_EVENTS - 1)), event];
    listeners.forEach(fn => fn([...events]));
  },

  /** Set engine active/idle state */
  setActive(val) {
    isActive = val;
    listeners.forEach(fn => fn([...events]));
  },

  /** Is the engine currently processing? */
  getActive() {
    return isActive;
  },

  /** Subscribe to events. Returns unsubscribe function. */
  subscribe(fn) {
    listeners.add(fn);
    // Immediately deliver current state
    fn([...events]);
    return () => listeners.delete(fn);
  },

  /** Get current events snapshot */
  getEvents() {
    return [...events];
  },
};
