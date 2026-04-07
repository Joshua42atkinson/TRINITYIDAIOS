import React, { useState, useRef, useEffect, useCallback } from 'react';
import { useCreative } from '../hooks/useCreative';
import { Command } from '@tauri-apps/plugin-shell';
import '../styles/art_studio_premium.css';

/* ─── Style selector options ─── */
const VISUAL_STYLES = ['steampunk', 'cyberpunk', 'fantasy', 'minimalist', 'retro', 'noir'];
const MUSIC_STYLES = ['orchestral', 'lofi', 'electronic', 'jazz', 'ambient', 'classical'];

/* ─── Hook Deck commands for TCG game dev ─── */
const FORGE_QUICK_COMMANDS = [
  { icon: '🔮', label: 'Float (Transform.y)', cmd: 'SpawnConcept', params: { id: 'test_float', label: 'Float Module', position: [0.0, 0.5, -5.0], python_script: 'transform.y += 1.0 * delta_time' } },
  { icon: '💨', label: 'Bounce (Velocity.y)', cmd: 'SpawnConcept', params: { id: 'test_bounce', label: 'Bounce Pad', position: [2.0, 0.5, -5.0], python_script: 'velocity.y += 15.0 * delta_time' } },
  { icon: '🪨', label: 'Spin (Transform.rot_y)', cmd: 'SpawnConcept', params: { id: 'test_spin', label: 'Spin Module', position: [-2.0, 0.5, -5.0], python_script: 'transform.rot_y += 3.14 * delta_time' } },
  { icon: '▶️', label: 'Play',        cmd: 'play',     params: {} },
  { icon: '⏹️', label: 'Stop',        cmd: 'stop',     params: {} },
];

function fmtTime(ts) {
  try {
    return new Date(ts).toLocaleTimeString([], { hour12: false, hour: '2-digit', minute: '2-digit', second: '2-digit' });
  } catch { return '—'; }
}

