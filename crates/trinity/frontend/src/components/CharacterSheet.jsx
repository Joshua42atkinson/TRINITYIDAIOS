import React, { useState, useEffect } from 'react';

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

  useEffect(() => {
    Promise.all([
      fetch('/api/character').then(r => r.ok ? r.json() : Promise.reject(`Character: ${r.status}`)),
      fetch('/api/quest').then(r => r.ok ? r.json() : Promise.reject(`Quest: ${r.status}`)),
      fetch('/api/pearl').then(r => r.ok ? r.json() : null).catch(() => null),
    ])
      .then(([charData, questData, pearlData]) => {
        setSheet(charData);
        setQuest(questData);
        if (pearlData && !pearlData.error) setPearl(pearlData);
      })
      .catch(err => {
        console.error("Failed to load character/quest", err);
        setError(String(err));
      });
  }, []);

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

        {/* ═══ HOOK BOOK — THE PEARL OF THE CHARACTER SHEET ═══ */}
        <a
          href="#chariot:PROFESSOR.md"
          style={{
            display: 'block', textDecoration: 'none', marginBottom: '28px',
            padding: '24px 28px', borderRadius: '16px',
            background: 'linear-gradient(135deg, rgba(167,139,250,0.14) 0%, rgba(34,211,238,0.06) 50%, rgba(167,139,250,0.08) 100%)',
            border: '2px solid rgba(167,139,250,0.3)',
            position: 'relative', overflow: 'hidden',
            transition: 'all 0.3s ease',
            boxShadow: '0 4px 24px rgba(167,139,250,0.08)',
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.borderColor = 'rgba(167,139,250,0.6)';
            e.currentTarget.style.boxShadow = '0 0 32px rgba(167,139,250,0.2), 0 8px 32px rgba(0,0,0,0.3)';
            e.currentTarget.style.transform = 'translateY(-3px)';
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.borderColor = 'rgba(167,139,250,0.3)';
            e.currentTarget.style.boxShadow = '0 4px 24px rgba(167,139,250,0.08)';
            e.currentTarget.style.transform = 'translateY(0)';
          }}
        >
          {/* Top accent line */}
          <div style={{
            position: 'absolute', top: 0, left: 0, right: 0, height: '3px',
            background: 'linear-gradient(90deg, transparent, rgba(167,139,250,0.5), rgba(34,211,238,0.3), transparent)',
          }} />
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <div>
              <div style={{
                fontSize: '9px', fontFamily: "'JetBrains Mono', monospace",
                color: '#a78bfa', letterSpacing: '3px', textTransform: 'uppercase',
                marginBottom: '6px', opacity: 0.8,
              }}>
                YOUR SPELL BOOK
              </div>
              <div style={{
                fontSize: '20px', fontWeight: 800, color: '#E2E8F0',
                fontFamily: "'Cinzel', serif", letterSpacing: '2px',
                marginBottom: '6px',
              }}>
                📖 The Hook Book
              </div>
              <div style={{
                fontSize: '13px', color: '#9CA3AF', lineHeight: 1.5,
              }}>
                "What can Trinity do?" — <span style={{ color: '#a78bfa', fontWeight: 600 }}>37 Hooks</span> · <span style={{ color: '#22d3ee', fontWeight: 600 }}>4 Schools</span> · Instant access to every workflow
              </div>
            </div>
            <div style={{
              fontSize: '28px', color: '#a78bfa', opacity: 0.4,
              transition: 'opacity 0.2s',
            }}>→</div>
          </div>
        </a>

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
                    <div style={{ display: 'flex', gap: '6px', marginBottom: '8px' }}>
                      <span style={s.typeBadge}>{artifact.artifact_type}</span>
                      <span style={s.supraBadge}>{artifact.aligned_supra_badge}</span>
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
    display: 'flex', justifyContent: 'space-between', alignItems: 'center',
    width: '100%', padding: '12px 16px', borderRadius: '8px',
    background: 'rgba(24, 22, 18, 0.72)', border: '1px solid rgba(207,185,145,0.1)',
    color: '#CFB991', cursor: 'pointer', fontFamily: "'Cinzel', serif",
    fontSize: '12px', letterSpacing: '2px', textTransform: 'uppercase',
    transition: 'background 0.2s',
  },
  vaultGrid: {
    display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(260px, 1fr))',
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
};
