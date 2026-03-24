import React, { useState, useRef, useEffect } from 'react';
import { useYardmaster } from '../hooks/useYardmaster';

const FOCUS_TAGS = [
  { id: 'system tools', icon: '📊', label: 'System' },
  { id: 'quest management', icon: '📜', label: 'Quests' },
  { id: 'code editing', icon: '🔧', label: 'Code' },
  { id: 'file search', icon: '🔍', label: 'Search' },
  { id: 'avatar creation', icon: '🎭', label: 'Avatar' },
  { id: 'image generation', icon: '🖼️', label: 'Image' },
];

const PERSONAS = [
  { id: 'dev', icon: '🔧', label: 'Dev', tip: 'Default mode — no slot pinning', slot: null },
  { id: 'recycler', icon: '🔮', label: 'Recycler', tip: 'Great Recycler — slot 0 (inhale: strategic planner)', slot: 0 },
  { id: 'programmer', icon: '⚙️', label: 'Pete', tip: 'Programmer Pete — slot 1 (exhale: builder/executor)', slot: 1 },
];

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
      // Remove very long file dumps (>2000 chars of raw code)
      if (trimmed.length > 2000 && !trimmed.startsWith('▶') && !trimmed.startsWith('#')) return false;
      return true;
    })
    .join('\n')
    // Collapse excessive blank lines left by filtering
    .replace(/\n{3,}/g, '\n\n')
    .trim();
}

// Simple markdown → HTML for AI messages
function renderYmMarkdown(text) {
  if (!text) return '';
  // Clean tool calls first
  const cleaned = cleanToolCalls(text);
  if (!cleaned) return '';
  let html = cleaned
    .replace(/```(\w*)\n([\s\S]*?)```/g, (_, lang, code) =>
      `<pre class="ym-code-block"><code>${code.replace(/</g, '&lt;').replace(/>/g, '&gt;')}</code></pre>`)
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
    if (t.startsWith('<pre') || t.startsWith('<ul') || t.startsWith('<ol') || t.startsWith('<h')) return t;
    return `<p>${t.replace(/\n/g, '<br/>')}</p>`;
  }).join('');

  return html;
}

/* ═══════════════════════════════════════════════
   YARDMASTER — IDE-Grade Development Studio
   ═══════════════════════════════════════════════ */
