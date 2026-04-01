import React, { useState, useEffect, useRef, useCallback } from 'react';
import PerspectiveSidebar from './PerspectiveSidebar';
import JournalViewer from './JournalViewer';
import MicButton from './MicButton';

// ─── Phase Data ────────────────────────────────────────────────────────────────
const PHASE_DATA = {
  Analysis:       { icon: '🔍', bloom: 'Remember', color: 'var(--gold)', group: 'ADDIE' },
  Design:         { icon: '📐', bloom: 'Understand', color: 'var(--blue)', group: 'ADDIE' },
  Development:    { icon: '🛠️', bloom: 'Apply', color: 'var(--purple)', group: 'ADDIE' },
  Implementation: { icon: '🚀', bloom: 'Apply', color: 'var(--green)', group: 'ADDIE' },
  Evaluation:     { icon: '📊', bloom: 'Evaluate', color: 'var(--accent)', group: 'ADDIE' },
  Contrast:       { icon: '⊕', bloom: 'Analyze', color: 'var(--gold)', group: 'CRAP' },
  Repetition:     { icon: '⧉', bloom: 'Apply', color: 'var(--blue)', group: 'CRAP' },
  Alignment:      { icon: '⌬', bloom: 'Evaluate', color: 'var(--purple)', group: 'CRAP' },
  Proximity:      { icon: '◈', bloom: 'Analyze', color: 'var(--green)', group: 'CRAP' },
  Envision:       { icon: '👁️', bloom: 'Evaluate', color: 'var(--gold)', group: 'EYE' },
  Yoke:           { icon: '🔗', bloom: 'Create', color: 'var(--accent)', group: 'EYE' },
  Evolve:         { icon: '🌱', bloom: 'Create', color: 'var(--green)', group: 'EYE' },
};

// ─── Hero's Journey chapter titles (12 stations) ───────────────────────────────
const CHAPTER_TITLES = [
  'The Ordinary World',  'The Call to Adventure', 'Refusal of the Call',
  'Meeting the Mentor',  'Crossing the Threshold', 'Tests, Allies & Enemies',
  'Approach to the Cave', 'The Ordeal',            'The Reward',
  'The Road Back',       'The Resurrection',       'Return with Elixir',
];

const PHASE_NAMES = Object.keys(PHASE_DATA);


// ─── Simple Markdown → HTML ────────────────────────────────────────────────────
function renderMarkdown(text) {
  if (!text) return '';
  let html = text
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
    if (trimmed.startsWith('<h') || trimmed.startsWith('<ul') || trimmed.startsWith('<ol')) return trimmed;
    return `<p>${trimmed.replace(/\n/g, '<br/>')}</p>`;
  }).join('');

  return html;
}

// ─── Phase Transition Ceremony ─────────────────────────────────────────────────
function PhaseTransition({ fromPhase, toPhase, xp, onDone }) {
  const pd = PHASE_DATA[toPhase] || PHASE_DATA.Analysis;
  const [visible, setVisible] = useState(false);

  useEffect(() => {
    requestAnimationFrame(() => setVisible(true));
    const t = setTimeout(() => {
      setVisible(false);
      setTimeout(onDone, 600);
    }, 3200);
    return () => clearTimeout(t);
  }, []);

  return (
    <div className={`phase-transition ${visible ? 'phase-transition--visible' : 'phase-transition--hidden'}`}>
      <div
        className="phase-transition__orb"
        style={{
          borderColor: pd.color,
          boxShadow: `0 0 40px ${pd.color}44, 0 0 80px ${pd.color}22`,
        }}
      >{pd.icon}</div>
      <div className="phase-transition__label">PHASE COMPLETE</div>
      <div className="phase-transition__title" style={{ color: pd.color }}>
        {fromPhase?.toUpperCase()} → {toPhase?.toUpperCase()}
      </div>
      <div className="phase-transition__meta">
        {pd.bloom} · {pd.group} Station
      </div>
      <div className="phase-transition__rewards">
        <div className="phase-transition__reward">
          <div className="phase-transition__reward-value" style={{ color: 'var(--gold)' }}>⚡ +{xp || 100}</div>
          <div className="phase-transition__reward-label">XP EARNED</div>
        </div>
        <div className="phase-transition__reward">
          <div className="phase-transition__reward-value" style={{ color: 'var(--accent)' }}>💨 +20</div>
          <div className="phase-transition__reward-label">STEAM</div>
        </div>
        <div className="phase-transition__reward">
          <div className="phase-transition__reward-value" style={{ color: 'var(--purple)' }}>✨ +1</div>
          <div className="phase-transition__reward-label">RESONANCE</div>
        </div>
      </div>
    </div>
  );
}

