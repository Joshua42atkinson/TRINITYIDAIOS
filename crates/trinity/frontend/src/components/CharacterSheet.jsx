import React, { useState, useEffect } from 'react';
import { marked } from 'marked';

marked.setOptions({
  gfm: true,
  breaks: false,
});

/**
 * THE ADDIECRAPEYE AGREEMENT — Character Sheet
 * 
 * The "Contract Between You and the Iron Road" — shows the full 12-station
 * ADDIECRAPEYE lifecycle as the agreement structure between user and computer.
 * 
 * Centerpiece: the 12 stations with status, Bloom's level, Hero's Journey title,
 * and what the user commits to at each phase.
 * 
 * Bottom: compressed Cognitive Logistics + Competency Scores + Artifact Vault.
 * 
 * Fetches from GET /api/character and GET /api/quest.
 */

// ── The 12 Stations ─────────────────────────────────────────────────────────────
// PEARL = Perspective · Engineering · Aesthetic · Research · Layout
// Every station is focused through one or more PEARL lenses.
// "Everything that is CRAP needs a PEARL" — design without focus is decoration.
const STATIONS = [
  { id: 'Analysis',       icon: '🔍', bloom: 'Remember',   group: 'ADDIE', groupColor: '#CFB991', chapter: 'The Ordinary World',        agreement: 'Identify the learning problem, audience, and constraints',  deliverable: 'Needs Analysis',       pearl: 'P·R', pearlFull: 'Perspective + Research — Who is the learner? What does the data say?' },
  { id: 'Design',         icon: '📐', bloom: 'Understand',  group: 'ADDIE', groupColor: '#CFB991', chapter: 'The Call to Adventure',      agreement: 'Plan objectives, assessments, and instructional strategy',  deliverable: 'Learning Objectives',  pearl: 'P·E', pearlFull: 'Perspective + Engineering — How will we reach them? What\'s the architecture?' },
  { id: 'Development',    icon: '🛠️', bloom: 'Apply',       group: 'ADDIE', groupColor: '#CFB991', chapter: 'Refusal of the Call',        agreement: 'Build the instructional materials and media',               deliverable: 'Lesson & Media',       pearl: 'E·A', pearlFull: 'Engineering + Aesthetic — Build it right, make it beautiful' },
  { id: 'Implementation', icon: '🚀', bloom: 'Apply',       group: 'ADDIE', groupColor: '#CFB991', chapter: 'Meeting the Mentor',         agreement: 'Deploy the instruction to learners',                        deliverable: 'Deployment Plan',      pearl: 'E·L', pearlFull: 'Engineering + Layout — Deploy in the right structure and context' },
  { id: 'Evaluation',     icon: '📊', bloom: 'Evaluate',    group: 'ADDIE', groupColor: '#CFB991', chapter: 'Crossing the Threshold',     agreement: 'Assess outcomes and refine based on evidence',              deliverable: 'Evaluation Report',    pearl: 'R',   pearlFull: 'Research — What does the evidence show? Did it work?' },
  { id: 'Contrast',       icon: '⊕',  bloom: 'Analyze',     group: 'CRAP',  groupColor: '#22d3ee', chapter: 'Tests, Allies & Enemies',    agreement: 'Make visual elements distinct and purposeful',              deliverable: 'Visual Hierarchy',     pearl: 'A·P', pearlFull: 'Aesthetic + Perspective — Distinct for WHOM? Design with the learner in mind' },
  { id: 'Repetition',     icon: '⧉',  bloom: 'Apply',       group: 'CRAP',  groupColor: '#22d3ee', chapter: 'Approach to the Cave',       agreement: 'Establish consistent design patterns',                      deliverable: 'Design System',        pearl: 'A·E', pearlFull: 'Aesthetic + Engineering — Consistent patterns built into the system' },
  { id: 'Alignment',      icon: '⌬',  bloom: 'Evaluate',    group: 'CRAP',  groupColor: '#22d3ee', chapter: 'The Ordeal',                agreement: 'Create visual structure and spatial order',                  deliverable: 'Structured Layout',    pearl: 'L·A', pearlFull: 'Layout + Aesthetic — Spatial order that serves comprehension' },
  { id: 'Proximity',      icon: '◈',  bloom: 'Analyze',     group: 'CRAP',  groupColor: '#22d3ee', chapter: 'The Reward',                agreement: 'Group related content for cognitive clarity',                deliverable: 'Content Grouping',     pearl: 'L·P', pearlFull: 'Layout + Perspective — Group for the learner, not the designer' },
  { id: 'Envision',       icon: '👁️', bloom: 'Evaluate',    group: 'EYE',   groupColor: '#a78bfa', chapter: 'The Road Back',              agreement: 'See the end state — imagine the finished product',           deliverable: 'Vision Statement',     pearl: 'P',   pearlFull: 'Perspective — See through the learner\'s eyes' },
  { id: 'Yoke',           icon: '🔗', bloom: 'Create',      group: 'EYE',   groupColor: '#a78bfa', chapter: 'The Resurrection',           agreement: 'Connect all pieces into a unified whole',                    deliverable: 'Game Design Doc',      pearl: 'PEARL', pearlFull: 'All five lenses unified — the PEARL is the thread that binds' },
  { id: 'Evolve',         icon: '🌱', bloom: 'Create',      group: 'EYE',   groupColor: '#a78bfa', chapter: 'Return with Elixir',         agreement: 'Iterate, improve, and transcend the current version',        deliverable: 'HTML5 Export',         pearl: 'R·P', pearlFull: 'Research + Perspective — Iterate based on evidence, evolve the lens' },
];

const GROUP_META = {
  ADDIE: { label: 'EXTRACT THE WISDOM', color: '#CFB991', range: '1–5' },
  CRAP:  { label: 'PLACE THE WISDOM',   color: '#22d3ee', range: '6–9' },
  EYE:   { label: 'REFINE THE WISDOM',  color: '#a78bfa', range: '10–12' },
};

// ── The Finish Line — what you walk away with ───────────────────────────────────
const FINISH_LINE = [
  { icon: '📜', label: 'Game Design Document',    desc: 'A complete blueprint compiled from all 12 phases', group: 'EYE' },
  { icon: '📝', label: 'HTML5 Interactive Game',   desc: 'A self-contained quiz or adventure that runs in any browser', group: 'EYE' },
  { icon: '📋', label: 'Lesson Plans & Rubrics',   desc: 'Bloom\'s-aligned materials ready for the classroom', group: 'ADDIE' },
  { icon: '🎨', label: 'Visual Design System',     desc: 'CRAP-polished layouts, assets, and brand patterns', group: 'CRAP' },
  { icon: '🏆', label: 'LDT Portfolio Artifact',   desc: '12 vaulted artifacts — your evidence of graduation', group: 'ALL' },
];

// ── Maturation Dimensions (computed from progress) ──────────────────────────────
const MATURATION_DIMS = [
  { key: 'content',     label: 'Content Readiness',    icon: '📄', phases: ['Analysis', 'Design'],               color: '#CFB991' },
  { key: 'production',  label: 'Production Quality',   icon: '🛠️', phases: ['Development', 'Implementation'],    color: '#fb923c' },
  { key: 'pedagogy',    label: 'Pedagogical Rigor',    icon: '📊', phases: ['Evaluation'],                       color: '#34d399' },
  { key: 'design',      label: 'Visual Design',        icon: '🎨', phases: ['Contrast', 'Repetition', 'Alignment', 'Proximity'], color: '#22d3ee' },
  { key: 'reflection',  label: 'Metacognitive Depth',  icon: '👁️', phases: ['Envision', 'Yoke', 'Evolve'],       color: '#a78bfa' },
  { key: 'portfolio',   label: 'Portfolio Completion',  icon: '🏆', phases: null, /* uses artifact count */      color: '#10b981' },
];

