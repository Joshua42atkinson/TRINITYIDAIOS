import React, { useState, useRef, useEffect, useCallback } from 'react';

/**
 * ZEN MODE — The Open Book
 * Left page: Great Recycler narration (AI response, voiced via Piper TTS)
 * Right page: User's writing (text input)
 * LitRPG audiobook aesthetic — immersive narration.
 */

export default function ZenMode() {
  const [messages, setMessages] = useState([
    {
      role: 'narrator',
      text: "Welcome to Zen Mode, traveler.\n\nThis is the quiet car. No buttons, no dashboards — just you and the page.\n\nI am the Great Recycler. I'll narrate your journey, and you'll write your story alongside mine. When you're ready, type your thoughts on the right and press Enter.\n\nWhat would you like to explore today?",
    },
  ]);
  const [input, setInput] = useState('');
  const [streaming, setStreaming] = useState(false);
  const [voiceOn, setVoiceOn] = useState(true);
  const [voiceEngine, setVoiceEngine] = useState('checking');
  const [speaking, setSpeaking] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [volume, setVolume] = useState(0.7);
  const [speed, setSpeed] = useState(0.95);
  const [showThinking, setShowThinking] = useState(false); // Hide streaming for immersive feel

  const audioRef = useRef(null);
  const leftRef = useRef(null);
  const rightRef = useRef(null);
  const hasSpokenWelcome = useRef(false);
  const volumeRef = useRef(volume);
  const speedRef = useRef(speed);
  volumeRef.current = volume;
  speedRef.current = speed;

  // Check voice sidecar on mount (health check only, no TTS)
  useEffect(() => {
    fetch('http://127.0.0.1:8200/health')
      .then((r) => r.ok ? r.json() : null)
      .then((d) => setVoiceEngine(d && d.status !== 'error' ? 'piper' : 'browser'))
      .catch(() => {
        // CORS may block direct check, try through proxy
        fetch('/api/tts', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ text: '.' }),
        })
          .then((r) => setVoiceEngine(r.ok ? 'piper' : 'browser'))
          .catch(() => setVoiceEngine('browser'));
      });
  }, []);

  // Stop all audio
  const stopAudio = useCallback(() => {
    window.speechSynthesis?.cancel();
    if (audioRef.current) {
      audioRef.current.pause();
      audioRef.current.currentTime = 0;
      audioRef.current = null;
    }
    setSpeaking(false);
  }, []);

  // Speak text — tries Piper, falls back to browser
  // Using refs for volume/speed to keep callback stable and prevent re-render loops
  const speak = useCallback(async (text) => {
    if (!voiceOn) return;
    stopAudio();
    setSpeaking(true);

    // Try Piper TTS
    try {
      const res = await fetch('/api/tts', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text }),
      });
      if (res.ok) {
        setVoiceEngine('piper');
        const blob = await res.blob();
        const url = URL.createObjectURL(blob);
        const audio = new Audio(url);
        audio.volume = volumeRef.current;
        audio.playbackRate = speedRef.current;
        audioRef.current = audio;
        audio.onended = () => { setSpeaking(false); URL.revokeObjectURL(url); };
        audio.onerror = () => { setSpeaking(false); URL.revokeObjectURL(url); };
        await audio.play();
        return;
      }
    } catch { /* fall through */ }

    // Fallback: browser TTS
    setVoiceEngine('browser');
    if (!window.speechSynthesis) { setSpeaking(false); return; }
    const utterance = new SpeechSynthesisUtterance(text);
    utterance.rate = speedRef.current;
    utterance.pitch = 0.85;
    utterance.volume = volumeRef.current;
    const voices = window.speechSynthesis.getVoices();
    const pref = voices.find((v) =>
      v.name.includes('Daniel') || v.name.includes('Google UK English Male') || v.name.includes('Male')
    );
    if (pref) utterance.voice = pref;
    utterance.onend = () => setSpeaking(false);
    utterance.onerror = () => setSpeaking(false);
    window.speechSynthesis.speak(utterance);
  }, [voiceOn, stopAudio]); // stable deps only — volume/speed via refs

  // Auto-scroll
  useEffect(() => {
    if (leftRef.current) leftRef.current.scrollTop = leftRef.current.scrollHeight;
    if (rightRef.current) rightRef.current.scrollTop = rightRef.current.scrollHeight;
  }, [messages]);

  // Welcome narration — fires exactly once
  useEffect(() => {
    if (hasSpokenWelcome.current) return;
    hasSpokenWelcome.current = true;
    const t = setTimeout(() => speak(messages[0].text), 1200);
    return () => clearTimeout(t);
  }, [speak]); // safe: speak only changes when voiceOn changes

  // Strip model chain-of-thought / meta-commentary — aggressive version
  const stripThinking = (text) => {
    let cleaned = text;

    // Strategy 1: If the model output contains quoted narration after reasoning,
    // extract just the narration. Look for the pattern: ...something like:"actual narration"
    const quotedMatch = cleaned.match(/(?:something like|here'?s?|response|narration)[:\s]*"([^"]{20,})"/is);
    if (quotedMatch) return quotedMatch[1].trim();

    // Strategy 2: Find where the actual narration begins — model starts speaking
    // in second person ("You") after its reasoning preamble
    const youSplit = cleaned.match(/^.*?(?:like:|response:|narration:|GO:|question\.?\s*\n)\s*(You\s)/is);
    if (youSplit) {
      const idx = cleaned.indexOf(youSplit[1], youSplit.index);
      if (idx > 0) cleaned = cleaned.slice(idx);
    }

    // Strategy 3: Strip known meta-commentary patterns from the start
    const stripPatterns = [
      /^.*?(?:let me (?:write|craft|create|respond))[^.]*\.?\s*/is,
      /^.*?(?:here(?:'s| is) (?:my|the|a) (?:response|narration))[:\s]*/is,
      /^.*?(?:I'll respond|I should respond|I need to|I want to)[^.]*\.?\s*/is,
      /^.*?(?:a response that|respond (?:in|as|with))[^.]*\.?\s*/is,
      /^.*?(?:The traveler (?:writes|says|asks))[:\s]*(?:['"][^'"]*['"])?\s*/is,
      /^.*?(?:something like)\s*:?\s*/is,
    ];
    for (const pat of stripPatterns) {
      cleaned = cleaned.replace(pat, '');
    }

    // Strategy 4: Remove lines that are clearly instructions/bullets
    cleaned = cleaned.split('\n').filter((line) => {
      const l = line.trim();
      if (!l) return true;
      // Skip meta-commentary lines
      if (/^[•\-\*]\s*(Speak|Is\s|End|Interpret|Be\s|Keep|Maximum|Three|One paragraph)/i.test(l)) return false;
      if (/^(— ❦ —|---|\*\*\*|~~~)/.test(l)) return false; // decorative dividers from the model
      if (/^a response that/i.test(l)) return false;
      return true;
    }).join('\n').trim();

    // Strategy 5: If there's a "— ❦ —" divider, take everything AFTER it
    const dividerIdx = cleaned.indexOf('— ❦ —');
    if (dividerIdx >= 0) {
      const afterDivider = cleaned.slice(dividerIdx + 5).trim();
      if (afterDivider.length > 20) cleaned = afterDivider;
    }

    return cleaned || text;
  };

  // Send message
  const sendMessage = async () => {
    if (!input.trim() || streaming) return;
    const userText = input.trim();
    setInput('');
    stopAudio();

    const thinkingMsg = showThinking ? '' : '✦ The Recycler is composing...';
    setMessages((prev) => [...prev, { role: 'user', text: userText }, { role: 'narrator', text: thinkingMsg }]);
    setStreaming(true);

    try {
      const res = await fetch('/api/chat/stream', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          message: `[NARRATOR MODE — respond in-character ONLY]\n\n"${userText}"\n\nYou ARE the Great Recycler. DO NOT plan, DO NOT explain your process, DO NOT use bullet points. Just narrate. Speak directly to "you" (the traveler). Three short paragraphs maximum. LitRPG audiobook style — poetic, warm, contemplative. End with one question. GO:`,
          max_tokens: 200,
        }),
      });

      if (!res.ok) throw new Error('Stream failed');
      const reader = res.body.getReader();
      const decoder = new TextDecoder();
      let fullText = '';

      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        const chunk = decoder.decode(value, { stream: true });
        for (const line of chunk.split('\n')) {
          if (!line.startsWith('data: ')) continue;
          const data = line.slice(6);
          if (data === '[DONE]') continue;
          try {
            const parsed = JSON.parse(data);
            const token = parsed.choices?.[0]?.delta?.content || parsed.token || parsed.content || '';
            if (token) {
              fullText += token;
              if (showThinking) {
                setMessages((prev) => {
                  const u = [...prev];
                  u[u.length - 1] = { role: 'narrator', text: fullText };
                  return u;
                });
              }
            }
          } catch {
            if (data && data !== '[DONE]') {
              fullText += data;
              if (showThinking) {
                setMessages((prev) => {
                  const u = [...prev];
                  u[u.length - 1] = { role: 'narrator', text: fullText };
                  return u;
                });
              }
            }
          }
        }
      }
      if (fullText) {
        const cleaned = stripThinking(fullText);
        setMessages((prev) => {
          const u = [...prev];
          u[u.length - 1] = { role: 'narrator', text: cleaned };
          return u;
        });
        speak(cleaned);
      }
    } catch (err) {
      console.error('ZEN stream error:', err);
      setMessages((prev) => {
        const u = [...prev];
        u[u.length - 1] = { role: 'narrator', text: 'The Great Recycler is resting. Check that llama-server is running.' };
        return u;
      });
    }
    setStreaming(false);
  };

  const narratorMessages = messages.filter((m) => m.role === 'narrator');
  const userMessages = messages.filter((m) => m.role === 'user');

  const btnStyle = (active) => ({
    padding: '4px 12px', borderRadius: '6px',
    background: active ? 'rgba(207, 185, 145, 0.12)' : 'transparent',
    border: '1px solid rgba(207, 185, 145, 0.2)',
    color: active ? '#CFB991' : '#4B5563',
    cursor: 'pointer', fontSize: '12px',
    fontFamily: "'Inter', sans-serif",
    transition: 'all 0.15s',
  });

  return (
    <div style={{
      gridColumn: '1 / -1', gridRow: 2,
      display: 'flex', flexDirection: 'column',
      overflow: 'hidden', background: '#0F0D0A',
    }}>
      {/* Header */}
      <header style={{
        padding: '12px 40px', flexShrink: 0,
        display: 'flex', alignItems: 'center', gap: '12px',
        borderBottom: '1px solid rgba(207, 185, 145, 0.1)',
        background: 'rgba(15, 13, 10, 0.95)',
      }}>
        <div style={{
          fontFamily: "'Cinzel Decorative', 'Cinzel', serif",
          fontSize: '18px', color: '#CFB991', letterSpacing: '3px',
        }}>
          ✦ ZEN MODE
        </div>
        <div style={{
          fontSize: '11px', color: '#6B7280',
          fontFamily: "'Inter', sans-serif",
        }}>
          The Open Book — narration & reflection
        </div>
        <div style={{
          fontSize: '10px', color: '#D97706',
          fontFamily: "'Inter', sans-serif",
          padding: '2px 8px',
          border: '1px solid rgba(217, 119, 6, 0.3)',
          borderRadius: '4px',
          background: 'rgba(217, 119, 6, 0.08)',
        }}>
          🚧 Under Construction — awaiting narrator fine-tune
        </div>
        <div style={{ flex: 1 }} />

        {/* Voice toggle */}
        <button
          onClick={() => {
            if (voiceOn) stopAudio();
            setVoiceOn((v) => !v);
          }}
          style={btnStyle(voiceOn)}
        >
          {voiceOn
            ? (speaking ? '🔊 ● Narrating...' : `🔊 ${voiceEngine === 'piper' ? 'Piper' : 'Browser'}`)
            : '🔇 Off'}
        </button>

        {/* Settings gear */}
        <button
          onClick={() => setShowSettings((v) => !v)}
          style={btnStyle(showSettings)}
        >
          ⚙
        </button>
      </header>

      {/* Settings panel */}
      {showSettings && (
        <div style={{
          padding: '12px 40px', flexShrink: 0,
          display: 'flex', gap: '32px', alignItems: 'center',
          borderBottom: '1px solid rgba(207, 185, 145, 0.08)',
          background: 'rgba(15, 13, 10, 0.9)',
          fontFamily: "'Inter', sans-serif", fontSize: '12px', color: '#9CA3AF',
        }}>
          <label style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            Volume
            <input
              type="range" min="0" max="1" step="0.05"
              value={volume}
              onChange={(e) => {
                const v = parseFloat(e.target.value);
                setVolume(v);
                if (audioRef.current) audioRef.current.volume = v;
              }}
              style={{ accentColor: '#CFB991', width: '100px' }}
            />
            <span style={{ color: '#CFB991', width: '30px' }}>{Math.round(volume * 100)}%</span>
          </label>
          <label style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            Speed
            <input
              type="range" min="0.5" max="1.5" step="0.05"
              value={speed}
              onChange={(e) => {
                const s = parseFloat(e.target.value);
                setSpeed(s);
                if (audioRef.current) audioRef.current.playbackRate = s;
              }}
              style={{ accentColor: '#CFB991', width: '100px' }}
            />
            <span style={{ color: '#CFB991', width: '35px' }}>{speed.toFixed(2)}×</span>
          </label>
          <div style={{ borderLeft: '1px solid rgba(207,185,145,0.15)', height: '20px' }} />
          <label style={{ display: 'flex', alignItems: 'center', gap: '6px', cursor: 'pointer' }}>
            <input
              type="checkbox"
              checked={showThinking}
              onChange={(e) => setShowThinking(e.target.checked)}
              style={{ accentColor: '#CFB991' }}
            />
            Show thinking
          </label>
          <div style={{ color: '#4B5563' }}>
            Engine: <span style={{ color: '#CFB991' }}>{voiceEngine}</span>
          </div>
          <div style={{ color: '#4B5563' }}>
            Burst: <span style={{ color: '#CFB991' }}>~30s / 80 words</span>
          </div>
        </div>
      )}

      {/* Open Book layout */}
      <div style={{
        flex: 1, display: 'grid',
        gridTemplateColumns: '1fr 1px 1fr',
        overflow: 'hidden',
      }}>
        {/* LEFT — Narrator */}
        <div ref={leftRef} style={{
          overflow: 'auto', padding: '40px',
          background: 'linear-gradient(135deg, rgba(207, 185, 145, 0.03), rgba(15, 13, 10, 1))',
          boxShadow: 'inset -8px 0 20px -8px rgba(0, 0, 0, 0.4)',
        }}>
          <div style={{
            fontFamily: "'Cinzel', serif", fontSize: '10px',
            color: '#4B5563', letterSpacing: '3px', textTransform: 'uppercase',
            marginBottom: '24px',
          }}>
            ♻️ The Great Recycler — Narration
          </div>

          {narratorMessages.map((m, i) => (
            <div key={i} style={{
              fontFamily: "'Crimson Text', serif",
              fontSize: '18px', lineHeight: 2,
              color: '#D4C9B8',
              marginBottom: '32px',
              whiteSpace: 'pre-wrap',
            }}>
              {m.text}
              {i < narratorMessages.length - 1 && (
                <div style={{
                  textAlign: 'center', margin: '24px 0',
                  color: '#4B5563', letterSpacing: '8px', fontSize: '12px',
                }}>— ❦ —</div>
              )}
            </div>
          ))}

          {streaming && (
            <span style={{
              display: 'inline-block', width: '8px', height: '18px',
              background: '#CFB991', borderRadius: '1px',
              animation: 'pulse 1s infinite',
              verticalAlign: 'text-bottom', marginLeft: '2px',
            }} />
          )}
        </div>

        {/* SPINE */}
        <div style={{
          background: 'linear-gradient(180deg, rgba(207,185,145,0.05), rgba(207,185,145,0.15) 20%, rgba(207,185,145,0.15) 80%, rgba(207,185,145,0.05))',
          boxShadow: '-2px 0 8px rgba(0,0,0,0.3), 2px 0 8px rgba(0,0,0,0.3)',
        }} />

        {/* RIGHT — User */}
        <div style={{
          display: 'flex', flexDirection: 'column', overflow: 'hidden',
          background: 'linear-gradient(225deg, rgba(207, 185, 145, 0.03), rgba(15, 13, 10, 1))',
          boxShadow: 'inset 8px 0 20px -8px rgba(0, 0, 0, 0.4)',
        }}>
          <div ref={rightRef} style={{ flex: 1, overflow: 'auto', padding: '40px' }}>
            <div style={{
              fontFamily: "'Cinzel', serif", fontSize: '10px',
              color: '#4B5563', letterSpacing: '3px', textTransform: 'uppercase',
              marginBottom: '24px',
            }}>
              ✍️ Your Story — Reflection
            </div>

            {userMessages.length === 0 ? (
              <div style={{
                fontFamily: "'Crimson Text', serif",
                fontSize: '16px', lineHeight: 2,
                color: '#4B5563', fontStyle: 'italic',
              }}>
                Your words will appear here as you write them...
              </div>
            ) : (
              userMessages.map((m, i) => (
                <div key={i} style={{
                  fontFamily: "'Crimson Text', serif",
                  fontSize: '18px', lineHeight: 2,
                  color: '#E2E8F0',
                  marginBottom: '24px',
                  paddingLeft: '16px',
                  borderLeft: '2px solid rgba(207, 185, 145, 0.15)',
                }}>
                  {m.text}
                </div>
              ))
            )}
          </div>

          {/* Input */}
          <div style={{
            padding: '16px 40px 24px',
            borderTop: '1px solid rgba(207, 185, 145, 0.08)',
            background: 'rgba(15, 13, 10, 0.8)',
          }}>
            <div style={{ display: 'flex', gap: '12px', alignItems: 'flex-end' }}>
              <textarea
                value={input}
                onChange={(e) => setInput(e.target.value)}
                onKeyDown={(e) => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); sendMessage(); } }}
                placeholder={streaming ? 'The Recycler is narrating...' : 'Write your reflection...'}
                disabled={streaming}
                rows={2}
                style={{
                  flex: 1, resize: 'none',
                  padding: '12px 16px',
                  background: 'rgba(207, 185, 145, 0.04)',
                  border: '1px solid rgba(207, 185, 145, 0.12)',
                  borderRadius: '8px',
                  color: '#E2E8F0',
                  fontFamily: "'Crimson Text', serif",
                  fontSize: '16px', lineHeight: 1.6,
                  outline: 'none',
                  transition: 'border-color 0.15s',
                }}
                onFocus={(e) => e.target.style.borderColor = 'rgba(207, 185, 145, 0.3)'}
                onBlur={(e) => e.target.style.borderColor = 'rgba(207, 185, 145, 0.12)'}
              />
              <button
                onClick={sendMessage}
                disabled={!input.trim() || streaming}
                style={{
                  padding: '12px 20px', borderRadius: '8px',
                  background: input.trim() && !streaming ? 'rgba(207, 185, 145, 0.12)' : 'transparent',
                  border: '1px solid rgba(207, 185, 145, 0.2)',
                  color: input.trim() && !streaming ? '#CFB991' : '#4B5563',
                  cursor: input.trim() && !streaming ? 'pointer' : 'not-allowed',
                  fontFamily: "'Cinzel', serif",
                  fontSize: '12px', letterSpacing: '1px',
                  transition: 'all 0.15s',
                }}
              >
                Send ↵
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
