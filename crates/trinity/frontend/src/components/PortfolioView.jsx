import React, { useState } from 'react';
import { portfolioData } from '../data/portfolioData';
import { competencyDomains, flagshipProjects, supportingArtifacts } from '../data/competencyData';
import { technologyBadges } from '../data/technologyBadges';

/**
 * PortfolioView — The Graduation Portfolio
 * 
 * Natively embedded in Trinity. Three sub-tabs:
 * 1. Overview — ADDIECRAPEYE progress, PEARL scores, system stats
 * 2. Evidence — Flagship projects, artifacts, competency filtering
 * 3. Badges — IBSTPI supra-badge categories with drill-down
 * 
 * Vision: This is the template. Trinity generates one per user at graduation.
 */

const ADDIECRAPEYE_PHASES = [
  { id: 'A1', name: 'Analyze', icon: '🔍', color: '#22d3ee', desc: 'Learner & context analysis' },
  { id: 'D1', name: 'Design', icon: '📐', color: '#a78bfa', desc: 'Learning objectives & strategies' },
  { id: 'D2', name: 'Develop', icon: '🔧', color: '#f59e0b', desc: 'Content & asset creation' },
  { id: 'I',  name: 'Implement', icon: '🚀', color: '#34d399', desc: 'Deployment & delivery' },
  { id: 'E1', name: 'Evaluate', icon: '📊', color: '#fb923c', desc: 'Formative & summative assessment' },
  { id: 'C',  name: 'Contrast', icon: '⚖️', color: '#818cf8', desc: 'Visual hierarchy & emphasis' },
  { id: 'R',  name: 'Repetition', icon: '🔁', color: '#22d3ee', desc: 'Consistency & pattern recognition' },
  { id: 'A2', name: 'Alignment', icon: '📏', color: '#a78bfa', desc: 'Visual & cognitive coherence' },
  { id: 'P',  name: 'Proximity', icon: '🧲', color: '#f59e0b', desc: 'Grouping related elements' },
  { id: 'E2', name: 'Empathize', icon: '💚', color: '#34d399', desc: 'User-centered perspective' },
  { id: 'Y',  name: 'Yield', icon: '🌾', color: '#fb923c', desc: 'Reflective iteration & harvest' },
  { id: 'E3', name: 'Evolve', icon: '🧬', color: '#818cf8', desc: 'Continuous systemic improvement' },
];

const COMP_COLORS = {
  professional: '#818cf8',
  planning: '#3b82f6',
  design: '#a855f7',
  evaluation: '#22c55e',
  research: '#f59e0b',
};

const SUB_TABS = [
  { id: 'overview', label: '📜 Overview', title: 'ADDIECRAPEYE Journey' },
  { id: 'evidence', label: '📦 Evidence', title: 'Artifacts & Projects' },
  { id: 'badges', label: '🎓 Badges', title: 'Competency Portfolio' },
];

