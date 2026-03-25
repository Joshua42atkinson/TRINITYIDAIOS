import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './styles/tokens.css';
import './styles/book.css';
import './styles/chariot.css';

// ══════════════════════════════════════════════════════
// Global API Path Interceptor
// When served at /trinity/ (behind Caddy/Cloudflare),
// auto-prefix all /api/ requests so they route correctly.
// ══════════════════════════════════════════════════════
const API_PREFIX = window.location.pathname.startsWith('/trinity') ? '/trinity' : '';

if (API_PREFIX) {
  const originalFetch = window.fetch;
  window.fetch = function(input, init) {
    if (typeof input === 'string' && input.startsWith('/api/')) {
      input = API_PREFIX + input;
    }
    return originalFetch.call(this, input, init);
  };

  const OriginalEventSource = window.EventSource;
  window.EventSource = function(url, opts) {
    if (typeof url === 'string' && url.startsWith('/api/')) {
      url = API_PREFIX + url;
    }
    return new OriginalEventSource(url, opts);
  };
  window.EventSource.prototype = OriginalEventSource.prototype;
}

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
