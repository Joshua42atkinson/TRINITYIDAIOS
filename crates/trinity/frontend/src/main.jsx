import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './styles/tokens.css';
import './styles/book.css';
import './styles/chariot.css';

// -------------------------------------------------------------------------
// OFFLINE TAURI INFERENCE PROXY
// Because standard fetch('/api/') fails under the Tauri desktop custom 
// protocol (tauri.localhost), we instantly reroute API calls to the 
// background Rust Trinity daemon on port 3000 if we detect desktop mode.
// -------------------------------------------------------------------------
const originalFetch = window.fetch;
window.fetch = async (input, init) => {
  if (typeof input === 'string' && input.startsWith('/api/')) {
    const isTauri = 
      window.__TAURI_INTERNALS__ || 
      window.__TAURI__ || 
      window.location.protocol.startsWith('tauri') || 
      window.location.hostname === 'tauri.localhost';

    if (isTauri) {
      input = `http://127.0.0.1:3000${input}`;
    }
  }
  return originalFetch(input, init);
};

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