// ─────────────── STYLES ───────────────
const s = {
  container: {
    height: '100%', overflow: 'auto',
    background: 'linear-gradient(180deg, #0f0d0a 0%, #181612 100%)',
    fontFamily: "'Crimson Text', serif",
    color: '#E2E8F0',
  },
  header: {
    textAlign: 'center', padding: '40px 24px 24px',
    borderBottom: '1px solid rgba(207,185,145,0.1)',
  },
  headerTitle: {
    fontSize: '28px', fontFamily: "'Cinzel', serif",
    color: '#CFB991', marginBottom: '8px',
    letterSpacing: '3px', textTransform: 'uppercase',
  },
  headerSub: {
    fontSize: '14px', color: '#6B7280',
    fontStyle: 'italic', maxWidth: '500px', margin: '0 auto',
  },
  tabBar: {
    display: 'flex', justifyContent: 'center', gap: '4px',
    padding: '16px 24px', borderBottom: '1px solid rgba(207,185,145,0.08)',
    background: 'rgba(15,13,10,0.5)',
  },
  tab: (active) => ({
    padding: '10px 24px', borderRadius: '8px',
    fontSize: '13px', fontFamily: "'Cinzel', serif",
    letterSpacing: '1px', cursor: 'pointer',
    border: active ? '1px solid rgba(207,185,145,0.3)' : '1px solid transparent',
    background: active ? 'rgba(207,185,145,0.1)' : 'transparent',
    color: active ? '#CFB991' : '#6B7280',
    transition: 'all 0.2s',
  }),
  content: {
    padding: '24px', maxWidth: '1100px', margin: '0 auto',
  },
  // Phase grid
  phaseGrid: {
    display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(150px, 1fr))',
    gap: '10px', marginBottom: '32px',
  },
  phaseCard: (color) => ({
    padding: '14px 12px', borderRadius: '10px',
    background: 'rgba(24,22,18,0.72)',
    border: `1px solid ${color}25`,
    borderLeft: `3px solid ${color}`,
    textAlign: 'center', transition: 'all 0.2s',
  }),
  phaseIcon: { fontSize: '20px', marginBottom: '4px' },
  phaseName: { fontSize: '12px', fontWeight: 'bold', color: '#E2E8F0', letterSpacing: '1px' },
  phaseDesc: { fontSize: '10px', color: '#6B7280', marginTop: '2px' },
  // Stats
  statsGrid: {
    display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)',
    gap: '12px', marginBottom: '32px',
  },
  statCard: {
    textAlign: 'center', padding: '16px',
    background: 'rgba(24,22,18,0.72)',
    border: '1px solid rgba(207,185,145,0.1)',
    borderRadius: '10px',
  },
  statValue: { fontSize: '28px', fontWeight: 'bold', color: '#CFB991', fontFamily: "'JetBrains Mono', monospace" },
  statLabel: { fontSize: '10px', color: '#6B7280', textTransform: 'uppercase', letterSpacing: '2px', marginTop: '4px' },
  // Section
  sectionTitle: {
    fontSize: '18px', fontFamily: "'Cinzel', serif",
    color: '#CFB991', letterSpacing: '2px', textTransform: 'uppercase',
    marginBottom: '16px', paddingBottom: '8px',
    borderBottom: '1px solid rgba(207,185,145,0.1)',
  },
  // Flagship
  flagshipCard: {
    marginBottom: '24px', borderRadius: '12px',
    background: 'rgba(24,22,18,0.72)',
    border: '1px solid rgba(207,185,145,0.15)',
    overflow: 'hidden',
  },
  flagshipHeader: {
    padding: '20px 24px',
    background: 'linear-gradient(135deg, rgba(207,185,145,0.08) 0%, rgba(24,22,18,0.9) 100%)',
    borderBottom: '1px solid rgba(207,185,145,0.1)',
  },
  flagshipBadge: {
    display: 'inline-flex', alignItems: 'center', gap: '6px',
    fontSize: '10px', fontFamily: "'Cinzel', serif", letterSpacing: '2px',
    color: '#CFB991', textTransform: 'uppercase',
    padding: '4px 10px', borderRadius: '20px',
    background: 'rgba(207,185,145,0.1)', border: '1px solid rgba(207,185,145,0.2)',
    marginBottom: '8px',
  },
  flagshipTitle: { fontSize: '22px', fontWeight: 'bold', color: '#E2E8F0', margin: '4px 0' },
  flagshipSub: { fontSize: '13px', color: '#9CA3AF', fontStyle: 'italic' },
  flagshipBody: { padding: '20px 24px' },
  flagshipHook: {
    padding: '12px 16px', borderRadius: '8px',
    background: 'rgba(15,13,10,0.5)',
    borderLeft: '3px solid #CFB991',
    fontSize: '14px', color: '#D1D5DB', lineHeight: 1.6,
    marginBottom: '16px',
  },
  compBadge: (color) => ({
    display: 'inline-flex', alignItems: 'center',
    fontSize: '10px', padding: '2px 8px', borderRadius: '12px',
    background: `${color}15`, border: `1px solid ${color}30`,
    color: color, fontWeight: 600, marginRight: '6px', marginBottom: '4px',
  }),
  // Artifact card
  artifactCard: {
    padding: '16px', borderRadius: '10px',
    background: 'rgba(24,22,18,0.72)',
    border: '1px solid rgba(207,185,145,0.08)',
    transition: 'all 0.2s', cursor: 'pointer',
    textDecoration: 'none', display: 'block', color: 'inherit',
  },
  artifactTitle: { fontSize: '14px', fontWeight: 'bold', color: '#E2E8F0', marginBottom: '6px' },
  artifactDesc: { fontSize: '12px', color: '#9CA3AF', lineHeight: 1.5, marginBottom: '8px' },
  artifactType: {
    fontSize: '9px', color: '#CFB991', textTransform: 'uppercase',
    letterSpacing: '2px', fontFamily: "'JetBrains Mono', monospace",
    marginBottom: '8px',
  },
  artifactGrid: {
    display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(280px, 1fr))',
    gap: '12px', marginBottom: '32px',
  },
  // Badge category
  categoryHeader: {
    display: 'flex', alignItems: 'center', gap: '12px',
    marginBottom: '16px', paddingBottom: '12px',
    borderBottom: '1px solid rgba(207,185,145,0.08)',
  },
  categoryIcon: {
    width: '44px', height: '44px', borderRadius: '12px',
    background: 'rgba(24,22,18,0.9)', border: '1px solid rgba(207,185,145,0.15)',
    display: 'flex', alignItems: 'center', justifyContent: 'center',
    fontSize: '20px',
  },
  categoryTitle: { fontSize: '20px', fontWeight: 'bold', color: '#E2E8F0' },
  categoryDesc: { fontSize: '12px', color: '#6B7280', marginTop: '2px' },
  badgeGrid: {
    display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(240px, 1fr))',
    gap: '10px', marginBottom: '32px',
  },
  badgeCard: {
    padding: '14px', borderRadius: '10px',
    background: 'rgba(24,22,18,0.6)',
    border: '1px solid rgba(207,185,145,0.08)',
    transition: 'all 0.2s', cursor: 'pointer',
  },
  badgeTitle: { fontSize: '13px', fontWeight: 'bold', color: '#E2E8F0', marginBottom: '4px' },
  badgeStatus: (done) => ({
    fontSize: '9px', textTransform: 'uppercase', letterSpacing: '2px',
    color: done ? '#34d399' : '#6B7280',
    fontFamily: "'JetBrains Mono', monospace",
  }),
  // Filter bar
  filterBar: {
    display: 'flex', flexWrap: 'wrap', gap: '6px', marginBottom: '16px',
  },
  filterBtn: (active, color) => ({
    padding: '6px 14px', borderRadius: '20px',
    fontSize: '11px', fontWeight: 600, cursor: 'pointer',
    border: active ? `1px solid ${color || '#CFB991'}` : '1px solid rgba(75,85,99,0.3)',
    background: active ? `${color || '#CFB991'}15` : 'transparent',
    color: active ? (color || '#CFB991') : '#6B7280',
    transition: 'all 0.15s',
  }),
  // Tech badge
  techBadge: {
    display: 'flex', alignItems: 'center', gap: '12px',
    padding: '12px 14px', borderRadius: '10px',
    background: 'rgba(24,22,18,0.6)',
    border: '1px solid rgba(207,185,145,0.06)',
    transition: 'all 0.2s',
    textDecoration: 'none', color: 'inherit',
  },
  techIcon: {
    width: '40px', height: '40px', borderRadius: '8px',
    background: 'rgba(207,185,145,0.08)', border: '1px solid rgba(207,185,145,0.15)',
    display: 'flex', alignItems: 'center', justifyContent: 'center',
    fontSize: '18px', flexShrink: 0,
  },
  techTitle: { fontSize: '13px', fontWeight: 'bold', color: '#E2E8F0' },
  techDesc: { fontSize: '10px', color: '#6B7280', marginTop: '2px' },
  // Expanded badge detail
  badgeDetail: {
    padding: '20px', borderRadius: '10px',
    background: 'rgba(24,22,18,0.9)',
    border: '1px solid rgba(207,185,145,0.15)',
    marginBottom: '16px',
  },
  detailClose: {
    float: 'right', background: 'none', border: 'none',
    color: '#6B7280', cursor: 'pointer', fontSize: '16px',
    padding: '4px 8px',
  },
};

