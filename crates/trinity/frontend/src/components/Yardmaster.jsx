import React, { useState, useRef, useEffect, useMemo } from 'react';
import { useYardmaster } from '../hooks/useYardmaster';
import { useCreative } from '../hooks/useCreative';
import MicButton from './MicButton';
import '../styles/yardmaster_premium.css';

const FOCUS_TAGS = [
  { id: 'system tools', icon: '📊', label: 'System' },
  { id: 'quest management', icon: '📜', label: 'Quests' },
  { id: 'code editing', icon: '🔧', label: 'Code' },
  { id: 'file search', icon: '🔍', label: 'Search' },
  { id: 'avatar creation', icon: '🎭', label: 'Avatar' },
  { id: 'image generation', icon: '🖼️', label: 'Image' },
];

const PROJECT_SCOPES = [
  { id: 'global',    icon: '🌐', label: 'Global',    path: null,                          tip: 'Full workspace — no constraints' },
  { id: 'frontend',  icon: '⚛️', label: 'Frontend',  path: 'crates/trinity/frontend/src', tip: 'React components, hooks, styles' },
  { id: 'backend',   icon: '🦀', label: 'Backend',   path: 'crates/trinity/src',          tip: 'Rust Axum server modules' },
  { id: 'protocol',  icon: '📐', label: 'Protocol',  path: 'crates/trinity-protocol/src', tip: 'Shared types and traits' },
  { id: 'quest',     icon: '🚂', label: 'Quest',     path: 'crates/trinity-quest/src',    tip: 'Quest board, XP, objectives' },
  { id: 'voice',     icon: '🎤', label: 'Voice',     path: 'crates/trinity-voice/src',    tip: 'SSML, VAAM, voice pipeline' },
  { id: 'templates', icon: '🎮', label: 'Templates', path: 'templates/',                  tip: 'Bevy game templates' },
  { id: 'docs',      icon: '📖', label: 'Docs',      path: 'docs/',                       tip: 'Documentation and maturation map' },
  { id: 'scripts',   icon: '⚡', label: 'Scripts',   path: 'scripts/',                    tip: 'Launch, systemd, and test scripts' },
];

const PERSONAS = [
  { id: 'dev', icon: '🔧', label: 'Dev', tip: 'Default mode — no slot pinning', slot: null },
  { id: 'recycler', icon: '🔮', label: 'Recycler', tip: 'Great Recycler — slot 0 (inhale: strategic planner)', slot: 0 },
  { id: 'programmer', icon: '⚙️', label: 'Pete', tip: 'Programmer Pete — slot 1 (exhale: builder/executor)', slot: 1 },
];

/* ─── Global Beast Logger Helpers ─── */
function tagClass(tag) {
  const t = (tag || '').toUpperCase();
  if (['SUCCESS', 'COMPLETE', 'DONE'].includes(t)) return 'beast-tag--success';
  if (['ERROR', 'FAILED', 'CRITICAL'].includes(t)) return 'beast-tag--error';
  if (t === 'COMFYUI') return 'beast-tag--comfyui'; // legacy — kept for log compatibility
  if (t === 'ACE_STEP') return 'beast-tag--ace';
  if (t === 'AVATAR') return 'beast-tag--avatar';
  if (t === 'FORGE' || t === 'YARD') return 'beast-tag--forge';
  if (t === 'SYSTEM') return 'beast-tag--ace';
  return '';
}

function fmtTime(ts) {
  try {
    return new Date(ts).toLocaleTimeString([], { hour12: false, hour: '2-digit', minute: '2-digit', second: '2-digit' });
  } catch { return '—'; }
}

// Strip raw JSON tool calls and truncated file content from AI output
function cleanToolCalls(text) {
  if (!text) return '';
  return text
    .split('\n')
    .filter(line => {
      const trimmed = line.trim();
      // Remove raw JSON tool calls like {"tool": "read_file", ...}
      if (trimmed.startsWith('{"tool"')) return false;
      // Remove lines that are just JSON tool calls with backticks
      if (trimmed.startsWith('``{"tool"')) return false;
      // Remove truncated file content markers
      if (trimmed.startsWith('[truncated:')) return false;
      return true;
    })
    .join('\n')
    // Collapse excessive blank lines left by filtering
    .replace(/\n{3,}/g, '\n\n')
    .trim();
}

