import React, { useState, useRef, useEffect, useCallback } from 'react';
import '../styles/zen.css';

/**
 * ZEN MODE — The Game Engine
 *
 * Three-panel layout:
 *   LEFT:         Great Recycler narration (streamed from Storyteller :8081)
 *   RIGHT-TOP:    User's story/reflections
 *   RIGHT-BOTTOM: Design Doc (auto-building from Director :8080 interpretation)
 *
 * Uses /api/chat/zen endpoint with typed SSE events:
 *   - event: interpretation → JSON design elements → updates Design Doc
 *   - event: narration      → streamed tokens → fills narrator panel
 *
 * VAAM vocabulary highlighting: detected words glow gold in the narration.
 * Pace is intentional — Zen Mode is not a race.
 */

const WELCOME = {
  role: 'narrator',
  text: "Welcome to Zen Mode, traveler.\n\nThis is the quiet car. No buttons, no dashboards — just you and the page.\n\nI am the Great Recycler. I'll narrate your journey, and you'll write your story alongside mine. When you're ready, type your thoughts below and press Enter.\n\nWhat would you like to explore today?",
};

// How many narrator/user messages to show before older ones "fall off" the visible scroll.
// All messages still persist in localStorage.
const VISIBLE_NARRATIONS = 5;

export default function ZenMode() {
  // Load from localStorage on init — full history persists locally
  const [messages, setMessages] = useState(() => {
    try {
      const saved = localStorage.getItem('zen_messages');
      if (saved) {
        const parsed = JSON.parse(saved);
        if (Array.isArray(parsed) && parsed.length > 0) return parsed;
      }
    } catch (e) { /* corrupted — start fresh */ }
    return [WELCOME];
  });

  // Auto-save messages to localStorage on every update
  useEffect(() => {
    try {
      localStorage.setItem('zen_messages', JSON.stringify(messages));
    } catch (e) { /* quota exceeded — degrade gracefully */ }
  }, [messages]);

  const [input, setInput] = useState('');
  const [streaming, setStreaming] = useState(false);
  const [voiceOn, setVoiceOn] = useState(true);
  const [voiceEngine, setVoiceEngine] = useState('checking');
  const [speaking, setSpeaking] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [volume, setVolume] = useState(0.7);
  const [speed, setSpeed] = useState(0.95);
  const [voicePreset, setVoicePreset] = useState('M1');

  // Design Doc state — auto-fills from Director, persists locally
  const [designDoc, setDesignDoc] = useState(() => {
    try {
      const saved = localStorage.getItem('zen_design_doc');
      if (saved) return JSON.parse(saved);
    } catch (e) { /* */ }
    return {
      subject: null, audience: null, bloom_level: null,
      learning_objectives: [], vocabulary: [], scope_creeps: [],
    };
  });

  // Persist Design Doc
  useEffect(() => {
    try {
      localStorage.setItem('zen_design_doc', JSON.stringify(designDoc));
    } catch (e) { /* */ }
  }, [designDoc]);

  // VAAM-detected vocabulary for highlighting
  const [vaamWords, setVaamWords] = useState(new Set());

  const audioRef = useRef(null);
  const leftRef = useRef(null);
  const rightRef = useRef(null);
  const hasSpokenWelcome = useRef(false);
  const volumeRef = useRef(volume);
  const speedRef = useRef(speed);
  volumeRef.current = volume;
  speedRef.current = speed;

  // Yak Bak Customizer State
  const [recordingVoice, setRecordingVoice] = useState(false);
  const mediaRecorderRef = useRef(null);
  const audioChunksRef = useRef([]);

  const handleRecordVoice = async () => {
    if (recordingVoice) {
      if (mediaRecorderRef.current) mediaRecorderRef.current.stop();
      return;
    }
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      mediaRecorderRef.current = new MediaRecorder(stream);
      audioChunksRef.current = [];

      mediaRecorderRef.current.ondataavailable = e => audioChunksRef.current.push(e.data);
      mediaRecorderRef.current.onstop = async () => {
        setRecordingVoice(false);
        stream.getTracks().forEach(t => t.stop());
        const blob = new Blob(audioChunksRef.current, { type: 'audio/webm' });
        
        const formData = new FormData();
        formData.append("audio_file", blob, "custom_voice.webm");
        try {
          await fetch('http://127.0.0.1:8200/clone', { method: 'POST', body: formData });
          setMessages(prev => [...prev, {role: 'narrator', text: '✦ Voice Matrix Accepted. The Great Recycler has parameterized your frequency. You are now the voice of the LitRPG World AI.'}]);
          enqueueSentence("Voice Matrix Accepted. The Great Recycler has parameterized your frequency. You are now the voice of the LitRPG World AI.");
        } catch (e) {
          console.error("Voice clone failed", e);
        }
      };
      
      mediaRecorderRef.current.start();
      setRecordingVoice(true);
      setTimeout(() => { 
        if (mediaRecorderRef.current && mediaRecorderRef.current.state === "recording") {
          mediaRecorderRef.current.stop(); 
        }
      }, 5000);
    } catch (e) {
      console.error("Microphone access denied", e);
    }
  };

  // Check voice sidecar on mount
  useEffect(() => {
    fetch('http://127.0.0.1:8200/health')
      .then((r) => r.ok ? r.json() : null)
      .then((d) => setVoiceEngine(d && d.status !== 'error' ? 'supertonic' : 'browser'))
      .catch(() => {
        fetch('/api/tts', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ text: '.' }),
        })
          .then((r) => setVoiceEngine(r.ok ? 'supertonic' : 'browser'))
          .catch(() => setVoiceEngine('browser'));
      });
  }, []);

  const audioQueueRef = useRef([]);
  const isPlayingRef = useRef(false);

  const stopAudio = useCallback(() => {
    window.speechSynthesis?.cancel();
    audioQueueRef.current = [];
    isPlayingRef.current = false;
    if (audioRef.current) {
      audioRef.current.pause();
      audioRef.current.currentTime = 0;
      audioRef.current = null;
    }
    setSpeaking(false);
  }, []);

  const processAudioQueue = useCallback(async () => {
    if (isPlayingRef.current || audioQueueRef.current.length === 0) return;
    isPlayingRef.current = true;
    const text = audioQueueRef.current.shift();

    if (!voiceOn) {
      isPlayingRef.current = false;
      processAudioQueue();
      return;
    }
    setSpeaking(true);
    try {
      const res = await fetch('/api/tts', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text, voice: voicePreset }),
      });
      if (res.ok) {
        setVoiceEngine('supertonic');
        const blob = await res.blob();
        const url = URL.createObjectURL(blob);
        const audio = new Audio(url);
        audio.volume = volumeRef.current;
        audio.playbackRate = speedRef.current;
        audioRef.current = audio;
        audio.onended = () => { 
          URL.revokeObjectURL(url); 
          isPlayingRef.current = false;
          processAudioQueue();
        };
        audio.onerror = () => { 
          URL.revokeObjectURL(url); 
          isPlayingRef.current = false;
          processAudioQueue();
        };
        await audio.play();
        return;
      }
    } catch { /* fall through */ }
    setVoiceEngine('browser');
    if (!window.speechSynthesis) { 
        isPlayingRef.current = false;
        processAudioQueue();
        return; 
    }
    const utterance = new SpeechSynthesisUtterance(text);
    utterance.rate = speedRef.current;
    utterance.pitch = 0.85;
    utterance.volume = volumeRef.current;
    const voices = window.speechSynthesis.getVoices();
    const pref = voices.find((v) =>
      v.name.includes('Daniel') || v.name.includes('Google UK English Male') || v.name.includes('Male')
    );
    if (pref) utterance.voice = pref;
    utterance.onend = () => { isPlayingRef.current = false; processAudioQueue(); };
    utterance.onerror = () => { isPlayingRef.current = false; processAudioQueue(); };
    window.speechSynthesis.speak(utterance);
  }, [voiceOn]);

  const enqueueSentence = useCallback((text) => {
    const clean = text.trim();
    if (!clean) return;
    audioQueueRef.current.push(clean);
    if (!isPlayingRef.current) {
        processAudioQueue();
    }
  }, [processAudioQueue]);

  // Auto-scroll
  useEffect(() => {
    if (leftRef.current) leftRef.current.scrollTop = leftRef.current.scrollHeight;
    if (rightRef.current) rightRef.current.scrollTop = rightRef.current.scrollHeight;
  }, [messages]);

  // Welcome narration
  useEffect(() => {
    if (hasSpokenWelcome.current) return;
    hasSpokenWelcome.current = true;
    const t = setTimeout(() => enqueueSentence(messages[0].text), 1200);
    return () => clearTimeout(t);
  }, [enqueueSentence]);

  // Merge Director interpretation into Design Doc
  const mergeInterpretation = useCallback((interp) => {
    setDesignDoc((prev) => {
      const next = { ...prev };
      if (interp.subject && interp.subject !== 'null') next.subject = interp.subject;
      if (interp.audience && interp.audience !== 'null') next.audience = interp.audience;
      if (interp.bloom_level && interp.bloom_level !== 'null') next.bloom_level = interp.bloom_level;
      if (interp.learning_objectives?.length) {
        const existing = new Set(next.learning_objectives);
        interp.learning_objectives.forEach((o) => { if (o) existing.add(o); });
        next.learning_objectives = [...existing];
      }
      if (interp.vocabulary?.length) {
        const existing = new Set(next.vocabulary);
        interp.vocabulary.forEach((w) => { if (w) existing.add(w); });
        next.vocabulary = [...existing];
        // Add to VAAM highlighting
        setVaamWords((prev) => {
          const next = new Set(prev);
          interp.vocabulary.forEach((w) => { if (w) next.add(w.toLowerCase()); });
          return next;
        });
      }
      if (interp.scope_creeps?.length) {
        const existing = new Set(next.scope_creeps);
        interp.scope_creeps.forEach((s) => { if (s) existing.add(s); });
        next.scope_creeps = [...existing];
      }
      return next;
    });
  }, []);

  // Highlight VAAM vocabulary words in narration text
  const highlightVaam = useCallback((text) => {
    if (vaamWords.size === 0) return text;
    const words = text.split(/(\s+)/);
    return words.map((word, i) => {
      const clean = word.toLowerCase().replace(/[^a-z]/g, '');
      if (clean && vaamWords.has(clean)) {
        return <span key={i} className="zen-vaam">{word}</span>;
      }
      return word;
    });
  }, [vaamWords]);

  // Send message — uses /api/chat/zen with typed SSE events
  const sendMessage = async () => {
    if (!input.trim() || streaming) return;
    const userText = input.trim();
    setInput('');
    stopAudio();

    setMessages((prev) => [...prev,
      { role: 'user', text: userText },
      { role: 'narrator', text: '✦ The Recycler is composing...' },
    ]);
    setStreaming(true);

    try {
      const res = await fetch('/api/chat/zen', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          message: userText,
          mode: 'zen',
          max_tokens: 200,
        }),
      });

      if (!res.ok) throw new Error('Stream failed');
      const reader = res.body.getReader();
      const decoder = new TextDecoder();
      let fullNarration = '';
      let buffer = '';
      let currentEvent = ''; // Persists across chunks — SSE event type
      let sentenceBuffer = '';

      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        buffer += decoder.decode(value, { stream: true });

        // Process complete lines
        const lines = buffer.split('\n');
        buffer = lines.pop() || ''; // Keep incomplete line

        for (const line of lines) {
          if (line.startsWith('event:')) {
            currentEvent = line.slice(6).trim();
          } else if (line.startsWith('data: ')) {
            const data = line.slice(6);
            if (data === '[DONE]') continue;

            if (currentEvent === 'interpretation') {
              // Director's design element extraction
              try {
                const interp = JSON.parse(data);
                mergeInterpretation(interp);
              } catch { /* ignore parse errors */ }
            } else if (currentEvent === 'narration') {
              // Storyteller token
              fullNarration += data;
              sentenceBuffer += data;
              
              // If token ends a sentence, queue it for speech
              if (/[.!?]\s?$|\n/.test(sentenceBuffer)) {
                  enqueueSentence(sentenceBuffer);
                  sentenceBuffer = '';
              }
              
              setMessages((prev) => {
                const u = [...prev];
                u[u.length - 1] = { role: 'narrator', text: fullNarration };
                return u;
              });
            } else {
              // Default: try to extract token from JSON
              try {
                const parsed = JSON.parse(data);
                const token = parsed.choices?.[0]?.delta?.content || parsed.token || parsed.content || '';
                if (token) {
                  fullNarration += token;
                  sentenceBuffer += token;
                  
                  if (/[.!?]\s?$|\n/.test(sentenceBuffer)) {
                      enqueueSentence(sentenceBuffer);
                      sentenceBuffer = '';
                  }

                  setMessages((prev) => {
                    const u = [...prev];
                    u[u.length - 1] = { role: 'narrator', text: fullNarration };
                    return u;
                  });
                }
              } catch {
                if (data && data !== '[DONE]') {
                  fullNarration += data;
                  sentenceBuffer += data;
                  
                  if (/[.!?]\s?$|\n/.test(sentenceBuffer)) {
                      enqueueSentence(sentenceBuffer);
                      sentenceBuffer = '';
                  }
                }
              }
            }
            // Don't reset currentEvent here — it persists until blank line (SSE spec)
          } else if (line.trim() === '') {
            // SSE event boundary — blank line resets event type
            currentEvent = '';
          }
        }
      }

      // Queue any leftover text that didn't end in punctuation
      if (sentenceBuffer.trim()) {
          enqueueSentence(sentenceBuffer);
      }

      if (fullNarration) {
        setMessages((prev) => {
          const u = [...prev];
          u[u.length - 1] = { role: 'narrator', text: fullNarration.trim() };
          return u;
        });
      }
    } catch (err) {
      console.error('ZEN stream error:', err);
      setMessages((prev) => {
        const u = [...prev];
        u[u.length - 1] = { role: 'narrator', text: 'The Great Recycler is resting. Check that llama-server is running on :8080 and :8081.' };
        return u;
      });
    }
    setStreaming(false);
  };

  // Hand In to Pete — submit design doc for quest completion
  const handInToPete = async () => {
    const fields = [designDoc.subject, designDoc.audience, designDoc.bloom_level];
    const filledFields = fields.filter(Boolean).length;
    if (filledFields < 2) return; // Need at least subject + audience

    try {
      // Get current objectives
      const res = await fetch('/api/quest/state');
      if (res.ok) {
        const state = await res.json();
        const objectives = state.quest?.phase_objectives || [];
        const incomplete = objectives.find((o) => !o.completed);
        if (incomplete) {
          await fetch('/api/quest/complete', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ objective_id: incomplete.id }),
          });
          setMessages((prev) => [...prev, {
            role: 'narrator',
            text: `✦ Pete reviews your work and nods approvingly. "${incomplete.description}" — marked complete. You feel the train lurch forward, gaining momentum.`,
          }]);
        }
      }
    } catch (err) {
      console.error('Hand-in failed:', err);
    }
  };

  const allNarratorMessages = messages.filter((m) => m.role === 'narrator');
  const allUserMessages = messages.filter((m) => m.role === 'user');
  // Rolling window — show only recent messages, older ones persist in localStorage
  const narratorMessages = allNarratorMessages.slice(-VISIBLE_NARRATIONS);
  const userMessages = allUserMessages.slice(-VISIBLE_NARRATIONS);
  const docFieldCount = [designDoc.subject, designDoc.audience, designDoc.bloom_level].filter(Boolean).length;

  const isReady = input.trim() && !streaming;

  return (
    <div className="zen-layout">
      {/* Header */}
      <header className="zen-header">
        <div className="zen-header__title">✦ ZEN MODE</div>
        <div className="zen-header__subtitle">
          The Game Engine — narration & design
        </div>
        <div className="zen-header__spacer" />

        <button
          id="zen-voice-toggle"
          onClick={() => { if (voiceOn) stopAudio(); setVoiceOn((v) => !v); }}
          className={`zen-btn ${voiceOn ? 'zen-btn--active' : ''}`}
        >
          {voiceOn
            ? (speaking ? '🔊 ● Narrating...' : `🔊 ${voiceEngine === 'supertonic' ? 'Supertonic' : 'Browser'}`)
            : '🔇 Off'}
        </button>
        <button
          id="zen-settings-toggle"
          onClick={() => setShowSettings((v) => !v)}
          className={`zen-btn ${showSettings ? 'zen-btn--active' : ''}`}
        >
          ⚙
        </button>
      </header>

      {/* Settings panel */}
      {showSettings && (
        <div className="zen-settings">
          <div className="zen-settings__group">
            <button
              id="zen-clear-session"
              onClick={() => {
                setMessages([]);
                setDesignDoc({ subject: '', audience: '', bloom_level: '', learning_objectives: [], vocabulary: [], scope_creeps: [] });
                localStorage.removeItem('zen_messages');
                localStorage.removeItem('zen_design_doc');
              }}
              className="zen-settings__clear-btn"
            >
              Clear Session
            </button>
          </div>
          <div className="zen-settings__spacer" />
          <button
            id="zen-yak-bak"
            onClick={handleRecordVoice}
            className={`zen-settings__record-btn ${recordingVoice ? 'zen-settings__record-btn--recording' : ''}`}
          >
            {recordingVoice ? '🔴 Recording (5s)...' : '🎙️ Yak Bak Customizer'}
          </button>
          <div className="zen-settings__group">
            <span>Voice</span>
            <select
              id="zen-voice-select"
              value={voicePreset}
              onChange={(e) => setVoicePreset(e.target.value)}
              className="zen-settings__select"
            >
              <optgroup label="Male">
                <option value="M1">M1 — Deep</option>
                <option value="M2">M2 — Warm</option>
                <option value="M3">M3 — Clear</option>
                <option value="M4">M4 — Bright</option>
                <option value="M5">M5 — Smooth</option>
              </optgroup>
              <optgroup label="Female">
                <option value="F1">F1 — Rich</option>
                <option value="F2">F2 — Soft</option>
                <option value="F3">F3 — Warm</option>
                <option value="F4">F4 — Bright</option>
                <option value="F5">F5 — Clear</option>
              </optgroup>
            </select>
          </div>
          <div className="zen-settings__group">
            <span>Volume</span>
            <input type="range" min="0" max="1" step="0.05" value={volume}
              onChange={(e) => { const v = parseFloat(e.target.value); setVolume(v); if (audioRef.current) audioRef.current.volume = v; }}
              className="zen-settings__range" />
            <span className="zen-settings__value">{Math.round(volume * 100)}%</span>
          </div>
          <div className="zen-settings__group">
            <span>Speed</span>
            <input type="range" min="0.5" max="1.5" step="0.05" value={speed}
              onChange={(e) => { const s = parseFloat(e.target.value); setSpeed(s); if (audioRef.current) audioRef.current.playbackRate = s; }}
              className="zen-settings__range" />
            <span className="zen-settings__value">{speed.toFixed(2)}×</span>
          </div>
          <div className="zen-settings__meta">
            Engine: <span className="zen-settings__meta-value">{voiceEngine}</span>
          </div>
          <div className="zen-settings__meta">
            Pipeline: <span className="zen-settings__meta-value">Director → Storyteller</span>
          </div>
        </div>
      )}

      {/* Three-Panel Layout */}
      <div className="zen-panels">
        {/* LEFT — Narrator */}
        <div ref={leftRef} className="zen-narrator">
          <div className="zen-narrator__label">
            ♻️ The Great Recycler — Narration
          </div>

          {narratorMessages.map((m, i) => (
            <div key={i} className="zen-narrator__message">
              {highlightVaam(m.text)}
              {i < narratorMessages.length - 1 && (
                <div className="zen-narrator__divider">— ❦ —</div>
              )}
            </div>
          ))}

          {streaming && <span className="zen-narrator__cursor" />}
        </div>

        {/* SPINE */}
        <div className="zen-spine" />

        {/* RIGHT — User Story + Design Doc */}
        <div className="zen-right">
          {/* User's reflections */}
          <div ref={rightRef} className="zen-reflections">
            <div className="zen-reflections__label">
              ✍️ Your Story — Reflection
            </div>

            {userMessages.length === 0 ? (
              <div className="zen-reflections__empty">
                Your words will appear here as you write them...
              </div>
            ) : (
              userMessages.map((m, i) => (
                <div key={i} className="zen-reflections__message">
                  {m.text}
                </div>
              ))
            )}
          </div>

          {/* Design Doc Panel */}
          <div className="zen-design-doc">
            <div className="zen-design-doc__header">
              📋 Design Doc
              {docFieldCount > 0 && (
                <span className="zen-design-doc__badge">{docFieldCount}/3 fields</span>
              )}
            </div>

            <div className="zen-design-doc__grid">
              <div>Subject: <span className={designDoc.subject ? 'zen-design-doc__value' : 'zen-design-doc__value--empty'}>
                {designDoc.subject || '—'}
              </span></div>
              <div>Audience: <span className={designDoc.audience ? 'zen-design-doc__value' : 'zen-design-doc__value--empty'}>
                {designDoc.audience || '—'}
              </span></div>
              <div>Bloom's: <span className={designDoc.bloom_level ? 'zen-design-doc__value' : 'zen-design-doc__value--empty'}>
                {designDoc.bloom_level || '—'}
              </span></div>
              <div>Vocab: <span className={designDoc.vocabulary.length ? 'zen-design-doc__value' : 'zen-design-doc__value--empty'}>
                {designDoc.vocabulary.length ? designDoc.vocabulary.slice(0, 5).join(', ') : '—'}
              </span></div>
            </div>

            {designDoc.learning_objectives.length > 0 && (
              <div className="zen-design-doc__objectives">
                Objectives: {designDoc.learning_objectives.map((o, i) => (
                  <span key={i} className="zen-design-doc__objective">• {o}</span>
                ))}
              </div>
            )}

            {designDoc.scope_creeps.length > 0 && (
              <div className="zen-design-doc__scope-creeps">
                ⚠ Scope Creeps: {designDoc.scope_creeps.join(', ')}
              </div>
            )}

            {docFieldCount >= 2 && (
              <button
                id="zen-hand-in"
                onClick={handInToPete}
                className="zen-handin-btn"
              >
                📤 Hand In to Shifu Pete
              </button>
            )}
          </div>

          {/* Input — Full Width */}
          <div className="zen-input">
            <div className="zen-input__row">
              <textarea
                id="zen-textarea"
                value={input}
                onChange={(e) => setInput(e.target.value)}
                onKeyDown={(e) => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); sendMessage(); } }}
                placeholder={streaming ? 'The Director is analyzing... the Recycler is narrating...' : 'Write your reflection...'}
                disabled={streaming}
                rows={2}
                className="zen-input__textarea"
              />
              <button
                id="zen-send"
                onClick={sendMessage}
                disabled={!isReady}
                className={`zen-input__send ${isReady ? 'zen-input__send--ready' : ''}`}
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