// ─────────────── OVERVIEW TAB ───────────────
function OverviewTab() {
  return (
    <div>
      <div style={s.sectionTitle}>THE 12 STATIONS OF ADDIECRAPEYE</div>
      <div style={s.phaseGrid}>
        {ADDIECRAPEYE_PHASES.map((phase) => (
          <div key={phase.id} style={s.phaseCard(phase.color)}>
            <div style={s.phaseIcon}>{phase.icon}</div>
            <div style={s.phaseName}>{phase.name}</div>
            <div style={s.phaseDesc}>{phase.desc}</div>
          </div>
        ))}
      </div>

      <div style={s.sectionTitle}>SYSTEM STATS</div>
      <div style={s.statsGrid}>
        {[
          { value: '194K', label: 'Lines of Rust' },
          { value: '179+', label: 'Passing Tests' },
          { value: '73', label: 'API Endpoints' },
          { value: '30', label: 'Agentic Tools' },
        ].map((stat) => (
          <div key={stat.label} style={s.statCard}>
            <div style={s.statValue}>{stat.value}</div>
            <div style={s.statLabel}>{stat.label}</div>
          </div>
        ))}
      </div>

      <div style={s.sectionTitle}>FRAMEWORKS IN PLAY</div>
      <div style={{ ...s.statsGrid, gridTemplateColumns: 'repeat(3, 1fr)' }}>
        {[
          { tag: 'ADDIE', desc: 'Instructional Design', cite: 'FSU, 1975', color: '#22c55e' },
          { tag: 'CRAP', desc: 'Visual Design Principles', cite: 'Williams, 1994', color: '#3b82f6' },
          { tag: 'EYE', desc: 'Vision & Iteration', cite: 'Atkinson, 2026', color: '#a855f7' },
        ].map((fw) => (
          <div key={fw.tag} style={{
            ...s.statCard,
            borderLeft: `3px solid ${fw.color}`,
            textAlign: 'left', padding: '16px 20px',
          }}>
            <div style={{ fontSize: '16px', fontWeight: 'bold', color: fw.color, fontFamily: "'JetBrains Mono', monospace" }}>{fw.tag}</div>
            <div style={{ fontSize: '13px', color: '#E2E8F0', marginTop: '4px' }}>{fw.desc}</div>
            <div style={{ fontSize: '10px', color: '#6B7280', marginTop: '2px', fontFamily: "'JetBrains Mono', monospace" }}>{fw.cite}</div>
          </div>
        ))}
      </div>

      <div style={s.sectionTitle}>TRINITY DELIVERS THREE PRODUCTS</div>
      <div style={{ ...s.statsGrid, gridTemplateColumns: 'repeat(3, 1fr)', marginBottom: '32px' }}>
        {[
          { tag: 'ID', emoji: '📋', name: 'LDT Portfolio', desc: 'The professional competency portfolio — proof of what you built', color: '#a78bfa' },
          { tag: 'AI', emoji: '📖', name: 'LitRPG Novel', desc: 'Your learning journey narrated as a lite novel — the story of becoming', color: '#22d3ee' },
          { tag: 'OS', emoji: '🎓', name: 'Capstone Product', desc: 'The course you build for others — your intellectual property', color: '#34d399' },
        ].map((d) => (
          <div key={d.tag} style={{
            ...s.statCard,
            borderLeft: `3px solid ${d.color}`,
            textAlign: 'left', padding: '18px 20px',
          }}>
            <div style={{ fontSize: '24px', marginBottom: '6px' }}>{d.emoji}</div>
            <div style={{ fontSize: '14px', fontWeight: 'bold', color: d.color, fontFamily: "'JetBrains Mono', monospace", marginBottom: '2px' }}>
              {d.tag} — {d.name}
            </div>
            <div style={{ fontSize: '12px', color: '#9CA3AF', lineHeight: 1.5 }}>{d.desc}</div>
          </div>
        ))}
      </div>

      <div style={{
        padding: '20px', borderRadius: '12px',
        background: 'linear-gradient(135deg, rgba(207,185,145,0.06) 0%, rgba(15,13,10,0.8) 100%)',
        border: '1px solid rgba(207,185,145,0.12)',
        textAlign: 'center',
      }}>
        <div style={{ fontSize: '14px', color: '#CFB991', fontFamily: "'Cinzel', serif", letterSpacing: '2px', marginBottom: '8px' }}>
          THE GRADUATION PORTFOLIO
        </div>
        <div style={{ fontSize: '13px', color: '#9CA3AF', lineHeight: 1.6, maxWidth: '600px', margin: '0 auto' }}>
          Every artifact below was created by traveling the Iron Road. This portfolio is the proof —
          not of what was memorized, but of what was <em style={{ color: '#CFB991' }}>built</em>.
          Trinity generates one for each Yardmaster who completes the journey.
        </div>
      </div>
    </div>
  );
}

