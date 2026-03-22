import React, { useState, useEffect, useRef } from 'react';
import PerspectiveSidebar from './PerspectiveSidebar';
import JournalViewer from './JournalViewer';

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

// ─── Station Quest Previews (mirrors quest_system.rs Ch1 ADDIECRAPEYE) ─────────
const STATION_QUESTS = {
  Analysis: {
    blurb: 'You see the problem clearly. Name your students, your struggle, your subject.',
    objectives: [
      'Describe yourself: What do you teach? Who are your students?',
      'Identify a lesson that could be more engaging',
      'List 3 things your students struggle with',
    ],
  },
  Design: {
    blurb: 'Sketch the journey. Hook — practice — aha. One measurable objective.',
    objectives: [
      'Sketch the learning journey in 3 moments: hook — practice — aha',
      "Write one measurable objective using a Bloom's verb",
      'Choose your delivery format: game, storyboard, simulation, or lesson plan',
    ],
  },
  Development: {
    blurb: 'Draft the opening 60 seconds. Build the practice. Write the feedback loop.',
    objectives: [
      'Draft the opening 60 seconds of your experience (the hook)',
      'Build one practice activity that lets learners attempt the skill',
      'Write the feedback loop: what does the learner see when they fail? When they succeed?',
    ],
  },
  Implementation: {
    blurb: 'Run through your draft. Find the friction. Fix it.',
    objectives: [
      'Run through your draft experience yourself — time it',
      'Identify one moment of friction (confusing, slow, unclear)',
      'Fix the friction and implement the corrected version',
    ],
  },
  Evaluation: {
    blurb: 'Define success. Compare origin to result. Write the proof.',
    objectives: [
      'Define success: what metric proves this experience worked?',
      "Compare your original 'Ordinary World' description to what you built",
      "Write one sentence: 'I know this works when I see a learner...'",
    ],
  },
  Contrast: {
    blurb: 'Find the bad example. Find the great one. Name what makes yours different.',
    objectives: [
      'Find a bad example of teaching your subject — name exactly what makes it forgettable',
      'Find a great example — name the one thing that makes it stick',
      'List how your design differs from both: what is your Contrast principle?',
    ],
  },
  Repetition: {
    blurb: 'One core concept. Three different contexts. Three different senses.',
    objectives: [
      'Identify the ONE core concept that must be encountered multiple times',
      'Design 3 different contexts where learners meet that concept (Pythagorean breadth)',
      'Assign each encounter to a different sense or modality (see / do / explain)',
    ],
  },
  Alignment: {
    blurb: 'Check every connection. Hook → objective → practice → metric.',
    objectives: [
      'Check: does your hook connect directly to your measurable objective?',
      'Check: does your practice match the verb in your objective (identify, build, compare...)?',
      'Check: does your success metric match what the activity actually produces?',
    ],
  },
  Proximity: {
    blurb: 'Cluster related content. Remove what doesn\'t belong. Map the spatial layout.',
    objectives: [
      'Cluster related content together — what belongs in Act 1 vs Act 2 vs Act 3?',
      "Remove one thing that doesn't belong in Ch 1's scope",
      'Draw (or describe) the spatial layout: where does the learner look first?',
    ],
  },
  Envision: {
    blurb: 'Write your Vision. Describe the emotional arc. Ask Pete if it holds.',
    objectives: [
      "Write your PEARL Vision: 'When this works, the learner will feel...'",
      'Describe the emotional arc: bored/confused → engaged → capable → proud',
      "Ask Pete: 'Does my experience create this arc?' — log Pete's Socratic response",
    ],
  },
  Yoke: {
    blurb: 'Connect abstract to concrete. Name the real-world moment. Write the metaphor.',
    objectives: [
      'Connect your learning objective to a real-world moment the student will face',
      'Name the stakeholder who benefits most when the student masters this',
      'Yoke the abstract concept to a concrete, memorable metaphor — one sentence',
    ],
  },
  Evolve: {
    blurb: 'Compare start to finish. Write the opening chapter. Commit to the Book.',
    objectives: [
      "Compare Ch 1's PEARL to what you imagined when you started — what changed?",
      "Write the opening paragraph of your LitRPG chapter: 'In the Ordinary World, the teacher saw...'",
      'Commit the Ch 1 design to your Book — the Iron Road continues to Ch 2',
    ],
  },
};

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
  useEffect(() => { requestAnimationFrame(() => setVisible(true)); }, []);

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
          This word has emerged from your vocabulary matrix. Tame it to grow your Bestiary — or let it go wild.
        </div>
        <div className="scope-modal__actions">
          <button className="scope-btn nope" id="scope-nope-btn" onClick={() => dismiss(onNope)}>
            ✗ SCOPE NOPE
          </button>
          <button className="scope-btn hope" id="scope-hope-btn" onClick={() => dismiss(onHope)}
            style={{ borderColor: col, color: col }}>
            ✓ SCOPE HOPE
          </button>
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
  const scrollRef = useRef(null);
  const prevPhaseRef = useRef(null);

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

  // Handle SSE events — scope creep, objective completion, phase advance
  useEffect(() => {
    if (!sseEvents?.length) return;
    sseEvents.forEach(ev => {
      try {
        const data = typeof ev === 'string' ? JSON.parse(ev) : ev;
        if (data.type === 'objective_completed') {
          setNarrative(n => [...n, {
            role: 'narrator',
            content: `「 ▸ QUEST ◂ Objective sealed. +${data.xp || 10} XP flows through the rails. The track ahead grows clearer. 」`,
          }]);
        }
        if (data.type === 'phase_advanced') {
          setNarrative(n => [...n, {
            role: 'narrator',
            content: `「 ▸ STATION ◂ The whistle screams. The locomotive lurches forward into ${data.new_phase || 'the next station'} — new tracks, new questions, new light. 」`,
          }]);
        }
        if (data.type === 'creep_tameable' && data.word) {
          setCreepModal({
            word: data.word, element: data.element || '⚪',
            logos: data.logos, pathos: data.pathos, ethos: data.ethos,
          });
        }
      } catch { /* silent */ }
    });
    if (onDismissEvent) sseEvents.forEach(ev => onDismissEvent(ev));
  }, [sseEvents]);

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

  const handleScopeHope = async () => {
    if (!creepModal?.word) return setCreepModal(null);
    try {
      await fetch('/api/bestiary/tame', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ word: creepModal.word, decision: 'hope' }),
      });
      setNarrative(n => [...n, {
        role: 'narrator',
        content: `「 ▸ TAMED ◂ "${creepModal.word}" bows its head. The word-creature joins your Bestiary — a new tool forged from chaos. +15 XP 」`,
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

      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        buffer += dec.decode(value, { stream: true });

        let idx;
        while ((idx = buffer.indexOf('\n')) !== -1) {
          const line = buffer.substring(0, idx);
          buffer = buffer.substring(idx + 1);

          if (!line || line.startsWith('event:') || line.startsWith(':')) continue;
          if (line.startsWith('data: ')) {
            const payload = line.substring(6);
            if (payload === '[DONE]') continue;

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

                {STATION_QUESTS[phase] && (
                  <>
                    <div className="station-overview__blurb">
                      {STATION_QUESTS[phase].blurb}
                    </div>
                    <div className="station-overview__objectives">
                      <div className="station-overview__objectives-label">QUEST OBJECTIVES</div>
                      {STATION_QUESTS[phase].objectives.map((desc, i) => {
                        const isDone = quest?.completed_phases?.includes(phase);
                        return (
                          <div key={i} className={`station-overview__obj ${isDone ? 'station-overview__obj--done' : ''}`}>
                            <span className="station-overview__obj-check">
                              {isDone ? '✓' : '○'}
                            </span>
                            {desc}
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

          {/* ── Journal Prompt ── */}
          <div className="journal-input">
            <input
              id="chat-input"
              className="chat-input"
              placeholder="Your words, Yardmaster..."
              value={message}
              onChange={e => setMessage(e.target.value)}
              onKeyDown={e => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); sendMessage(); }}}
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
