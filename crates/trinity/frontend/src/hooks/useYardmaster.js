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
  const [forgeLines, setForgeLines] = useState([
    { type: 'system', text: 'system.init()' },
    { type: '', text: 'Forge standing by…' },
  ]);
  const [sending, setSending] = useState(false);
  const [focus, setFocus] = useState(new Set());
  const [hardware, setHardware] = useState(null);
  const [tools, setTools] = useState([]);
  const [thinking, setThinking] = useState('');      // current reasoning chain
  const [questState, setQuestState] = useState(null); // ADDIECRAPEYE state
  const [turnInfo, setTurnInfo] = useState({ turn: 0, maxTurns: 8, continuations: 0 });
  const [modelInfo] = useState({
    name: 'Mistral Small 4 119B',
    reasoning: 'high',
    context: '500K (2×256K)',
    active_experts: '6.5B / 119B',
  });
  const abortRef = useRef(null);

  // Fetch hardware + tools + quest + chat history on mount
  useEffect(() => {
    const t1 = setTimeout(async () => {
      try {
        const res = await fetch('/api/hardware');
        if (res.ok) setHardware(await res.json());
      } catch (_) {}
    }, 300);
    const t2 = setTimeout(async () => {
      try {
        const res = await fetch('/api/tools');
        if (res.ok) setTools(await res.json());
      } catch (_) {}
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
      } catch (_) {}
    }, 100);
    // Fetch quest state
    fetchQuest();
    return () => { clearTimeout(t1); clearTimeout(t2); clearTimeout(t3); };
  }, []);

  const fetchQuest = async () => {
    try {
      const res = await fetch('/api/quest');
      if (res.ok) {
        const data = await res.json();
        setQuestState(data);
      }
    } catch (_) {}
  };

  const addForge = useCallback((text, type = '') => {
    setForgeLines((prev) => [...prev, { text, type, ts: Date.now() }].slice(-200));
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
    setTurnInfo({ turn: 0, maxTurns: 8, continuations: 0 });
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

    addForge(`Executing: ${text.substring(0, 40)}${text.length > 40 ? '…' : ''}`, 'command');

    // Create AI response placeholder
    const aiId = Date.now();
    setMessages((prev) => [...prev, { role: 'ai', speaker: 'YARDMASTER', content: '', id: aiId }]);

    try {
      const controller = new AbortController();
      abortRef.current = controller;

      const res = await fetch('/api/chat/yardmaster', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          message: fullMsg,
          max_tokens: 16384,
          max_turns: 8,
          mode: persona || 'dev',
          scope: scopePath || null,
          // Send conversation history for rolling context
          history: messages
            .filter(m => m.content) // skip empty placeholders
            .slice(-20) // last 20 messages
            .map(m => ({ role: m.role === 'ai' ? 'assistant' : m.role, content: m.content })),
        }),
        signal: controller.signal,
      });

      if (!res.ok) {
        const errText = `Server error: ${res.status}`;
        setMessages((prev) => prev.map((m) => m.id === aiId ? { ...m, content: errText } : m));
        addForge(errText, 'error');
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
            addForge('Work sequence completed.', 'success');
            currentEvent = '';
            continue;
          }

          // Route based on event type
          if (currentEvent === 'thinking' && payload.startsWith('{')) {
            try {
              const j = JSON.parse(payload);
              if (j.thinking) {
                setThinking(j.thinking);
                addForge('🧠 Reasoning…', 'system');
              }
            } catch {}
            currentEvent = '';
            continue;
          }

          if (currentEvent === 'resources' && payload.startsWith('{')) {
            try {
              const j = JSON.parse(payload);
              addForge(
                `⛏️ Coal: -${j.coal_burned?.toFixed(1)} | 💨 Steam: +${j.steam_gained?.toFixed(1)} | ⭐ XP: +${j.xp_gained}`,
                'success'
              );
            } catch {}
            currentEvent = '';
            continue;
          }

          if (currentEvent === 'skill' && payload.startsWith('{')) {
            try {
              const j = JSON.parse(payload);
              addForge(
                `🎲 D20: ${j.roll} (need ${j.dc}) — ${j.success ? '✅ Pass' : '❌ Fail'}${j.critical ? ' ⚡CRIT!' : ''}${j.fumble ? ' 💀FUMBLE!' : ''}`,
                j.success ? 'success' : 'error'
              );
            } catch {}
            currentEvent = '';
            continue;
          }

          if (currentEvent === 'narrative') {
            // Narrative text events — append as separate styled message
            addForge(`📖 ${payload.substring(0, 120)}`, 'system');
            currentEvent = '';
            continue;
          }

          // ── VAAM vocabulary detection events ──
          if (currentEvent === 'vaam' && payload.startsWith('{')) {
            try {
              const j = JSON.parse(payload);
              const wordCount = j.words_detected || j.total_words || 0;
              const coal = j.coal_earned || j.total_coal || 0;
              addForge(
                `📚 VAAM: ${wordCount} vocabulary words detected (+${coal} coal)`,
                'vaam'
              );
            } catch {}
            currentEvent = '';
            continue;
          }

           // ── Status events (thinking, tool execution — keeps Cloudflare alive) ──
           if (currentEvent === 'status' && payload.startsWith('{')) {
             try {
               const j = JSON.parse(payload);
               if (j.status === 'thinking' && j.turn) {
                 setTurnInfo(prev => ({ ...prev, turn: j.turn }));
                 addForge(`⏳ ${j.message || 'Thinking...'}`, 'system');
               } else if (j.status === 'tool') {
                 addForge(`🔧 ${j.message || `Running ${j.tool}...`}`, 'command');
               } else if (j.status === 'connected') {
                 addForge(`✅ ${j.message || 'Connected'}`, 'success');
               }
             } catch {}
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
              addForge(
                `${level} Cognitive Load: Grade ${grade} | ${complex} complex words`,
                'system'
              );
            } catch {}
            currentEvent = '';
            continue;
          }

          // Image generation events — inline image in chat
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
              addForge(`🖼️ Image generated: ${imgData.filename}`, 'success');
            } catch {}
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
                addForge(`Tool: ${j.tool}(${JSON.stringify(j.params || {}).substring(0, 60)})`, 'command');
              }
              // Tool result
              if (j.type === 'tool_result') {
                const preview = (j.content || '').substring(0, 200);
                addForge(`Result: ${preview}${j.content?.length > 200 ? '…' : ''}`, 'result');
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
        addForge(`Error: ${err.message}`, 'error');
      }
    }

    // Refresh quest state after response completes
    fetchQuest();

    abortRef.current = null;
    setSending(false);
    activityBus.setActive(false);
  }, [sending, focus, addForge, messages]);

  return {
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
  };
}
