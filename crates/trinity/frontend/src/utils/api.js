/**
 * API base path helper
 * 
 * When Trinity is served at /trinity/ (behind Caddy/Cloudflare),
 * API calls must go to /trinity/api/... not /api/...
 * 
 * This auto-detects the prefix from the current URL.
 */

// If we're at ldtatkinson.com/trinity/, the base is "/trinity"
// If we're at localhost:3000/, the base is ""
const getBase = () => {
  const path = window.location.pathname;
  if (path.startsWith('/trinity')) return '/trinity';
  return '';
};

const BASE = getBase();

export const api = (endpoint) => `${BASE}${endpoint}`;

// Convenience: api('/api/quest') → '/trinity/api/quest' or '/api/quest'
export default api;