export default function Yardmaster() {
  const {
    messages,
    forgeLines,
    sending,
    focus,
    hardware,
    tools,
    thinking,
    questState,
    turnInfo,
    modelInfo,
    toggleFocus,
    sendMessage,
  } = useYardmaster();

  const [input, setInput] = useState('');
  const [persona, setPersona] = useState('dev');
  const [thinkingOpen, setThinkingOpen] = useState(true);
  const chatRef = useRef(null);
  const forgeRef = useRef(null);
  const inputRef = useRef(null);

  // Auto-scroll chat and forge
  useEffect(() => {
    if (chatRef.current) chatRef.current.scrollTop = chatRef.current.scrollHeight;
  }, [messages]);
  useEffect(() => {
    if (forgeRef.current) forgeRef.current.scrollTop = forgeRef.current.scrollHeight;
  }, [forgeLines]);

  // Focus input on mount and after sending completes
  useEffect(() => {
    if (!sending && inputRef.current) {
      inputRef.current.focus();
    }
  }, [sending]);

  const handleSend = () => {
    if (!input.trim()) return;
    sendMessage(input.trim(), persona);
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

        {/* LEFT: Quest Sidebar */}
        <div className="ym-quest-sidebar">
          <div className="card ym-quest-card">
            <div className="card-header">📜 QUEST STATUS</div>
            {questState ? (
              <div className="ym-quest-content">
                <div className="ym-quest-chapter">
                  {questState.quest?.quest_title || 'The Ordinary World'}
                </div>
                <div className="ym-quest-phase">
                  Phase: <strong>{questState.quest?.current_phase || 'Analysis'}</strong>
                </div>
                <div className="ym-quest-subject">
                  {questState.quest?.subject || 'No subject'}
                </div>

                {/* Objectives */}
                <div className="ym-quest-objectives">
                  {(questState.quest?.phase_objectives || []).map((obj, i) => (
                    <div key={i} className={`ym-quest-obj ${obj.completed ? 'ym-quest-obj--done' : ''}`}>
                      <span className="ym-quest-obj__check">
                        {obj.completed ? '✓' : '○'}
                      </span>
                      <span className="ym-quest-obj__text">{obj.description}</span>
                    </div>
                  ))}
                </div>

                {/* Stats */}
                <div className="ym-quest-stats">
                  <div className="ym-quest-stat">
                    <span className="ym-quest-stat__val">⛏️ {questState.stats?.coal_reserves?.toFixed(0) || 0}</span>
                    <span className="ym-quest-stat__label">Coal</span>
                  </div>
                  <div className="ym-quest-stat">
                    <span className="ym-quest-stat__val">💨 {questState.quest?.steam_generated?.toFixed(0) || 0}</span>
                    <span className="ym-quest-stat__label">Steam</span>
                  </div>
                  <div className="ym-quest-stat">
                    <span className="ym-quest-stat__val">⭐ {questState.stats?.total_xp || 0}</span>
                    <span className="ym-quest-stat__label">XP</span>
                  </div>
                </div>
              </div>
            ) : (
              <div className="ym-hw-loading">Loading quest…</div>
            )}
          </div>

          {/* Thinking Panel */}
          <div className="card ym-thinking-card">
            <div
              className="card-header ym-thinking-header"
              onClick={() => setThinkingOpen(!thinkingOpen)}
            >
              🧠 REASONING {thinking ? '●' : '○'}
              <span className="ym-thinking-toggle">{thinkingOpen ? '▾' : '▸'}</span>
            </div>
            {thinkingOpen && (
              <div className="ym-thinking-content">
                {thinking ? (
                  <pre className="ym-thinking-text">{thinking}</pre>
                ) : (
                  <div className="ym-hw-loading">No reasoning yet — send a message to see the model think.</div>
                )}
              </div>
            )}
          </div>
        </div>

        {/* CENTER: Chat + Forge */}
        <div className="ym-chat-col">
          {/* Focus Buttons */}
          <div className="ym-focus-bar">
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

          {/* Chat Messages */}
          <div className="ym-messages" ref={chatRef}>
            {messages.map((msg, i) => (
              <div key={i} className={`chat-msg ${msg.role === 'user' ? 'ym-msg-user' : msg.role === 'image' ? 'ym-msg-image' : 'ym-msg-ai'}`}>
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

          {/* Forge Terminal */}
          <div className="forge-section">
            <div className="forge-header">
              <span className="forge-header__icon">⚒️</span>
              <span className="forge-header__title">THE FORGE</span>
              <span className="forge-header__count">{forgeLines.length} events</span>
            </div>
            <div className="forge-terminal" ref={forgeRef}>
              {forgeLines.map((line, i) => {
                if (line.type === 'result' && line.text.length > 80) {
                  return (
                    <details key={i} className="forge-line forge-details">
                      <summary className="forge-summary">
                        {line.text.substring(0, 60)}…
                      </summary>
                      <pre className="forge-result-full">{line.text}</pre>
                    </details>
                  );
                }
                return (
                  <div key={i} className={`forge-line ${line.type || ''}`}>
                    {line.text}
                  </div>
                );
              })}
              <div className="forge-line">
                <span className="forge-cursor" />
              </div>
            </div>
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
            <button
              className="chat-send ym-send-btn"
              onClick={handleSend}
              disabled={sending || !input.trim()}
            >
              {sending ? 'WORKING…' : 'SEND'}
            </button>
          </div>
        </div>

        {/* RIGHT: System + Tools */}
        <div className="ym-explorer">
          {/* System Status */}
          <div className="card ym-system-card">
            <div className="card-header">⚙️ SYSTEM STATUS</div>
            {hardware ? (
              <div className="ym-hardware">
                {hardware.cpu && <div className="ym-hw-row"><span>CPU</span><span>{hardware.cpu}</span></div>}
                {hardware.memory && <div className="ym-hw-row"><span>Memory</span><span>{hardware.memory}</span></div>}
                {hardware.gpu && <div className="ym-hw-row"><span>GPU</span><span>{hardware.gpu}</span></div>}
                {hardware.disk && <div className="ym-hw-row"><span>Disk</span><span>{hardware.disk}</span></div>}
                {hardware.llm_status && (
                  <div className="ym-hw-row">
                    <span>LLM</span>
                    <span className={hardware.llm_status === 'connected' ? 'ym-hw-ok' : 'ym-hw-err'}>
                      {hardware.llm_status}
                    </span>
                  </div>
                )}
                {!hardware.cpu && !hardware.memory && (
                  <pre className="ym-hw-raw">{JSON.stringify(hardware, null, 2)}</pre>
                )}
              </div>
            ) : (
              <div className="ym-hw-loading">Checking system…</div>
            )}
          </div>

          {/* Available Tools */}
          <div className="card ym-tools-card">
            <div className="card-header">🔧 TOOLS ({tools.length})</div>
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

          {/* Model Switcher */}
          <ModelSwitcher />

          {/* RAG Search */}
          <RagSearch />
        </div>
      </div>
    </div>
  );
}

/* ── Model Switcher sub-component ── */
function ModelSwitcher() {
  const [models, setModels] = React.useState([]);
  const [active, setActive] = React.useState('');
  const [refreshing, setRefreshing] = React.useState(false);

  React.useEffect(() => {
    fetch('/api/models').then(r => r.json()).then(d => {
      setModels(Array.isArray(d) ? d : d.models || []);
    }).catch(() => {});
    fetch('/api/models/active').then(r => r.json()).then(d => {
      setActive(d.model_name || d.name || '');
    }).catch(() => {});
  }, []);

  const switchModel = async (name) => {
    await fetch('/api/models/switch', {
      method: 'POST', headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ model_name: name }),
    });
    setActive(name);
  };

  const refresh = async () => {
    setRefreshing(true);
    await fetch('/api/inference/refresh', { method: 'POST' }).catch(() => {});
    setRefreshing(false);
  };

  return (
    <div className="card ym-system-card">
      <div className="card-header">🧠 MODELS
        <button className="ym-tool-desc" onClick={refresh} style={{ float: 'right', cursor: 'pointer', background: 'none', border: 'none', color: '#CFB991', fontSize: '10px' }}>
          {refreshing ? '⟳' : '↻ Refresh'}
        </button>
      </div>
      <div className="ym-hardware">
        {models.length === 0 && <div className="ym-hw-loading">No models detected</div>}
        {(Array.isArray(models) ? models : []).map((m, i) => {
          const name = typeof m === 'string' ? m : m.name || m.model_name || `model-${i}`;
          const isActive = name === active;
          return (
            <div key={i} className="ym-hw-row" style={{ cursor: 'pointer', opacity: isActive ? 1 : 0.5 }} onClick={() => switchModel(name)}>
              <span>{isActive ? '🟢' : '⚪'} {name}</span>
              {isActive && <span style={{ color: '#34d399', fontSize: '9px' }}>ACTIVE</span>}
            </div>
          );
        })}
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
