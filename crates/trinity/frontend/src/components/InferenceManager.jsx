import React, { useState, useEffect, useCallback, useRef } from 'react';

/* ═══════════════════════════════════════════════════════════════════
   INFERENCE MANAGER — AI Model Fleet Control Panel
   Professional RAM-aware model management for P.A.R.T.Y. architecture
   Sprint 7: "The Control Tower"
   ═══════════════════════════════════════════════════════════════════ */

const ROLE_COLORS = {
  'P (Pete)':       { bg: 'rgba(250, 204, 21, 0.12)', border: '#FACC15', text: '#FACC15', icon: '🧠' },
  'A (Aesthetics)': { bg: 'rgba(168, 139, 250, 0.12)', border: '#A78BFA', text: '#A78BFA', icon: '🎨' },
  'R (Research)':   { bg: 'rgba(6, 182, 212, 0.12)',   border: '#06B6D4', text: '#06B6D4', icon: '🔬' },
  'T (Tempo)':      { bg: 'rgba(16, 185, 129, 0.12)',  border: '#10B981', text: '#10B981', icon: '🎵' },
  'Y (Yardmaster)': { bg: 'rgba(59, 130, 246, 0.12)',  border: '#3B82F6', text: '#3B82F6', icon: '⚙️' },
};

const CAP_BADGES = {
  'text':      { label: 'Text', color: '#FACC15' },
  'vision':    { label: 'Vision', color: '#A78BFA' },
  'audio-in':  { label: 'Audio', color: '#06B6D4' },
  'tts':       { label: 'TTS', color: '#10B981' },
  'music':     { label: 'Music', color: '#10B981' },
  'music-gen': { label: 'Music', color: '#10B981' },
  'embeddings':{ label: 'Embed', color: '#06B6D4' },
  'image-gen': { label: 'Image', color: '#A78BFA' },
  'video-gen': { label: 'Video', color: '#A78BFA' },
  '3d-mesh':   { label: '3D', color: '#A78BFA' },
  'code':      { label: 'Code', color: '#3B82F6' },
  'tools':     { label: 'Tools', color: '#3B82F6' },
  'reasoning': { label: 'Reason', color: '#3B82F6' },
};

// Map model IDs to sidecar_start tool params
const STARTABLE_MODELS = {
  'longcat-next-74b': 'pete',
  'nomic-embed-v1.5': 'research',
  'flux-dev': 'aesthetics',
  'acestep-1.5': null, // Bundled with LongCat
  'yardmaster-reap': 'yardmaster',
};

// ═══ Global Fleet Status Emitter ═══
// Other components (PhaseWorkspace, ArtStudio) can listen to this
// to know if inference is available. Avoids prop drilling.
function emitFleetStatus(online, primaryModel) {
  window.dispatchEvent(new CustomEvent('trinity-fleet-status', {
    detail: { online, primaryModel }
  }));
}

/* ─── RAM Visualization ─── */
function RAMBar({ used, total, models }) {
  const pct = Math.min((used / total) * 100, 100);
  const available = total - used;

  const onlineModels = (models || []).filter(m => m.status === 'online' && m.ram_gb > 0);
  const totalModelRam = onlineModels.reduce((s, m) => s + m.ram_gb, 0);
  const systemRam = Math.max(0, used - totalModelRam);

  const barColor = pct > 85 ? '#F43F5E' : pct > 65 ? '#FACC15' : '#10B981';

  return (
    <div className="im-ram-container">
      <div className="im-ram-header">
        <span className="im-ram-label">UNIFIED MEMORY</span>
        <span className="im-ram-value" style={{ color: barColor }}>
          {used.toFixed(1)} / {total.toFixed(1)} GB
        </span>
      </div>
      <div className="im-ram-bar">
        {systemRam > 0 && (
          <div 
            className="im-ram-segment im-ram-segment--system"
            style={{ width: `${(systemRam / total) * 100}%` }}
            title={`System: ${systemRam.toFixed(1)} GB`}
          />
        )}
        {onlineModels.map(m => {
          const role = ROLE_COLORS[m.role] || ROLE_COLORS['P (Pete)'];
          return (
            <div 
              key={m.id}
              className="im-ram-segment"
              style={{ 
                width: `${(m.ram_gb / total) * 100}%`,
                background: role.border,
                opacity: 0.7,
              }}
              title={`${m.name}: ${m.ram_gb} GB`}
            />
          );
        })}
      </div>
      <div className="im-ram-legend">
        <span className="im-ram-legend-item">
          <span className="im-ram-dot" style={{ background: '#475569' }} />
          System ({systemRam > 0 ? systemRam.toFixed(0) : '?'}G)
        </span>
        {onlineModels.map(m => {
          const role = ROLE_COLORS[m.role] || ROLE_COLORS['P (Pete)'];
          return (
            <span key={m.id} className="im-ram-legend-item">
              <span className="im-ram-dot" style={{ background: role.border }} />
              {m.role?.split(' ')[0]} ({m.ram_gb}G)
            </span>
          );
        })}
        <span className="im-ram-legend-item" style={{ marginLeft: 'auto', color: '#10B981' }}>
          {available.toFixed(1)}G free
        </span>
      </div>
    </div>
  );
}

