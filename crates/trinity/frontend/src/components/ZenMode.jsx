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
  text: "You are a creator who teaches. And this is your forge.\n\nThe Iron Road is not about perfection — it is about reliability. There are harder substances, easier materials. But iron just gets done. And so will you.\n\nEverything you say here becomes something real — a lesson, a game, a course, an experience. You speak your subject, and the system listens. Not just to respond, but to build.\n\nWatch the right side of this page. As you talk about what you want to teach and who you want to reach, your product takes shape — Subject, Audience, Learning Objectives, Vocabulary — extracted from your own words, not a template.\n\nI am the Great Recycler. I turn your ideas into narrative, your narrative into structure, and your structure into something you can hand to a student.\n\nLay the tracks. Hammer down. Build one brick higher every time.\n\nSo — what do you teach? Who needs to learn it? Tell me like you're explaining it to a friend over coffee. The Codex will do the rest.",
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
  const [speed, setSpeed] = useState(1.15);
  const [voicePreset, setVoicePreset] = useState('M2');
  const [modelInfo, setModelInfo] = useState({ name: '...', status: 'checking' });

  // Fetch active model info on mount
  useEffect(() => {
    (async () => {
      try {
        const [modelRes, healthRes] = await Promise.all([
          fetch('/api/models/active'),
          fetch('/api/health'),
        ]);
        const model = modelRes.ok ? await modelRes.json() : {};
        const health = healthRes.ok ? await healthRes.json() : {};
        setModelInfo({
          name: model.model_name || model.name || 'Unknown',
          status: health.status || 'unknown',
        });
      } catch { setModelInfo({ name: 'Offline', status: 'error' }); }
    })();
  }, []);

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
    utterance.pitch = 1.0;
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
          phase: activePhase,
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
        u[u.length - 1] = { role: 'narrator', text: 'The Great Recycler is gathering coal for the engine. Please wait a moment and try again.' };
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

  // Active ADDIECRAPEYE phase — sent with each message so the Recycler knows context
  const [activePhase, setActivePhase] = useState(() => {
    try { return localStorage.getItem('zen_phase') || 'Analysis'; } catch { return 'Analysis'; }
  });

  useEffect(() => {
    try { localStorage.setItem('zen_phase', activePhase); } catch {}
  }, [activePhase]);

  const PHASES = [
    { key: 'Analysis',       letter: 'A', group: 'addie', tip: 'Analyze' },
    { key: 'Design',         letter: 'D', group: 'addie', tip: 'Design' },
    { key: 'Development',    letter: 'D', group: 'addie', tip: 'Develop' },
    { key: 'Implementation', letter: 'I', group: 'addie', tip: 'Implement' },
    { key: 'Evaluation',     letter: 'E', group: 'addie', tip: 'Evaluate' },
    { key: 'Contrast',       letter: 'C', group: 'crap',  tip: 'Contrast' },
    { key: 'Repetition',     letter: 'R', group: 'crap',  tip: 'Repetition' },
    { key: 'Alignment',      letter: 'A', group: 'crap',  tip: 'Alignment' },
    { key: 'Proximity',      letter: 'P', group: 'crap',  tip: 'Proximity' },
    { key: 'Envision',       letter: 'E', group: 'eye',   tip: 'Envision' },
    { key: 'Yoke',           letter: 'Y', group: 'eye',   tip: 'Yoke' },
    { key: 'Evolve',         letter: 'E', group: 'eye',   tip: 'Evolve' },
  ];

  const startNewChapter = () => {
    stopAudio();
    setMessages([WELCOME]);
    setDesignDoc({ subject: null, audience: null, bloom_level: null, learning_objectives: [], vocabulary: [], scope_creeps: [] });
    setVaamWords(new Set());
    setActivePhase('Analysis');
    localStorage.removeItem('zen_messages');
    localStorage.removeItem('zen_design_doc');
    localStorage.setItem('zen_phase', 'Analysis');
  };

  return (
    <div className="zen-layout">
      <header className="zen-header">
        <div className="zen-header__title">✦ STORY MODE</div>
        <div className="zen-header__subtitle">The Codex — your words become products</div>
        <div className="zen-header__spacer" />
        <button id="zen-new-chapter" onClick={startNewChapter} className="zen-btn">📖 New Chapter</button>
        <button id="zen-voice-toggle" onClick={() => { if (voiceOn) stopAudio(); setVoiceOn((v) => !v); }}
          className={`zen-btn ${voiceOn ? 'zen-btn--active' : ''}`}>
          {voiceOn ? (speaking ? '🔊 ● Narrating...' : `🔊 ${voiceEngine === 'supertonic' ? 'Supertonic' : 'Browser'}`) : '🔇 Off'}
        </button>
        <button id="zen-settings-toggle" onClick={() => setShowSettings((v) => !v)}
          className={`zen-btn ${showSettings ? 'zen-btn--active' : ''}`}>⚙</button>
      </header>

      {showSettings && (
        <div className="zen-settings">
          <div className="zen-settings__group">
            <button id="zen-clear-session" onClick={startNewChapter} className="zen-settings__clear-btn">Clear Session</button>
          </div>
          <div className="zen-settings__spacer" />
          <button id="zen-yak-bak" onClick={handleRecordVoice}
            className={`zen-settings__record-btn ${recordingVoice ? 'zen-settings__record-btn--recording' : ''}`}>
            {recordingVoice ? '🔴 Recording...' : '🎙️ Yak Bak'}
          </button>
          <div className="zen-settings__group">
            <span>Voice</span>
            <select id="zen-voice-select" value={voicePreset} onChange={(e) => setVoicePreset(e.target.value)} className="zen-settings__select">
              <optgroup label="Male"><option value="M1">M1</option><option value="M2">M2</option><option value="M3">M3</option></optgroup>
              <optgroup label="Female"><option value="F1">F1</option><option value="F2">F2</option><option value="F3">F3</option></optgroup>
            </select>
          </div>
          <div className="zen-settings__group">
            <span>Speed</span>
            <input type="range" min="0.5" max="1.5" step="0.05" value={speed}
              onChange={(e) => { const s = parseFloat(e.target.value); setSpeed(s); if (audioRef.current) audioRef.current.playbackRate = s; }}
              className="zen-settings__range" />
            <span className="zen-settings__value">{speed.toFixed(2)}×</span>
          </div>
          <div className="zen-settings__group">
            <span className={`zen-settings__meta-value ${modelInfo.status === 'healthy' ? '' : 'zen-settings__meta-value--warn'}`}>
              {modelInfo.status === 'healthy' ? '🟢' : '🟡'} {modelInfo.name}
            </span>
          </div>
          <div className="zen-settings__group" style={{ borderTop: '1px solid rgba(207,185,145,0.1)', paddingTop: '8px', marginTop: '4px' }}>
            <button className="zen-settings__clear-btn" style={{ fontSize: '10px' }} onClick={() => {
              fetch('/api/voice/text', { method: 'POST', headers: {'Content-Type':'application/json'},
                body: JSON.stringify({ text: 'Trinity voice system active', voice: voicePreset })
              }).catch(() => {});
            }}>🔊 Test Voice</button>
            <button className="zen-settings__clear-btn" style={{ fontSize: '10px' }} onClick={() => {
              const input = document.createElement('input');
              input.type = 'file'; input.accept = '.pdf,.md,.docx,.html';
              input.onchange = async (e) => {
                const f = e.target.files[0]; if (!f) return;
                const fd = new FormData(); fd.append('file', f);
                await fetch('/api/character/portfolio/artifact', { method: 'POST', body: fd });
              };
              input.click();
            }}>📎 Upload Artifact</button>
          </div>
        </div>
      )}

      {/* ═══ Main: Tabs + Book + Product ═══ */}
      <div className="zen-main">
        {/* Vertical ADDIECRAPEYE Tabs */}
        <nav className="zen-phases" aria-label="ADDIECRAPEYE phases">
          <div className="zen-phases__group">
            <div className="zen-phases__group-label">ADDIE</div>
            {PHASES.slice(0, 5).map((p) => (
              <button key={p.key} title={p.tip}
                className={`zen-phase-tab ${activePhase === p.key ? 'zen-phase-tab--active' : ''} zen-phase-tab--${p.group}`}
                onClick={() => setActivePhase(p.key)}>
                <span className="zen-phase-tab__letter">{p.letter}</span>
              </button>
            ))}
          </div>
          <div className="zen-phases__group">
            <div className="zen-phases__group-label">CRAP</div>
            {PHASES.slice(5, 9).map((p) => (
              <button key={p.key} title={p.tip}
                className={`zen-phase-tab ${activePhase === p.key ? 'zen-phase-tab--active' : ''} zen-phase-tab--${p.group}`}
                onClick={() => setActivePhase(p.key)}>
                <span className="zen-phase-tab__letter">{p.letter}</span>
              </button>
            ))}
          </div>
          <div className="zen-phases__group">
            <div className="zen-phases__group-label">EYE</div>
            {PHASES.slice(9, 12).map((p) => (
              <button key={p.key} title={p.tip}
                className={`zen-phase-tab ${activePhase === p.key ? 'zen-phase-tab--active' : ''} zen-phase-tab--${p.group}`}
                onClick={() => setActivePhase(p.key)}>
                <span className="zen-phase-tab__letter">{p.letter}</span>
              </button>
            ))}
          </div>
          <div className="zen-phases__active-name">{activePhase}</div>
        </nav>

        {/* The Codex — Book + Input + Product */}
        <div className="zen-codex">
          <div className="zen-panels">
            <div ref={leftRef} className="zen-narrator">
              <div className="zen-narrator__label">♻️ The Great Recycler</div>
              {narratorMessages.map((m, i) => (
                <div key={i} className="zen-narrator__message">
                  {highlightVaam(m.text)}
                  {i === narratorMessages.length - 1 && !streaming && m.text.length > 20 && (
                    <div className="zen-rlhf">
                      <button className="zen-rlhf__btn" title="This helped — steam ↑" onClick={() => {
                        fetch('/api/rlhf/resonance', { method: 'POST', headers: {'Content-Type':'application/json'},
                          body: JSON.stringify({ score: 1, phase: activePhase, message_id: `zen-${i}` })
                        });
                      }}>👍</button>
                      <button className="zen-rlhf__btn" title="Not helpful — friction ↑" onClick={() => {
                        fetch('/api/rlhf/resonance', { method: 'POST', headers: {'Content-Type':'application/json'},
                          body: JSON.stringify({ score: -1, phase: activePhase, message_id: `zen-${i}` })
                        });
                      }}>👎</button>
                    </div>
                  )}
                  {i < narratorMessages.length - 1 && <div className="zen-narrator__divider">— ❦ —</div>}
                </div>
              ))}
              {streaming && <span className="zen-narrator__cursor" />}
            </div>
            <div className="zen-spine" />
            <div ref={rightRef} className="zen-reflections">
              <div className="zen-reflections__label">✍️ Your Words</div>
              {userMessages.length === 0 ? (
                <div className="zen-reflections__empty">Your words appear here as you type below.</div>
              ) : userMessages.map((m, i) => (
                <div key={i} className="zen-reflections__message">{m.text}</div>
              ))}
            </div>
          </div>

          <div className="zen-input">
            <div className="zen-input__row">
              <div className="zen-input__phase-badge">{activePhase}</div>
              <textarea id="zen-textarea" value={input} onChange={(e) => setInput(e.target.value)}
                onKeyDown={(e) => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); sendMessage(); } }}
                placeholder={streaming ? 'Composing...' : `Speak about ${activePhase.toLowerCase()}...`}
                disabled={streaming} rows={2} className="zen-input__textarea" />
              <button id="zen-send" onClick={sendMessage} disabled={!isReady}
                className={`zen-input__send ${isReady ? 'zen-input__send--ready' : ''}`}>Send ↵</button>
            </div>
          </div>

          <div className="zen-design-doc">
            <div className="zen-design-doc__header">
              ⚗️ YOUR PRODUCT
              <span className="zen-design-doc__badge">{docFieldCount === 0 ? 'Awaiting input...' : `${docFieldCount}/3`}</span>
            </div>
            <div className="zen-product-progress">
              <div className="zen-product-progress__fill" style={{ width: `${Math.round(((docFieldCount * 20) + (designDoc.vocabulary.length > 0 ? 20 : 0) + (designDoc.learning_objectives.length > 0 ? 20 : 0)))}%` }} />
            </div>
            <div className="zen-design-doc__grid">
              <div>Subject: <span className={designDoc.subject ? 'zen-design-doc__value' : 'zen-design-doc__value--empty'}>{designDoc.subject || '...'}</span></div>
              <div>Audience: <span className={designDoc.audience ? 'zen-design-doc__value' : 'zen-design-doc__value--empty'}>{designDoc.audience || '...'}</span></div>
              <div>Bloom's: <span className={designDoc.bloom_level ? 'zen-design-doc__value' : 'zen-design-doc__value--empty'}>{designDoc.bloom_level || '...'}</span></div>
              <div>Vocab: <span className={designDoc.vocabulary.length ? 'zen-design-doc__value' : 'zen-design-doc__value--empty'}>{designDoc.vocabulary.length ? designDoc.vocabulary.slice(0,5).join(', ') : '...'}</span></div>
            </div>
            {designDoc.learning_objectives.length > 0 && (
              <div className="zen-design-doc__objectives">
                🎯 {designDoc.learning_objectives.map((o, i) => <span key={i} className="zen-design-doc__objective">• {o}</span>)}
              </div>
            )}
            {docFieldCount >= 2 && (
              <div className="zen-design-doc__actions">
                <button id="zen-hand-in" onClick={handInToPete} className="zen-handin-btn">📤 HAND IN TO PETE</button>
                <button id="zen-export" onClick={() => {
                  window.open('/api/eye/export?format=json', '_blank');
                }} className="zen-handin-btn zen-export-btn">📥 EXPORT DESIGN DOC</button>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