// ─────────────── EVIDENCE TAB ───────────────
function EvidenceTab() {
  const [domainFilter, setDomainFilter] = useState(null);

  const filtered = domainFilter
    ? supportingArtifacts.filter(a => a.competencies.includes(domainFilter))
    : supportingArtifacts;

  return (
    <div>
      {/* Flagship Projects */}
      <div style={s.sectionTitle}>FLAGSHIP PROJECTS</div>
      {flagshipProjects.map((project) => (
        <div key={project.id} style={s.flagshipCard}>
          <div style={s.flagshipHeader}>
            <div style={s.flagshipBadge}>✦ Flagship Project</div>
            <div style={s.flagshipTitle}>{project.title}</div>
            <div style={s.flagshipSub}>{project.subtitle}</div>
          </div>
          <div style={s.flagshipBody}>
            <div style={s.flagshipHook}>{project.hook}</div>
            <div style={{ marginBottom: '12px' }}>
              <div style={{ fontSize: '10px', color: '#6B7280', textTransform: 'uppercase', letterSpacing: '2px', marginBottom: '6px' }}>
                IBSTPI Competencies
              </div>
              {project.competencies.map((comp, i) => (
                <div key={i} style={{ display: 'flex', alignItems: 'flex-start', gap: '8px', marginBottom: '6px' }}>
                  <span style={s.compBadge(COMP_COLORS[comp.domain])}>
                    {competencyDomains[comp.domain]?.title}
                  </span>
                  <span style={{ fontSize: '12px', color: '#9CA3AF', lineHeight: 1.5 }}>{comp.description}</span>
                </div>
              ))}
            </div>
            <div>
              <div style={{ fontSize: '10px', color: '#6B7280', textTransform: 'uppercase', letterSpacing: '2px', marginBottom: '6px' }}>
                Key Highlights
              </div>
              {project.highlights.map((h, i) => (
                <div key={i} style={{ fontSize: '12px', color: '#D1D5DB', padding: '4px 0', paddingLeft: '12px', borderLeft: '2px solid rgba(207,185,145,0.15)' }}>
                  {h}
                </div>
              ))}
            </div>
          </div>
        </div>
      ))}

      {/* Filter Bar */}
      <div style={s.sectionTitle}>SUPPORTING ARTIFACTS</div>
      <div style={s.filterBar}>
        <button
          style={s.filterBtn(!domainFilter, '#CFB991')}
          onClick={() => setDomainFilter(null)}
        >All</button>
        {Object.values(competencyDomains).map((domain) => (
          <button
            key={domain.id}
            style={s.filterBtn(domainFilter === domain.id, COMP_COLORS[domain.id])}
            onClick={() => setDomainFilter(domainFilter === domain.id ? null : domain.id)}
          >{domain.title}</button>
        ))}
      </div>

      <div style={s.artifactGrid}>
        {filtered.map((artifact) => (
          <a
            key={artifact.id}
            href={artifact.pdfPath}
            target="_blank"
            rel="noopener noreferrer"
            style={s.artifactCard}
            onMouseEnter={(e) => {
              e.currentTarget.style.borderColor = 'rgba(207,185,145,0.3)';
              e.currentTarget.style.transform = 'translateY(-2px)';
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.borderColor = 'rgba(207,185,145,0.08)';
              e.currentTarget.style.transform = 'translateY(0)';
            }}
          >
            <div style={s.artifactType}>{artifact.type}</div>
            <div style={s.artifactTitle}>{artifact.title}</div>
            <div style={s.artifactDesc}>{artifact.description}</div>
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '4px' }}>
              {artifact.competencies.map((comp) => (
                <span key={comp} style={s.compBadge(COMP_COLORS[comp])}>
                  {competencyDomains[comp]?.title}
                </span>
              ))}
            </div>
          </a>
        ))}
      </div>

      {/* Technology Badges */}
      <div style={s.sectionTitle}>LEGACY — LDT TECHNOLOGY BADGES</div>
      <div style={{ fontSize: '12px', color: '#6B7280', marginBottom: '16px', fontStyle: 'italic' }}>
        External credentials earned through Purdue's LDT program — the foundation upon which Trinity was built.
      </div>
      <div style={s.artifactGrid}>
        {technologyBadges.map((badge) => (
          <a
            key={badge.id}
            href={badge.link}
            target="_blank"
            rel="noopener noreferrer"
            style={s.techBadge}
            onMouseEnter={(e) => {
              e.currentTarget.style.borderColor = 'rgba(207,185,145,0.2)';
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.borderColor = 'rgba(207,185,145,0.06)';
            }}
          >
            <div style={s.techIcon}>🏅</div>
            <div>
              <div style={s.techTitle}>{badge.title}</div>
              <div style={s.techDesc}>{badge.description}</div>
            </div>
          </a>
        ))}
      </div>
    </div>
  );
}