/* ─── Model Card with Action Buttons ─── */
function ModelCard({ model, availableRam, onStart, onStop, actionStates }) {
  const role = ROLE_COLORS[model.role] || { bg: 'rgba(255,255,255,0.05)', border: '#475569', text: '#94A3B8', icon: '📦' };
  const isOnline = model.status === 'online';
  const canFit = model.ram_gb <= availableRam || model.ram_gb === 0;
  const isNative = model.ram_gb === 0;
  const canStart = STARTABLE_MODELS[model.id] !== undefined && STARTABLE_MODELS[model.id] !== null;
  const isStarting = actionStates[model.id] === 'starting';
  const isPrimary = model.id === 'longcat-next-74b';

  return (
    <div 
      className={`im-model-card ${isOnline ? 'im-model-card--online' : ''}`}
      style={{ borderColor: isOnline ? role.border : 'rgba(255,255,255,0.06)' }}
    >
      <div className="im-model-header">
        <div className="im-model-identity">
          <span className="im-model-icon">{role.icon}</span>
          <div>
            <div className="im-model-name">{model.name}</div>
            <div className="im-model-role" style={{ color: role.text }}>{model.role}</div>
          </div>
        </div>
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          {/* Start Button */}
          {canStart && !isOnline && (
            <button
              className="im-action-btn im-action-btn--start"
              onClick={() => onStart(model.id)}
              disabled={isStarting || (!canFit && model.ram_gb > 0)}
              title={!canFit && model.ram_gb > 0 ? `Not enough RAM (needs ${model.ram_gb}G, ${availableRam.toFixed(0)}G free)` : `Start ${model.name}`}
            >
              {isStarting ? (
                <><span className="im-action-spinner" /> IGNITING...</>
              ) : (
                <>🔥 IGNITE</>
              )}
            </button>
          )}
          {/* Stop Button (only for primary model) */}
          {isOnline && isPrimary && (
            <button
              className="im-action-btn im-action-btn--stop"
              onClick={onStop}
              title="Stop LongCat"
            >
              ■ STOP
            </button>
          )}
          {isOnline && !isPrimary && (
            <button
              className="im-action-btn im-action-btn--active"
              title="Model is running"
              disabled
            >
              ● LIVE
            </button>
          )}
          <div className={`im-status-pill ${isOnline ? 'im-status-pill--on' : ''}`}>
            <span className="im-status-dot" />
            {isOnline ? 'ONLINE' : 'OFFLINE'}
          </div>
        </div>
      </div>

      <div className="im-model-desc">{model.description}</div>

      <div className="im-model-specs">
        {model.ram_gb > 0 && (
          <div className="im-spec">
            <span className="im-spec-label">RAM</span>
            <span className={`im-spec-value ${!canFit && !isOnline ? 'im-spec-value--warn' : ''}`}>
              {model.ram_gb} GB
              {!canFit && !isOnline && <span className="im-warn-icon" title="Not enough RAM">⚠</span>}
            </span>
          </div>
        )}
        {isNative && (
          <div className="im-spec">
            <span className="im-spec-label">RAM</span>
            <span className="im-spec-value" style={{ color: '#10B981' }}>Bundled</span>
          </div>
        )}
        {model.context_len > 0 && (
          <div className="im-spec">
            <span className="im-spec-label">CTX</span>
            <span className="im-spec-value">{(model.context_len / 1024).toFixed(0)}K</span>
          </div>
        )}
        <div className="im-spec">
          <span className="im-spec-label">QUANT</span>
          <span className="im-spec-value">{model.quantization}</span>
        </div>
        <div className="im-spec">
          <span className="im-spec-label">PORT</span>
          <span className="im-spec-value">:{model.port}</span>
        </div>
      </div>

      <div className="im-model-caps">
        {(model.capabilities || []).map(cap => {
          const badge = CAP_BADGES[cap] || { label: cap, color: '#475569' };
          return (
            <span 
              key={cap} 
              className="im-cap-badge"
              style={{ borderColor: badge.color, color: badge.color }}
            >
              {badge.label}
            </span>
          );
        })}
      </div>
    </div>
  );
}