// Simple markdown → HTML for AI messages
// FIX: Extract code blocks FIRST, then escape HTML on remaining text,
// then reinsert code blocks. This prevents <pre>/<code> from being mangled.
function renderYmMarkdown(text) {
  if (!text) return '';
  const cleaned = cleanToolCalls(text);
  if (!cleaned) return '';

  // Step 0: Convert markdown images BEFORE any escaping
  // Pattern: ![alt text](url) → <img> with inline style
  // We handle this first so the URL isn't mangled by HTML-escaping
  const imageTokens = [];
  let withImageTokens = cleaned.replace(/!\[([^\]]*)\]\(([^)]+)\)/g, (_, alt, url) => {
    const idx = imageTokens.length;
    const safeUrl = url.replace(/"/g, '&quot;');
    const safeAlt = alt.replace(/</g, '&lt;').replace(/>/g, '&gt;');
    if (safeUrl.match(/\.(mp3|wav|ogg)$/i)) {
      imageTokens.push(`<div class="ym-audio-container"><audio controls src="${safeUrl}" style="width:100%; max-width:400px; margin:8px 0; display:block;" /><div class="ym-image-caption">${safeAlt || 'Audio clip'}</div></div>`);
    } else {
      imageTokens.push(`<div class="ym-image-container"><img src="${safeUrl}" alt="${safeAlt}" class="ym-inline-image" loading="lazy" style="max-width:100%;border-radius:8px;margin:8px 0;display:block;" /><div class="ym-image-caption">${safeAlt || 'Generated image'}</div></div>`);
    }
    return `__IMGTOKEN_${idx}__`;
  });

  // Step 1: Extract fenced code blocks into placeholders
  const codeBlocks = [];
  let withPlaceholders = withImageTokens.replace(/```(\w*)\n([\s\S]*?)```/g, (_, lang, code) => {
    const idx = codeBlocks.length;
    // Escape HTML inside code blocks separately
    const escaped = code.replace(/</g, '&lt;').replace(/>/g, '&gt;');
    codeBlocks.push(`<pre class="ym-code-block"><code>${escaped}</code></pre>`);
    return `__CODEBLOCK_${idx}__`;
  });

  // Step 2: Escape HTML in the rest (so <thinking> tags show as text)
  let html = withPlaceholders
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;');

  // Step 3: Apply markdown formatting
  html = html
    .replace(/^### (.+)$/gm, '<h3 class="ym-h3">$1</h3>')
    .replace(/^## (.+)$/gm, '<h2 class="ym-h2">$1</h2>')
    .replace(/^# (.+)$/gm, '<h1 class="ym-h1">$1</h1>')
    .replace(/`([^`]+)`/g, '<code class="ym-inline-code">$1</code>')
    .replace(/\*\*\*(.+?)\*\*\*/g, '<strong><em>$1</em></strong>')
    .replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
    .replace(/\*(.+?)\*/g, '<em>$1</em>')
    .replace(/^[*-] (.+)$/gm, '<li>$1</li>')
    .replace(/^\d+\. (.+)$/gm, '<li>$1</li>');

  html = html.replace(/((?:<li>.*<\/li>\n?)+)/g, '<ul>$1</ul>');
  html = html.split(/\n{2,}/).map(block => {
    const t = block.trim();
    if (!t) return '';
    if (t.startsWith('<pre') || t.startsWith('<ul') || t.startsWith('<ol') || t.startsWith('<h') || t.startsWith('__CODEBLOCK_') || t.startsWith('__IMGTOKEN_')) return t;
    return `<p>${t.replace(/\n/g, '<br/>')}</p>`;
  }).join('');

  // Step 4: Reinsert code blocks
  codeBlocks.forEach((block, i) => {
    html = html.replace(`__CODEBLOCK_${i}__`, block);
  });

  // Step 5: Reinsert image tokens
  imageTokens.forEach((block, i) => {
    html = html.replace(`__IMGTOKEN_${i}__`, block);
  });

  return html;
}

/* ═══════════════════════════════════════════════
   YARDMASTER — IDE-Grade Development Studio
   ═══════════════════════════════════════════════ */
export default function Yardmaster() {
  const {
    messages,
    activityLogs: forgeLines,
    sending,
    focus,
    hardware,
    tools,
    thinking,
    questState,
    turnInfo,
    modelInfo,
    taskQueue,
    toggleFocus,
    sendMessage,
    cancelRequest,
    startSession,
  } = useYardmaster();

  const { logs: creativeLogs } = useCreative();
  const [sessionInput, setSessionInput] = useState('');
  const [showSessionInput, setShowSessionInput] = useState(false);

  const allLogs = useMemo(() => {
    const formattedYards = forgeLines.map((line, i) => ({
      id: `yard-${i}`,
      timestamp: line.timestamp || new Date().toISOString(),
      tag: line.type === 'result' ? 'FORGE' : 'YARD',
      message: line.text,
    }));
    return [...creativeLogs, ...formattedYards].sort((a, b) => new Date(a.timestamp) - new Date(b.timestamp));
  }, [creativeLogs, forgeLines]);

  const [input, setInput] = useState('');
  const [persona, setPersona] = useState('dev');
  const [scope, setScope] = useState('global');
  const [thinkingOpen, setThinkingOpen] = useState(true);
  const chatRef = useRef(null);
  const forgeRef = useRef(null);
  const inputRef = useRef(null);

  const activeScope = PROJECT_SCOPES.find(s => s.id === scope) || PROJECT_SCOPES[0];

  // Auto-scroll chat and beast logger
  useEffect(() => {
    if (chatRef.current) chatRef.current.scrollTop = chatRef.current.scrollHeight;
  }, [messages]);
  useEffect(() => {
    if (forgeRef.current) forgeRef.current.scrollTop = forgeRef.current.scrollHeight;
  }, [allLogs]);

  // Ignition state is SERVER-DRIVEN via hardware.ignition_status
  // No local timer — survives tab switches because it lives in AppState.
  const ignitionStatus = hardware?.ignition_status || 'idle';
  const isIgniting = !['idle', 'ready', 'failed'].includes(ignitionStatus);
  const isOnline = hardware?.inference_server === 'connected';

  // Poll hardware at 2s during ignition, 10s otherwise (keeps UI in sync)
  useEffect(() => {
    const pollHz = isIgniting ? 2000 : 10000;
    const poll = setInterval(async () => {
      try {
        const res = await fetch('/api/hardware');
        if (res.ok) {
          const data = await res.json();
          // Update the hardware state in parent hook via ref
          if (window.__trinityHardwareUpdate) window.__trinityHardwareUpdate(data);
        }
      } catch (_) {}
    }, pollHz);
    return () => clearInterval(poll);
  }, [isIgniting]);

  // Phase labels for human-readable display
  const phaseLabels = {
    launching: '🚀 Connecting to vLLM...',
    daemon_up: '⚙️ Daemon active...',
    server_starting: '🔧 Starting API server...',
    polling: '📡 Waiting for server...',
    loading_model: '🧠 Loading Great Recycler...',
    ready: '🟢 FURNACE ONLINE',
    failed: '❌ Ignition Failed — Click to Retry',
  };

  // Focus input on mount and after sending completes
  useEffect(() => {
    if (!sending && inputRef.current) {
      inputRef.current.focus();
    }
  }, [sending]);

  const handleSend = () => {
    if (!input.trim()) return;
    sendMessage(input.trim(), persona, activeScope.path);
    setInput('');
    if (inputRef.current) inputRef.current.style.height = 'auto';
    requestAnimationFrame(() => {
      if (inputRef.current) inputRef.current.focus();
    });
  };

  const handleKey = (e) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  const handleInput = (e) => {
    setInput(e.target.value);
    e.target.style.height = 'auto';
    e.target.style.height = Math.min(e.target.scrollHeight, 120) + 'px';
  };

  return (
    <div className="ym-studio">
      {/* ── Model Info Bar ── */}
      <div className="ym-model-bar">
        <div className="ym-model-bar__left">
          <span className="ym-model-bar__dot" />
          <span className="ym-model-bar__name">{modelInfo.name}</span>
          <span className="ym-model-bar__badge">reasoning: {modelInfo.reasoning}</span>
          <span className="ym-model-bar__badge">{modelInfo.context} ctx</span>
          <span className="ym-model-bar__badge">active: {modelInfo.active_experts}</span>
        </div>
        <div className="ym-model-bar__right">
          <div className="ym-persona-toggle">
            {PERSONAS.map((p) => (
              <button
                key={p.id}
                className={`ym-persona-btn ${persona === p.id ? 'ym-persona-active' : ''}`}
                onClick={() => setPersona(p.id)}
                title={p.tip}
              >
                {p.icon} {p.label}
              </button>
            ))}
            {PERSONAS.find(p => p.id === persona)?.slot != null && (
              <span className="ym-model-bar__badge ym-badge-slot">
                KV#{PERSONAS.find(p => p.id === persona).slot}
              </span>
            )}
          </div>
          {sending && (
            <span className={`ym-model-bar__badge ym-badge-live ${turnInfo.continuations > 0 ? 'ym-badge-continuation' : ''}`}>
              {turnInfo.continuations > 0
                ? `🔄 continuation ${turnInfo.continuations}`
                : `⚡ turn ${turnInfo.turn}/8`}
            </span>
          )}
          {!sending && <span className="ym-model-bar__badge">idle</span>}
        </div>
      </div>

      {/* ── Three-column layout ── */}
      <div className="ym-3col-layout">

        {/* LEFT: Work Session Sidebar */}
        <div className="ym-quest-sidebar">
          <div className="card ym-quest-card">
            <div className="card-header" style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <span>🎯 WORK SESSION</span>
              <button
                className="ym-session-new-btn"
                onClick={() => setShowSessionInput(!showSessionInput)}
                title="Start a new work session"
              >
                {showSessionInput ? '✕' : '＋'}
              </button>
            </div>

            {/* New Session Input */}
            {showSessionInput && (
              <div className="ym-session-input-row">
                <input
                  className="ym-session-input"
                  placeholder="What are you working on?"
                  value={sessionInput}
                  onChange={e => setSessionInput(e.target.value)}
                  onKeyDown={e => {
                    if (e.key === 'Enter' && sessionInput.trim()) {
                      startSession(sessionInput.trim());
                      setSessionInput('');
                      setShowSessionInput(false);
                    }
                  }}
                  autoFocus
                />
                <button
                  className="ym-session-go-btn"
                  onClick={() => {
                    if (sessionInput.trim()) {
                      startSession(sessionInput.trim());
                      setSessionInput('');
                      setShowSessionInput(false);
                    }
                  }}
                  disabled={!sessionInput.trim()}
                >
                  GO
                </button>
              </div>
            )}

            {/* Session Info */}
            <div className="ym-quest-content">
              <div className="ym-quest-chapter">
                {questState?.subject || 'No active session'}
              </div>
              {questState?.subject && (
                <div className="ym-quest-phase">
                  Phase: <strong>{questState?.phase || 'Analysis'}</strong>
                </div>
              )}

              {/* Live Task Queue */}
              {taskQueue.length > 0 && (
                <div className="ym-task-queue">
                  <div className="ym-task-header">
                    <span>📋 Tasks</span>
                    <span className="ym-task-progress">
                      {taskQueue.filter(t => t.done).length}/{taskQueue.length}
                    </span>
                  </div>
                  <div className="ym-task-bar">
                    <div
                      className="ym-task-bar__fill"
                      style={{ width: `${(taskQueue.filter(t => t.done).length / taskQueue.length) * 100}%` }}
                    />
                  </div>
                  {taskQueue.map((task) => (
                    <div key={task.index} className={`ym-task-item ${task.done ? 'ym-task-item--done' : ''}`}>
                      <span className="ym-task-check">{task.done ? '✓' : '○'}</span>
                      <span className="ym-task-text">{task.text}</span>
                    </div>
                  ))}
                </div>
              )}

              {/* Stats (compact) */}
              <div className="ym-quest-stats">
                <div className="ym-quest-stat">
                  <span className="ym-quest-stat__val">⛏️ {questState?.coal?.toFixed(0) || 0}</span>
                  <span className="ym-quest-stat__label">Coal</span>
                </div>
                <div className="ym-quest-stat">
                  <span className="ym-quest-stat__val">💨 {questState?.steam?.toFixed(0) || 0}</span>
                  <span className="ym-quest-stat__label">Steam</span>
                </div>
                <div className="ym-quest-stat">
                  <span className="ym-quest-stat__val">⭐ {questState?.xp || 0}</span>
                  <span className="ym-quest-stat__label">XP</span>
                </div>
              </div>
            </div>
          </div>

          {/* Beast Logger Master Terminal */}
          <div className="beast-logger card" style={{ flex: 1, minHeight: 0, marginTop: '16px', marginBottom: '0', overflowY: 'auto' }} ref={forgeRef}>
            <div className="card-header" style={{ position: 'sticky', top: 0, background: 'var(--bg-card)', zIndex: 10 }}>
              🖥️ BEAST LOGGER — ROOT DEV ZONE <span style={{ float: 'right', fontSize: '10px', color: 'var(--text-dim)' }}>{allLogs.length} events</span>
            </div>
            {allLogs.length === 0 ? (
              <div style={{ padding: '20px', color: 'var(--text-dim)', fontStyle: 'italic', textAlign: 'center' }}>
                System initializing telemetry stream...
              </div>
            ) : (
              allLogs.slice(-150).map((log, i) => {
                const isExpandable = log.message?.length > 120;
                return (
                  <div key={log.id || i} className="beast-log-line" style={{ display: 'flex', gap: '12px', padding: '4px 8px', borderBottom: '1px solid rgba(255,255,255,0.02)', fontFamily: 'var(--mono)', fontSize: '11px', alignItems: 'flex-start' }}>
                    <span style={{ color: 'var(--text-dim)', flexShrink: 0 }}>[{fmtTime(log.timestamp)}]</span>
                    <span className={`beast-tag ${tagClass(log.tag)}`} style={{ minWidth: '70px', flexShrink: 0, textAlign: 'center' }}>[{log.tag}]</span>
                    {isExpandable ? (
                      <details className="forge-details" style={{ flex: 1, color: 'var(--text)', wordWrap: 'break-word', whiteSpace: 'pre-wrap' }}>
                        <summary className="forge-summary">{log.message.substring(0, 120)}…</summary>
                        <div style={{ marginTop: '4px', opacity: 0.8 }}>{log.message}</div>
                      </details>
                    ) : (
                      <span style={{ flex: 1, color: 'var(--text)', wordWrap: 'break-word', whiteSpace: 'pre-wrap' }}>{log.message}</span>
                    )}
                  </div>
                );
              })
            )}
            <div className="forge-line" style={{ padding: '4px 8px' }}>
              <span className="forge-cursor" />
            </div>
          </div>
        </div>

        {/* CENTER: Chat + Forge */}
        <div 
          className="ym-chat-col"
          onDragOver={(e) => {
            e.preventDefault();
            e.currentTarget.style.boxShadow = 'inset 0 0 40px rgba(52, 211, 153, 0.1)';
          }}
          onDragLeave={(e) => {
            e.currentTarget.style.boxShadow = 'none';
          }}
          onDrop={(e) => {
            e.preventDefault();
            e.currentTarget.style.boxShadow = 'none';
            const hookStr = e.dataTransfer.getData('text/plain');
            if (hookStr && hookStr.startsWith('CastHook:')) {
              const hookId = hookStr.split(':')[1];
              const prompts = {
                'Pearl': '🔮 I am casting the PEARL hook. Please review the current codebase against the 5 PEARL dimensions (Purpose, Evidence, Alignment, Rigor, Learner-centricity) and provide a scorecard.',
                'Coal': '🪨 I am casting the COAL hook. Please generate the raw boilerplate and scaffolding for the next feature, focusing on getting it working before we refine it.',
                'Steam': '💨 I am casting the STEAM hook. Please review the current implementation and optimize it for performance and cognitive load reduction.',
                'Hook': '🪝 I am casting the HOOK hook. Let’s integrate a new interactive system or API here to increase engagement.',
                'Mirror': '🪞 I am casting the MIRROR hook. Adopt your Socratic persona and ask me 3 deep questions about why I am designing this feature this way.',
                'Compass': '🧭 I am casting the COMPASS hook. Please analyze my current trajectory and provide a roadmap for the next 3 steps in the ADDIECRAPEYE framework.'
              };
              const msg = prompts[hookId] || `I am casting the ${hookId} hook. Please assist me.`;
              sendMessage(msg, 'programmer', activeScope.path);
            }
          }}
          style={{ transition: 'box-shadow 0.2s' }}
        >
          {/* Scope + Focus Buttons */}
          <div className="ym-focus-bar">
            <div style={{ display: 'flex', gap: '4px', flexWrap: 'wrap', alignItems: 'center' }}>
              <span style={{ fontSize: '9px', color: '#64748b', fontFamily: "'JetBrains Mono', monospace", letterSpacing: '1px', marginRight: '4px' }}>SCOPE:</span>
              {PROJECT_SCOPES.map((s) => (
                <button
                  key={s.id}
                  className={`focus-btn ${scope === s.id ? 'active' : ''}`}
                  onClick={() => setScope(s.id)}
                  title={s.tip}
                  style={{ fontSize: '10px', padding: '2px 6px' }}
                >
                  {s.icon} {s.label}
                </button>
              ))}
            </div>
            <div style={{ display: 'flex', gap: '4px', flexWrap: 'wrap', alignItems: 'center', marginTop: '4px' }}>
              <span style={{ fontSize: '9px', color: '#64748b', fontFamily: "'JetBrains Mono', monospace", letterSpacing: '1px', marginRight: '4px' }}>FOCUS:</span>
              {FOCUS_TAGS.map((t) => (
                <button
                  key={t.id}
                  className={`focus-btn ${focus.has(t.id) ? 'active' : ''}`}
                  onClick={() => toggleFocus(t.id)}
                >
                  {t.icon} {t.label}
                </button>
              ))}
            </div>
          </div>

          {/* Thinking / Reasoning Panel */}
          {thinking && (
            <div className="ym-thinking-panel">
              <button
                className="ym-thinking-toggle"
                onClick={() => setThinkingOpen(!thinkingOpen)}
              >
                🧠 REASONING {thinkingOpen ? '▾' : '▸'}
              </button>
              {thinkingOpen && (
                <div className="ym-thinking-content">{thinking}</div>
              )}
            </div>
          )}

          {/* Chat Messages */}
          <div className="ym-messages" ref={chatRef}>
            {messages.map((msg, i) => (
              <div key={i} className={`chat-msg ${msg.role === 'user' ? 'ym-msg-user' : msg.role === 'image' || msg.role === 'audio' ? 'ym-msg-image' : msg.role === 'error' ? 'ym-msg-error' : 'ym-msg-ai'}`}>
                {msg.speaker && (
                  <div className="ym-speaker">{msg.speaker}</div>
                )}
                {msg.role === 'image' ? (
                  <div className="ym-msg-body ym-image-container">
                    <img
                      src={msg.url || `data:image/png;base64,${msg.base64}`}
                      alt={msg.filename || 'Generated image'}
                      className="ym-inline-image"
                    />
                    <div className="ym-image-caption">{msg.content}</div>
                  </div>
                ) : msg.role === 'audio' ? (
                  <div className="ym-msg-body ym-audio-container">
                    <audio 
                      controls 
                      src={msg.url || `data:audio/wav;base64,${msg.base64}`} 
                      style={{ width: '100%', height: '32px', borderRadius: '4px' }} 
                    />
                    <div className="ym-image-caption">{msg.content}</div>
                  </div>
                ) : msg.role === 'user' ? (
                  <div className="ym-msg-body">{msg.content}</div>
                ) : (
                  <div
                    className="ym-msg-body"
                    dangerouslySetInnerHTML={{
                      __html: renderYmMarkdown(msg.content) || (sending && i === messages.length - 1 ? '▍' : ''),
                    }}
                  />
                )}
              </div>
            ))}
          </div>



          {/* Input */}
          <div className="ym-input-row">
            <textarea
              ref={inputRef}
              id="ym-input"
              className="chat-input ym-input"
              placeholder="What do you need done? (Enter to send, Shift+Enter for newline)"
              value={input}
              onChange={handleInput}
              onKeyDown={handleKey}
              disabled={sending}
              rows={1}
            />
            <MicButton
              onTranscript={(text) => setInput(prev => prev ? prev + ' ' + text : text)}
              disabled={sending}
            />
            {sending ? (
              <button
                className="chat-send ym-stop-btn"
                onClick={cancelRequest}
                title="Stop the agent"
              >
                ⛔ STOP
              </button>
            ) : (
              <button
                className="chat-send ym-send-btn"
                onClick={handleSend}
                disabled={!input.trim()}
              >
                SEND
              </button>
            )}
          </div>
        </div>

        {/* RIGHT: System + Tools */}
        <div className="ym-explorer">
          {/* System Status */}
          <div className="card ym-system-card">
            <div className="card-header">⚙️ SYSTEM STATUS</div>
            {hardware ? (
              <div className="ym-hardware">
                <div className="ym-hw-row"><span>CPU</span><span>{hardware.cpu_load?.toFixed(1)}%</span></div>
                <div className="ym-hw-row"><span>Memory</span><span>{hardware.mem_used_gb?.toFixed(1)} / {hardware.mem_total_gb?.toFixed(0)} GB ({hardware.mem_percent?.toFixed(1)}%)</span></div>
                <div className="ym-hw-row"><span>GPU</span><span>{hardware.gpu_load?.toFixed(0)}%</span></div>
                {hardware.npu_load > 0 && <div className="ym-hw-row"><span>NPU</span><span>{hardware.npu_load?.toFixed(0)}%</span></div>}
                <div className="ym-hw-row">
                  <span>LLM</span>
                  <span className={hardware.inference_server === 'connected' ? 'ym-hw-ok' : 'ym-hw-err'}>
                    {hardware.inference_server || 'unknown'}
                  </span>
                </div>
                <div className="ym-hw-row"><span>Database</span><span className={hardware.database === 'connected' ? 'ym-hw-ok' : 'ym-hw-err'}>{hardware.database || 'unknown'}</span></div>
                <button
                  className="ym-send-btn"
                  style={{
                    width: '100%', marginTop: '12px', padding: '10px',
                    background: isOnline
                      ? 'linear-gradient(135deg, #22c55e, #10b981)'
                      : ignitionStatus === 'failed'
                        ? 'linear-gradient(135deg, #ef4444, #dc2626)'
                        : 'linear-gradient(135deg, #ef4444, #f59e0b)',
                    color: isOnline ? '#fff' : '#000',
                    fontSize: '11px',
                    opacity: isOnline ? 0.7 : 1,
                    cursor: (isIgniting && ignitionStatus !== 'failed') ? 'wait' : 'pointer',
                  }}
                  onClick={async (e) => {
                    if (isIgniting || isOnline) return;
                    try {
                      const st = JSON.parse(localStorage.getItem('trinitySetupState') || '{"engine":"vllm"}');
                      await fetch('/api/system/backend-start', {
                          method: 'POST',
                          headers: { 'Content-Type': 'application/json' },
                          body: JSON.stringify({ backend: st.engine })
                      });
                    } catch(e) { /* fallback */ }
                  }}
                  disabled={isIgniting && ignitionStatus !== 'failed'}
                  title="Connects to vLLM on port 8010"
                >
                  {isOnline
                    ? '🟢 FURNACE ONLINE'
                    : phaseLabels[ignitionStatus] || '🔥 IGNITE FURNACE'
                  }
                </button>
              </div>
            ) : (
              <div className="ym-hw-loading">Checking system…</div>
            )}
          </div>

          {/* Available Tools -> The Hook Book */}
          <div className="card ym-tools-card">
            <div className="card-header">📖 THE HOOK BOOK ({tools.length})</div>
            <div className="ym-tool-grid">
              {tools.length === 0 && <div className="ym-hw-loading">Loading tools…</div>}
              {tools.map((tool) => (
                <div key={tool.name} className="ym-tool-item">
                  <div className="ym-tool-name">{tool.name}</div>
                  <div className="ym-tool-desc">{tool.description}</div>
                </div>
              ))}
            </div>
          </div>

          {/* RAG Search */}
          <RagSearch />

          {/* Background Jobs */}
          <JobsPanel />
        </div>
      </div>
    </div>
  );
}


/* ── RAG Search sub-component ── */
function RagSearch() {
  const [query, setQuery] = React.useState('');
  const [results, setResults] = React.useState([]);
  const [stats, setStats] = React.useState(null);

  React.useEffect(() => {
    fetch('/api/rag/stats').then(r => r.json()).then(setStats).catch(() => {});
  }, []);

  const search = async () => {
    if (!query.trim()) return;
    try {
      const res = await fetch('/api/rag/search', {
        method: 'POST', headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ query: query.trim(), limit: 5 }),
      });
      if (res.ok) setResults(await res.json());
    } catch {}
  };

  return (
    <div className="card ym-system-card">
      <div className="card-header">🔍 RAG KNOWLEDGE
        {stats && <span style={{ float: 'right', fontSize: '9px', color: '#6B7280' }}>{stats.total_documents || stats.documents || 0} docs</span>}
      </div>
      <div className="ym-hardware">
        <div style={{ display: 'flex', gap: '4px', marginBottom: '6px' }}>
          <input
            className="chat-input"
            style={{ fontSize: '11px', padding: '4px 8px', flex: 1 }}
            placeholder="Search knowledge base…"
            value={query}
            onChange={e => setQuery(e.target.value)}
            onKeyDown={e => e.key === 'Enter' && search()}
          />
          <button onClick={search} style={{ background: 'none', border: '1px solid rgba(207,185,145,0.2)', borderRadius: '4px', color: '#CFB991', cursor: 'pointer', padding: '2px 8px', fontSize: '10px' }}>🔍</button>
        </div>
        {results.length > 0 && results.map((r, i) => (
          <div key={i} className="ym-hw-row" style={{ flexDirection: 'column', alignItems: 'flex-start' }}>
            <span style={{ fontSize: '10px', color: '#CFB991' }}>{r.title || r.source || `Result ${i+1}`}</span>
            <span style={{ fontSize: '9px', color: '#6B7280' }}>{(r.content || r.text || '').slice(0, 120)}…</span>
          </div>
        ))}
      </div>
    </div>
  );
}

/* ── Background Jobs sub-component ── */
function JobsPanel() {
  const [jobs, setJobs] = React.useState([]);
  const [total, setTotal] = React.useState(0);
  const [isSubmitting, setIsSubmitting] = React.useState(false);
  const [newTask, setNewTask] = React.useState('');
  const [mode, setMode] = React.useState('dev');

  const fetchJobs = async () => {
    try {
      const res = await fetch('/api/jobs');
      if (res.ok) {
        const data = await res.json();
        setJobs(data.jobs || []);
        setTotal(data.total || 0);
      }
    } catch {}
  };

  React.useEffect(() => {
    fetchJobs();
    const interval = setInterval(fetchJobs, 5000);
    return () => clearInterval(interval);
  }, []);

  const submitJob = async () => {
    if (!newTask.trim()) return;
    setIsSubmitting(true);
    try {
      const res = await fetch('/api/jobs', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: newTask.trim(), mode })
      });
      if (res.ok) {
        setNewTask('');
        fetchJobs();
      }
    } catch {} finally {
      setIsSubmitting(false);
    }
  };

  const cancelJob = async (id) => {
    try {
      await fetch(`/api/jobs/${id}`, { method: 'DELETE' });
      fetchJobs();
    } catch {}
  };

  return (
    <div className="card ym-system-card" style={{ marginTop: '16px' }}>
      <div className="card-header">🔧 OVERNIGHT CREW
        <span style={{ float: 'right', fontSize: '9px', color: '#6B7280' }}>
          {jobs.filter(j => j.status === 'running').length} active / {total} total
        </span>
      </div>
      
      <div className="ym-hardware" style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
        {/* Job Queue List */}
        {jobs.length === 0 && <div className="ym-hw-row" style={{ color: '#6B7280', fontSize: '10px' }}>No background jobs</div>}
        
        {jobs.slice(0, 5).map(job => (
          <div key={job.id} className="ym-hw-row" style={{ flexDirection: 'column', alignItems: 'flex-start', borderLeft: job.status === 'running' ? '2px solid #3b82f6' : job.status === 'complete' ? '2px solid #22c55e' : '2px solid #ef4444', paddingLeft: '6px' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', width: '100%' }}>
              <span style={{ fontSize: '10px', color: '#e5e7eb', fontWeight: 500 }}>
                {job.status === 'running' ? '🔄' : job.status === 'complete' ? '✅' : '🛑'} {job.status.toUpperCase()}
              </span>
              <span style={{ fontSize: '9px', color: '#9ca3af' }}>Turns: {job.turns_used}</span>
            </div>
            <div style={{ fontSize: '10px', color: '#d1d5db', marginTop: '2px', whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis', width: '100%' }}>
              "{job.message}"
            </div>
            {job.status === 'running' && (
              <button onClick={() => cancelJob(job.id)} style={{ marginTop: '4px', background: '#374151', border: '1px solid #4b5563', borderRadius: '4px', color: '#f87171', fontSize: '9px', padding: '2px 6px', cursor: 'pointer' }}>
                Cancel Job
              </button>
            )}
            {job.output_path && (
              <div style={{ fontSize: '9px', color: '#6B7280', marginTop: '2px', wordBreak: 'break-all' }}>
                Saved to: {job.output_path.split('/').pop()}
              </div>
            )}
          </div>
        ))}
        
        {/* Submit New Job */}
        <div style={{ borderTop: '1px solid rgba(255,255,255,0.05)', paddingTop: '8px', marginTop: '4px', display: 'flex', flexDirection: 'column', gap: '4px' }}>
          <div style={{ fontSize: '10px', color: '#9ca3af', marginBottom: '2px' }}>New Autonomous Task:</div>
          <select 
            value={mode} 
            onChange={e => setMode(e.target.value)}
            style={{ fontSize: '10px', padding: '4px', background: 'rgba(0,0,0,0.5)', border: '1px solid rgba(255,255,255,0.1)', color: '#fff', borderRadius: '4px' }}
          >
            <option value="dev">Dev Mode (No Persona)</option>
            <option value="programmer">Programmer Pete (Builder)</option>
            <option value="recycler">Great Recycler (Strategist)</option>
          </select>
          <textarea
            className="chat-input"
            style={{ fontSize: '11px', padding: '6px', minHeight: '50px', resize: 'vertical' }}
            placeholder="e.g. Build the Truth e-learning module..."
            value={newTask}
            onChange={e => setNewTask(e.target.value)}
          />
          <button 
            onClick={submitJob}
            disabled={isSubmitting || !newTask.trim()}
            style={{ 
              background: isSubmitting ? '#374151' : 'rgba(59, 130, 246, 0.2)', 
              border: '1px solid #3b82f6', 
              color: '#93c5fd', 
              padding: '6px', 
              borderRadius: '6px', 
              cursor: (isSubmitting || !newTask.trim()) ? 'not-allowed' : 'pointer',
              fontSize: '11px',
              fontWeight: 500
            }}
          >
            {isSubmitting ? 'Queueing...' : '► Start Background Job'}
          </button>
        </div>
      </div>
    </div>
  );
}