export default function CharacterSheet() {
  const [sheet, setSheet] = useState(null);
  const [quest, setQuest] = useState(null);
  const [pearl, setPearl] = useState(null);
  const [error, setError] = useState(null);
  const [vaultOpen, setVaultOpen] = useState(false);
  const [activeTab, setActiveTab] = useState('contract'); // 'contract', 'hookbook', 'identity'
  const [hookBookHtml, setHookBookHtml] = useState('');
  const [backstoryText, setBackstoryText] = useState('');
  const [alignmentText, setAlignmentText] = useState('');
  const [locomotiveProfile, setLocomotiveProfile] = useState('AnalyzerClass');
  const [appearanceData, setAppearanceData] = useState('');
  const [audioPrefs, setAudioPrefs] = useState({
    genre: '', voice_id: '', bg_music_genre: '', music_flow_enabled: true
  });
  const [savingBackstory, setSavingBackstory] = useState(false);
  const [eyeData, setEyeData] = useState(null);

  useEffect(() => {
    Promise.all([
      fetch('/api/character').then(r => r.ok ? r.json() : Promise.reject(`Character: ${r.status}`)),
      fetch('/api/quest').then(r => r.ok ? r.json() : Promise.reject(`Quest: ${r.status}`)),
      fetch('/api/pearl').then(r => r.ok ? r.json() : null).catch(() => null),
    ])
      .then(([charData, questData, pearlData]) => {
        setSheet(charData);
        setQuest(questData);
        setBackstoryText(charData.backstory || '');
        setAlignmentText(charData.alignment || '');
        setLocomotiveProfile(charData.locomotive_profile || 'AnalyzerClass');
        setAppearanceData(charData.appearance || '');
        if (charData.audio_preferences) {
          setAudioPrefs(charData.audio_preferences);
        }
        if (pearlData && !pearlData.error) setPearl(pearlData);
      })
      .catch(err => {
        console.error("Failed to load character/quest", err);
        setError(String(err));
      });
  }, []);

  useEffect(() => {
    if (activeTab === 'eyeportfolio' && !eyeData) {
      fetch('/api/eye/preview')
        .then(r => r.ok ? r.json() : null)
        .then(data => setEyeData(data))
        .catch(err => console.error("Failed to load EYE container", err));
    }
  }, [activeTab]);

  const saveBackstory = async () => {
    setSavingBackstory(true);
    try {
      const payload = {
        backstory: backstoryText,
        alignment: alignmentText,
        locomotive_profile: locomotiveProfile,
        appearance: appearanceData,
        audio_preferences: audioPrefs
      };
      const resp = await fetch('/api/character', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      });
      if (!resp.ok) throw new Error('Failed to save');
      const updatedSheet = await resp.json();
      setSheet(updatedSheet);
    } catch (err) {
      console.error(err);
    } finally {
      setSavingBackstory(false);
    }
  };

  if (error) return (
    <div style={s.loading}>
      <span style={{ color: '#ef4444' }}>⚠ Heavilon Event: {error}</span>
    </div>
  );

  if (!sheet) return (
    <div style={s.loading}>
      <span style={s.loadingPulse}>Igniting the Firebox...</span>
    </div>
  );

  const ldt = sheet.ldt_portfolio || {
    completed_challenges: 0, gate_review_status: 'Locked', artifact_vault: [],
    ibstpi_score: 0, atd_score: 0, aect_score: 0, qm_alignment_score: 0,
    heavilon_events_survived: 0, memorial_steps_climbed: 0,
  };
  const current_coal = sheet.current_coal ?? 100;
  const current_steam = sheet.current_steam ?? 0;
  const track_friction = sheet.track_friction ?? 0;
  const cargo_slots = sheet.cargo_slots ?? 7;
  const activePhase = quest?.phase || 'Analysis';
  const completedPhases = quest?.completed_phases || [];

  // ── Maturation scoring ──
  const computeMaturation = (dim) => {
    if (dim.key === 'portfolio') return Math.min((ldt.completed_challenges / 12) * 100, 100);
    const done = dim.phases.filter(p => completedPhases.includes(p)).length;
    return Math.min((done / dim.phases.length) * 100, 100);
  };
  const overallMaturation = Math.round(
    MATURATION_DIMS.reduce((sum, d) => sum + computeMaturation(d), 0) / MATURATION_DIMS.length
  );

  const userClass = sheet.user_class || 'Player';
  const classEmoji = { SubjectMatterExpert: '🧑‍🏫', InstructionalDesigner: '🎓', Stakeholder: '📊', Player: '🎮' };
  const classColors = { SubjectMatterExpert: '#f59e0b', InstructionalDesigner: '#a78bfa', Stakeholder: '#34d399', Player: '#22d3ee' };

  const getStatus = (stationId) => {
    if (completedPhases.includes(stationId)) return 'complete';
    if (stationId === activePhase) return 'active';
    return 'locked';
  };

  const statusBadge = (status) => {
    const cfg = {
      complete: { text: '✅ COMPLETE', bg: 'rgba(16,185,129,0.12)', border: 'rgba(16,185,129,0.3)', color: '#10b981' },
      active:   { text: '🔶 ACTIVE',   bg: 'rgba(245,158,11,0.12)', border: 'rgba(245,158,11,0.3)', color: '#f59e0b' },
      locked:   { text: '🔒 LOCKED',   bg: 'rgba(100,116,139,0.08)', border: 'rgba(100,116,139,0.2)', color: '#64748b' },
    }[status];
    return (
      <span style={{
        fontSize: '10px', fontFamily: "'JetBrains Mono', monospace", fontWeight: 700,
        padding: '3px 8px', borderRadius: '4px', letterSpacing: '1px',
        background: cfg.bg, border: `1px solid ${cfg.border}`, color: cfg.color,
      }}>
        {cfg.text}
      </span>
    );
  };

  // ── Render ──
  return (
    <div style={s.container}>
      <div style={s.glassCard}>



        {/* ═══ HEADER ═══ */}
        <header style={s.header}>
          <div>
            <div style={{ display: 'flex', alignItems: 'center', gap: '16px', flexWrap: 'wrap' }}>
              <h1 style={s.title}>{sheet.alias}</h1>
              <span style={{
                fontSize: '13px', padding: '4px 12px', borderRadius: '4px',
                background: `${classColors[userClass] || '#22d3ee'}15`,
                border: `1px solid ${classColors[userClass] || '#22d3ee'}40`,
                color: classColors[userClass] || '#22d3ee',
                fontFamily: "'JetBrains Mono', monospace", letterSpacing: '2px', textTransform: 'uppercase',
              }}>
                {classEmoji[userClass] || '🎮'} {userClass.replace(/([A-Z])/g, ' $1').trim()}
              </span>
            </div>
            <p style={s.subtitle}>
              Level {sheet.resonance_level}
              {' — '}<span style={{ color: '#8E7744' }}>{(sheet.locomotive_profile || 'AnalyzerClass').replace(/([A-Z])/g, ' $1').trim()}</span>
            </p>
          </div>
          <div style={{ textAlign: 'right' }}>
            <p style={s.gateLabel}>GATE REVIEW</p>
            <p style={s.gateValue}>{ldt.gate_review_status}</p>
            <p style={{ fontSize: '12px', color: '#4B5563', fontFamily: "'JetBrains Mono', monospace", marginTop: '4px' }}>{sheet.total_xp} XP</p>
          </div>
        </header>

        {/* ═══ TAB NAVIGATION ═══ */}
        <div style={s.tabNav}>
          {['contract', 'eyeportfolio', 'hookbook', 'identity'].map(tab => (
            <button
              key={tab}
              onClick={() => setActiveTab(tab)}
              style={{
                ...s.tabButton,
                ...(activeTab === tab ? s.tabActive : s.tabInactive)
              }}
            >
              {tab === 'contract' ? '📜 The Contract' : tab === 'eyeportfolio' ? '👁️ The EYE Portfolio' : tab === 'hookbook' ? '📖 Hook Book' : '👤 Player Identity'}
            </button>
          ))}
        </div>

        {activeTab === 'contract' && (
          <>
        {/* ═══ YOUR PEARL — The User's Contract ═══ */}
        {pearl && (
          <div style={s.pearlSection}>
            <div style={s.pearlHeader}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
                <span style={{ fontSize: '24px' }}>🔮</span>
                <div>
                  <div style={s.pearlSectionTitle}>YOUR PEARL — The Contract You Wrote</div>
                  <div style={{ fontSize: '11px', color: '#64748b' }}>
                    This is YOUR definition of success. Pete's Socratic interview shaped it. You can refine it anytime.
                  </div>
                </div>
              </div>
            </div>
            <div style={s.pearlContent}>
              <div style={s.pearlSubject}>
                <span style={{ fontSize: '18px' }}>{pearl.medium_icon || '🎮'}</span>
                <div>
                  <div style={{ fontSize: '16px', fontWeight: 700, color: '#E2E8F0' }}>{pearl.subject || 'No subject yet'}</div>
                  <div style={{ fontSize: '11px', color: '#64748b' }}>via {pearl.medium || 'Game'}</div>
                </div>
              </div>
              {pearl.vision ? (
                <div style={s.pearlVision}>"{pearl.vision}"</div>
              ) : (
                <div style={{ ...s.pearlVision, color: '#4B5563' }}>⚠ No vision set yet — talk to Pete to define your success criteria</div>
              )}
              <div style={s.pearlScores}>
                {[
                  { label: 'ADDIE', score: pearl.addie_score, color: '#CFB991' },
                  { label: 'CRAP',  score: pearl.crap_score,  color: '#22d3ee' },
                  { label: 'EYE',   score: pearl.eye_score,   color: '#a78bfa' },
                ].map(a => (
                  <div key={a.label} style={{ flex: 1 }}>
                    <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: '9px', color: '#64748b', marginBottom: '3px', fontFamily: "'JetBrains Mono', monospace", letterSpacing: '1px' }}>
                      <span>{a.label}</span>
                      <span style={{ color: a.color }}>{Math.round((a.score || 0) * 100)}%</span>
                    </div>
                    <div style={s.matBarTrack}>
                      <div style={{ height: '6px', borderRadius: '3px', width: `${(a.score || 0) * 100}%`, background: a.color, transition: 'width 0.5s' }} />
                    </div>
                  </div>
                ))}
                <div style={{ textAlign: 'center', fontFamily: "'JetBrains Mono', monospace" }}>
                  <div style={{ fontSize: '8px', color: '#64748b', letterSpacing: '1px' }}>ALIGN</div>
                  <div style={{ fontSize: '16px', fontWeight: 'bold', color: (pearl.alignment || 0) >= 0.6 ? '#10b981' : '#f59e0b' }}>
                    {Math.round((pearl.alignment || 0) * 100)}%
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* ═══ THE FINISH LINE — What You're Building ═══ */}
        <div style={s.finishLineSection}>
          <div style={s.finishLineHeader}>
            <span style={s.finishLineIcon}>🏁</span>
            <div>
              <div style={s.finishLineTitle}>WHAT YOU'RE BUILDING</div>
              <div style={s.finishLineSub}>Complete the 12 stations and walk away with these deliverables.</div>
            </div>
          </div>
          <div style={s.finishLineGrid}>
            {FINISH_LINE.map(fl => {
              const groupDone = fl.group === 'ALL'
                ? ldt.completed_challenges >= 12
                : STATIONS.filter(st => st.group === fl.group).every(st => completedPhases.includes(st.id));
              return (
                <div key={fl.label} style={{
                  ...s.finishLineCard,
                  opacity: groupDone ? 1 : 0.55,
                  borderColor: groupDone ? 'rgba(16,185,129,0.4)' : 'rgba(207,185,145,0.1)',
                }}>
                  <span style={{ fontSize: '24px' }}>{fl.icon}</span>
                  <div>
                    <div style={{ fontSize: '13px', fontWeight: 700, color: groupDone ? '#34d399' : '#E2E8F0' }}>
                      {groupDone ? '✓ ' : ''}{fl.label}
                    </div>
                    <div style={{ fontSize: '11px', color: '#64748b', marginTop: '2px' }}>{fl.desc}</div>
                  </div>
                </div>
              );
            })}
          </div>
        </div>

        {/* ═══ MATURATION SCORECARD ═══ */}
        <div style={s.maturationSection}>
          <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '16px', flexWrap: 'wrap', gap: '8px' }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
              <span style={{ fontSize: '20px' }}>📈</span>
              <div>
                <div style={s.maturationTitle}>MATURATION MAP</div>
                <div style={{ fontSize: '11px', color: '#64748b' }}>How ready is your project across each dimension?</div>
              </div>
            </div>
            <div style={s.overallPill}>
              <span style={{ fontSize: '10px', color: '#64748b', letterSpacing: '1px', textTransform: 'uppercase' }}>Overall</span>
              <span style={{ fontSize: '22px', fontWeight: 'bold', color: overallMaturation >= 80 ? '#10b981' : overallMaturation >= 40 ? '#f59e0b' : '#ef4444' }}>
                {overallMaturation}%
              </span>
            </div>
          </div>
          <div style={s.maturationGrid}>
            {MATURATION_DIMS.map(dim => {
              const pct = Math.round(computeMaturation(dim));
              return (
                <div key={dim.key} style={s.maturationRow}>
                  <div style={s.maturationLabel}>
                    <span>{dim.icon}</span>
                    <span style={{ color: '#94a3b8' }}>{dim.label}</span>
                  </div>
                  <div style={s.matBarTrack}>
                    <div style={{
                      height: '8px', borderRadius: '4px',
                      width: `${pct}%`, background: dim.color,
                      transition: 'width 0.8s ease-out',
                      boxShadow: pct > 0 ? `0 0 8px ${dim.color}44` : 'none',
                    }} />
                  </div>
                  <span style={{ ...s.matPct, color: dim.color }}>{pct}%</span>
                </div>
              );
            })}
          </div>
        </div>

        {/* ═══ AGREEMENT BANNER ═══ */}
        <div style={s.agreementBanner}>
          <div style={s.agreementIcon}>📜</div>
          <div>
            <div style={s.agreementTitle}>THE ADDIECRAPEYE AGREEMENT</div>
            <div style={s.agreementSub}>
              This is the contract between you and the Iron Road — 12 stations, each a commitment to growth.
              Complete all phases to earn Gate Review.
            </div>
          </div>
          <div style={s.progressPill}>
            <span style={{ color: '#34d399', fontWeight: 'bold', fontSize: '18px' }}>{ldt.completed_challenges}</span>
            <span style={{ color: '#64748b' }}>/12</span>
          </div>
        </div>

        {/* ═══ THE 12-STATION AGREEMENT GRID ═══ */}
        {['ADDIE', 'CRAP', 'EYE'].map(group => {
          const gm = GROUP_META[group];
          const groupStations = STATIONS.filter(st => st.group === group);
          return (
            <div key={group} style={{ marginBottom: '24px' }}>
              {/* Group Header */}
              <div style={{
                display: 'flex', alignItems: 'center', gap: '12px',
                marginBottom: '12px', paddingBottom: '8px',
                borderBottom: `1px solid ${gm.color}25`,
              }}>
                <span style={{
                  fontSize: '11px', fontFamily: "'Cinzel', serif",
                  color: gm.color, letterSpacing: '3px', fontWeight: 700,
                }}>
                  {group}
                </span>
                <span style={{
                  fontSize: '10px', color: '#64748b', fontFamily: "'Inter', sans-serif",
                  letterSpacing: '1px',
                }}>
                  {gm.label}
                </span>
                <span style={{
                  fontSize: '10px', color: '#4B5563', fontFamily: "'JetBrains Mono', monospace",
                  marginLeft: 'auto',
                }}>
                  STATIONS {gm.range}
                </span>
              </div>

              {/* Station Cards */}
              <div style={s.stationGrid}>
                {groupStations.map((st, i) => {
                  const status = getStatus(st.id);
                  const stIdx = STATIONS.indexOf(st) + 1;
                  const isActive = status === 'active';
                  const objectives = isActive ? (quest?.objectives || []) : [];
                  return (
                    <div key={st.id} style={{
                      ...s.stationCard,
                      borderLeftColor: status === 'complete' ? '#10b981' :
                                       status === 'active' ? '#f59e0b' : '#27272a',
                      opacity: status === 'locked' ? 0.6 : 1,
                    }}>
                      {/* Top row: number + icon + name + status */}
                      <div style={s.stationTop}>
                        <span style={{ ...s.stationNum, color: st.groupColor }}>{stIdx}</span>
                        <span style={{ fontSize: '20px' }}>{st.icon}</span>
                        <div style={{ flex: 1 }}>
                          <div style={{ ...s.stationName, color: status === 'locked' ? '#64748b' : '#E2E8F0' }}>
                            {st.id}
                          </div>
                          <div style={s.stationMeta}>
                            Bloom's: {st.bloom} · Produces: {st.deliverable}
                          </div>
                        </div>
                        {statusBadge(status)}
                      </div>

                      {/* PEARL lens + Chapter title */}
                      <div style={{ paddingLeft: '38px' }}>
                        <span title={st.pearlFull} style={s.pearlBadge}>
                          🔮 {st.pearl}
                        </span>
                        <div style={s.chapterTitle}>
                          "{st.chapter}"
                        </div>
                      </div>

                      {/* Agreement — what the user commits to */}
                      <div style={s.agreementText}>
                        {st.agreement}
                      </div>

                      {/* Active phase: show real quest objectives */}
                      {isActive && objectives.length > 0 && (
                        <div style={s.questBlock}>
                          <div style={s.questLabel}>QUEST OBJECTIVES</div>
                          {objectives.map((obj, oi) => (
                            <div key={oi} style={{
                              ...s.questItem,
                              opacity: obj.completed ? 0.5 : 1,
                              textDecoration: obj.completed ? 'line-through' : 'none',
                            }}>
                              <span style={{ color: obj.completed ? '#10b981' : '#f59e0b', flexShrink: 0 }}>
                                {obj.completed ? '✓' : '○'}
                              </span>
                              <span>{obj.description}</span>
                            </div>
                          ))}
                        </div>
                      )}
                    </div>
                  );
                })}
              </div>
            </div>
          );
        })}

        {/* ═══ BOTTOM: LOGISTICS + SCORES ═══ */}
        <div style={s.bottomGrid}>

          {/* LEFT: Cognitive Logistics */}
          <div style={s.bottomPanel}>
            <h3 style={s.sectionTitle}>⚙️ Cognitive Logistics</h3>

            {/* Bars */}
            {[
              { label: '🔥 Coal (Attention)', value: current_coal, max: 100, unit: '%', gradient: 'linear-gradient(90deg, #ea580c, #fb923c)' },
              { label: '⚡ Steam (Momentum)', value: current_steam, max: 100, unit: ' PSI', gradient: 'linear-gradient(90deg, #0891b2, #22d3ee)' },
              { label: '⛓ Friction (Extraneous)', value: track_friction, max: 100, unit: '%', gradient: '#dc2626' },
            ].map(bar => (
              <div key={bar.label} style={{ marginBottom: '12px' }}>
                <div style={s.barHeader}>
                  <span>{bar.label}</span>
                  <span>{Math.round(bar.value)}{bar.unit}</span>
                </div>
                <div style={s.barTrack}>
                  <div style={{
                    height: '5px', borderRadius: '3px',
                    width: `${Math.min(bar.value, bar.max)}%`,
                    background: bar.gradient,
                    transition: 'width 0.5s ease-out',
                  }} />
                </div>
              </div>
            ))}

            {/* Cargo Slots */}
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginTop: '8px' }}>
              <span style={s.cargoLabel}>Cargo Slots</span>
              <span style={{ color: '#cbd5e1', fontSize: '12px', fontFamily: "'JetBrains Mono', monospace" }}>{cargo_slots}/9</span>
            </div>
            <div style={{ display: 'flex', gap: '4px', marginTop: '6px' }}>
              {Array.from({ length: 9 }, (_, i) => (
                <div key={i} style={{
                  flex: 1, height: '10px', borderRadius: '3px',
                  background: i < cargo_slots
                    ? 'linear-gradient(90deg, #f59e0b, #fbbf24)'
                    : 'rgba(39,39,42,0.8)',
                  border: i < cargo_slots
                    ? '1px solid rgba(245,158,11,0.4)'
                    : '1px solid #3f3f46',
                }} />
              ))}
            </div>
          </div>

          {/* RIGHT: Competency Scores */}
          <div style={s.bottomPanel}>
            <h3 style={s.sectionTitle}>🏆 Competency Scores</h3>
            <div style={s.scoreGrid}>
              {[
                { label: 'QM Score', value: ldt.qm_alignment_score.toFixed(1), color: '#22d3ee' },
                { label: 'IBSTPI', value: `${ldt.ibstpi_score.toFixed(0)}%`, color: '#E2E8F0' },
                { label: 'ATD', value: `${ldt.atd_score.toFixed(0)}%`, color: '#a78bfa' },
                { label: 'AECT', value: `🛡️ ${ldt.aect_score.toFixed(0)}%`, color: '#34d399' },
                { label: 'Heavilon', value: ldt.heavilon_events_survived, color: '#fb923c' },
                { label: 'Memorial', value: `${ldt.memorial_steps_climbed}/17`, color: '#f59e0b' },
              ].map(sc => (
                <div key={sc.label} style={s.scoreCard}>
                  <div style={s.scoreLabel}>{sc.label}</div>
                  <div style={{ ...s.scoreValue, color: sc.color }}>{sc.value}</div>
                </div>
              ))}
            </div>
          </div>
        </div>

        {/* ═══ ARTIFACT VAULT (collapsible) ═══ */}
        <div style={{ marginTop: '16px' }}>
          <button onClick={() => setVaultOpen(v => !v)} style={s.vaultToggle}>
            <span>📖 Artifact Vault ({ldt.artifact_vault.length} / 12)</span>
            <span style={{ fontSize: '12px', opacity: 0.6 }}>{vaultOpen ? '▾' : '▸'}</span>
          </button>
          {vaultOpen && (
            <div style={s.vaultGrid}>
              {ldt.artifact_vault.length === 0 ? (
                <div style={s.emptyVault}>
                  The vault is empty. Lay some iron, Yardmaster.
                </div>
              ) : (
                ldt.artifact_vault.map((artifact, idx) => (
                  <div key={idx} style={s.artifactCard}>
                    <h4 style={{ fontWeight: 'bold', color: '#E2E8F0', fontSize: '15px', margin: '0 0 6px 0' }}>
                      {artifact.title}
                    </h4>
                    <div style={{ display: 'flex', gap: '6px', marginBottom: '8px', flexWrap: 'wrap' }}>
                      <span style={s.typeBadge}>{artifact.artifact_type}</span>
                      <span style={s.supraBadge}>{artifact.aligned_supra_badge}</span>
                      {artifact.hooks_cast && artifact.hooks_cast.map(hook => (
                        <span key={hook} style={{
                          fontSize: '8px', background: 'rgba(167,139,250,0.1)', color: '#a78bfa',
                          padding: '2px 6px', borderRadius: '4px', textTransform: 'uppercase',
                          letterSpacing: '1px', border: '1px solid rgba(167,139,250,0.3)',
                        }}>
                          📖 {hook}
                        </span>
                      ))}
                    </div>
                    <p style={s.reflection}>"{artifact.reflection_journal}"</p>
                    <div style={s.artifactFooter}>
                      <span style={{ color: '#22d3ee' }}>QM: {artifact.qm_score}/100</span>
                      {artifact.aect_ethics_cleared
                        ? <span style={{ color: '#10b981' }}>✓ AECT</span>
                        : <span style={{ color: '#ef4444' }}>Ethics Review Req</span>}
                    </div>
                  </div>
                ))
              )}
            </div>
          )}
        </div>


      {/* End of Contract Tab */}
          </>
        )}

        {/* ═══ THE EYE PORTFOLIO ═══ */}
        {activeTab === 'eyeportfolio' && (
          <div style={{ marginTop: '24px', padding: '16px', borderRadius: '12px', background: 'rgba(15,13,10,0.6)', border: '1px solid rgba(22,163,74,0.3)' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '24px', borderBottom: '1px solid rgba(22,163,74,0.3)', paddingBottom: '16px' }}>
              <div>
                <h2 style={{ fontFamily: "'Cinzel', serif", color: '#4ade80', fontSize: '24px', margin: 0 }}>The Executive Summary</h2>
                <p style={{ color: '#94a3b8', fontSize: '12px', marginTop: '4px' }}>An aggregation of your ADDIECRAPEYE outputs. The Great Recycler uses this to build the final product.</p>
              </div>
              <div style={{ display: 'flex', gap: '8px' }}>
                <a href="/api/eye/export?format=html5_quiz" target="_blank" rel="noreferrer" style={{ background: '#CFB991', color: '#1A1A1A', padding: '8px 16px', borderRadius: '4px', textDecoration: 'none', fontSize: '13px', fontWeight: 'bold' }}>Export Quiz</a>
                <a href="/api/eye/export?format=html5_adventure" target="_blank" rel="noreferrer" style={{ background: '#10b981', color: '#1A1A1A', padding: '8px 16px', borderRadius: '4px', textDecoration: 'none', fontSize: '13px', fontWeight: 'bold' }}>Export Adventure</a>
                <a href="/api/eye/export?format=zip_portfolio" target="_blank" rel="noreferrer" style={{ background: '#4ec9b0', color: '#1A1A1A', padding: '8px 16px', borderRadius: '4px', textDecoration: 'none', fontSize: '13px', fontWeight: 'bold', border: '1px solid #CFB991' }}>Download Portfolio ZIP</a>
              </div>
            </div>

            {!eyeData ? (
              <div style={{ color: '#94a3b8', textAlign: 'center', padding: '40px' }}>Syncing EYE Container...</div>
            ) : (
              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '24px' }}>
                {/* Left Column: Constraints & Meta */}
                <div>
                  <div style={{ background: 'rgba(0,0,0,0.3)', borderRadius: '8px', padding: '16px', border: '1px solid rgba(255,255,255,0.05)', marginBottom: '16px' }}>
                    <h3 style={{ margin: '0 0 12px 0', fontSize: '14px', color: '#cbd5e1', textTransform: 'uppercase', letterSpacing: '1px' }}>Project Metadata</h3>
                    <div style={{ display: 'grid', gridTemplateColumns: '120px 1fr', gap: '8px', fontSize: '13px' }}>
                      <span style={{ color: '#64748b' }}>Title:</span> <span style={{ color: '#f8fafc' }}>{eyeData.metadata?.title || 'Unknown'}</span>
                      <span style={{ color: '#64748b' }}>Subject:</span> <span style={{ color: '#10b981' }}>{eyeData.metadata?.subject || 'Unknown'}</span>
                      <span style={{ color: '#64748b' }}>PEARL Phase:</span> <span style={{ color: '#a78bfa' }}>{eyeData.metadata?.pearl_phase}</span>
                      <span style={{ color: '#64748b' }}>Align Grade:</span> <span style={{ color: '#f59e0b' }}>{eyeData.metadata?.alignment_grade}</span>
                    </div>
                  </div>

                  <div style={{ background: 'rgba(0,0,0,0.3)', borderRadius: '8px', padding: '16px', border: '1px solid rgba(255,255,255,0.05)' }}>
                    <h3 style={{ margin: '0 0 12px 0', fontSize: '14px', color: '#cbd5e1', textTransform: 'uppercase', letterSpacing: '1px' }}>PEARL Constraints (Lens)</h3>
                    <p style={{ fontSize: '13px', color: '#cbd5e1', lineHeight: '1.6', whiteSpace: 'pre-wrap' }}>
                      {eyeData.pearl_summary || "No PEARL constraints have been established. Define them in Pete's terminal."}
                    </p>
                  </div>
                </div>

                {/* Right Column: Content Checklists */}
                <div>
                  <div style={{ background: 'rgba(0,0,0,0.3)', borderRadius: '8px', padding: '16px', border: '1px solid rgba(255,255,255,0.05)', marginBottom: '16px', maxHeight: '300px', overflowY: 'auto' }}>
                    <h3 style={{ margin: '0 0 12px 0', fontSize: '14px', color: '#cbd5e1', textTransform: 'uppercase', letterSpacing: '1px' }}>Objectives Traceability</h3>
                    {eyeData.objectives && eyeData.objectives.length > 0 ? (
                      <ul style={{ listStyle: 'none', padding: 0, margin: 0 }}>
                        {eyeData.objectives.map((obj, i) => (
                          <li key={i} style={{ fontSize: '12px', marginBottom: '8px', display: 'flex', gap: '8px', alignItems: 'flex-start' }}>
                            <span style={{ color: obj.completed ? '#10b981' : '#64748b' }}>{obj.completed ? '✅' : '⏳'}</span>
                            <div>
                              <span style={{ color: '#94a3b8', fontSize: '10px', display: 'block' }}>[{obj.phase}]</span>
                              <span style={{ color: obj.completed ? '#cbd5e1' : '#64748b' }}>{obj.description}</span>
                            </div>
                          </li>
                        ))}
                      </ul>
                    ) : (
                      <div style={{ fontSize: '12px', color: '#64748b' }}>No objectives tracked yet.</div>
                    )}
                  </div>

                  <div style={{ background: 'rgba(0,0,0,0.3)', borderRadius: '8px', padding: '16px', border: '1px solid rgba(255,255,255,0.05)' }}>
                    <h3 style={{ margin: '0 0 12px 0', fontSize: '14px', color: '#cbd5e1', textTransform: 'uppercase', letterSpacing: '1px' }}>Generated Assets</h3>
                    {eyeData.assets && eyeData.assets.length > 0 ? (
                      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(60px, 1fr))', gap: '8px' }}>
                        {eyeData.assets.map((asset, i) => (
                          <div key={i} style={{ background: 'rgba(255,255,255,0.05)', padding: '8px', borderRadius: '4px', textAlign: 'center', fontSize: '10px', color: '#cbd5e1' }}>
                            <div style={{ fontSize: '16px', marginBottom: '4px' }}>
                              {asset.asset_type === 'png' ? '🖼️' : asset.asset_type === 'wav' ? '🔊' : '📄'}
                            </div>
                            <div style={{ whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>{asset.filename}</div>
                          </div>
                        ))}
                      </div>
                    ) : (
                      <div style={{ fontSize: '12px', color: '#64748b' }}>No artifacts generated by the Art Studio yet.</div>
                    )}
                  </div>
                </div>
              </div>
            )}
          </div>
        )}

        {/* ═══ THE HOOK BOOK TAB ═══ */}
        {activeTab === 'hookbook' && (
          <div className="chariot-content" style={{ marginTop: '24px', padding: '16px', borderRadius: '12px', background: 'rgba(15,13,10,0.6)', border: '1px solid rgba(207,185,145,0.1)' }}>
            
            {/* Purdue LDT Alignment Badges */}
            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: '8px', marginBottom: '24px', paddingBottom: '24px', borderBottom: '1px solid rgba(207,185,145,0.1)' }}>
              {[
                { school: '🏫 Pedagogy', purdue: 'EDCI 51300 / 53100', color: '#f59e0b', desc: 'Learning Theory' },
                { school: '🎨 Creation', purdue: 'EDCI 57200 / 56900', color: '#a78bfa', desc: 'Multimedia Design' },
                { school: '⚙️ Systems', purdue: 'EDCI 52800', color: '#34d399', desc: 'Human Performance' },
                { school: '🎭 Identity', purdue: 'EDCI 57700 / AECT', color: '#22d3ee', desc: 'Assessment & Ethics' }
              ].map(badge => (
                <div key={badge.school} style={{ background: 'rgba(24, 22, 18, 0.5)', padding: '12px', borderRadius: '8px', border: `1px solid ${badge.color}30` }}>
                  <div style={{ fontSize: '11px', fontFamily: "'Cinzel', serif", color: '#E2E8F0', marginBottom: '4px' }}>{badge.school}</div>
                  <div style={{ fontSize: '9px', fontFamily: "'JetBrains Mono', monospace", color: badge.color, fontWeight: 'bold' }}>{badge.purdue}</div>
                  <div style={{ fontSize: '10px', color: '#94a3b8', marginTop: '2px' }}>{badge.desc}</div>
                </div>
              ))}
            </div>

            {/* The User's Deck */}
            <div style={{ marginBottom: '40px' }}>
              <div style={{ fontSize: '20px', fontFamily: "'Cinzel', serif", color: '#a78bfa', marginBottom: '16px', borderBottom: '1px solid rgba(167,139,250,0.3)', paddingBottom: '8px', display: 'flex', justifyContent: 'space-between', alignItems: 'baseline' }}>
                <span>Your Active Deck</span>
                <span style={{ fontSize: '12px', color: '#94a3b8', fontFamily: "'JetBrains Mono', monospace" }}>{Object.keys(ldt?.hook_deck || {}).length} Spells Acquired</span>
              </div>
              
              {Object.keys(ldt?.hook_deck || {}).length === 0 ? (
                <div style={{ color: '#6B7280', fontSize: '14px', padding: '20px', fontStyle: 'italic', background: 'rgba(0,0,0,0.2)', borderRadius: '8px' }}>
                  No Hooks acquired yet. Speak to the Great Recycler to discover tools.
                </div>
              ) : (
                <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(220px, 1fr))', gap: '16px' }}>
                  {Object.values(ldt.hook_deck).map(hook => (
                    <div key={hook.id} style={{
                      background: 'linear-gradient(135deg, rgba(24,22,30,0.9), rgba(15,13,20,0.9))',
                      border: '1px solid rgba(167,139,250,0.4)', borderRadius: '8px', padding: '16px',
                      boxShadow: '0 8px 16px rgba(0,0,0,0.6)', 
                      transition: 'transform 0.2s', cursor: 'default'
                    }} className="hook-card-hover">
                      <div style={{ fontSize: '11px', color: '#a78bfa', textTransform: 'uppercase', marginBottom: '4px', letterSpacing: '1px' }}>{hook.school}</div>
                      <div style={{ fontSize: '16px', fontWeight: 'bold', color: '#E2E8F0', fontFamily: "'Cinzel', serif" }}>{hook.title}</div>
                      
                      <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: '12px', color: '#94a3b8', marginTop: '16px', marginBottom: '6px', fontFamily: "'JetBrains Mono', monospace" }}>
                        <span>Lvl {hook.level}</span>
                        <span>{hook.xp} / {hook.level * 100} XP</span>
                      </div>
                      
                      <div style={{ width: '100%', height: '6px', background: 'rgba(255,255,255,0.05)', borderRadius: '3px', overflow: 'hidden' }}>
                        <div style={{ 
                          width: `${Math.min((hook.xp / (hook.level * 100)) * 100, 100)}%`, 
                          height: '100%', background: 'linear-gradient(90deg, #8b5cf6, #d946ef)' 
                        }} />
                      </div>

                      <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: '10px', color: '#64748b', marginTop: '12px' }}>
                        <span>ID: {hook.id}</span>
                        <span>🐾 {hook.creeps_tamed} Creeps Tamed</span>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>

            <div style={{ fontSize: '20px', fontFamily: "'Cinzel', serif", color: '#E2E8F0', marginBottom: '16px', borderBottom: '1px solid rgba(255,255,255,0.1)', paddingBottom: '8px' }}>
              The Library of Hooks
            </div>

            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(200px, 1fr))', gap: '12px' }}>
              {HOOK_CATALOG.map(catHook => {
                const acquired = ldt?.hook_deck?.[catHook.id] || Object.values(ldt?.hook_deck || {}).find(h => h.title === catHook.title);
                return (
                  <div key={catHook.title} style={{
                    background: acquired ? 'linear-gradient(135deg, rgba(207,185,145,0.1), rgba(15,13,20,0.9))' : 'rgba(0,0,0,0.4)',
                    border: `1px solid ${acquired ? '#CFB991' : 'rgba(255,255,255,0.05)'}`, 
                    borderRadius: '6px', padding: '12px',
                    opacity: acquired ? 1 : 0.5,
                  }}>
                    <div style={{ fontSize: '9px', color: acquired ? '#a78bfa' : '#64748b', textTransform: 'uppercase', marginBottom: '4px' }}>{catHook.school}</div>
                    <div style={{ fontSize: '14px', fontWeight: 'bold', color: acquired ? '#E2E8F0' : '#94a3b8', fontFamily: "'Cinzel', serif" }}>
                      {catHook.title} {acquired && '✨'}
                    </div>
                    <div style={{ fontSize: '10px', color: '#64748b', marginTop: '8px' }}>{catHook.desc}</div>
                  </div>
                );
              })}
            </div>
            
            {/* The Trading Card System Tip */}
            <div style={{ marginTop: '32px', padding: '16px', borderRadius: '8px', background: 'rgba(34,211,238,0.05)', borderLeft: '4px solid #22d3ee' }}>
              <div style={{ fontFamily: "'Cinzel', serif", color: '#22d3ee', fontSize: '14px', marginBottom: '8px' }}>💡 Iron Road Trading Cards</div>
              <div style={{ fontSize: '12px', color: '#94a3b8', lineHeight: 1.5 }}>
                Did you know? The Iron Road treats these Hooks like cards in a deck. As you encounter Scope Creeps, the AI tags their strengths and weaknesses against specific Hooks!
              </div>
            </div>
          </div>
        )}

        {/* ═══ PLAYER IDENTITY TAB ═══ */}
        {activeTab === 'identity' && (
          <div style={{ marginTop: '24px' }}>
            <h2 style={{ fontFamily: "'Cinzel', serif", color: '#CFB991', letterSpacing: '2px', borderBottom: '1px solid rgba(207,185,145,0.2)', paddingBottom: '8px', marginBottom: '16px' }}>Player Handbook</h2>
            
            <div style={{ display: 'grid', gridTemplateColumns: 'minmax(200px, 1fr) 2fr', gap: '24px' }}>
              {/* Left Column: Visual Identity & Vitals */}
              <div>
                <div style={{ background: 'rgba(24, 22, 18, 0.72)', border: '1px solid rgba(207,185,145,0.3)', borderRadius: '8px', padding: '16px', marginBottom: '16px' }}>
                  <h3 style={{ fontFamily: "'Cinzel', serif", color: '#E2E8F0', marginTop: 0, fontSize: '16px' }}>Visual Identity</h3>
                  <p style={{ fontSize: '11px', color: '#94a3b8', marginBottom: '12px' }}>Drag an image here to upload your avatar.</p>
                  
                  <div 
                    style={{ 
                      width: '140px', height: '140px', border: '2px dashed #4B5563', borderRadius: '50%',
                      margin: '16px auto', display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center',
                      background: appearanceData ? `url(${appearanceData}) center/cover` : 'rgba(0,0,0,0.4)',
                      cursor: 'pointer', position: 'relative', overflow: 'hidden', boxShadow: 'inset 0 4px 12px rgba(0,0,0,0.5)'
                    }}
                    onDragOver={e => e.preventDefault()}
                    onDrop={e => {
                      e.preventDefault();
                      const file = e.dataTransfer.files[0];
                      if (file && file.type.startsWith('image/')) {
                        const reader = new FileReader();
                        reader.onload = ev => {
                          // Basic client-side resize to save base64 space
                          const img = new Image();
                          img.onload = () => {
                            const canvas = document.createElement('canvas');
                            const MAX = 256;
                            const scale = Math.min(MAX/img.width, MAX/img.height);
                            canvas.width = img.width * scale;
                            canvas.height = img.height * scale;
                            const ctx = canvas.getContext('2d');
                            ctx.drawImage(img, 0, 0, canvas.width, canvas.height);
                            setAppearanceData(canvas.toDataURL('image/jpeg', 0.8));
                          };
                          img.src = ev.target.result;
                        };
                        reader.readAsDataURL(file);
                      }
                    }}
                  >
                    {!appearanceData && <div style={{ textAlign: 'center', opacity: 0.6 }}>
                      <span style={{ fontSize: '36px', display: 'block', marginBottom: '8px' }}>👁️</span>
                    </div>}
                    <div style={{ position: 'absolute', bottom: 0, width: '100%', background: 'rgba(0,0,0,0.7)', padding: '6px', textAlign: 'center', fontSize: '9px', color: '#cbd5e1', textTransform: 'uppercase', letterSpacing: '1px' }}>Drop Image</div>
                  </div>
                </div>

                <div style={{ background: 'rgba(24, 22, 18, 0.72)', border: '1px solid rgba(167,139,250,0.3)', borderRadius: '8px', padding: '16px' }}>
                  <h3 style={{ fontFamily: "'Cinzel', serif", color: '#a78bfa', marginTop: 0, fontSize: '16px' }}>Audio Preferences</h3>
                  <div style={{ marginBottom: '12px' }}>
                    <label style={{ display: 'block', fontSize: '11px', color: '#94a3b8', marginBottom: '4px' }}>TTS Voice ID</label>
                    <select value={audioPrefs.voice_id} onChange={e => setAudioPrefs({...audioPrefs, voice_id: e.target.value})} style={s.input}>
                      <option value="am_adam">Adam (Male, Professional)</option>
                      <option value="am_michael">Michael (Male, Warm)</option>
                      <option value="am_fenrir">Fenrir (Male, Deep)</option>
                      <option value="am_echo">Echo (Male, Neutral)</option>
                      <option value="af_heart">Heart (Female, Clear)</option>
                      <option value="af_bella">Bella (Female, Soft)</option>
                    </select>
                  </div>
                  <div style={{ marginBottom: '12px' }}>
                    <label style={{ display: 'block', fontSize: '11px', color: '#94a3b8', marginBottom: '4px' }}>World Genre (Music/Tone)</label>
                    <select value={audioPrefs.genre} onChange={e => setAudioPrefs({...audioPrefs, genre: e.target.value})} style={s.input}>
                      <option value="Cinematic">Cinematic / Epic</option>
                      <option value="Lo-Fi">Lo-Fi / Chill</option>
                      <option value="Synthwave">Synthwave / Cyberpunk</option>
                      <option value="Steampunk">Steampunk / Acoustic</option>
                      <option value="Ambient">Minimalist / Ambient</option>
                    </select>
                  </div>
                  <label style={{ display: 'flex', alignItems: 'center', gap: '8px', fontSize: '12px', color: '#E2E8F0', cursor: 'pointer' }}>
                    <input type="checkbox" checked={audioPrefs.music_flow_enabled} onChange={e => setAudioPrefs({...audioPrefs, music_flow_enabled: e.target.checked})} />
                    Enable Music Flow
                  </label>
                </div>
              </div>

              {/* Right Column: Narrative & Stats */}
              <div>
                <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px', marginBottom: '16px' }}>
                  <div style={{ background: 'rgba(24, 22, 18, 0.72)', border: '1px solid rgba(207,185,145,0.3)', borderRadius: '8px', padding: '16px' }}>
                    <label style={{ display: 'block', fontFamily: "'Cinzel', serif", color: '#E2E8F0', fontSize: '14px', marginBottom: '8px' }}>Alignment</label>
                    <input 
                      type="text" 
                      value={alignmentText} 
                      onChange={e => setAlignmentText(e.target.value)} 
                      placeholder="e.g. Chaotic Constructor" 
                      style={s.input} 
                    />
                  </div>
                  <div style={{ background: 'rgba(24, 22, 18, 0.72)', border: '1px solid rgba(207,185,145,0.3)', borderRadius: '8px', padding: '16px' }}>
                    <label style={{ display: 'block', fontFamily: "'Cinzel', serif", color: '#E2E8F0', fontSize: '14px', marginBottom: '8px' }}>Locomotive Profile (Playstyle)</label>
                    <p style={{ fontSize: '10px', color: '#94a3b8', marginBottom: '8px' }}>Your natural approach to design and learning.</p>
                    <select value={locomotiveProfile} onChange={e => setLocomotiveProfile(e.target.value)} style={s.input}>
                      <option value="AnalyzerClass">🧠 The Sage (Analyzer Class)</option>
                      <option value="InterceptorExpress">⚔️ The Hero (Interceptor Express)</option>
                      <option value="AllTerrainSwitcher">🎭 The Jester (All-Terrain Switcher)</option>
                      <option value="ArmoredSupplyTrain">💚 The Caregiver (Armored Supply Train)</option>
                    </select>
                  </div>
                </div>

                <div style={{ background: 'rgba(24, 22, 18, 0.72)', border: '1px solid rgba(207,185,145,0.3)', borderRadius: '8px', padding: '16px' }}>
                  <label style={{ display: 'block', fontFamily: "'Cinzel', serif", color: '#E2E8F0', fontSize: '16px', marginBottom: '4px' }}>
                    Player Profile (Backstory)
                  </label>
                  <p style={{ fontSize: '12px', fontStyle: 'italic', color: '#CFB991', marginBottom: '8px', fontFamily: "'Crimson Text', serif" }}>
                    "We are the stories we tell ourselves."
                  </p>
                  <p style={{ fontSize: '11px', color: '#94a3b8', marginBottom: '12px' }}>
                    Describe your authentic self as the Instructional Designer working on this project. What drives your design?
                  </p>
                  <textarea
                    style={s.textarea}
                    value={backstoryText}
                    onChange={(e) => setBackstoryText(e.target.value)}
                    placeholder="E.g., An instructional designer focused on building engaging, active-learning environments rather than passive compliance training..."
                  />
                </div>

                <div style={{ display: 'flex', justifyContent: 'flex-end', marginTop: '16px' }}>
                  <button
                    onClick={saveBackstory}
                    disabled={savingBackstory}
                    style={{
                      background: savingBackstory ? '#4B5563' : '#CFB991',
                      color: '#1A1A1A', padding: '10px 24px', borderRadius: '6px',
                      fontFamily: "'JetBrains Mono', monospace", fontWeight: 'bold', border: 'none', 
                      cursor: savingBackstory ? 'not-allowed' : 'pointer', transition: 'background 0.2s',
                      boxShadow: '0 4px 12px rgba(207,185,145,0.2)'
                    }}
                  >
                    {savingBackstory ? 'Carving into Stone...' : 'Save Identity Form'}
                  </button>
                </div>
              </div>
            </div>
          </div>
        )}

      </div>
    </div>
  );
}