/* ─── Fleet Summary Bar ─── */
function FleetSummary({ models }) {
  const online = (models || []).filter(m => m.status === 'online');
  const total = (models || []).length;
  const caps = new Set();
  online.forEach(m => (m.capabilities || []).forEach(c => caps.add(c)));
  
  return (
    <div className="im-fleet-summary">
      <div className="im-fleet-count">
        <span className="im-fleet-count-num" style={{ color: online.length > 0 ? '#10B981' : '#F43F5E' }}>
          {online.length}
        </span>
        <span className="im-fleet-count-label">/{total} online</span>
      </div>
      <div className="im-fleet-caps">
        {['text', 'vision', 'audio-in', 'image-gen', 'music-gen', 'code', 'embeddings'].map(cap => {
          const active = caps.has(cap);
          const badge = CAP_BADGES[cap] || { label: cap, color: '#475569' };
          return (
            <span 
              key={cap}
              className={`im-fleet-cap ${active ? 'im-fleet-cap--active' : ''}`}
              style={active ? { borderColor: badge.color, color: badge.color } : {}}
              title={active ? `${badge.label}: Available` : `${badge.label}: No model loaded`}
            >
              {badge.label}
            </span>
          );
        })}
      </div>
    </div>
  );
}

/* ═══════════════════════════════════════════════════════════════════
   MAIN COMPONENT
   ═══════════════════════════════════════════════════════════════════ */
