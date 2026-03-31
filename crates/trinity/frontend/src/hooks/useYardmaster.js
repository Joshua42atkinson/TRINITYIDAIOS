

import { useState, useEffect, useCallback, useRef } from 'react';
import { activityBus } from './activityBus';

/**
 * useYardmaster — hook for Yardmaster IDE tab state.
 * Streaming chat via SSE, tool activity log, system info,
 * thinking/reasoning display, quest status, model info.
 */
export function useYardmaster() {
  const [messages, setMessages] = useState([
    { role: 'ai', speaker: 'YARDMASTER', content: 'Ready. Chat naturally or use the focus buttons to guide the session.' },
  ]);
  const [activityLogs, setActivityLogs] = useState([
    { type: 'system', text: 'system.init()' },
    { type: '', text: 'Studio standing by…' },
  ]);
  const [sending, setSending] = useState(false);
  const [focus, setFocus] = useState(new Set());
  const [hardware, setHardware] = useState(null);
  const [tools, setTools] = useState([]);
  const [thinking, setThinking] = useState('');      // current reasoning chain
  const [questState, setQuestState] = useState(null); // ADDIECRAPEYE state
  const [turnInfo, setTurnInfo] = useState({ turn: 0, maxTurns: 65, continuations: 0 });
  const [taskQueue, setTaskQueue] = useState([]); // Work session task list
  const [modelInfo, setModelInfo] = useState({
    name: 'No Model Mounted',
    reasoning: '—',
    context: '—',
    active_experts: '—',
    status: 'unmounted',
  });
  const abortRef = useRef(null);

  // Fetch hardware + tools + quest + chat history on mount
  useEffect(() => {
    const t1 = setTimeout(async () => {
      try {
        const res = await fetch('/api/hardware');
        if (res.ok) setHardware(await res.json());
      } catch (_) { }
    }, 300);
    // Expose hardware updater for Yardmaster's ignition polling loop
    window.__trinityHardwareUpdate = setHardware;
    const t2 = setTimeout(async () => {
      try {
        const res = await fetch('/api/tools');
        if (res.ok) setTools(await res.json());
      } catch (_) { }
    }, 500);
    // Load chat history from DB
    const t3 = setTimeout(async () => {
      try {
        const res = await fetch('/api/sessions/history?limit=50');
        if (res.ok) {
          const history = await res.json();
          if (history && history.length > 0) {
            const restored = history.map(m => ({
              role: m.role === 'assistant' ? 'ai' : m.role,
              speaker: m.role === 'assistant' ? 'YARDMASTER' : undefined,
              content: m.content,
            }));
            setMessages([
              { role: 'ai', speaker: 'YARDMASTER', content: 'Ready. Chat naturally or use the focus buttons to guide the session.' },
              ...restored,
            ]);
          }
        }
      } catch (_) { }
    }, 100);
    // Fetch quest state
    fetchQuest();
    // Fetch model status and poll
    const fetchModelStatus = async () => {
      try {
        const res = await fetch('/api/model/status');
        if (res.ok) {
          const data = await res.json();
          const name = data.status === 'mounted'
            ? (data.model_name || data.model_path?.split('/').pop()?.replace(/\.gguf.*$/, '') || data.inference_mode || 'Connected')
            : data.status === 'loading'
            ? 'Loading...'
            : 'No Model Mounted';
          setModelInfo({
            name,
            reasoning: data.status === 'mounted' ? 'high' : '—',
            context: data.status === 'mounted' ? '256K' : '—',
            active_experts: data.status === 'mounted' ? data.inference_mode || 'http' : data.inference_mode || '—',
            status: data.status,
          });
        }
      } catch (_) { }
    };
    fetchModelStatus();
    const modelPoll = setInterval(fetchModelStatus, 5000);
    return () => { clearTimeout(t1); clearTimeout(t2); clearTimeout(t3); clearInterval(modelPoll); };
  }, []);

  const fetchQuest = async () => {
    try {
      const res = await fetch('/api/quest');
      if (res.ok) {
        const data = await res.json();
        setQuestState(data);
      }
    } catch (_) { }
  };

  const logActivity = useCallback((text, type = '') => {
    setActivityLogs((prev) => [...prev, { text, type, ts: Date.now() }].slice(-200));
    // Mirror to global activity bus for the persistent Yard bar
    activityBus.emit(text, type);
  }, []);

  const toggleFocus = useCallback((tag) => {
    setFocus((prev) => {
      const next = new Set(prev);
      if (next.has(tag)) next.delete(tag);
      else next.add(tag);
      return next;
    });
  }, []);

  const sendMessage = useCallback(async (text, persona, scopePath) => {
    if (!text.trim() || sending) return;
    setSending(true);
    setThinking(''); // clear previous thinking
    setTurnInfo({ turn: 0, maxTurns: 65, continuations: 0 });
    activityBus.setActive(true);

    // Add user message
    setMessages((prev) => [...prev, { role: 'user', content: text }]);

    // Append focus and scope context
    let fullMsg = text;
    if (focus.size > 0) {
      fullMsg += '\n\nSession focus: ' + Array.from(focus).join(', ');
    }
    if (scopePath) {
      fullMsg += `\n\nProject scope: ${scopePath} — focus tools and file operations within this directory.`;
    }

    logActivity(`Executing: ${text.substring(0, 40)}${text.length > 40 ? '…' : ''}`, 'command');

    // Create AI response placeholder with logs support
    const aiId = Date.now();
    setMessages((prev) => [...prev, { role: 'ai', speaker: 'YARDMASTER', content: '', logs: [], id: aiId }]);

    try {
      const controller = new AbortController();
      abortRef.current = controller;

      const res = await fetch('/api/chat/yardmaster', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          message: fullMsg,
          max_tokens: 32768,
          max_turns: 65,
          mode: persona || 'dev',
          scope: scopePath || null,
          history: messages
            .filter(m => m.content)
            .slice(-20)
            .map(m => {
              // Strip UI tool indicator badges from LLM context to prevent hallucination looping
              let cleanContent = m.content
                .split('\n')
                .filter(line => !line.match(/^`?[🟢🟡🔴] ▶/))
                .filter(line => line.trim() !== '`✓`' && line.trim() !== '✓')
                .join('\n');
              return { role: m.role === 'ai' ? 'assistant' : m.role, content: cleanContent };
            }),
        }),
        signal: controller.signal,
      });

      if (!res.ok) {
        logActivity(`Agent error: ${res.status}`, 'error');
        setMessages((prev) => prev.map((m) => m.id === aiId ? { ...m, role: 'error', content: "🚫 THE FURNACE IS COLD! The Great Recycler cannot speak while the firebox sleeps. Ignite LM Studio on port 1234 or click [🔥 IGNITE FURNACE] to light the coal yourself!" } : m));
        setSending(false);
        return;
      }

      const reader = res.body.getReader();
      const decoder = new TextDecoder();
      let buffer = '';
      let currentEvent = '';
      let turnCounter = 0;

      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        buffer += decoder.decode(value, { stream: true });

        let idx;
        while ((idx = buffer.indexOf('\n')) !== -1) {
          const line = buffer.substring(0, idx);
          buffer = buffer.substring(idx + 1);

          // Track SSE event types
          if (line.startsWith('event: ')) {
            currentEvent = line.substring(7).trim();
            continue;
          }
          if (line.startsWith(':') || !line) continue;
          if (!line.startsWith('data: ')) continue;

          const payload = line.substring(6);
          if (payload === '[DONE]') {
            logActivity('Work sequence completed.', 'success');
            currentEvent = '';
            continue;
          }

          // Route based on event type
          if (currentEvent === 'llm_offline' || currentEvent === 'error') {
            logActivity(`Agent error: ${payload}`, 'error');
            setMessages((prev) => prev.map((m) => m.id === aiId ? { ...m, role: 'error', content: "🚫 THE FURNACE IS COLD! The Great Recycler cannot speak while the firebox sleeps. Ignite LM Studio on port 1234 or click [🔥 IGNITE FURNACE] to light the coal yourself!" } : m));
            setSending(false);
            return;
          }
          
          if (currentEvent === 'thinking' && payload.startsWith('{')) {
            try {
              const j = JSON.parse(payload);
              if (j.thinking) {
                setThinking(j.thinking);
                const msg = '🧠 Reasoning…';
                logActivity(msg, 'system');
                setMessages((prev) => prev.map((m) => m.id === aiId ? { ...m, logs: [...(m.logs || []), { text: msg, type: 'system' }] } : m));
              }
            } catch { }
            currentEvent = '';
            continue;
          }

          if (currentEvent === 'resources' && payload.startsWith('{')) {
            try {
              const j = JSON.parse(payload);
              logActivity(
                `⛏️ Coal: -${j.coal_burned?.toFixed(1)} | 💨 Steam: +${j.steam_gained?.toFixed(1)} | ⭐ XP: +${j.xp_gained}`,
                'success'
              );
            } catch { }
            currentEvent = '';
            continue;
          }

          if (currentEvent === 'skill' && payload.startsWith('{')) {
            try {
              const j = JSON.parse(payload);
              logActivity(
                `🎲 D20: ${j.roll} (need ${j.dc}) — ${j.success ? '✅ Pass' : '❌ Fail'}${j.critical ? ' ⚡CRIT!' : ''}${j.fumble ? ' 💀FUMBLE!' : ''}`,
                j.success ? 'success' : 'error'
              );
            } catch { }
            currentEvent = '';
            continue;
          }

          if (currentEvent === 'narrative') {
            // Narrative text events — append as separate styled message
            logActivity(`📖 ${payload.substring(0, 120)}`, 'system');
            currentEvent = '';
            continue;
          }

          // ── VAAM vocabulary detection events ──
          if (currentEvent === 'vaam' && payload.startsWith('{')) {
            try {
              const j = JSON.parse(payload);
              const wordCount = j.words_detected || j.total_words || 0;
              const coal = j.coal_earned || j.total_coal || 0;
              logActivity(
                `📚 VAAM: ${wordCount} vocabulary words detected (+${coal} coal)`,
                'vaam'
              );
            } catch { }
            currentEvent = '';
            continue;
          }

          // ── Status events (thinking, tool execution — keeps Cloudflare alive) ──
          if (currentEvent === 'status' && payload.startsWith('{')) {
            try {
              const j = JSON.parse(payload);
              if (j.status === 'thinking' && j.turn) {
                setTurnInfo(prev => ({ ...prev, turn: j.turn }));
                const text = `⏳ ${j.message || 'Thinking...'}`;
                logActivity(text, 'system');
                setMessages((prev) => prev.map((m) => m.id === aiId ? { ...m, logs: [...(m.logs || []), { text, type: 'system' }] } : m));
              } else if (j.status === 'tool') {
                const text = `🔧 ${j.message || `Running ${j.tool}...`}`;
                logActivity(text, 'command');
                setMessages((prev) => prev.map((m) => m.id === aiId ? { ...m, logs: [...(m.logs || []), { text, type: 'command' }] } : m));
              } else if (j.status === 'connected') {
                const text = `✅ ${j.message || 'Connected'}`;
                logActivity(text, 'success');
                setMessages((prev) => prev.map((m) => m.id === aiId ? { ...m, logs: [...(m.logs || []), { text, type: 'success' }] } : m));
              }
            } catch { }
            currentEvent = '';
            continue;
          }

          // ── Cognitive Load events ──
          if (currentEvent === 'cognitive_load' && payload.startsWith('{')) {
            try {
              const j = JSON.parse(payload);
              const grade = j.flesch_grade?.toFixed(1) || '?';
              const complex = j.complex_words || 0;
              const level = parseFloat(grade) > 12 ? '🔴' : parseFloat(grade) > 8 ? '🟡' : '🟢';
              const text = `${level} Cognitive Load: Grade ${grade} | ${complex} complex words`;
              logActivity(text, 'system');
              setMessages((prev) => prev.map((m) => m.id === aiId ? { ...m, logs: [...(m.logs || []), { text, type: 'system' }] } : m));
            } catch { }
            currentEvent = '';
            continue;
          }

          // Image generation events
          if (currentEvent === 'image' && payload.startsWith('{')) {
            try {
              const imgData = JSON.parse(payload);
              setMessages((prev) => [
                ...prev,
                {
                  role: 'image',
                  filename: imgData.filename,
                  url: imgData.url,
                  base64: imgData.base64,
                  content: `🖼️ Generated: ${imgData.filename}`,
                },
              ]);
              logActivity(`🖼️ Image generated: ${imgData.filename}`, 'success');
            } catch { }
            currentEvent = '';
            continue;
          }

          // ── Real-Time Agent ↔ WASM SSE Bridge (Project Forge) ──
          if (currentEvent === 'forge_command' && payload.startsWith('{')) {
            try {
              const j = JSON.parse(payload);
              // Broadcast to the React iframe (ArtStudio.jsx catches this)
              window.postMessage({
                type: 'forge_command',
                command: j.command,
                payload: j.payload || {},
                requestId: `agent-${Date.now()}`
              }, window.location.origin);
              logActivity(`WASM Bridge Executing: ${j.command}`, 'command');
            } catch (e) {
              console.error("Agent ↔ WASM Bridge parse error", e);
            }
            currentEvent = '';
            continue;
          }

          // Default data handling (content tokens)
          currentEvent = '';

          if (payload.startsWith('{')) {
            try {
              const j = JSON.parse(payload);
              // Content token
              if (j.content && !j.type && !j.ssml && !j.roll) {
                // Track tool turn indicators
                if (j.content.includes('`▶')) {
                  turnCounter++;
                  setTurnInfo(prev => ({ ...prev, turn: turnCounter }));
                }
                setMessages((prev) => prev.map((m) =>
                  m.id === aiId ? { ...m, content: m.content + j.content } : m
                ));
              }
              // Tool invocation
              if (j.tool) {
                const text = `Tool: ${j.tool}(${JSON.stringify(j.params || {}).substring(0, 60)})`;
                logActivity(text, 'command');
                setMessages((prev) => prev.map((m) => m.id === aiId ? { ...m, logs: [...(m.logs || []), { text, type: 'command' }] } : m));
              }
              // Tool result
              if (j.type === 'tool_result') {
                const preview = (j.content || '').substring(0, 200);
                const text = `Result: ${preview}${j.content?.length > 200 ? '…' : ''}`;
                logActivity(text, 'result');
                const logText = (j.content || '').substring(0, 2000) + (j.content?.length > 2000 ? '\n...[Truncated]' : '');
                setMessages((prev) => prev.map((m) => m.id === aiId ? { ...m, logs: [...(m.logs || []), { text: logText, type: 'result' }] } : m));
                // Parse task_queue results to update the Work Session card in real-time
                const content = j.content || '';
                if (content.includes('Task #') || content.includes('TASK QUEUE') || content.includes('All tasks complete')) {
                  parseTaskQueue(content);
                }
              }
            } catch {
              // Not valid JSON, treat as text
              setMessages((prev) => prev.map((m) =>
                m.id === aiId ? { ...m, content: m.content + payload } : m
              ));
            }
          } else {
            setMessages((prev) => prev.map((m) =>
              m.id === aiId ? { ...m, content: m.content + payload } : m
            ));
          }
        }
      }
    } catch (err) {
      if (err.name !== 'AbortError') {
        setMessages((prev) => prev.map((m) =>
          m.id === aiId ? { ...m, content: m.content + `\nError: ${err.message}` } : m
        ));
        logActivity(`Error: ${err.message}`, 'error');
      } else {
        logActivity('Agent execution manually aborted.', 'system');
      }
    }

    fetchQuest();
    abortRef.current = null;
    setSending(false);
    activityBus.setActive(false);
  }, [sending, focus, logActivity, messages]);

  const cancelRequest = useCallback(() => {
    if (abortRef.current) {
      abortRef.current.abort();
      logActivity('🚫 STOP signal sent. Aborting agent thread.', 'error');
    }
  }, [logActivity]);

  // Parse task_queue markdown output into structured task items
  const parseTaskQueue = useCallback((text) => {
    const lines = text.split('\n');
    const tasks = [];
    for (const line of lines) {
      const matchUnchecked = line.match(/^- \[ \] (\d+)\.\s*(.+)/);
      const matchChecked = line.match(/^- \[x\] (\d+)\.\s*(.+)/);
      if (matchUnchecked) {
        tasks.push({ index: parseInt(matchUnchecked[1]), text: matchUnchecked[2].trim(), done: false });
      } else if (matchChecked) {
        tasks.push({ index: parseInt(matchChecked[1]), text: matchChecked[2].trim(), done: true });
      }
    }
    if (tasks.length > 0) {
      setTaskQueue(tasks);
    }
    // Handle "Task #N marked complete" — update in place
    const completeMatch = text.match(/Task #(\d+) marked complete/);
    if (completeMatch) {
      const idx = parseInt(completeMatch[1]);
      setTaskQueue(prev => prev.map(t => t.index === idx ? { ...t, done: true } : t));
    }
    // Handle "Task #N added: ..."
    const addMatch = text.match(/Task #(\d+) added: (.+)/);
    if (addMatch) {
      const idx = parseInt(addMatch[1]);
      const taskText = addMatch[2].trim();
      setTaskQueue(prev => {
        // Don't duplicate
        if (prev.some(t => t.index === idx)) return prev;
        return [...prev, { index: idx, text: taskText, done: false }];
      });
    }
  }, []);

  // Start a new work session — sets the quest subject and clears task queue
  const startSession = useCallback(async (sessionName) => {
    if (!sessionName?.trim()) return;
    try {
      await fetch('/api/quest/subject', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ subject: sessionName.trim() }),
      });
      setTaskQueue([]);
      fetchQuest();
      logActivity(`🎯 Session started: ${sessionName}`, 'success');
    } catch (e) {
      logActivity(`Failed to start session: ${e.message}`, 'error');
    }
  }, [logActivity]);

  return {
    messages,
    activityLogs,
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
  };
}