// ─── Styles ──────────────────────────────────────────────────────────────────────

const s = {
  container: {
    padding: '32px 24px', position: 'relative',
    maxWidth: '1200px', margin: '0 auto', fontFamily: "'Crimson Text', serif", fontSize: '14px',
    lineHeight: 1.5,
  },
  loading: { padding: '32px', fontFamily: "'Cinzel', serif", fontSize: '24px', color: '#CFB991' },
  loadingPulse: { animation: 'pulse 2s ease-in-out infinite', letterSpacing: '4px' },
  glassCard: {
    maxWidth: '1200px', margin: '0 auto',
    background: 'rgba(24, 22, 18, 0.88)', backdropFilter: 'blur(12px)',
    border: '1px solid rgba(207, 185, 145, 0.15)',
    boxShadow: '0 0 20px rgba(207, 185, 145, 0.05)',
    borderRadius: '10px', padding: '28px',
  },
  // PEARL — The User's Contract
  pearlSection: {
    marginBottom: '24px', padding: '20px',
    background: 'linear-gradient(135deg, rgba(167,139,250,0.08), rgba(167,139,250,0.02))',
    border: '1px solid rgba(167,139,250,0.2)',
    borderRadius: '10px',
  },
  pearlHeader: { marginBottom: '14px' },
  pearlSectionTitle: {
    fontSize: '13px', fontFamily: "'Cinzel', serif", color: '#a78bfa',
    letterSpacing: '3px', fontWeight: 700, textTransform: 'uppercase',
  },
  pearlContent: {},
  pearlSubject: {
    display: 'flex', alignItems: 'center', gap: '10px',
    marginBottom: '10px',
  },
  pearlVision: {
    fontSize: '14px', color: '#94a3b8', fontStyle: 'italic',
    fontFamily: "'Crimson Text', serif", lineHeight: 1.5,
    background: 'rgba(24,22,18,0.6)', padding: '10px 14px',
    borderRadius: '6px', borderLeft: '3px solid rgba(167,139,250,0.3)',
    marginBottom: '14px',
  },
  pearlScores: {
    display: 'flex', gap: '12px', alignItems: 'flex-end',
  },
  // Header
  header: {
    display: 'flex', justifyContent: 'space-between', alignItems: 'flex-end',
    borderBottom: '1px solid rgba(207, 185, 145, 0.2)',
    paddingBottom: '20px', marginBottom: '24px', flexWrap: 'wrap', gap: '16px',
  },
  title: {
    fontSize: '36px', fontFamily: "'Cinzel', serif", color: '#CFB991',
    textTransform: 'uppercase', letterSpacing: '5px',
    textShadow: '0 0 8px rgba(207, 185, 145, 0.4)', margin: 0,
  },
  subtitle: { fontSize: '13px', color: '#6B7280', marginTop: '6px' },
  gateLabel: {
    fontSize: '10px', textTransform: 'uppercase', color: '#8E7744',
    fontFamily: "'Cinzel', serif", letterSpacing: '3px', marginBottom: '4px',
  },
  gateValue: {
    fontSize: '22px', fontWeight: 'bold', color: '#10B981',
    fontFamily: "'JetBrains Mono', monospace",
  },
  // Agreement Banner
  // Finish Line
  finishLineSection: {
    marginBottom: '24px', padding: '20px',
    background: 'linear-gradient(135deg, rgba(16,185,129,0.06), rgba(16,185,129,0.01))',
    border: '1px solid rgba(16,185,129,0.15)', borderRadius: '10px',
  },
  finishLineHeader: {
    display: 'flex', alignItems: 'center', gap: '12px', marginBottom: '16px',
  },
  finishLineIcon: { fontSize: '28px' },
  finishLineTitle: {
    fontSize: '13px', fontFamily: "'Cinzel', serif", color: '#34d399',
    letterSpacing: '3px', fontWeight: 700, textTransform: 'uppercase',
  },
  finishLineSub: { fontSize: '11px', color: '#64748b', marginTop: '2px' },
  finishLineGrid: {
    display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(200px, 1fr))',
    gap: '10px',
  },
  finishLineCard: {
    display: 'flex', alignItems: 'flex-start', gap: '10px',
    padding: '12px 14px', borderRadius: '8px',
    background: 'rgba(24,22,18,0.72)',
    border: '1px solid rgba(207,185,145,0.1)',
    transition: 'opacity 0.3s',
  },
  // Maturation
  maturationSection: {
    marginBottom: '24px', padding: '20px',
    background: 'rgba(24,22,18,0.72)',
    border: '1px solid rgba(207,185,145,0.1)', borderRadius: '10px',
  },
  maturationTitle: {
    fontSize: '13px', fontFamily: "'Cinzel', serif", color: '#CFB991',
    letterSpacing: '3px', fontWeight: 700, textTransform: 'uppercase',
  },
  overallPill: {
    display: 'flex', flexDirection: 'column', alignItems: 'center',
    background: 'rgba(24,22,18,0.5)', border: '1px solid rgba(207,185,145,0.1)',
    borderRadius: '8px', padding: '6px 16px',
    fontFamily: "'JetBrains Mono', monospace",
  },
  maturationGrid: { display: 'flex', flexDirection: 'column', gap: '10px' },
  maturationRow: {
    display: 'grid', gridTemplateColumns: '160px 1fr 50px',
    alignItems: 'center', gap: '12px',
  },
  maturationLabel: {
    display: 'flex', alignItems: 'center', gap: '8px',
    fontSize: '11px', fontFamily: "'JetBrains Mono', monospace",
  },
  matBarTrack: {
    width: '100%', background: '#1A1916', height: '8px',
    borderRadius: '4px', overflow: 'hidden',
    border: '1px solid rgba(207,185,145,0.08)',
  },
  matPct: {
    fontSize: '12px', fontWeight: 'bold', textAlign: 'right',
    fontFamily: "'JetBrains Mono', monospace",
  },
  // Agreement
  agreementBanner: {
    display: 'flex', alignItems: 'center', gap: '16px',
    background: 'linear-gradient(135deg, rgba(207,185,145,0.08), rgba(207,185,145,0.02))',
    border: '1px solid rgba(207,185,145,0.15)',
    borderRadius: '10px', padding: '16px 20px', marginBottom: '28px',
    flexWrap: 'wrap',
  },
  agreementIcon: { fontSize: '32px', flexShrink: 0 },
  agreementTitle: {
    fontSize: '14px', fontFamily: "'Cinzel', serif", color: '#CFB991',
    letterSpacing: '3px', fontWeight: 700, textTransform: 'uppercase',
  },
  agreementSub: {
    fontSize: '12px', color: '#94a3b8', marginTop: '4px', lineHeight: 1.4,
    maxWidth: '600px',
  },
  progressPill: {
    marginLeft: 'auto', display: 'flex', alignItems: 'baseline', gap: '2px',
    fontFamily: "'JetBrains Mono', monospace", fontSize: '14px',
    background: 'rgba(16,185,129,0.08)', border: '1px solid rgba(16,185,129,0.2)',
    borderRadius: '8px', padding: '8px 16px',
  },
  // Station Grid
  stationGrid: {
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fill, minmax(320px, 1fr))',
    gap: '10px',
  },
  stationCard: {
    background: 'rgba(24, 22, 18, 0.72)',
    borderLeft: '3px solid #27272a',
    borderRadius: '8px', padding: '14px 16px',
    transition: 'opacity 0.3s',
  },
  stationTop: {
    display: 'flex', alignItems: 'center', gap: '10px', marginBottom: '6px',
  },
  stationNum: {
    fontSize: '20px', fontFamily: "'Cinzel', serif", fontWeight: 700,
    width: '28px', textAlign: 'center', flexShrink: 0,
  },
  stationName: {
    fontSize: '14px', fontWeight: 700, fontFamily: "'Cinzel', serif",
    letterSpacing: '1px', textTransform: 'uppercase',
  },
  stationMeta: {
    fontSize: '10px', color: '#64748b', fontFamily: "'JetBrains Mono', monospace",
    letterSpacing: '1px', marginTop: '1px',
  },
  chapterTitle: {
    fontSize: '12px', color: '#8E7744', fontStyle: 'italic',
    fontFamily: "'Crimson Text', serif", marginBottom: '4px',
  },
  pearlBadge: {
    display: 'inline-block', fontSize: '9px', fontFamily: "'JetBrains Mono', monospace",
    color: '#a78bfa', background: 'rgba(167,139,250,0.08)',
    border: '1px solid rgba(167,139,250,0.2)',
    padding: '2px 8px', borderRadius: '4px',
    letterSpacing: '1px', marginBottom: '4px', cursor: 'help',
  },
  agreementText: {
    fontSize: '12px', color: '#94a3b8', lineHeight: 1.4,
    paddingLeft: '38px',
  },
  questBlock: {
    marginTop: '10px', paddingTop: '8px', paddingLeft: '38px',
    borderTop: '1px solid rgba(245,158,11,0.15)',
  },
  questLabel: {
    fontSize: '9px', fontFamily: "'JetBrains Mono', monospace",
    color: '#f59e0b', letterSpacing: '2px', textTransform: 'uppercase',
    marginBottom: '6px', fontWeight: 700,
  },
  questItem: {
    display: 'flex', gap: '8px', alignItems: 'flex-start',
    fontSize: '11px', color: '#cbd5e1', marginBottom: '4px',
    fontFamily: "'Inter', sans-serif", lineHeight: 1.3,
  },
  // Bottom Grid
  bottomGrid: {
    display: 'grid', gridTemplateColumns: '1fr 1fr',
    gap: '16px', marginTop: '28px',
    paddingTop: '20px', borderTop: '1px solid rgba(207,185,145,0.1)',
  },
  bottomPanel: {
    background: 'rgba(24, 22, 18, 0.72)',
    padding: '20px', borderRadius: '10px',
    border: '1px solid rgba(207, 185, 145, 0.1)',
  },
  sectionTitle: {
    fontSize: '11px', fontFamily: "'Cinzel', serif", color: '#CFB991',
    letterSpacing: '2px', textTransform: 'uppercase',
    marginBottom: '16px', paddingBottom: '6px',
    borderBottom: '1px solid rgba(207, 185, 145, 0.1)', margin: '0 0 16px 0',
  },
  barHeader: {
    display: 'flex', justifyContent: 'space-between',
    fontSize: '10px', fontFamily: "'JetBrains Mono', monospace",
    fontWeight: 'bold', textTransform: 'uppercase', letterSpacing: '1px',
    marginBottom: '5px', color: '#94a3b8',
  },
  barTrack: {
    width: '100%', background: '#1A1916', height: '5px',
    borderRadius: '3px', overflow: 'hidden',
    border: '1px solid rgba(207, 185, 145, 0.08)',
  },
  cargoLabel: {
    color: '#64748b', fontSize: '10px', fontFamily: "'JetBrains Mono', monospace",
    textTransform: 'uppercase', letterSpacing: '2px',
  },
  scoreGrid: {
    display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)',
    gap: '8px', fontFamily: "'JetBrains Mono', monospace", textAlign: 'center',
  },
  scoreCard: {
    background: 'rgba(24, 22, 18, 0.5)', padding: '10px',
    borderRadius: '6px', border: '1px solid rgba(207, 185, 145, 0.08)',
  },
  scoreLabel: {
    fontSize: '8px', color: '#4B5563', textTransform: 'uppercase',
    letterSpacing: '2px', marginBottom: '4px',
  },
  scoreValue: { fontSize: '18px', color: '#E2E8F0' },
  // Vault
  vaultToggle: {
    width: '100%', background: 'rgba(34,211,238,0.1)', border: '1px solid rgba(34,211,238,0.3)',
    color: '#22d3ee', padding: '12px', borderRadius: '6px', fontFamily: "'Cinzel', serif", fontWeight: 'bold',
    cursor: 'pointer', display: 'flex', justifyContent: 'space-between', alignItems: 'center',
    transition: 'background 0.2s',
  },
  vaultGrid: {
    gap: '12px', marginTop: '12px', maxHeight: '350px', overflowY: 'auto',
    paddingRight: '8px',
  },
  emptyVault: {
    gridColumn: '1 / -1', textAlign: 'center', padding: '28px',
    color: '#4B5563', fontStyle: 'italic', fontFamily: "'Crimson Text', serif",
    background: 'rgba(24, 22, 18, 0.5)', border: '1px dashed rgba(207,185,145,0.15)',
    borderRadius: '6px',
  },
  artifactCard: {
    background: 'rgba(24, 22, 18, 0.72)', padding: '16px',
    borderLeft: '2px solid #CFB991', borderRadius: '6px',
    display: 'flex', flexDirection: 'column', justifyContent: 'space-between',
  },
  typeBadge: {
    fontSize: '8px', background: '#1A1916', color: '#CFB991',
    padding: '2px 6px', borderRadius: '4px', textTransform: 'uppercase',
    letterSpacing: '2px', border: '1px solid rgba(207,185,145,0.15)',
  },
  supraBadge: {
    fontSize: '8px', background: 'rgba(16,185,129,0.1)', color: '#10B981',
    padding: '2px 6px', borderRadius: '4px', textTransform: 'uppercase',
    letterSpacing: '2px', border: '1px solid rgba(16,185,129,0.3)',
  },
  reflection: {
    fontSize: '12px', color: '#6B7280', fontStyle: 'italic',
    fontFamily: "'Crimson Text', serif", overflow: 'hidden',
    textOverflow: 'ellipsis', display: '-webkit-box',
    WebkitLineClamp: 2, WebkitBoxOrient: 'vertical',
    background: 'rgba(18,17,14,0.5)', padding: '6px 8px',
    borderRadius: '4px', borderLeft: '1px solid rgba(207,185,145,0.1)',
  },
  artifactFooter: {
    display: 'flex', justifyContent: 'space-between', alignItems: 'center',
    fontSize: '10px', fontFamily: "'JetBrains Mono', monospace",
    borderTop: '1px solid rgba(207,185,145,0.08)',
    paddingTop: '8px', marginTop: '10px',
  },
  // Tab Navigation
  tabNav: {
    display: 'flex', gap: '8px', marginBottom: '24px',
    borderBottom: '1px solid rgba(207,185,145,0.1)', paddingBottom: '8px',
  },
  tabButton: {
    padding: '10px 20px', borderRadius: '8px', border: '1px solid transparent',
    background: 'transparent', color: '#94a3b8', cursor: 'pointer',
    fontFamily: "'JetBrains Mono', monospace", fontSize: '13px', letterSpacing: '1px',
    transition: 'all 0.2s', textTransform: 'uppercase',
  },
  tabActive: {
    background: 'rgba(207,185,145,0.08)', color: '#CFB991',
    borderColor: 'rgba(207,185,145,0.3)', fontWeight: 'bold',
  },
  tabInactive: {
    opacity: 0.7,
  },
  input: {
    width: '100%', padding: '10px',
    backgroundColor: 'rgba(0, 0, 0, 0.4)',
    color: '#CFB991', border: '1px solid rgba(207,185,145,0.3)',
    borderRadius: '4px', outline: 'none',
    fontFamily: "'JetBrains Mono', monospace", fontSize: '13px',
  },
  textarea: {
    width: '100%', height: '100px', padding: '10px',
    backgroundColor: 'rgba(0, 0, 0, 0.4)',
    color: '#CFB991', border: '1px solid rgba(207,185,145,0.3)',
    borderRadius: '4px', outline: 'none',
    fontFamily: "'JetBrains Mono', monospace", fontSize: '13px',
    resize: 'vertical',
  }
};