// ─── Scope Creep Modal (Hope / Nope) ──────────────────────────────────────────
function ScopeCreepModal({ creep, onHope, onNope }) {
  const [visible, setVisible] = useState(false);
  const [deck, setDeck] = useState([]);
  const [selectedHook, setSelectedHook] = useState('');

  useEffect(() => {
    requestAnimationFrame(() => setVisible(true));
    fetch('/api/character')
      .then(r => r.json())
      .then(data => {
        if (data && data.ldt_portfolio && data.ldt_portfolio.hook_deck) {
          setDeck(Object.values(data.ldt_portfolio.hook_deck));
        }
      })
      .catch(() => {});
  }, []);

  const dismiss = (action) => {
    setVisible(false);
    setTimeout(() => action(), 350);
  };

  const element = creep?.element || '⚪';
  const elementColors = {
    '🔥': '#ff6b35', '💧': '#4ec9b0', '🪨': '#cfb991',
    '💨': '#89d4f5', '🌑': '#9b59b6', '✨': '#f39c12', '⚪': '#888',
  };
  const col = elementColors[element] || '#cfb991';

  return (
    <div className={`scope-modal ${visible ? 'scope-modal--visible' : 'scope-modal--hidden'}`}>
      <div
        className={`scope-modal__card ${visible ? 'scope-modal__card--visible' : 'scope-modal__card--hidden'}`}
        style={{ borderColor: `${col}44`, boxShadow: `0 0 60px ${col}22` }}
      >
        <div className="scope-modal__header">
          <div className="scope-modal__element">{element}</div>
          <div className="scope-modal__title" style={{ color: col }}>🐾 CREEP DISCOVERED</div>
          <div
            className="scope-modal__word"
            style={{ background: `${col}11`, border: `1px solid ${col}33` }}
          >
            {creep?.word || 'Unknown'}
          </div>
        </div>
        <div className="scope-modal__stats">
          {[
            { label: 'LOGOS', val: creep?.logos || '?', color: 'var(--gold)' },
            { label: 'PATHOS', val: creep?.pathos || '?', color: 'var(--accent)' },
            { label: 'ETHOS', val: creep?.ethos || '?', color: 'var(--purple)' },
          ].map(s => (
            <div key={s.label} className="scope-modal__stat">
              <div className="scope-modal__stat-value" style={{ color: s.color }}>{s.val}</div>
              <div className="scope-modal__stat-label">{s.label}</div>
            </div>
          ))}
        </div>
        <div className="scope-modal__desc">
          This feature has emerged from the chaos. Tame it to gain project maturity — or let it go wild.
        </div>
        <div className="scope-modal__actions" style={{ flexDirection: 'column' }}>
          {deck.length > 0 && (
            <select 
               className="scope-modal__hook-select"
               value={selectedHook}
               onChange={(e) => setSelectedHook(e.target.value)}
               style={{ 
                 width: '100%', marginBottom: '12px', padding: '8px', 
                 background: 'rgba(0,0,0,0.4)', color: '#cfb991', 
                 border: `1px solid ${col}aa`, borderRadius: '4px',
                 fontFamily: '"Crimson Text", serif', fontSize: '1.1rem'
               }}
            >
              <option value="">-- Select a spell to tame this anomaly --</option>
              {deck.map(h => <option key={h.id} value={h.id}>Lv{h.level} {h.title} ({h.school})</option>)}
            </select>
          )}
          <div style={{ display: 'flex', gap: '12px', width: '100%' }}>
            <button className="scope-btn nope" id="scope-nope-btn" onClick={() => dismiss(() => onNope())}>
              ✗ LEAVE WILD
            </button>
            <button 
              className="scope-btn hope" 
              id="scope-hope-btn" 
              onClick={() => dismiss(() => onHope(selectedHook))}
              disabled={!selectedHook && deck.length > 0}
              style={{ 
                borderColor: col, color: col, 
                opacity: (!selectedHook && deck.length > 0) ? 0.3 : 1,
                cursor: (!selectedHook && deck.length > 0) ? 'not-allowed' : 'pointer'
              }}
            >
              ✓ CAST HOOK & TAME
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}

// ─── Main PhaseWorkspace (Narrative-First LitRPG) ──────────────────────────────
export default function PhaseWorkspace({ quest, sseEvents, onDismissEvent, onRefetch, viewPhase, allPhases, onClearView }) {
  const [message, setMessage] = useState('');
  const [narrative, setNarrative] = useState([]); // { role, content, speaker? }
  const [isStreaming, setIsStreaming] = useState(false);
  const [transition, setTransition] = useState(null);
  const [creepModal, setCreepModal] = useState(null);
  const [objectivesOpen, setObjectivesOpen] = useState(true);
  const [sessionZero, setSessionZero] = useState({ step: 0, answers: {} });
  const [lowCoalWarned, setLowCoalWarned] = useState(false);
  const [journalOpen, setJournalOpen] = useState(false);
  const [voiceOn, setVoiceOn] = useState(false);
  const [voicePreset, setVoicePreset] = useState('M1');
  const scrollRef = useRef(null);
  const prevPhaseRef = useRef(null);
  const audioQueueRef = useRef([]);
  const isPlayingRef = useRef(false);
  const audioRef = useRef(null);

  const processAudioQueue = React.useCallback(async () => {
    if (isPlayingRef.current || audioQueueRef.current.length === 0) return;
    isPlayingRef.current = true;
    const text = audioQueueRef.current.shift();

    if (!voiceOn) {
        isPlayingRef.current = false;
        processAudioQueue();
        return;
    }

    try {
      const res = await fetch('/api/tts', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text, voice: voicePreset }),
      });
      if (res.ok) {
        const blob = await res.blob();
        const url = URL.createObjectURL(blob);
        const audio = new Audio(url);
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
        return; // Web Audio API takes care of playback
      }
    } catch { /* Fallthrough to browser */ }
    
    // Fallback if Supertonic sidecar fails
    if (window.speechSynthesis) {
        const utterance = new SpeechSynthesisUtterance(text);
        utterance.rate = 1.0;
        utterance.onend = () => { isPlayingRef.current = false; processAudioQueue(); };
        utterance.onerror = () => { isPlayingRef.current = false; processAudioQueue(); };
        window.speechSynthesis.speak(utterance);
    } else {
        isPlayingRef.current = false;
        processAudioQueue();
    }
  }, [voiceOn]);

  const enqueueSentence = React.useCallback((text) => {
    const clean = text.trim();
    if (!clean || !voiceOn) return;
    audioQueueRef.current.push(clean);
    if (!isPlayingRef.current) {
        processAudioQueue();
    }
  }, [processAudioQueue, voiceOn]);

  const activePhase = quest?.phase || 'Analysis';
  const isViewing = viewPhase && viewPhase !== activePhase;
  const phase = isViewing ? viewPhase : activePhase;
  const pd = PHASE_DATA[phase] || PHASE_DATA.Analysis;
  const objectives = quest?.objectives || [];
  const allComplete = objectives.length > 0 && objectives.every(o => o.completed);
  const completedCount = objectives.filter(o => o.completed).length;
  const phaseIdx = PHASE_NAMES.indexOf(phase);
  const chapterTitle = CHAPTER_TITLES[phaseIdx] || 'Unknown Chapter';

  // Auto-scroll narrative
  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [narrative]);

  // Detect phase change → transition ceremony + inline banner
  useEffect(() => {
    const prev = prevPhaseRef.current;
    if (prev && prev !== phase) {
      setTransition({ fromPhase: prev, toPhase: phase, xp: quest?.xp || 0 });
      setNarrative(n => [...n, { role: 'banner', content: phase }]);
    }
    prevPhaseRef.current = phase;
  }, [phase]);

  // Initial welcome — Pete introduces the Iron Road + Session Zero
  useEffect(() => {
    if (quest?.subject && narrative.length === 0) {
      setNarrative([
        { role: 'banner', content: phase },
        {
          role: 'narrator',
          content: `The platform stretches before you — iron rails humming with potential. You have chosen **${quest.subject}** as your cargo, the pearl of wisdom you will carry across twelve stations.`,
        },
        {
          role: 'assistant', speaker: 'PETE',
          content: `You step aboard the locomotive, and the furnace answers with a low rumble. I'm Pete — your Conductor on the Iron Road.\n\nBefore we lay the first track, I need to know who I'm riding with. Three questions — they'll shape the journey ahead.\n\n**Question 1 of 3:** What's your teaching experience level? (e.g. first-year teacher, 10 years K-12, college adjunct, no formal experience yet)`,
        },
      ]);
      setSessionZero({ step: 1, answers: {} });
    }
  }, [quest?.subject]);

  // Low coal warning
  useEffect(() => {
    if (quest?.coal !== undefined && quest.coal < 20 && !lowCoalWarned) {
      setLowCoalWarned(true);
      setNarrative(n => [...n, {
        role: 'narrator',
        content: '「 ▸ WARNING ◂ The furnace gutters, its light barely a whisper. The Yardmaster needs rest — even the Iron Road has water stops. 」',
      }]);
    }
    if (quest?.coal >= 20) setLowCoalWarned(false);
  }, [quest?.coal]);

  const onRefetchRef = useRef(onRefetch);
  useEffect(() => { onRefetchRef.current = onRefetch; }, [onRefetch]);

  // Extracted event handler for both SSE and chat stream
  const handleStreamEvent = useCallback((data) => {
    if (!data) return;
    let needsRefetch = false;
    
    if (data.type === 'objective_completed' || data.type === 'quest_sync') {
      needsRefetch = true;
      setNarrative(n => [...n, {
        role: 'narrator',
        content: `「 ▸ QUEST ◂ Objective sealed. +${data.xp || 10} XP flows through the rails. The track ahead grows clearer. 」`,
      }]);
    }
    
    if (data.type === 'phase_advanced') {
      needsRefetch = true;
      setNarrative(n => [...n, {
        role: 'narrator',
        content: `「 ▸ STATION ◂ The whistle screams. The locomotive lurches forward into ${data.new_phase || 'the next station'} — new tracks, new questions, new light. 」`,
      }]);
    }
    
    if (data.type === 'creep_tameable') {
      setCreepModal({
        word: data.word || data.name || 'Unknown', 
        element: data.element || '⚪',
        logos: data.logos || Math.floor(Math.random() * 10) + 1, 
        pathos: data.pathos || Math.floor(Math.random() * 10) + 1, 
        ethos: data.ethos || Math.floor(Math.random() * 10) + 1,
      });
    }

    if (data.type === 'vaam' && data.detections?.length > 0) {
      const words = data.detections.map(d => d.word).join(', ');
      const coal = data.total_coal || 0;
      const mastered = data.detections.filter(d => d.mastered).length;
      const messages = [
        `「 ▸ VAAM ◂ The rails hum with recognition — **${words}**. +${coal} coal feeds the furnace. 」`,
        `「 ▸ VAAM ◂ Words of power shimmer on the track — **${words}**. The vocabulary matrix grows stronger. 」`,
        `「 ▸ VAAM ◂ The Great Recycler senses domain mastery — **${words}** etched into the rails. +${coal} coal. 」`,
      ];
      setNarrative(n => [...n, {
        role: 'narrator',
        content: mastered > 0
          ? `「 ▸ MASTERY ◂ **${words}** — fully tamed. The word-creatures bow. The Bestiary grows. 」`
          : messages[Math.floor(Math.random() * messages.length)],
      }]);
    }

    if (data.type === 'cognitive_load' && data.friction !== undefined) {
      if (data.friction > 50) {
        setNarrative(n => [...n, {
          role: 'narrator',
          content: '「 ▸ FRICTION ◂ The tracks groan under weight. The Gilbreth Protocol suggests — simplify. Reduce scope. Breathe. 」',
        }]);
      }
    }

    if (data.type === 'resources') {
      needsRefetch = true;
    }

    if (needsRefetch && onRefetchRef.current) onRefetchRef.current();
  }, []);

  // Handle SSE events from global hook
  useEffect(() => {
    if (!sseEvents?.length) return;
    sseEvents.forEach(ev => {
      try {
        const data = typeof ev === 'string' ? JSON.parse(ev) : ev;
        handleStreamEvent(data);
      } catch { /* silent */ }
    });
    if (onDismissEvent) sseEvents.forEach(ev => onDismissEvent(ev.id || ev));
  }, [sseEvents, onDismissEvent, handleStreamEvent]);

  // Handle SSE events from local chat stream dispatch
  useEffect(() => {
    const fn = (e) => handleStreamEvent(e.detail);
    window.addEventListener('trinity-stream-event', fn);
    return () => window.removeEventListener('trinity-stream-event', fn);
  }, [handleStreamEvent]);

  // ── Actions ──

  const completeObjective = async (objectiveId) => {
    try {
      const res = await fetch('/api/quest/complete', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ objective_id: objectiveId }),
      });
      if (res.ok && onRefetch) onRefetch();
    } catch (err) {
      console.error('Complete objective failed:', err);
    }
  };

  const advancePhase = async () => {
    try {
      const res = await fetch('/api/quest/advance', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({}),
      });
      if (res.ok && onRefetch) onRefetch();
    } catch (err) {
      console.error('Advance failed:', err);
    }
  };

  const handleScopeHope = async (hookId) => {
    if (!creepModal?.word) return setCreepModal(null);
    try {
      await fetch('/api/bestiary/tame', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ word: creepModal.word, decision: 'hope', hook_id: hookId }),
      });
      setNarrative(n => [...n, {
        role: 'narrator',
        content: `「 ▸ COMPONENT TAMED ◂ "${creepModal.word}" has been integrated systemically. A Hook was cast, Project Maturity advances. +15 XP 」`,
      }]);
      if (onRefetch) onRefetch();
    } catch {}
    setCreepModal(null);
  };

  const handleScopeNope = async () => {
    if (!creepModal?.word) return setCreepModal(null);
    try {
      await fetch('/api/bestiary/tame', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ word: creepModal.word, decision: 'nope' }),
      });
      setNarrative(n => [...n, {
        role: 'narrator',
        content: `「 ▸ NOPE ◂ "${creepModal.word}" dissolves back into the vocabulary fog. Sometimes the bravest act is knowing what to leave behind. 」`,
      }]);
    } catch {}
    setCreepModal(null);
  };

  const exportGDD = async () => {
    try {
      const res = await fetch('/api/quest/compile', { method: 'POST' });
      const data = await res.json();
      setNarrative(n => [...n, {
        role: 'narrator',
        content: `「 ▸ COMPILED ◂ The Great Recycler binds the pages — "${data.title || 'Game Design Document'}" now rests in the archives. ${data.phases_completed || 0} stations recorded. 」`,
      }]);
    } catch {
      setNarrative(n => [...n, {
        role: 'narrator',
        content: '「 ▸ BLOCKED ◂ The pages are blank — more stations must be cleared before the Great Recycler can compile the document. 」',
      }]);
    }
  };

  // ── SSE Streaming Chat ──

  const sendMessage = async () => {
    if (!message.trim() || isStreaming) return;
    const userMsg = message.trim();
    setMessage('');
    setNarrative(n => [...n, { role: 'user', content: userMsg }]);

    // Visible consequence: coal burn on user message
    setNarrative(n => [...n, {
      role: 'system',
      content: '「 COAL -2 」 The firebox dims as your attention is spent.',
    }]);

    // Session Zero: capture character creation answers
    if (sessionZero.step >= 1 && sessionZero.step <= 3) {
      const fieldMap = { 1: 'experience', 2: 'audience', 3: 'success_vision' };
      const field = fieldMap[sessionZero.step];
      const newAnswers = { ...sessionZero.answers, [field]: userMsg };
      const nextStep = sessionZero.step + 1;
      setSessionZero({ step: nextStep, answers: newAnswers });

      if (nextStep > 3) {
        try {
          await fetch('/api/character', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(newAnswers),
          });
          if (onRefetch) onRefetch();
        } catch (err) {
          console.error('Character sheet update failed:', err);
        }
      }
    }

    setIsStreaming(true);
    setNarrative(n => [...n, { role: 'assistant', speaker: 'PETE', content: '' }]);

    try {
      const res = await fetch('/api/chat/stream', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          message: userMsg,
          mode: 'iron-road',
          max_tokens: 4096,
          use_rag: true,
        }),
      });

      if (!res.ok) {
        setNarrative(n => {
          const copy = [...n];
          copy[copy.length - 1] = {
            role: 'assistant', speaker: 'PETE',
            content: `⚠ The Great Recycler encountered an error (${res.status}). The tracks may need repair.`,
          };
          return copy;
        });
        setIsStreaming(false);
        return;
      }

      const reader = res.body.getReader();
      const dec = new TextDecoder();
      let buffer = '';
      let fullText = '';
      let sentenceBuffer = '';

      let currentEvent = null;

      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        buffer += dec.decode(value, { stream: true });

        let idx;
        while ((idx = buffer.indexOf('\n')) !== -1) {
          const line = buffer.substring(0, idx);
          buffer = buffer.substring(idx + 1);

          if (!line || line.startsWith(':')) continue;

          if (line.startsWith('event:')) {
            currentEvent = line.substring(6).trim();
            continue;
          }

          if (line.startsWith('data: ')) {
            const payload = line.substring(6);
            if (payload === '[DONE]') continue;

            if (currentEvent) {
              try {
                const data = JSON.parse(payload);
                const evObj = { type: currentEvent, ...data, id: Date.now() };
                // Dispatch globally for PerspectiveSidebar, GameHUD, etc.
                window.dispatchEvent(new CustomEvent('trinity-stream-event', { detail: evObj }));
              } catch (e) { }
              currentEvent = null;
              continue; // Don't parse this data as chat tokens
            }

            let token = '';
            if (payload.startsWith('{')) {
              try {
                const j = JSON.parse(payload);
                token = j.content || j.choices?.[0]?.delta?.content || '';
              } catch {
                token = payload;
              }
            } else {
              token = payload;
            }

            if (token) {
              fullText += token;
              sentenceBuffer += token;
              
              if (/[.!?]\s?$|\n/.test(sentenceBuffer)) {
                  if (voiceOn) enqueueSentence(sentenceBuffer);
                  sentenceBuffer = '';
              }

              const captured = fullText;
              setNarrative(n => {
                const copy = [...n];
                const last = copy[copy.length - 1];
                if (last?.role === 'assistant') {
                  copy[copy.length - 1] = { ...last, content: captured };
                }
                return copy;
              });
            }
          }
        }
      }
      if (sentenceBuffer.trim() && voiceOn) {
          enqueueSentence(sentenceBuffer);
      }
    } catch (err) {
      setNarrative(n => {
        const copy = [...n];
        const last = copy[copy.length - 1];
        if (last?.role === 'assistant' && !last.content) {
          copy[copy.length - 1] = {
            ...last,
            content: '⚠ Connection lost. The Great Recycler will return when the tracks are clear.',
          };
        }
        return copy;
      });
    }
    setIsStreaming(false);

    // Visible consequence: steam gain after Pete responds
    setNarrative(n => [...n, {
      role: 'system',
      content: '「 STEAM +5 」 Momentum builds — the wheels catch the rail.',
    }]);

    // Session Zero: Pete asks the next character-creation question
    if (sessionZero.step === 2) {
      setNarrative(n => [...n, {
        role: 'assistant', speaker: 'PETE',
        content: `You nod, and the answer lands like a rivet driven home. Good — that reshapes the track ahead.\n\n**Question 2 of 3:** Who are your students? Tell me about their age group, grade level, or any context that matters. Paint the faces in the seats — I need to see who we're building this for.`,
      }]);
    } else if (sessionZero.step === 3) {
      setNarrative(n => [...n, {
        role: 'assistant', speaker: 'PETE',
        content: `The Great Recycler scratches it into the ledger. Your audience is taking shape.\n\n**Question 3 of 3:** What does success look like for this lesson? Not what they'll *know* — what they'll be able to *do*. Paint me the victory condition. What does the world look like when your students walk out of that room changed?`,
      }]);
    } else if (sessionZero.step === 4) {
      setSessionZero({ step: 0, answers: {} });
      setNarrative(n => [...n, {
        role: 'narrator',
        content: '「 ▸ CHARACTER SHEET ◂ The Great Recycler seals the ledger with a hiss of steam. The Yardmaster\'s identity is forged. 」',
      }, {
        role: 'assistant', speaker: 'PETE',
        content: `The furnace roars to life — *now* we ride.\n\nYour Character Sheet is locked in, Yardmaster. I know who you are, who you teach, and what the finish line looks like.\n\nLook to the **${phase}** station. Those quest objectives on the board? Those are the tracks we need to lay. Each one moves the train forward.\n\nWhich objective calls to you first?`,
      }]);
    }
  };

  // ── Render ──

  return (
    <>
      {transition && (
        <PhaseTransition
          fromPhase={transition.fromPhase} toPhase={transition.toPhase}
          xp={transition.xp} onDone={() => setTransition(null)}
        />
      )}
      {creepModal && (
        <ScopeCreepModal creep={creepModal} onHope={handleScopeHope} onNope={handleScopeNope} />
      )}

      <div className="phase-workspace">
        {/* ── Phase Header ── */}
        <div className="phase-header">
          <div className="phase-header__info">
            <span className="phase-header__icon">{pd.icon}</span>
            <div>
              <div className="phase-header__name" style={{ color: pd.color }}>
                {phase.toUpperCase()}
              </div>
              <div className="phase-header__meta">
                {pd.group} · Bloom's: {pd.bloom} · {quest?.chapter_title || 'The Ordinary World'}
              </div>
            </div>
          </div>

          <div className="phase-header__actions">
            <div className="phase-header__progress">
              <div className="phase-header__progress-track">
                <div
                  className="phase-header__progress-fill"
                  style={{
                    width: `${objectives.length ? (completedCount / objectives.length) * 100 : 0}%`,
                    background: `linear-gradient(to right, var(--gold-dark), ${pd.color})`,
                  }}
                />
              </div>
              <span className="phase-header__progress-label">
                {completedCount}/{objectives.length}
              </span>
            </div>

            <button
              id="audio-toggle-btn"
              className="gdd-export-btn"
              style={voiceOn ? { background: 'var(--accent)', color: '#fff', borderColor: 'var(--accent)' } : {}}
              onClick={() => setVoiceOn(v => !v)}
              title="Toggle Socratic Voice Narration"
            >
              {voiceOn ? '🔊 VOICE ON' : '🔈 VOICE OFF'}
            </button>

            {voiceOn && (
              <select
                value={voicePreset}
                onChange={(e) => setVoicePreset(e.target.value)}
                className="gdd-export-btn"
                style={{ cursor: 'pointer', padding: '4px 8px' }}
                title="Select narrator voice"
              >
                <option value="M1">M1</option>
                <option value="M2">M2</option>
                <option value="M3">M3</option>
                <option value="M4">M4</option>
                <option value="M5">M5</option>
                <option value="F1">F1</option>
                <option value="F2">F2</option>
                <option value="F3">F3</option>
                <option value="F4">F4</option>
                <option value="F5">F5</option>
              </select>
            )}

            <button
              id="objectives-toggle"
              className="gdd-export-btn"
              onClick={() => setObjectivesOpen(o => !o)}
              title="Toggle quest objectives"
            >
              {objectivesOpen ? '▾' : '▸'} QUESTS
            </button>

            <button id="export-gdd-btn" className="gdd-export-btn" onClick={exportGDD} title="Compile GDD">
              📜 GDD
            </button>
            <button id="export-quiz-btn" className="gdd-export-btn" onClick={() => window.open('/api/eye/export?format=html5_quiz', '_blank')} title="Export HTML5 Quiz">
              📝 Quiz
            </button>
            <button id="export-adventure-btn" className="gdd-export-btn" onClick={() => window.open('/api/eye/export?format=html5_adventure', '_blank')} title="Export Choose-Your-Adventure">
              🗺️ Adventure
            </button>
          </div>
        </div>

        {/* ── Collapsible Quest Objectives ── */}
        {objectivesOpen && (
          <div className="objectives-section">
            <div className="objectives-label">QUEST OBJECTIVES</div>
            {objectives.map(obj => (
              <div
                key={obj.id} id={`obj-${obj.id}`}
                className={`objective-item ${obj.completed ? 'objective-item--done' : ''}`}
                onClick={() => !obj.completed && completeObjective(obj.id)}
              >
                <div className={`objective-check ${obj.completed ? 'objective-check--done' : ''}`}>
                  {obj.completed ? '✓' : ''}
                </div>
                <div className={`objective-text ${obj.completed ? 'objective-text--done' : ''}`}>
                  {obj.description}
                </div>
              </div>
            ))}
            {allComplete && (
              <div className="handoff-banner">
                <div className="handoff-banner__icon">🔮 → ⚙️</div>
                <div className="handoff-banner__text">
                  <strong>Vision set.</strong> The Great Recycler has mapped this station.
                  Pete is ready to build.
                </div>
                <div className="handoff-banner__actions">
                  <button
                    id="advance-phase-btn" className="advance-btn"
                    style={{ background: `linear-gradient(135deg, var(--gold-dark), ${pd.color})` }}
                    onClick={advancePhase}
                  >
                    ⚡ ADVANCE STATION
                  </button>
                  <button
                    className="gdd-export-btn handoff-banner__yard-btn"
                    onClick={() => {
                      // Switch to Yardmaster mode via footer mode toggle
                      fetch('/api/mode', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ mode: 'yardmaster' }),
                      }).catch(() => {});
                      // Dispatch custom event for App.jsx to pick up
                      window.dispatchEvent(new CustomEvent('trinity-mode', { detail: 'yardmaster' }));
                    }}
                  >
                    🔧 OPEN PETE'S WORKSHOP
                  </button>
                </div>
              </div>
            )}
          </div>
        )}

        {/* ── The Narrative — Book of the Iron Road ── */}
        <div className="book-stage">
          {isViewing ? (
            <div className="narrative-scroll">
              <div className="station-overview">
                <div className="station-overview__chapter">
                  CHAPTER {phaseIdx + 1}
                </div>
                <div className="station-overview__title">
                  {chapterTitle}
                </div>
                <div className="station-overview__phase">
                  {pd.icon} {phase.toUpperCase()}
                </div>
                <div className="station-overview__meta">
                  Bloom's: {pd.bloom} · {pd.group} Station
                </div>

                <div className="station-overview__status">
                  {quest?.completed_phases?.includes(phase) ? (
                    <div className="station-overview__badge station-overview__badge--done">
                      ✓ STATION COMPLETE
                    </div>
                  ) : phase === activePhase ? (
                    <div className="station-overview__badge station-overview__badge--active">
                      ◈ ACTIVE STATION — {completedCount}/{objectives.length} objectives
                    </div>
                  ) : (
                    <div className="station-overview__badge station-overview__badge--locked">
                      ◇ NOT YET REACHED
                    </div>
                  )}
                </div>

                {objectives.length > 0 && (
                  <>
                    <div className="station-overview__objectives">
                      <div className="station-overview__objectives-label">DYNAMIC QUEST OBJECTIVES</div>
                      {objectives.map((obj, i) => {
                        const isDone = obj.completed;
                        return (
                          <div key={i} className={`station-overview__obj ${isDone ? 'station-overview__obj--done' : ''}`}>
                            <span className="station-overview__obj-check">
                              {isDone ? '✓' : '○'}
                            </span>
                            {obj.description}
                          </div>
                        );
                      })}
                    </div>
                  </>
                )}

                <button className="station-overview__return" onClick={onClearView}>
                  ← Return to {activePhase} Station
                </button>
              </div>
            </div>
          ) : (
            <div className="narrative-scroll" ref={scrollRef}>
            {narrative.length === 0 && (
              <div className="book-empty">
                <div className="book-empty__icon">🚂</div>
                <p>The Great Recycler awaits your words, Yardmaster.</p>
                <p>The Iron Road stretches forward — speak, and the journey begins.</p>
              </div>
            )}

            {narrative.map((msg, i) => {
              if (msg.role === 'banner') {
                const bpd = PHASE_DATA[msg.content] || PHASE_DATA.Analysis;
                return (
                  <div key={i} className="phase-banner">
                    CHAPTER {Object.keys(PHASE_DATA).indexOf(msg.content) + 1}: {msg.content.toUpperCase()}
                    <span className="blooms-tag">Bloom's: {bpd.bloom} · {bpd.group} Station</span>
                  </div>
                );
              }

              if (msg.role === 'system') {
                return (
                  <div key={i} className="msg msg-system">
                    {msg.content}
                  </div>
                );
              }

              if (msg.role === 'narrator') {
                return (
                  <div key={i} className="msg msg-narrator">
                    <div className="narrator-label">🔮 THE GREAT RECYCLER <span className="slot-badge slot-badge--0">slot 0</span></div>
                    <div dangerouslySetInnerHTML={{ __html: renderMarkdown(msg.content) }} />
                  </div>
                );
              }

              if (msg.role === 'image') {
                return (
                  <div key={i} className="msg msg-narrator narrative-image-panel">
                    <div className="narrator-label">🖼️ ART STUDIO</div>
                    <img
                      src={msg.url || `data:image/png;base64,${msg.base64}`}
                      alt={msg.content || 'Generated artwork'}
                      style={{ maxWidth: '100%', borderRadius: '8px', margin: '0.5rem 0' }}
                    />
                    {msg.content && <p className="image-caption" style={{ opacity: 0.7, fontSize: '0.85rem', fontStyle: 'italic', margin: '0.25rem 0 0' }}>{msg.content}</p>}
                  </div>
                );
              }

              if (msg.role === 'user') {
                return (
                  <div key={i} className="msg msg-user">
                    "{msg.content}"
                  </div>
                );
              }

              return (
                <div key={i} className="msg msg-ai">
                  <div className="speaker">⚙️ {msg.speaker || 'PETE'} <span className="slot-badge slot-badge--1">slot 1</span></div>
                  <div dangerouslySetInnerHTML={{ __html: renderMarkdown(msg.content) }} />
                </div>
              );
            })}

            {isStreaming && (
              <div className="typing-indicator">
                <div className="typing-dots">
                  <span /><span /><span />
                </div>
                The quill scratches across the page...
              </div>
            )}
            <div className="scroll-anchor" />
          </div>
          )}

          {/* ── Journal toggle ── */}
          <button
            className="journal-btn"
            style={{ margin: '0.3rem 0', alignSelf: 'flex-start' }}
            onClick={() => setJournalOpen(!journalOpen)}
          >{journalOpen ? '📜 Close Journal' : '📜 Journal'}</button>

          {journalOpen && (
            <JournalViewer onClose={() => setJournalOpen(false)} />
          )}

          {/* ── Ring 6: Perspective Engine ── */}
          <PerspectiveSidebar
            sseEvents={sseEvents}
            onDismissEvent={onDismissEvent}
          />

          {/* ── Journal Prompt Drop Zone ── */}
          <div 
            className="journal-input"
            onDragOver={(e) => {
              e.preventDefault();
              e.currentTarget.style.boxShadow = '0 0 15px rgba(207, 185, 145, 0.3)';
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
                  'Pearl': '🔮 I am casting the PEARL hook. Please review my current Phase progress against the 5 PEARL evaluation dimensions (Purpose, Evidence, Alignment, Rigor, Learner-centricity).',
                  'Coal': '🪨 I am casting the COAL hook. I am feeling stuck. Please generate some raw, unfiltered ideas to help me complete my current objective.',
                  'Steam': '💨 I am casting the STEAM hook. I have momentum! What is the fastest micro-step I can take right now to accelerate my progress?',
                  'Hook': '🪝 I am casting the HOOK hook. How can I make this specific lesson phase more engaging or interactive for the learner?',
                  'Mirror': '🪞 I am casting the MIRROR hook. Please ask me a deep Socratic reflection question about the design choices I have made so far.',
                  'Compass': '🧭 I am casting the COMPASS hook. Please re-orient me. Where exactly am I in the ADDIECRAPEYE lifecycle, and what is my overarching goal?'
                };
                const msg = prompts[hookId] || `I am casting the ${hookId} hook. Please guide me.`;
                setMessage(msg);
                setTimeout(() => {
                   document.getElementById('chat-input')?.focus();
                }, 50);
              }
            }}
            style={{ transition: 'box-shadow 0.2s' }}
          >
            <input
              id="chat-input"
              className="chat-input"
              placeholder="Your words, Yardmaster..."
              value={message}
              onChange={e => setMessage(e.target.value)}
              onKeyDown={e => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); sendMessage(); }}}
              disabled={isStreaming}
            />
            <MicButton
              onTranscript={(text) => setMessage(prev => prev ? prev + ' ' + text : text)}
              disabled={isStreaming}
            />
            <button
              className="chat-send" id="chat-send-btn"
              onClick={sendMessage}
              disabled={!message.trim() || isStreaming}
            >
              {isStreaming ? '···' : '↵'}
            </button>
          </div>
        </div>
      </div>
    </>
  );
}