// ─────────────── BADGES TAB ───────────────
function BadgesTab() {
  const [expandedBadge, setExpandedBadge] = useState(null);
  const { categories, badges } = portfolioData;

  const CATEGORY_ICONS = {
    foundations: '⚙️',
    planning: '🔍',
    design: '🎨',
    evaluation: '📊',
  };

  return (
    <div>
      <div style={{ fontSize: '13px', color: '#9CA3AF', marginBottom: '24px', lineHeight: 1.6 }}>
        A systematic collection of artifacts demonstrating mastery of the Purdue LDT Competencies,
        organized by IBSTPI domain. Each badge represents verified competency.
      </div>

      {categories.map((category) => {
        const categoryBadges = badges.filter(b => b.categoryId === category.id);
        return (
          <div key={category.id} style={{ marginBottom: '32px' }}>
            <div style={s.categoryHeader}>
              <div style={s.categoryIcon}>{CATEGORY_ICONS[category.id] || '📋'}</div>
              <div>
                <div style={s.categoryTitle}>{category.title}</div>
                <div style={s.categoryDesc}>{category.description}</div>
              </div>
            </div>

            <div style={s.badgeGrid}>
              {categoryBadges.map((badge) => (
                <div key={badge.id}>
                  <div
                    style={{
                      ...s.badgeCard,
                      borderColor: expandedBadge === badge.id ? 'rgba(207,185,145,0.3)' : 'rgba(207,185,145,0.08)',
                    }}
                    onClick={() => setExpandedBadge(expandedBadge === badge.id ? null : badge.id)}
                    onMouseEnter={(e) => {
                      if (expandedBadge !== badge.id) e.currentTarget.style.borderColor = 'rgba(207,185,145,0.15)';
                    }}
                    onMouseLeave={(e) => {
                      if (expandedBadge !== badge.id) e.currentTarget.style.borderColor = 'rgba(207,185,145,0.08)';
                    }}
                  >
                    <div style={s.badgeTitle}>{badge.title}</div>
                    <div style={s.badgeStatus(true)}>✓ Verified</div>
                    {badge.description && (
                      <div style={{ fontSize: '11px', color: '#6B7280', marginTop: '6px', lineHeight: 1.4 }}>
                        {badge.description}
                      </div>
                    )}
                  </div>

                  {expandedBadge === badge.id && (
                    <div style={s.badgeDetail}>
                      <button style={s.detailClose} onClick={() => setExpandedBadge(null)}>✕</button>
                      <div style={{ fontSize: '16px', fontWeight: 'bold', color: '#CFB991', marginBottom: '8px' }}>
                        {badge.title}
                      </div>
                      {badge.description && (
                        <div style={{ fontSize: '13px', color: '#D1D5DB', lineHeight: 1.6, marginBottom: '12px' }}>
                          {badge.description}
                        </div>
                      )}
                      {badge.artifacts && badge.artifacts.length > 0 && (
                        <div>
                          <div style={{ fontSize: '10px', color: '#6B7280', textTransform: 'uppercase', letterSpacing: '2px', marginBottom: '8px' }}>
                            Evidence Artifacts
                          </div>
                          {badge.artifacts.map((art, i) => (
                            <div key={i} style={{
                              padding: '8px 12px', marginBottom: '6px', borderRadius: '6px',
                              background: 'rgba(15,13,10,0.5)',
                              borderLeft: '2px solid rgba(207,185,145,0.2)',
                              fontSize: '12px', color: '#9CA3AF',
                            }}>
                              <div style={{ fontWeight: 'bold', color: '#E2E8F0', marginBottom: '2px' }}>{art.title}</div>
                              {art.reflection && <div style={{ fontStyle: 'italic' }}>"{art.reflection}"</div>}
                            </div>
                          ))}
                        </div>
                      )}
                    </div>
                  )}
                </div>
              ))}
            </div>
          </div>
        );
      })}
    </div>
  );
}