function fmtSize(bytes) {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

// ─── Simple Markdown → HTML ────────────────────────────────────────────────────
// Handles: headings, bold, italic, code, lists, AND markdown images from tool results.
function renderMarkdown(text) {
  if (!text) return '';

  // Extract markdown images before HTML-escaping
  const imageTokens = [];
  let processed = text.replace(/!\[([^\]]*)\]\(([^)]+)\)/g, (_, alt, url) => {
    const idx = imageTokens.length;
    const safeUrl = url.replace(/"/g, '&quot;');
    const safeAlt = (alt || 'Generated image').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    imageTokens.push(
      `<div style="margin:12px 0;text-align:center;">` +
      `<img src="${safeUrl}" alt="${safeAlt}" loading="lazy" ` +
      `style="max-width:100%;max-height:400px;border-radius:8px;border:1px solid rgba(52,211,153,0.2);display:block;margin:0 auto;" />` +
      `<div style="font-size:11px;color:rgba(52,211,153,0.5);margin-top:4px;font-style:italic;">${safeAlt}</div>` +
      `</div>`
    );
    return `__IMGTOKEN_${idx}__`;
  });

  let html = processed
    .replace(/^### (.+)$/gm, '<h3>$1</h3>')
    .replace(/^## (.+)$/gm, '<h2>$1</h2>')
    .replace(/^# (.+)$/gm, '<h1>$1</h1>')
    .replace(/\*\*\*(.+?)\*\*\*/g, '<strong><em>$1</em></strong>')
    .replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
    .replace(/\*(.+?)\*/g, '<em>$1</em>')
    .replace(/`([^`]+)`/g, '<code>$1</code>')
    .replace(/^[*-] (.+)$/gm, '<li>$1</li>')
    .replace(/^\d+\. (.+)$/gm, '<li>$1</li>');

  html = html.replace(/((?:<li>.*<\/li>\n?)+)/g, '<ul>$1</ul>');
  html = html.split(/\n{2,}/).map(block => {
    const trimmed = block.trim();
    if (!trimmed) return '';
    if (trimmed.startsWith('<h') || trimmed.startsWith('<ul') || trimmed.startsWith('<ol') || trimmed.startsWith('__IMGTOKEN_')) return trimmed;
    return `<p>${trimmed.replace(/\n/g, '<br/>')}</p>`;
  }).join('');

  imageTokens.forEach((block, i) => {
    html = html.replace(`__IMGTOKEN_${i}__`, block);
  });

  return html;
}

function typeIcon(type) {
  switch (type) {
    case 'image': return '🖼️';
    case 'video': return '🎬';
    case 'audio': return '🎵';
    case 'mesh':  return '🎲';
    default:      return '📄';
  }
}

/* ─── Sidecar Status Badge ─── */
function StatusBadge({ label, icon, sidecar, active }) {
  const running = sidecar?.running || active;
  return (
    <div 
      className={`premium-badge ${running ? 'premium-badge--on' : ''}`}
      title={sidecar?.message || (active ? 'Connected' : 'Checking status...')}
    >
      <span className="premium-badge__dot" />
      <span>{icon}</span>
      <span>{label}</span>
    </div>
  );
}

/* ─── Asset Card ─── */
function AssetCard({ asset, onPreview }) {
  const isImage = asset.asset_type === 'image';
  const isVideo = asset.asset_type === 'video';
  const isAudio = asset.asset_type === 'audio';

  return (
    <div
      className={`premium-asset-card ${isImage || isVideo ? 'premium-asset-card--clickable' : ''}`}
      onClick={() => (isImage || isVideo) && onPreview(asset)}
    >
      {isImage && (
        <img
          className="premium-asset-card__img"
          src={asset.url}
          alt={asset.filename}
          loading="lazy"
        />
      )}
      {isVideo && (
        <div className="art-asset-card__video-thumb">
          <video src={asset.url} muted preload="metadata" />
          <div className="art-asset-card__play-badge">▶</div>
        </div>
      )}
      {isAudio && (
        <div className="art-asset-card__audio">
          <audio controls preload="metadata" src={asset.url} />
        </div>
      )}
      {!isImage && !isVideo && !isAudio && (
        <div className="art-asset-card__icon-thumb">
          <span>{typeIcon(asset.asset_type)}</span>
        </div>
      )}
      <div className="premium-asset-card__footer">
        <span className="art-asset-card__type-badge">{typeIcon(asset.asset_type)}</span>
        <span className="premium-asset-card__title" title={asset.filename}>
          {asset.filename.length > 24 ? asset.filename.slice(0, 22) + '…' : asset.filename}
        </span>
        <span className="art-asset-card__size">{fmtSize(asset.size_bytes)}</span>
      </div>
    </div>
  );
}

/* ─── Preview Modal ─── */
function PreviewModal({ asset, onClose }) {
  if (!asset) return null;
  const isImage = asset.asset_type === 'image';
  const isVideo = asset.asset_type === 'video';

  return (
    <div className="art-preview-modal" onClick={onClose}>
      <div className="art-preview-modal__content" onClick={(e) => e.stopPropagation()}>
        <button className="art-preview-modal__close" onClick={onClose}>✕</button>
        {isImage && <img src={asset.url} alt={asset.filename} />}
        {isVideo && <video src={asset.url} controls autoPlay />}
        <div className="art-preview-modal__info">
          <span>{asset.filename}</span>
          <span>{fmtSize(asset.size_bytes)}</span>
          <span>{fmtTime(asset.created_at)}</span>
        </div>
      </div>
    </div>
  );
}

/* ═══════════════════════════════════════════════
   FORGE VIEWPORT — The Window to the Imagination
   ═══════════════════════════════════════════════ */
function ForgeViewport({ onLog, forgeRef }) {
  const iframeRef = useRef(null);
  const [forgeState, setForgeState] = useState('loading'); // loading | ready | standby | error
  const [engineVariant, setEngineVariant] = useState(null);

  // Listen for messages from Forge iframe (and from Yardmaster Agent)
  useEffect(() => {
    const handler = (event) => {
      // 1. Agent ↔ WASM Pass-through (from useYardmaster.js -> window)
      if (event.source === window && event.data?.type === 'forge_command') {
         if (iframeRef.current?.contentWindow) {
             iframeRef.current.contentWindow.postMessage(event.data, '*');
             onLog?.({ tag: 'FORGE', message: `▶ (Agent) ${event.data.command}` });
         }
         return;
      }

      // 2. Iframe ↔ React (Standard bridge)
      if (event.source !== iframeRef.current?.contentWindow) return;
      const { type, ...data } = event.data || {};

      switch (type) {
        case 'forge_ready':
          setForgeState('ready');
          setEngineVariant(data.variant);
          onLog?.({ tag: 'FORGE', message: `Engine ready (${data.variant})` });
          break;
        case 'forge_standby':
          setForgeState('standby');
          onLog?.({ tag: 'FORGE', message: data.message || 'Engine in standby' });
          break;
        case 'forge_result':
          if (data.success) {
            if (data.requestId && data.requestId.startsWith('sync-')) {
               fetch('/api/forge/sync', {
                 method: 'POST',
                 headers: { 'Content-Type': 'application/json' },
                 body: JSON.stringify(data.data || [])
               }).catch(e => console.error("Forge sync failed:", e));
            } else {
               onLog?.({ tag: 'FORGE', message: `✓ Command executed` });
            }
          } else {
            onLog?.({ tag: 'ERROR', message: `Forge: ${data.error}` });
          }
          break;
        case 'forge_pong':
          // Connection confirmed
          break;
        case 'forge_event':
          if (data.event) {
             const sysMsg = `[SYSTEM: WASM Event] ${data.event}: ${JSON.stringify(data.params || {})}`;
             onLog?.({ tag: 'EVENT', message: data.event });
             window.dispatchEvent(new CustomEvent('yardmaster-send', { detail: sysMsg }));
          }
          break;
      }
    };

    window.addEventListener('message', handler);
    return () => window.removeEventListener('message', handler);
  }, [onLog]);

  useEffect(() => {
    if (forgeRef) {
      forgeRef.current = {
        sendCommand: (command, payload, requestId = null) => {
          fetch('/api/daydream/command', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ command, params: payload || {} })
          }).catch(console.error);

          if (!requestId?.startsWith('sync-')) {
            onLog?.({ tag: 'DAYDREAM', message: `→ ${command}` });
          }
        },
        getState: () => forgeState,
      };
    }
  }, [forgeState, forgeRef, onLog]);

  useEffect(() => {
    if (forgeState !== 'ready') return;
    const interval = setInterval(() => {
      forgeRef.current?.sendCommand('get_scene_graph', {}, `sync-${Date.now()}`);
    }, 3000);
    return () => clearInterval(interval);
  }, [forgeState, forgeRef]);

  return (
    <div className="premium-forge-container" id="forge-viewport">
      <div className="forge-viewport__header" style={{ position: 'absolute', top: 10, left: 10, zIndex: 5, color: '#fff' }}>
        <span className="forge-viewport__title">
          🔮 DAYDREAM STUDIO
          {engineVariant && <span className="forge-viewport__variant"> {engineVariant.toUpperCase()}</span>}
        </span>
        <span className={`forge-viewport__status forge-viewport__status--${forgeState}`}>
          {forgeState === 'ready' ? ' ● LIVE' : forgeState === 'standby' ? ' ○ STANDBY' : ' ◌ LOADING'}
        </span>
      </div>
      {forgeState !== 'ready' && (
        <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', height: '100%', color: '#fff', background: '#0f172a' }}>
          <div style={{ fontSize: '48px', marginBottom: '16px' }}>🌌</div>
          <h2 style={{ marginBottom: '8px' }}>The DAYDREAM is Dormant</h2>
          <p style={{ opacity: 0.7, marginBottom: '24px', maxWidth: '400px', textAlign: 'center' }}>
            The 3D Game Engine runs as a high-performance native Daydream sidecar.
          </p>
          <button 
            onClick={async () => {
              try {
                setForgeState('loading');
                const cmd = Command.sidecar('daydream');
                const child = await cmd.spawn();
                onLog?.({ tag: 'FORGE', message: `Engine spawned (PID: ${child.pid})` });
                setForgeState('ready');
                setEngineVariant('native');
              } catch(e) {
                console.error(e);
                setForgeState('error');
              }
            }}
            style={{ padding: '12px 24px', background: '#34d399', color: '#000', border: 'none', borderRadius: '4px', cursor: 'pointer', fontSize: '16px', fontWeight: 'bold' }}
          >
            Launch DAYDREAM ENGINE
          </button>
        </div>
      )}
      {forgeState === 'ready' && (
        <div 
          onDragOver={(e) => {
            e.preventDefault();
            e.currentTarget.style.boxShadow = 'inset 0 0 40px rgba(52, 211, 153, 0.4)';
          }}
          onDragLeave={(e) => {
             e.currentTarget.style.boxShadow = 'none';
          }}
          onDrop={(e) => {
            e.preventDefault();
            e.currentTarget.style.boxShadow = 'none';
            try {
              // Try text payload first for simple commands
              const hookStr = e.dataTransfer.getData('text/plain');
              if (hookStr && hookStr.startsWith('CastHook:')) {
                const hookId = hookStr.split(':')[1];
                const prompts = {
                  'Pearl': '🔮 I am casting the PEARL hook. Please generate a Bevy mechanic or visual representing the ultimate goal of our lesson plan.',
                  'Coal': '🪨 I am casting the COAL hook. Build a physical constraint or rigid-body obstacle in the 3D space that the learner must overcome.',
                  'Steam': '💨 I am casting the STEAM hook. Create an engaging physical momentum-builder (like a jump pad or speed boost) to accelerate the learner.',
                  'Hook': '🪝 I am casting the HOOK hook. Create a highly interactive gravity point or grappling anchor here.',
                  'Mirror': '🪞 I am casting the MIRROR hook. Please build an interactive puzzle or assessment collider that challenges the user.',
                  'Compass': '🧭 I am casting the COMPASS hook. Generate a navigational cue or pathfinding laser to guide the learner.'
                };
                const msg = prompts[hookId] || `I am casting the ${hookId} hook into the Art Studio.`;
                setActiveTool('chat'); // Switch to Pete chat tab
                setMessage(msg);
                setTimeout(() => document.getElementById('chat-input')?.focus(), 50);
                return;
              }
              // Try JSON payload if not found
              const jsonStr = e.dataTransfer.getData('application/json');
              if (jsonStr) {
                const payload = JSON.parse(jsonStr);
                forgeRef.current?.sendCommand(payload.command, payload.params);
              }
            } catch (err) {
               console.error("Drop failed in Daydream:", err);
            }
          }}
          style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', height: '100%', color: '#fff', background: '#020617', transition: 'box-shadow 0.2s' }}>
           <div style={{ fontSize: '64px', marginBottom: '16px', opacity: 0.8 }}>🎮</div>
           <h2 style={{ marginBottom: '8px' }}>Active DAYDREAM Environment</h2>
           <p style={{ opacity: 0.6, maxWidth: '400px', textAlign: 'center' }}>
             The native 3D Engine is rendering directly to the desktop window. Drag and drop your Socratic Hook Cards here to physically cast them into the simulation.
           </p>
        </div>
      )}
    </div>
  );
}

/* ═══════════════════════════════════════════════
   FORGE TOOLBAR — Quick entity spawning
   ═══════════════════════════════════════════════ */
function ForgeToolbar({ forgeRef }) {
  const [customCmd, setCustomCmd] = useState('');

  const sendQuick = (cmd, params) => {
    forgeRef.current?.sendCommand(cmd, params);
  };

  const handleCustomSubmit = (e) => {
    e.preventDefault();
    if (!customCmd.trim()) return;
    try {
      const parsed = JSON.parse(customCmd);
      forgeRef.current?.sendCommand(parsed.command, parsed.params || {});
      setCustomCmd('');
    } catch {
      forgeRef.current?.sendCommand(customCmd.trim(), {});
      setCustomCmd('');
    }
  };

  return (
    <div className="forge-toolbar card" style={{ padding: '16px', color: '#fff' }}>
      <div className="card-header" style={{ marginBottom: '16px' }}>🐍 PYO3 ENGINE SCRIPTING</div>
      <div className="forge-toolbar__sim-controls" style={{ display: 'flex', gap: '8px', marginBottom: '12px', paddingBottom: '12px', borderBottom: '1px solid rgba(255,255,255,0.05)'}}>
        <button className="forge-toolbar__btn" onClick={() => sendQuick('play_scene')} title="play_scene" style={{ flex: 1, color: '#34d399', background: 'transparent', border: '1px solid #34d399', borderRadius: '4px', padding: '4px' }}>
          ▶ PLAY
        </button>
        <button className="forge-toolbar__btn" onClick={() => sendQuick('pause_scene')} title="pause_scene" style={{ flex: 1, color: '#fcd34d', background: 'transparent', border: '1px solid #fcd34d', borderRadius: '4px', padding: '4px' }}>
          ⏸ PAUSE
        </button>
        <button className="forge-toolbar__btn" onClick={() => sendQuick('reset_scene')} title="reset_scene" style={{ flex: 1, color: '#ef4444', background: 'transparent', border: '1px solid #ef4444', borderRadius: '4px', padding: '4px' }}>
          ⏹ STOP
        </button>
      </div>
      <div className="forge-toolbar__quick" style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '8px', marginBottom: '16px' }}>
        {FORGE_QUICK_COMMANDS.map((q) => (
          <button
            key={q.label}
            className="forge-toolbar__btn"
            onClick={() => sendQuick(q.cmd, q.params)}
            title={`${q.cmd}: ${q.label}`}
            style={{ background: 'rgba(255,255,255,0.05)', color: '#fff', border: '1px solid rgba(255,255,255,0.1)', display: 'flex', alignItems: 'center', gap: '6px', padding: '6px', borderRadius: '4px', cursor: 'pointer' }}
          >
            <span>{q.icon}</span>
            <span>{q.label}</span>
          </button>
        ))}
      </div>
      <form className="forge-toolbar__custom" onSubmit={handleCustomSubmit} style={{ display: 'flex', gap: '8px' }}>
        <input
          className="art-input"
          placeholder='{"command":"CastHook","params":{"hook":"Pearl"}}'
          value={customCmd}
          onChange={(e) => setCustomCmd(e.target.value)}
          style={{ flex: 1, background: 'rgba(255,255,255,0.05)', color: '#fff', border: '1px solid rgba(255,255,255,0.2)', padding: '6px', borderRadius: '4px' }}
        />
        <button type="submit" className="forge-toolbar__send" style={{ background: '#34d399', color: '#000', border: 'none', padding: '6px 12px', borderRadius: '4px', cursor: 'pointer' }}>⏎</button>
      </form>
    </div>
  );
}

/* ═══════════════════════════════════════════════
   ART STUDIO — Main Component
   The Window to the Imagination
   ═══════════════════════════════════════════════ */
export default function ArtStudio() {
  const {
    status,
    assets,
    fetchAssets,
  } = useCreative();

  const [previewAsset, setPreviewAsset] = useState(null);
  const [activeTool, setActiveTool] = useState('chat'); // chat | gallery | forgeTools
  
  // Chat State
  const [message, setMessage] = useState('');
  const [narrative, setNarrative] = useState([]);
  const [isStreaming, setIsStreaming] = useState(false);
  
  const forgeRef = useRef(null);
  const scrollRef = useRef(null);
  const fileInputRef = useRef(null);
  
  // File System State
  const [currentFolder, setCurrentFolder] = useState('/');
  
  const virtualFolders = React.useMemo(() => {
    const fMap = {};
    assets.forEach(a => {
      let prefix = 'Uncategorized';
      if (a.filename && a.filename.includes('_')) {
        prefix = a.filename.split('_')[0];
      }
      fMap[prefix] = (fMap[prefix] || 0) + 1;
    });
    return Object.keys(fMap).map(k => ({ name: k, count: fMap[k] })).sort((a,b) => a.name.localeCompare(b.name));
  }, [assets]);

  const activeAssets = React.useMemo(() => {
    if (currentFolder === '/') return [];
    return assets.filter(a => {
      if (currentFolder === 'Uncategorized') return !a.filename || !a.filename.includes('_');
      return a.filename && a.filename.startsWith(currentFolder + '_');
    });
  }, [assets, currentFolder]);

  // Auto-scroll chat
  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [narrative]);

  const sendMessage = async () => {
    if (!message.trim() || isStreaming) return;
    const userMsg = message.trim();
    setMessage('');
    setNarrative(n => [...n, { role: 'user', content: userMsg }]);
    setIsStreaming(true);
    setNarrative(n => [...n, { role: 'assistant', speaker: 'PETE', content: '' }]);

    try {
      const res = await fetch('/api/chat/stream', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: userMsg, mode: 'creative-studio', max_tokens: 4096, use_rag: true }),
      });

      if (!res.ok) {
        setNarrative(n => {
          const copy = [...n];
          copy[copy.length - 1] = { role: 'error', speaker: 'SYSTEM', content: "🚫 THE FURNACE IS COLD! The Great Recycler cannot speak while the firebox sleeps. Start vLLM on port 8001 or click [🔥 IGNITE FURNACE] to light the coal yourself!" };
          return copy;
        });
        setIsStreaming(false);
        return;
      }

      const reader = res.body.getReader();
      const dec = new TextDecoder();
      let buffer = '';
      let fullText = '';

      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        buffer += dec.decode(value, { stream: true });

        let idx;
        while ((idx = buffer.indexOf('\n')) !== -1) {
          const line = buffer.substring(0, idx);
          buffer = buffer.substring(idx + 1);

          if (!line || line.startsWith(':')) continue;
          
          if (line.startsWith('event: ')) {
            const currentEvent = line.substring(7).trim();
            if (currentEvent === 'llm_offline' || currentEvent === 'error') {
               setNarrative(n => {
                 const copy = [...n];
                 copy[copy.length - 1] = { role: 'error', speaker: 'SYSTEM', content: "🚫 THE FURNACE IS COLD! The Great Recycler cannot speak while the firebox sleeps. Start vLLM on port 8001 or click [🔥 IGNITE FURNACE] to light the coal yourself!" };
                 return copy;
               });
               setIsStreaming(false);
               return; 
            }
            continue;
          }

          if (line.startsWith('data: ')) {
            const payload = line.substring(6);
            if (payload === '[DONE]') continue;

            let token = '';
            if (payload.startsWith('{')) {
              try {
                const j = JSON.parse(payload);
                token = j.content || j.choices?.[0]?.delta?.content || '';
              } catch { token = payload; }
            } else { token = payload; }

            if (token) {
              fullText += token;
              setNarrative(n => {
                const copy = [...n];
                const last = copy[copy.length - 1];
                if (last?.role === 'assistant') copy[copy.length - 1] = { ...last, content: fullText };
                return copy;
              });
            }
          }
        }
      }
    } catch (err) {
      setNarrative(n => {
        const copy = [...n];
        const last = copy[copy.length - 1];
        if (last?.role === 'assistant' && !last.content) {
          copy[copy.length - 1] = { ...last, content: '⚠ Connection lost.' };
        }
        return copy;
      });
    }
    setIsStreaming(false);
  };

  const addForgeLog = useCallback((log) => {
    // Forge logs are now suppressed in the ArtStudio. They belong to the Yardmaster.
  }, []);

  return (
    <div className="premium-art-studio">
      {/* ── Status Bar ── */}
      <div className="premium-status-bar">
        <div className="premium-status-bar__title">✦ ART · Window to the Imagination</div>
        <div className="premium-status-badges">
          <StatusBadge label="Daydream" icon="🔮" active={forgeRef.current?.getState?.() === 'ready'} />
          <StatusBadge label="vLLM Omni" icon="🎨" sidecar={status.vllm} />
          <StatusBadge label="MusicGPT" icon="🎵" sidecar={status.musicgpt} />
        </div>
      </div>

      {/* ── Content Area ── */}
      <div className="premium-workspace">
        
        {/* ── LEFT PANE: Socratic Panel (Chat, Gallery, Tools) ── */}
        <div className="premium-socratic-panel">
          <div className="premium-tabs">
            <button className={`premium-tab ${activeTool === 'chat' ? 'premium-tab--active' : ''}`} onClick={() => setActiveTool('chat')}>💬 Chat</button>
            <button className={`premium-tab ${activeTool === 'gallery' ? 'premium-tab--active' : ''}`} onClick={() => setActiveTool('gallery')}>📁 Gallery ({assets.length})</button>
            <button className={`premium-tab ${activeTool === 'forgeTools' ? 'premium-tab--active' : ''}`} onClick={() => setActiveTool('forgeTools')}>🐍 PyO3 Sandbox</button>
          </div>

          {activeTool === 'chat' && (
            <div style={{ display: 'flex', flexDirection: 'column', height: '100%', flex: 1, minHeight: 0 }}>
              <div className="premium-chat-scroll" ref={scrollRef}>
                {narrative.length === 0 && (
                  <div style={{ margin: 'auto', textAlign: 'center', opacity: 0.5 }}>
                    <div style={{ fontSize: '32px', marginBottom: '16px' }}>🎨</div>
                    <p>The Studio is ready.</p>
                    <p>Tell Pete what you want to create.</p>
                  </div>
                )}
                
                {narrative.map((msg, i) => {
                  if (msg.role === 'user') {
                    return <div key={i} className="premium-msg premium-msg--user">"{msg.content}"</div>;
                  }
                  if (msg.role === 'error') {
                    return (
                      <div key={i} className="premium-msg premium-msg--error">
                        <div className="premium-speaker-badge" style={{ color: '#ef4444' }}>⚠️ {msg.speaker || 'SYSTEM'}</div>
                        <div dangerouslySetInnerHTML={{ __html: renderMarkdown(msg.content) }} />
                      </div>
                    );
                  }
                  return (
                    <div key={i} className="premium-msg premium-msg--ai">
                      <div className="premium-speaker-badge">⚙️ {msg.speaker || 'PETE'}</div>
                      <div dangerouslySetInnerHTML={{ __html: renderMarkdown(msg.content) }} />
                    </div>
                  );
                })}
                {isStreaming && (
                  <div className="typing-indicator" style={{ marginTop: 'auto', opacity: 0.5, fontStyle: 'italic', fontSize: '13px' }}>
                    Pete is thinking...
                  </div>
                )}
              </div>
              
              <div className="premium-input-area">
                <input
                  id="chat-input"
                  className="premium-input"
                  placeholder="Design a scary monster, Pete..."
                  value={message}
                  onChange={e => setMessage(e.target.value)}
                  onKeyDown={e => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); sendMessage(); }}}
                  disabled={isStreaming}
                />
                <button
                  className="premium-send-btn"
                  onClick={sendMessage}
                  disabled={!message.trim() || isStreaming}
                >
                  {isStreaming ? '···' : '↵'}
                </button>
              </div>
            </div>
          )}

          {activeTool === 'gallery' && (
            <div style={{ display: 'flex', flexDirection: 'column', height: '100%', padding: '16px' }}>
              <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '16px', paddingBottom: '12px', borderBottom: '1px solid rgba(255,255,255,0.05)' }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                  <button 
                    onClick={() => setCurrentFolder('/')}
                    style={{ background: 'transparent', border: 'none', color: currentFolder === '/' ? '#FFD700' : 'rgba(255,255,255,0.5)', cursor: 'pointer', padding: 0, fontSize: '14px' }}
                  >
                    /vault
                  </button>
                  {currentFolder !== '/' && (
                    <>
                      <span style={{ color: 'rgba(255,255,255,0.3)' }}>›</span>
                      <span style={{ color: '#FFD700', fontSize: '14px' }}>{currentFolder}</span>
                    </>
                  )}
                </div>
                <button onClick={fetchAssets} title="Refresh vault" style={{ background: 'transparent', border: 'none', color: 'rgba(255,255,255,0.3)', cursor: 'pointer' }}>⟳</button>
              </div>
              
              <div style={{ flex: 1, overflowY: 'auto', paddingRight: '8px' }}>
                {assets.length === 0 ? (
                  <div style={{ textAlign: 'center', opacity: 0.5, marginTop: '40px' }}>
                    <div style={{ fontSize: '32px' }}>🗄️</div>
                    <div>Vault is empty.</div>
                  </div>
                ) : currentFolder === '/' ? (
                  /* Root View: Show Folders */
                  <div style={{ display: 'grid', gridTemplateColumns: 'minmax(0, 1fr)', gap: '8px' }}>
                    {virtualFolders.map(f => (
                      <div 
                        key={f.name}
                        onClick={() => setCurrentFolder(f.name)}
                        style={{
                          display: 'flex', alignItems: 'center', justifyContent: 'space-between',
                          padding: '12px 16px', background: 'rgba(255,255,255,0.02)',
                          borderRadius: '8px', cursor: 'pointer', border: '1px solid rgba(255,255,255,0.05)',
                          transition: 'all 0.2s'
                        }}
                        onMouseEnter={e => e.currentTarget.style.background = 'rgba(255,255,255,0.05)'}
                        onMouseLeave={e => e.currentTarget.style.background = 'rgba(255,255,255,0.02)'}
                      >
                        <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                          <span style={{ fontSize: '20px' }}>📁</span>
                          <span style={{ fontFamily: 'var(--mono)', fontSize: '13px', color: '#fff' }}>
                            {f.name}
                          </span>
                        </div>
                        <span style={{ fontSize: '11px', color: 'rgba(255,255,255,0.5)', background: 'rgba(0,0,0,0.3)', padding: '2px 8px', borderRadius: '12px' }}>
                          {f.count}
                        </span>
                      </div>
                    ))}
                  </div>
                ) : (
                  /* Folder View: Show Assets */
                  <div className="premium-gallery">
                    {activeAssets.length === 0 && (
                      <div style={{ gridColumn: '1 / -1', textAlign: 'center', padding: '20px', color: 'rgba(255,255,255,0.3)', fontStyle: 'italic' }}>
                        Folder is empty.
                      </div>
                    )}
                    {activeAssets.map((asset) => (
                      <AssetCard
                        key={asset.filename}
                        asset={asset}
                        onPreview={setPreviewAsset}
                      />
                    ))}
                  </div>
                )}
              </div>
            </div>
          )}

          {activeTool === 'forgeTools' && (
            <ForgeToolbar forgeRef={forgeRef} />
          )}

        </div>

        {/* ── RIGHT PANE: Forge World ── */}
        <ForgeViewport onLog={addForgeLog} forgeRef={forgeRef} />

      </div>

      {/* Preview Modal */}
      <PreviewModal asset={previewAsset} onClose={() => setPreviewAsset(null)} />
    </div>
  );
}