const HOOK_CATALOG = [
  { id: 'Socratic Interview', title: 'Socratic Interview', school: 'Pedagogy', desc: 'Asks WHY before you build.' },
  { id: '12-Station Quest', title: '12-Station Quest', school: 'Pedagogy', desc: 'ADDIECRAPEYE framework.' },
  { id: 'Bloom\'s Extraction', title: 'Bloom\'s Extraction', school: 'Pedagogy', desc: 'Tags cognitive progression.' },
  { id: 'Scope Creep Combat', title: 'Scope Creep Combat', school: 'Pedagogy', desc: 'Tames out-of-scope ideas.' },
  { id: 'Quality Scorecard', title: 'Quality Scorecard', school: 'Pedagogy', desc: 'QM and IBSTPI grading.' },
  { id: 'PEARL Review', title: 'PEARL Review', school: 'Pedagogy', desc: '5-dimension quality gate.' },
  { id: 'Design Doc Export', title: 'Design Doc Export', school: 'Pedagogy', desc: 'One-click Markdown export.' },
  { id: 'Image Generation', title: 'Image Generation', school: 'Creation', desc: 'vLLM-Omni text-to-image.' },
  { id: 'Music Composition', title: 'Music Composition', school: 'Creation', desc: 'MusicGPT OST creation.' },
  { id: 'Voice Narration', title: 'Voice Narration', school: 'Creation', desc: 'Kokoro TTS.' },
  { id: '30 Agentic Tools', title: 'Agentic Tools', school: 'Systems', desc: 'Shell, I/O, Web.' },
  { id: 'Vector Database', title: 'Vector Database', school: 'Systems', desc: 'RAG semantic search.' },
  { id: 'CowCatcher', title: 'CowCatcher', school: 'Systems', desc: 'Input sanitization engine.' },
  { id: 'Character Sheet', title: 'Character Sheet', school: 'Identity', desc: 'Living portfolio system.' },
  { id: 'Steam/Coal Resources', title: 'Steam & Coal', school: 'Identity', desc: 'Momentum and raw fuel.' },
  { id: 'Ghost Train Detection', title: 'Ghost Train', school: 'Identity', desc: 'Detects imposter syndrome.' },
];