export default function InferenceManager() {
  const [resources, setResources] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [lastUpdate, setLastUpdate] = useState(null);
  const [actionStates, setActionStates] = useState({}); // model_id -> 'starting' | null
  const prevOnlineRef = useRef(false);

  const fetchResources = useCallback(async () => {
    try {
      const res = await fetch('/api/inference/resources');
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const data = await res.json();
      setResources(data);
      setError(null);
      setLastUpdate(new Date());

      // Emit fleet status globally
      const models = data?.models || [];
      const peteOnline = models.some(m => m.id === 'longcat-next-74b' && m.status === 'online');
      const anyOnline = models.some(m => m.status === 'online' && (m.capabilities || []).includes('text'));
      emitFleetStatus(peteOnline || anyOnline, peteOnline ? 'longcat-omni' : null);

      // Toast on state change
      if (peteOnline && !prevOnlineRef.current) {
        window.dispatchEvent(new CustomEvent('trinity-toast', {
          detail: { message: '✅ Pete is awake — ready to chat', type: 'success' }
        }));
      } else if (!peteOnline && prevOnlineRef.current) {
        window.dispatchEvent(new CustomEvent('trinity-toast', {
          detail: { message: '🔴 Pete went offline — inference unavailable', type: 'warning' }
        }));
      }
      prevOnlineRef.current = peteOnline;
    } catch (err) {
      setError(err.message);
      emitFleetStatus(false, null);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchResources();
    const interval = setInterval(fetchResources, 8000); // Poll every 8s
    return () => clearInterval(interval);
  }, [fetchResources]);

  const handleStart = useCallback(async (modelId) => {
    setActionStates(prev => ({ ...prev, [modelId]: 'starting' }));
    try {
      const res = await fetch('/api/inference/start', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
      });
      const data = await res.json();
      console.log('LongCat start response:', data);
      
      if (data.status !== 'error') {
        // Start polling aggressively to catch when it comes online
        const fastPoll = setInterval(fetchResources, 3000);
        setTimeout(() => clearInterval(fastPoll), 120000); // 2 minutes
      }
    } catch (err) {
      console.error('Failed to start LongCat:', err);
    }
    // Clear starting state after model load time (~90s for NF4)
    setTimeout(() => {
      setActionStates(prev => ({ ...prev, [modelId]: null }));
    }, 90000);
  }, [fetchResources]);

  const handleStop = useCallback(async () => {
    try {
      const res = await fetch('/api/inference/stop', { method: 'POST' });
      const data = await res.json();
      console.log('LongCat stop response:', data);
      await fetchResources();
    } catch (err) {
      console.error('Failed to stop LongCat:', err);
    }
  }, [fetchResources]);

  const handleRefreshBackends = useCallback(async () => {
    try {
      await fetch('/api/inference/refresh', { method: 'POST' });
      await fetchResources();
    } catch (err) {
      console.error('Refresh failed:', err);
    }
  }, [fetchResources]);

  if (loading && !resources) {
    return (
      <div className="im-container">
        <div className="im-loading">
          <div className="im-loading-pulse" />
          <span>Scanning P.A.R.T.Y. Fleet...</span>
        </div>
      </div>
    );
  }

  if (error && !resources) {
    return (
      <div className="im-container">
        <div className="im-error">
          <span>⚠️ Fleet endpoint unavailable — restart Trinity to enable</span>
          <button onClick={fetchResources} className="im-retry-btn">Retry</button>
        </div>
      </div>
    );
  }

  const sys = resources?.system || {};
  const models = resources?.models || [];
  const constraints = resources?.constraints || {};
  const onlineCount = models.filter(m => m.status === 'online').length;

  return (
    <div className="im-container">
      {/* Header */}
      <div className="im-header">
        <div className="im-title-row">
          <h2 className="im-title">⚡ Inference Fleet</h2>
          <div style={{ display: 'flex', gap: '6px' }}>
            <button 
              className="im-refresh-btn"
              onClick={handleRefreshBackends}
              title="Re-probe all backends"
            >
              🔍
            </button>
            <button 
              className="im-refresh-btn" 
              onClick={fetchResources}
              title="Refresh fleet status"
            >
              ⟳
            </button>
          </div>
        </div>
        <div className="im-subtitle">
          {sys.gpu || 'System resources loading...'}
        </div>
      </div>

      {/* Fleet Summary */}
      <FleetSummary models={models} />

      {/* RAM Visualization */}
      <RAMBar 
        used={sys.used_ram_gb || 0} 
        total={sys.total_ram_gb || 128} 
        models={models}
      />

      {/* Constraint Advisory */}
      {constraints.recommendation && (
        <div className={`im-advisory ${
          constraints.recommendation.includes('tight') ? 'im-advisory--warn' :
          constraints.recommendation.includes('Moderate') ? 'im-advisory--caution' :
          'im-advisory--ok'
        }`}>
          <span className="im-advisory-icon">
            {constraints.recommendation.includes('tight') ? '🔴' :
             constraints.recommendation.includes('Moderate') ? '🟡' : '🟢'}
          </span>
          <span>{constraints.recommendation}</span>
        </div>
      )}

      {/* Quick Start Banner (when nothing is online) */}
      {onlineCount === 0 && (
        <div className="im-quickstart">
          <div className="im-quickstart-icon">🚂💤</div>
          <div className="im-quickstart-text">
            <strong>All engines are cold.</strong> Start Pete to enable chat, or use the IGNITE buttons below.
          </div>
          <button 
            className="im-action-btn im-action-btn--start im-quickstart-btn"
            onClick={() => handleStart('longcat-next-74b')}
            disabled={actionStates['longcat-next-74b'] === 'starting'}
          >
            {actionStates['longcat-next-74b'] === 'starting' ? (
              <><span className="im-action-spinner" /> IGNITING PETE...</>
            ) : (
              <>🔥 IGNITE PETE</>
            )}
          </button>
        </div>
      )}

      {/* Model Grid */}
      <div className="im-model-grid">
        {models.map(model => (
          <ModelCard 
            key={model.id} 
            model={model}
            availableRam={sys.available_ram_gb || 0}
            onStart={handleStart}
            onStop={handleStop}
            actionStates={actionStates}
          />
        ))}
      </div>

      {/* Footer */}
      <div className="im-footer">
        <span className="im-footer-note">{constraints.note || ''}</span>
        {lastUpdate && (
          <span className="im-footer-time">
            Updated {lastUpdate.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' })}
          </span>
        )}
      </div>
    </div>
  );
}
