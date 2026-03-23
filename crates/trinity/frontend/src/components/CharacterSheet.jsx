import React, { useState, useEffect } from 'react';

/**
 * THE IRON ROAD CHARACTER SHEET
 * 
 * The "Bridge of Perception" — visualizes the CharacterSheet as an immersive
 * LitRPG HUD that maps Purdue LDT Portfolio requirements to game physics.
 * 
 * Glassmorphism aesthetic with Cinzel/Crimson Pro typography.
 * Fetches from GET /api/character.
 */
export default function CharacterSheet() {
  const [sheet, setSheet] = useState(null);
  const [error, setError] = useState(null);

  useEffect(() => {
    fetch('/api/character')
      .then(res => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .then(data => setSheet(data))
      .catch(err => {
        console.error("Heavilon Event: Failed to load sheet", err);
        setError(err.message);
      });
  }, []);

  if (error) return (
    <div style={styles.loading}>
      <span style={{ color: '#ef4444' }}>⚠ Heavilon Event: {error}</span>
    </div>
  );

  if (!sheet) return (
    <div style={styles.loading}>
      <span style={styles.loadingText}>Igniting the Firebox...</span>
    </div>
  );

  const ldt_portfolio = sheet.ldt_portfolio || {
    completed_challenges: 0,
    gate_review_status: 'Locked',
    artifact_vault: [],
    ibstpi_score: 0,
    atd_score: 0,
    aect_score: 0,
    qm_alignment_score: 0,
    heavilon_events_survived: 0,
    memorial_steps_climbed: 0,
  };
  const current_coal = sheet.current_coal ?? 100;
  const current_steam = sheet.current_steam ?? 0;
  const track_friction = sheet.track_friction ?? 0;
  const progressPercent = Math.min((ldt_portfolio.completed_challenges / 12) * 100, 100);
  const shadow_status = sheet.shadow_status || 'Clear';
  const cargo_slots = sheet.cargo_slots ?? 7;
  const session_intent = sheet.session_intent || null;

  // C1: UserClass emoji mapping (mirrors character_sheet.rs:L388-429)
  const classEmoji = {
    SubjectMatterExpert: '🧑‍🏫', InstructionalDesigner: '🎓',
    Stakeholder: '📊', Player: '🎮',
  };
  const classColors = {
    SubjectMatterExpert: '#f59e0b', InstructionalDesigner: '#a78bfa',
    Stakeholder: '#34d399', Player: '#22d3ee',
  };
  const userClass = sheet.user_class || 'Player';
  const locomotiveProfile = sheet.locomotive_profile || 'AnalyzerClass';

  return (
    <div style={styles.container}>
      <div style={styles.glassCard}>
        
        {/* HEADER: Identity */}
        <header style={styles.header}>
          <div>
            <div style={{ display: 'flex', alignItems: 'center', gap: '16px' }}>
              <h1 style={styles.title}>{sheet.alias}</h1>
              <span style={{
                fontSize: '14px', padding: '4px 12px', borderRadius: '4px',
                background: `${classColors[userClass] || '#22d3ee'}15`,
                border: `1px solid ${classColors[userClass] || '#22d3ee'}40`,
                color: classColors[userClass] || '#22d3ee',
                fontFamily: 'monospace', letterSpacing: '2px', textTransform: 'uppercase',
              }}>
                {classEmoji[userClass] || '🎮'} {userClass.replace(/([A-Z])/g, ' $1').trim()}
              </span>
            </div>
            <p style={styles.subtitle}>
              Level {sheet.resonance_level}
              {' '} — <span style={styles.profileLabel}>{locomotiveProfile.replace(/([A-Z])/g, ' $1').trim()}</span>
            </p>
          </div>
          <div style={styles.headerRight}>
            <p style={styles.gateLabel}>GATE REVIEW STATUS</p>
            <p style={styles.gateValue}>{ldt_portfolio.gate_review_status}</p>
            <p style={styles.xpLabel}>{sheet.total_xp} XP Total</p>
          </div>
        </header>

        {/* MAIN GRID */}
        <div style={styles.grid}>
          
          {/* LEFT COLUMN: Cognitive Logistics */}
          <div style={styles.leftCol}>
            <div style={styles.panel}>
              <h2 style={styles.sectionTitle}>⚙️ Cognitive Logistics</h2>
              
              {/* Coal Bar */}
              <div style={styles.barContainer}>
                <div style={styles.barHeader}>
                  <span style={{ color: '#fb923c' }}>🔥 Coal (Attention)</span>
                  <span>{Math.round(current_coal)}%</span>
                </div>
                <div style={styles.barTrack}>
                  <div style={{ ...styles.barFill, width: `${current_coal}%`, background: 'linear-gradient(90deg, #ea580c, #fb923c)' }} />
                </div>
              </div>

              {/* Steam Bar */}
              <div style={styles.barContainer}>
                <div style={styles.barHeader}>
                  <span style={{ color: '#22d3ee' }}>⚡ Steam (Momentum)</span>
                  <span>{Math.round(current_steam)} PSI</span>
                </div>
                <div style={styles.barTrack}>
                  <div style={{ ...styles.barFill, width: `${Math.min(current_steam, 100)}%`, background: 'linear-gradient(90deg, #0891b2, #22d3ee)' }} />
                </div>
              </div>

              {/* Friction Bar */}
              <div style={styles.barContainer}>
                <div style={styles.barHeader}>
                  <span style={{ color: '#ef4444' }}>Track Friction (Extraneous)</span>
                  <span>{Math.round(track_friction)}%</span>
                </div>
                <div style={styles.barTrack}>
                  <div style={{ ...styles.barFill, width: `${track_friction}%`, background: '#dc2626' }} />
                </div>
              </div>

              {/* R2: Cargo Slots — Miller's 7±2 */}
              <div style={{ marginTop: '16px', paddingTop: '16px', borderTop: '1px solid #27272a' }}>
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '8px' }}>
                  <span style={{ color: '#94a3b8', fontSize: '10px', fontFamily: 'monospace', textTransform: 'uppercase', letterSpacing: '2px' }}>Cargo Slots (Working Memory)</span>
                  <span style={{ color: '#cbd5e1', fontSize: '12px', fontFamily: 'monospace' }}>{cargo_slots} / 9</span>
                </div>
                <div style={{ display: 'flex', gap: '6px' }}>
                  {Array.from({ length: 9 }, (_, i) => (
                    <div key={i} style={{
                      width: '100%', height: '12px', borderRadius: '3px',
                      background: i < cargo_slots
                        ? 'linear-gradient(90deg, #f59e0b, #fbbf24)'
                        : 'rgba(39,39,42,0.8)',
                      border: i < cargo_slots
                        ? '1px solid rgba(245,158,11,0.4)'
                        : '1px solid #3f3f46',
                      boxShadow: i < cargo_slots ? '0 0 6px rgba(245,158,11,0.2)' : 'none',
                    }} />
                  ))}
                </div>
              </div>
            </div>

            {/* Intent Engineering */}
            <div style={styles.panel}>
              <h3 style={{ ...styles.sectionTitle, color: '#a78bfa' }}>🎯 The Firebox (Intent)</h3>
              {/* A2: Session Intent — "Today's Mission" */}
              {session_intent && (
                <div style={{
                  background: 'rgba(167,139,250,0.06)', padding: '10px 12px',
                  borderRadius: '4px', marginBottom: '12px',
                  border: '1px solid rgba(167,139,250,0.2)',
                  fontStyle: 'italic', fontSize: '13px', color: '#c4b5fd',
                }}>
                  "📡 {session_intent}"
                </div>
              )}
              <div style={styles.intentRow}>
                <span style={styles.intentLabel}>Posture</span>
                <span style={{ color: '#c4b5fd', fontWeight: 'bold' }}>{sheet.intent_posture}</span>
              </div>
              <div style={styles.intentRow}>
                <span style={styles.intentLabel}>Vulnerability</span>
                <span style={{ color: '#38bdf8' }}>{(sheet.vulnerability * 100).toFixed(0)}%</span>
              </div>
              <div style={{
                ...styles.groundingBadge,
                borderColor: sheet.grounding_complete ? 'rgba(16,185,129,0.4)' : 'rgba(239,68,68,0.4)',
                background: sheet.grounding_complete ? 'rgba(16,185,129,0.1)' : 'rgba(239,68,68,0.1)',
                color: sheet.grounding_complete ? '#10b981' : '#ef4444',
              }}>
                {sheet.grounding_complete ? '⚓ Grounding Active' : '⚠ Grounding Required'}
              </div>

              {/* C2: Shadow Status — Ghost Train indicator */}
              <div style={{
                display: 'flex', justifyContent: 'space-between', alignItems: 'center',
                background: shadow_status === 'Clear' ? 'rgba(16,185,129,0.08)' :
                            shadow_status === 'Processed' ? 'rgba(139,92,246,0.08)' :
                            'rgba(239,68,68,0.08)',
                padding: '8px 12px', borderRadius: '4px', marginTop: '8px',
                fontFamily: 'monospace', fontSize: '13px',
                border: `1px solid ${shadow_status === 'Clear' ? 'rgba(16,185,129,0.3)' :
                                     shadow_status === 'Processed' ? 'rgba(139,92,246,0.3)' :
                                     'rgba(239,68,68,0.3)'}`,
              }}>
                <span style={{ color: '#64748b', textTransform: 'uppercase', letterSpacing: '2px', fontSize: '10px' }}>Shadow</span>
                <span style={{
                  color: shadow_status === 'Clear' ? '#10b981' :
                         shadow_status === 'Stirring' ? '#f59e0b' :
                         shadow_status === 'Active' ? '#ef4444' : '#8b5cf6',
                  fontWeight: 'bold',
                }}>
                  {shadow_status === 'Clear' ? '👻 Clear' :
                   shadow_status === 'Stirring' ? '🌫️ Stirring' :
                   shadow_status === 'Active' ? '🚂 Ghost Train Active' : '✨ Processed'}
                </span>
              </div>
            </div>
          </div>

          {/* RIGHT COLUMN: LDT Portfolio */}
          <div style={styles.rightCol}>
            
            {/* Progress Bar */}
            <div style={{ ...styles.panel, borderColor: 'rgba(16,185,129,0.3)' }}>
              <h2 style={{ ...styles.sectionTitle, color: '#34d399' }}>🏆 The Iron Road: LDT Portfolio</h2>
              
              <div style={styles.progressHeader}>
                <span style={{ color: '#94a3b8', fontSize: '12px', textTransform: 'uppercase', letterSpacing: '2px' }}>Gate Review Progression</span>
                <span style={{ color: '#34d399', fontWeight: 'bold', fontSize: '20px' }}>{ldt_portfolio.completed_challenges} / 12 Artifacts</span>
              </div>
              <div style={{ ...styles.barTrack, height: '16px' }}>
                <div style={{ 
                  ...styles.barFill, 
                  height: '16px',
                  width: `${progressPercent}%`, 
                  background: 'linear-gradient(90deg, #047857, #34d399)',
                  transition: 'width 1s ease-out',
                }} />
              </div>

              {/* Score Grid */}
              {/* R3 + A1: Full score grid — 6 cards (3×2) */}
              <div style={{ ...styles.scoreGrid, gridTemplateColumns: 'repeat(3, 1fr)' }}>
                <div style={styles.scoreCard}>
                  <p style={styles.scoreLabel}>QM Score</p>
                  <p style={{ ...styles.scoreValue, color: '#22d3ee' }}>{ldt_portfolio.qm_alignment_score.toFixed(1)}</p>
                </div>
                <div style={styles.scoreCard}>
                  <p style={styles.scoreLabel}>IBSTPI</p>
                  <p style={styles.scoreValue}>{ldt_portfolio.ibstpi_score.toFixed(0)}%</p>
                </div>
                <div style={styles.scoreCard}>
                  <p style={styles.scoreLabel}>ATD</p>
                  <p style={{ ...styles.scoreValue, color: '#a78bfa' }}>{ldt_portfolio.atd_score.toFixed(0)}%</p>
                </div>
                <div style={styles.scoreCard}>
                  <p style={styles.scoreLabel}>AECT Ethics</p>
                  <p style={{ ...styles.scoreValue, color: '#34d399' }}>🛡️ {ldt_portfolio.aect_score.toFixed(0)}%</p>
                </div>
                <div style={styles.scoreCard}>
                  <p style={styles.scoreLabel}>Heavilon Events</p>
                  <p style={{ ...styles.scoreValue, color: '#fb923c' }}>{ldt_portfolio.heavilon_events_survived}</p>
                </div>
                <div style={styles.scoreCard}>
                  <p style={styles.scoreLabel}>Memorial Steps</p>
                  <p style={{ ...styles.scoreValue, color: '#f59e0b' }}>{ldt_portfolio.memorial_steps_climbed}/17</p>
                </div>
              </div>
            </div>

            {/* Artifact Vault */}
            <div>
              <h3 style={styles.sectionTitle}>📖 Subconscious Inventory (Vault)</h3>
              <div style={styles.vaultGrid}>
                {ldt_portfolio.artifact_vault.length === 0 ? (
                  <div style={styles.emptyVault}>
                    The vault is empty. Lay some iron, Yardmaster.
                  </div>
                ) : (
                  ldt_portfolio.artifact_vault.map((artifact, idx) => (
                    <div key={idx} style={styles.artifactCard}>
                      <h4 style={styles.artifactTitle}>{artifact.title}</h4>
                      <div style={styles.badgeRow}>
                        <span style={styles.typeBadge}>{artifact.artifact_type}</span>
                        <span style={styles.supraBadge}>{artifact.aligned_supra_badge}</span>
                      </div>
                      <p style={styles.reflection}>"{artifact.reflection_journal}"</p>
                      <div style={styles.artifactFooter}>
                        <span style={{ color: '#22d3ee' }}>QM: {artifact.qm_score}/100</span>
                        {artifact.aect_ethics_cleared ? (
                          <span style={{ color: '#10b981' }}>✓ AECT Cleared</span>
                        ) : (
                          <span style={{ color: '#ef4444' }}>Ethics Review Req</span>
                        )}
                      </div>
                    </div>
                  ))
                )}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

// ─── Styles (Aligned to tokens.css design system) ────────────────────────────
// Uses: --gold (#CFB991), --bg (#131210), --bg-card, --border-glass, --font-*

const styles = {
  container: {
    minHeight: '100vh',
    background: '#131210',
    color: '#E2E8F0',
    padding: '32px',
    fontFamily: "'Inter', sans-serif",
    fontSize: '14px',
    lineHeight: 1.5,
    overflowY: 'auto',
  },
  loading: {
    padding: '32px',
    fontFamily: "'Cinzel', serif",
    fontSize: '24px',
    color: '#CFB991',
  },
  loadingText: {
    animation: 'pulse 2s ease-in-out infinite',
    letterSpacing: '4px',
  },
  glassCard: {
    maxWidth: '1400px',
    margin: '0 auto',
    background: 'rgba(24, 22, 18, 0.88)',
    backdropFilter: 'blur(12px)',
    border: '1px solid rgba(207, 185, 145, 0.15)',
    boxShadow: '0 0 20px rgba(207, 185, 145, 0.05)',
    borderRadius: '10px',
    padding: '32px',
  },
  header: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'flex-end',
    borderBottom: '1px solid rgba(207, 185, 145, 0.2)',
    paddingBottom: '24px',
    marginBottom: '32px',
    flexWrap: 'wrap',
    gap: '16px',
  },
  title: {
    fontSize: '42px',
    fontFamily: "'Cinzel', serif",
    color: '#CFB991',
    textTransform: 'uppercase',
    letterSpacing: '6px',
    textShadow: '0 0 8px rgba(207, 185, 145, 0.4)',
    margin: 0,
  },
  subtitle: {
    fontSize: '14px',
    fontFamily: "'Inter', sans-serif",
    color: '#6B7280',
    marginTop: '8px',
  },
  profileLabel: { color: '#8E7744' },
  headerRight: { textAlign: 'right' },
  gateLabel: {
    fontSize: '11px',
    textTransform: 'uppercase',
    color: '#8E7744',
    fontFamily: "'Cinzel', serif",
    letterSpacing: '3px',
    marginBottom: '4px',
  },
  gateValue: {
    fontSize: '24px',
    fontWeight: 'bold',
    color: '#10B981',
    fontFamily: "'JetBrains Mono', monospace",
  },
  xpLabel: {
    fontSize: '12px',
    color: '#4B5563',
    fontFamily: "'JetBrains Mono', monospace",
    marginTop: '4px',
  },
  grid: {
    display: 'grid',
    gridTemplateColumns: '1fr 2fr',
    gap: '40px',
  },
  leftCol: { display: 'flex', flexDirection: 'column', gap: '24px' },
  rightCol: { display: 'flex', flexDirection: 'column', gap: '24px' },
  panel: {
    background: 'rgba(24, 22, 18, 0.72)',
    padding: '24px',
    borderRadius: '10px',
    border: '1px solid rgba(207, 185, 145, 0.1)',
  },
  sectionTitle: {
    fontSize: '13px',
    fontFamily: "'Cinzel', serif",
    color: '#CFB991',
    letterSpacing: '2px',
    textTransform: 'uppercase',
    marginBottom: '24px',
    paddingBottom: '8px',
    borderBottom: '1px solid rgba(207, 185, 145, 0.1)',
    margin: '0 0 24px 0',
  },
  barContainer: { marginBottom: '20px' },
  barHeader: {
    display: 'flex',
    justifyContent: 'space-between',
    fontSize: '11px',
    fontFamily: "'JetBrains Mono', monospace",
    fontWeight: 'bold',
    textTransform: 'uppercase',
    letterSpacing: '1px',
    marginBottom: '8px',
  },
  barTrack: {
    width: '100%',
    background: '#1A1916',
    height: '6px',
    borderRadius: '3px',
    overflow: 'hidden',
    border: '1px solid rgba(207, 185, 145, 0.08)',
  },
  barFill: {
    height: '6px',
    transition: 'width 0.5s ease-out',
    borderRadius: '3px',
  },
  intentRow: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    background: 'rgba(24, 22, 18, 0.5)',
    padding: '8px 12px',
    borderRadius: '6px',
    marginBottom: '8px',
    fontFamily: "'JetBrains Mono', monospace",
    fontSize: '13px',
  },
  intentLabel: {
    color: '#4B5563',
    textTransform: 'uppercase',
    letterSpacing: '2px',
    fontSize: '10px',
  },
  groundingBadge: {
    padding: '12px',
    borderRadius: '6px',
    border: '1px solid',
    textAlign: 'center',
    marginTop: '8px',
    fontWeight: 'bold',
    fontSize: '14px',
  },
  progressHeader: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'flex-end',
    marginBottom: '12px',
    fontFamily: "'JetBrains Mono', monospace",
  },
  scoreGrid: {
    display: 'grid',
    gridTemplateColumns: 'repeat(4, 1fr)',
    gap: '12px',
    marginTop: '24px',
    paddingTop: '16px',
    fontFamily: "'JetBrains Mono', monospace",
    textAlign: 'center',
  },
  scoreCard: {
    background: 'rgba(24, 22, 18, 0.5)',
    padding: '12px',
    borderRadius: '6px',
    border: '1px solid rgba(207, 185, 145, 0.08)',
  },
  scoreLabel: {
    fontSize: '9px',
    color: '#4B5563',
    textTransform: 'uppercase',
    letterSpacing: '2px',
    marginBottom: '4px',
  },
  scoreValue: {
    fontSize: '20px',
    color: '#E2E8F0',
  },
  vaultGrid: {
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fill, minmax(280px, 1fr))',
    gap: '16px',
    maxHeight: '400px',
    overflowY: 'auto',
    paddingRight: '8px',
  },
  emptyVault: {
    gridColumn: '1 / -1',
    textAlign: 'center',
    padding: '32px',
    color: '#4B5563',
    fontStyle: 'italic',
    fontFamily: "'Crimson Text', serif",
    background: 'rgba(24, 22, 18, 0.5)',
    border: '1px dashed rgba(207, 185, 145, 0.15)',
    borderRadius: '6px',
  },
  artifactCard: {
    background: 'rgba(24, 22, 18, 0.72)',
    padding: '20px',
    borderLeft: '2px solid #CFB991',
    borderRadius: '6px',
    display: 'flex',
    flexDirection: 'column',
    justifyContent: 'space-between',
  },
  artifactTitle: {
    fontWeight: 'bold',
    color: '#E2E8F0',
    fontSize: '16px',
    margin: '0 0 8px 0',
  },
  badgeRow: {
    display: 'flex',
    gap: '8px',
    marginBottom: '12px',
  },
  typeBadge: {
    fontSize: '9px',
    background: '#1A1916',
    color: '#CFB991',
    padding: '2px 8px',
    borderRadius: '4px',
    textTransform: 'uppercase',
    letterSpacing: '2px',
    border: '1px solid rgba(207, 185, 145, 0.15)',
  },
  supraBadge: {
    fontSize: '9px',
    background: 'rgba(16, 185, 129, 0.1)',
    color: '#10B981',
    padding: '2px 8px',
    borderRadius: '4px',
    textTransform: 'uppercase',
    letterSpacing: '2px',
    border: '1px solid rgba(16, 185, 129, 0.3)',
  },
  reflection: {
    fontSize: '13px',
    color: '#6B7280',
    fontStyle: 'italic',
    fontFamily: "'Crimson Text', serif",
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    display: '-webkit-box',
    WebkitLineClamp: 3,
    WebkitBoxOrient: 'vertical',
    background: 'rgba(18, 17, 14, 0.5)',
    padding: '8px',
    borderRadius: '4px',
    borderLeft: '1px solid rgba(207, 185, 145, 0.1)',
  },
  artifactFooter: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    fontSize: '11px',
    fontFamily: "'JetBrains Mono', monospace",
    borderTop: '1px solid rgba(207, 185, 145, 0.08)',
    paddingTop: '12px',
    marginTop: '16px',
  },
};