// ─────────────── MAIN COMPONENT ───────────────
export default function PortfolioView() {
  const [activeSubTab, setActiveSubTab] = useState('overview');

  return (
    <div style={s.container}>
      {/* Header */}
      <div style={s.header}>
        <div style={s.headerTitle}>The Graduation Portfolio</div>
        <div style={s.headerSub}>
          Built by doing, not by reading. Every artifact is evidence of a completed station on the Iron Road.
        </div>
      </div>

      {/* Sub-tab bar */}
      <div style={s.tabBar}>
        {SUB_TABS.map((tab) => (
          <button
            key={tab.id}
            style={s.tab(activeSubTab === tab.id)}
            onClick={() => setActiveSubTab(tab.id)}
            onMouseEnter={(e) => {
              if (activeSubTab !== tab.id) e.currentTarget.style.color = '#CFB991';
            }}
            onMouseLeave={(e) => {
              if (activeSubTab !== tab.id) e.currentTarget.style.color = '#6B7280';
            }}
          >
            {tab.label}
          </button>
        ))}
      </div>

      {/* Tab content */}
      <div style={s.content}>
        {activeSubTab === 'overview' && <OverviewTab />}
        {activeSubTab === 'evidence' && <EvidenceTab />}
        {activeSubTab === 'badges' && <BadgesTab />}
      </div>
    </div>
  );
}
