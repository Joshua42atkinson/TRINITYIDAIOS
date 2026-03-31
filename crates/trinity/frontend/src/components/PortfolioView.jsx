import React, { useState } from 'react';
import { portfolioData } from '../data/portfolioData';
import { competencyDomains, flagshipProjects, supportingArtifacts } from '../data/competencyData';
import { technologyBadges } from '../data/technologyBadges';

/**
 * PortfolioView — The Graduation Portfolio
 * 
 * This is the TEMPLATE portfolio that Trinity generates for each user.
 * Joshua's rich LDT portfolio lives at ldtatkinson.com (the host).
 * This component is the USER's view — what any graduate earns.
 * 
 * Trinity delivers: ID (this portfolio) · AI (LitRPG novel) · OS (their capstone product)
 */

const ADDIECRAPEYE_PHASES = [
  { id: 'A1', name: 'Analyze',    icon: '🔍', color: '#22d3ee', group: 'ADDIE' },
  { id: 'D1', name: 'Design',     icon: '📐', color: '#a78bfa', group: 'ADDIE' },
  { id: 'D2', name: 'Develop',    icon: '🔧', color: '#f59e0b', group: 'ADDIE' },
  { id: 'I',  name: 'Implement',  icon: '🚀', color: '#34d399', group: 'ADDIE' },
  { id: 'E1', name: 'Evaluate',   icon: '📊', color: '#fb923c', group: 'ADDIE' },
  { id: 'C',  name: 'Contrast',   icon: '⚖️', color: '#818cf8', group: 'CRAP' },
  { id: 'R',  name: 'Repetition', icon: '🔁', color: '#22d3ee', group: 'CRAP' },
  { id: 'A2', name: 'Alignment',  icon: '📏', color: '#a78bfa', group: 'CRAP' },
  { id: 'P',  name: 'Proximity',  icon: '🧲', color: '#f59e0b', group: 'CRAP' },
  { id: 'E2', name: 'Envision',   icon: '🔮', color: '#34d399', group: 'EYE' },
  { id: 'Y',  name: 'Yoke',       icon: '🤝', color: '#fb923c', group: 'EYE' },
  { id: 'E3', name: 'Evolve',     icon: '🧬', color: '#818cf8', group: 'EYE' },
];

const COMP_COLORS = {
  professional: '#818cf8', planning: '#3b82f6',
  design: '#a855f7', evaluation: '#22c55e', research: '#f59e0b',
};

const GROUP_COLORS = { ADDIE: '#22c55e', CRAP: '#3b82f6', EYE: '#a855f7' };

const SUB_TABS = [
  { id: 'overview', label: '📜 Overview' },
  { id: 'evidence', label: '📦 Evidence' },
  { id: 'badges',   label: '🎓 Badges' },
];

/* ═══════════════════════ CSS KEYFRAMES (injected once) ═══════════════════════ */
const STYLE_ID = 'portfolio-view-styles';
if (typeof document !== 'undefined' && !document.getElementById(STYLE_ID)) {
  const style = document.createElement('style');
  style.id = STYLE_ID;
  style.textContent = `
    @keyframes pv-shimmer {
      0% { background-position: -200% 0; }
      100% { background-position: 200% 0; }
    }
    @keyframes pv-fadeIn {
      from { opacity: 0; transform: translateY(12px); }
      to { opacity: 1; transform: translateY(0); }
    }
    @keyframes pv-glow {
      0%, 100% { box-shadow: 0 0 12px rgba(207,185,145,0.05); }
      50% { box-shadow: 0 0 24px rgba(207,185,145,0.12); }
    }
    .pv-card { animation: pv-fadeIn 0.35s ease-out both; }
    .pv-card:hover { transform: translateY(-3px) !important; box-shadow: 0 8px 32px rgba(0,0,0,0.4), 0 0 20px rgba(207,185,145,0.08) !important; }
    .pv-phase:hover { transform: scale(1.04); box-shadow: 0 4px 20px rgba(0,0,0,0.3); }
    .pv-badge:hover { border-color: rgba(207,185,145,0.3) !important; transform: translateY(-2px); }
    .pv-stat { animation: pv-glow 3s ease-in-out infinite; }
    .pv-filter:hover { transform: scale(1.03); }
    .pv-tech:hover { border-color: rgba(207,185,145,0.25) !important; background: rgba(207,185,145,0.04) !important; }
  `;
  document.head.appendChild(style);
}

/* ═══════════════════════ OVERVIEW TAB ═══════════════════════ */
function OverviewTab() {
  return (
    <div>
      {/* ── ADDIECRAPEYE Phase Grid ── */}
      <div style={{
        display: 'flex', alignItems: 'center', gap: '12px', marginBottom: '20px',
      }}>
        <div style={{
          fontFamily: "'Cinzel', serif", fontSize: '15px', color: '#CFB991',
          letterSpacing: '3px', textTransform: 'uppercase',
        }}>The 12 Stations</div>
        <div style={{ flex: 1, height: '1px', background: 'linear-gradient(90deg, rgba(207,185,145,0.2) 0%, transparent 100%)' }} />
        <div style={{ display: 'flex', gap: '6px' }}>
          {Object.entries(GROUP_COLORS).map(([g, c]) => (
            <span key={g} style={{
              fontSize: '9px', padding: '2px 8px', borderRadius: '10px',
              background: `${c}12`, border: `1px solid ${c}30`, color: c,
              fontFamily: "'JetBrains Mono', monospace", letterSpacing: '1px',
            }}>{g}</span>
          ))}
        </div>
      </div>

      <div style={{
        display: 'grid', gridTemplateColumns: 'repeat(6, 1fr)',
        gap: '8px', marginBottom: '36px',
      }}>
        {ADDIECRAPEYE_PHASES.map((phase, i) => (
          <div key={phase.id} className="pv-phase" style={{
            padding: '16px 10px', borderRadius: '12px', textAlign: 'center',
            background: `linear-gradient(145deg, rgba(24,22,18,0.85) 0%, ${phase.color}08 100%)`,
            border: `1px solid ${phase.color}20`,
            borderBottom: `3px solid ${phase.color}50`,
            transition: 'all 0.25s ease',
            animationDelay: `${i * 40}ms`,
            cursor: 'default',
          }}>
            <div style={{ fontSize: '22px', marginBottom: '6px', filter: 'drop-shadow(0 2px 4px rgba(0,0,0,0.3))' }}>{phase.icon}</div>
            <div style={{
              fontSize: '11px', fontWeight: 700, color: phase.color,
              letterSpacing: '0.5px', fontFamily: "'JetBrains Mono', monospace",
            }}>{phase.name}</div>
            <div style={{
              fontSize: '8px', color: '#6B7280', marginTop: '3px',
              textTransform: 'uppercase', letterSpacing: '1px',
            }}>{phase.group}</div>
          </div>
        ))}
      </div>

      {/* ── System Stats ── */}
      <div style={{
        display: 'flex', alignItems: 'center', gap: '12px', marginBottom: '16px',
      }}>
        <div style={{
          fontFamily: "'Cinzel', serif", fontSize: '15px', color: '#CFB991',
          letterSpacing: '3px', textTransform: 'uppercase',
        }}>System Telemetry</div>
        <div style={{ flex: 1, height: '1px', background: 'linear-gradient(90deg, rgba(207,185,145,0.2) 0%, transparent 100%)' }} />
      </div>

      <div style={{
        display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)',
        gap: '12px', marginBottom: '36px',
      }}>
        {[
          { value: '200K+', label: 'Total LOC', icon: '⟨/⟩', color: '#22d3ee' },
          { value: '24K', label: 'Core Rust', icon: '⚙️', color: '#34d399' },
          { value: '73', label: 'API Endpoints', icon: '⊕', color: '#f59e0b' },
          { value: '30', label: 'Agentic Tools', icon: '⊘', color: '#a78bfa' },
        ].map((stat) => (
          <div key={stat.label} className="pv-stat" style={{
            textAlign: 'center', padding: '20px 16px',
            background: 'linear-gradient(145deg, rgba(24,22,18,0.9) 0%, rgba(24,22,18,0.6) 100%)',
            border: '1px solid rgba(207,185,145,0.08)',
            borderRadius: '14px', position: 'relative', overflow: 'hidden',
          }}>
            <div style={{
              position: 'absolute', top: 0, left: 0, right: 0, height: '2px',
              background: `linear-gradient(90deg, transparent, ${stat.color}60, transparent)`,
            }} />
            <div style={{ fontSize: '10px', color: stat.color, marginBottom: '6px', opacity: 0.7 }}>{stat.icon}</div>
            <div style={{
              fontSize: '32px', fontWeight: 800, color: '#CFB991',
              fontFamily: "'JetBrains Mono', monospace",
              textShadow: '0 2px 12px rgba(207,185,145,0.2)',
            }}>{stat.value}</div>
            <div style={{
              fontSize: '9px', color: '#6B7280', textTransform: 'uppercase',
              letterSpacing: '2px', marginTop: '6px',
            }}>{stat.label}</div>
          </div>
        ))}
      </div>

      {/* ── Frameworks ── */}
      <div style={{
        display: 'flex', alignItems: 'center', gap: '12px', marginBottom: '16px',
      }}>
        <div style={{
          fontFamily: "'Cinzel', serif", fontSize: '15px', color: '#CFB991',
          letterSpacing: '3px', textTransform: 'uppercase',
        }}>Frameworks in Play</div>
        <div style={{ flex: 1, height: '1px', background: 'linear-gradient(90deg, rgba(207,185,145,0.2) 0%, transparent 100%)' }} />
      </div>

      <div style={{
        display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)',
        gap: '12px', marginBottom: '36px',
      }}>
        {[
          { tag: 'ADDIE', desc: 'Instructional Systems Design', cite: 'FSU, 1975', color: '#22c55e', phases: 'A · D · D · I · E' },
          { tag: 'CRAP', desc: 'Visual Design Principles', cite: 'Williams, 1994', color: '#3b82f6', phases: 'C · R · A · P' },
          { tag: 'EYE', desc: 'Envision · Yoke · Evolve', cite: 'Atkinson, 2026', color: '#a855f7', phases: 'E · Y · E' },
        ].map((fw) => (
          <div key={fw.tag} style={{
            padding: '20px', borderRadius: '14px',
            background: `linear-gradient(145deg, ${fw.color}06 0%, rgba(24,22,18,0.8) 100%)`,
            border: `1px solid ${fw.color}18`,
            borderLeft: `4px solid ${fw.color}60`,
            position: 'relative', overflow: 'hidden',
          }}>
            <div style={{
              fontSize: '20px', fontWeight: 900, color: fw.color,
              fontFamily: "'JetBrains Mono', monospace", marginBottom: '4px',
              textShadow: `0 0 20px ${fw.color}30`,
            }}>{fw.tag}</div>
            <div style={{ fontSize: '13px', color: '#E2E8F0', fontWeight: 600, marginBottom: '2px' }}>{fw.desc}</div>
            <div style={{ fontSize: '10px', color: '#6B7280', fontFamily: "'JetBrains Mono', monospace" }}>{fw.cite}</div>
            <div style={{
              fontSize: '9px', color: fw.color, opacity: 0.5,
              fontFamily: "'JetBrains Mono', monospace", marginTop: '8px',
              letterSpacing: '2px',
            }}>{fw.phases}</div>
          </div>
        ))}
      </div>

      {/* ── Trinity Delivers Three Products ── */}
      <div style={{
        display: 'flex', alignItems: 'center', gap: '12px', marginBottom: '16px',
      }}>
        <div style={{
          fontFamily: "'Cinzel', serif", fontSize: '15px', color: '#CFB991',
          letterSpacing: '3px', textTransform: 'uppercase',
        }}>Trinity Delivers</div>
        <div style={{ flex: 1, height: '1px', background: 'linear-gradient(90deg, rgba(207,185,145,0.2) 0%, transparent 100%)' }} />
      </div>

      <div style={{
        display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)',
        gap: '14px', marginBottom: '36px',
      }}>
        {[
          { tag: 'ID', emoji: '📋', name: 'LDT Portfolio', desc: 'Professional competency portfolio — proof of what you built', color: '#a78bfa' },
          { tag: 'AI', emoji: '📖', name: 'LitRPG Novel', desc: 'Learning journey narrated as a lite novel — the story of becoming', color: '#22d3ee' },
          { tag: 'OS', emoji: '🎓', name: 'Capstone Product', desc: 'The course you build for others — your intellectual property', color: '#34d399' },
        ].map((d) => (
          <div key={d.tag} className="pv-card" style={{
            padding: '24px 20px', borderRadius: '14px',
            background: `linear-gradient(145deg, ${d.color}08 0%, rgba(24,22,18,0.85) 100%)`,
            border: `1px solid ${d.color}15`,
            borderLeft: `4px solid ${d.color}50`,
            transition: 'all 0.3s ease',
          }}>
            <div style={{ fontSize: '28px', marginBottom: '10px', filter: 'drop-shadow(0 2px 6px rgba(0,0,0,0.3))' }}>{d.emoji}</div>
            <div style={{
              display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '6px',
            }}>
              <span style={{
                fontSize: '14px', fontWeight: 900, color: d.color,
                fontFamily: "'JetBrains Mono', monospace",
                textShadow: `0 0 16px ${d.color}30`,
              }}>{d.tag}</span>
              <span style={{ fontSize: '13px', fontWeight: 700, color: '#E2E8F0' }}>— {d.name}</span>
            </div>
            <div style={{ fontSize: '12px', color: '#9CA3AF', lineHeight: 1.6 }}>{d.desc}</div>
          </div>
        ))}
      </div>

      {/* ── Footer callout ── */}
      <div style={{
        padding: '24px', borderRadius: '14px',
        background: 'linear-gradient(135deg, rgba(207,185,145,0.04) 0%, rgba(15,13,10,0.8) 50%, rgba(207,185,145,0.03) 100%)',
        border: '1px solid rgba(207,185,145,0.1)',
        textAlign: 'center', position: 'relative', overflow: 'hidden',
      }}>
        <div style={{
          position: 'absolute', top: 0, left: 0, right: 0, height: '1px',
          background: 'linear-gradient(90deg, transparent, rgba(207,185,145,0.3), transparent)',
        }} />
        <div style={{
          fontSize: '13px', color: '#CFB991', fontFamily: "'Cinzel', serif",
          letterSpacing: '3px', marginBottom: '10px',
        }}>
          THE GRADUATION PORTFOLIO
        </div>
        <div style={{
          fontSize: '13px', color: '#9CA3AF', lineHeight: 1.7,
          maxWidth: '560px', margin: '0 auto',
        }}>
          Every artifact is evidence of a completed station on the Iron Road.
          Not of what was memorized, but of what was <em style={{ color: '#CFB991', fontWeight: 600 }}>built</em>.
          Trinity generates one for each Yardmaster who completes the journey.
        </div>
      </div>
    </div>
  );
}

/* ═══════════════════════ EVIDENCE TAB ═══════════════════════ */
function EvidenceTab() {
  const [domainFilter, setDomainFilter] = useState(null);

  const filtered = domainFilter
    ? supportingArtifacts.filter(a => a.competencies.includes(domainFilter))
    : supportingArtifacts;

  return (
    <div>
      {/* Flagship Projects */}
      <div style={{
        display: 'flex', alignItems: 'center', gap: '12px', marginBottom: '20px',
      }}>
        <div style={{
          fontFamily: "'Cinzel', serif", fontSize: '15px', color: '#CFB991',
          letterSpacing: '3px', textTransform: 'uppercase',
        }}>Flagship Projects</div>
        <div style={{ flex: 1, height: '1px', background: 'linear-gradient(90deg, rgba(207,185,145,0.2) 0%, transparent 100%)' }} />
      </div>

      {flagshipProjects.map((project, pi) => (
        <div key={project.id} className="pv-card" style={{
          marginBottom: '20px', borderRadius: '14px',
          background: 'rgba(24,22,18,0.7)',
          border: '1px solid rgba(207,185,145,0.1)',
          overflow: 'hidden', transition: 'all 0.3s ease',
          animationDelay: `${pi * 100}ms`,
        }}>
          {/* Header */}
          <div style={{
            padding: '24px 28px',
            background: 'linear-gradient(135deg, rgba(207,185,145,0.06) 0%, rgba(24,22,18,0.95) 100%)',
            borderBottom: '1px solid rgba(207,185,145,0.08)',
            position: 'relative',
          }}>
            <div style={{
              position: 'absolute', bottom: 0, left: 0, right: 0, height: '1px',
              background: 'linear-gradient(90deg, transparent, rgba(207,185,145,0.15), transparent)',
            }} />
            <div style={{
              display: 'inline-flex', alignItems: 'center', gap: '6px',
              fontSize: '9px', fontFamily: "'Cinzel', serif", letterSpacing: '2px',
              color: '#CFB991', textTransform: 'uppercase',
              padding: '4px 12px', borderRadius: '20px',
              background: 'rgba(207,185,145,0.08)', border: '1px solid rgba(207,185,145,0.15)',
              marginBottom: '10px',
            }}>
              ✦ Flagship Project
            </div>
            <div style={{ fontSize: '22px', fontWeight: 700, color: '#E2E8F0', marginBottom: '4px' }}>
              {project.title}
            </div>
            <div style={{ fontSize: '13px', color: '#9CA3AF', fontStyle: 'italic' }}>
              {project.subtitle}
            </div>
          </div>

          {/* Body */}
          <div style={{ padding: '24px 28px' }}>
            {/* Hook */}
            <div style={{
              padding: '14px 18px', borderRadius: '10px',
              background: 'rgba(15,13,10,0.5)',
              borderLeft: '3px solid rgba(207,185,145,0.4)',
              fontSize: '14px', color: '#D1D5DB', lineHeight: 1.7,
              marginBottom: '20px', fontStyle: 'italic',
            }}>
              "{project.hook}"
            </div>

            {/* Competencies */}
            <div style={{
              fontSize: '9px', color: '#6B7280', textTransform: 'uppercase',
              letterSpacing: '2px', marginBottom: '10px', fontFamily: "'JetBrains Mono', monospace",
            }}>
              IBSTPI Competencies
            </div>
            <div style={{ marginBottom: '20px' }}>
              {project.competencies.map((comp, i) => (
                <div key={i} style={{
                  display: 'flex', alignItems: 'flex-start', gap: '10px',
                  marginBottom: '8px', padding: '8px 12px', borderRadius: '8px',
                  background: `${COMP_COLORS[comp.domain]}06`,
                }}>
                  <span style={{
                    display: 'inline-flex', alignItems: 'center',
                    fontSize: '9px', padding: '3px 10px', borderRadius: '12px',
                    background: `${COMP_COLORS[comp.domain]}12`,
                    border: `1px solid ${COMP_COLORS[comp.domain]}25`,
                    color: COMP_COLORS[comp.domain], fontWeight: 700, flexShrink: 0,
                    whiteSpace: 'nowrap',
                  }}>
                    {competencyDomains[comp.domain]?.title}
                  </span>
                  <span style={{ fontSize: '12px', color: '#9CA3AF', lineHeight: 1.6 }}>
                    {comp.description}
                  </span>
                </div>
              ))}
            </div>

            {/* Highlights */}
            <div style={{
              fontSize: '9px', color: '#6B7280', textTransform: 'uppercase',
              letterSpacing: '2px', marginBottom: '10px', fontFamily: "'JetBrains Mono', monospace",
            }}>
              Key Highlights
            </div>
            {project.highlights.map((h, i) => (
              <div key={i} style={{
                fontSize: '12px', color: '#D1D5DB', padding: '6px 0 6px 14px',
                borderLeft: '2px solid rgba(207,185,145,0.12)',
                marginBottom: '4px',
              }}>
                {h}
              </div>
            ))}
          </div>
        </div>
      ))}

      {/* Supporting Artifacts */}
      <div style={{
        display: 'flex', alignItems: 'center', gap: '12px', marginTop: '36px', marginBottom: '16px',
      }}>
        <div style={{
          fontFamily: "'Cinzel', serif", fontSize: '15px', color: '#CFB991',
          letterSpacing: '3px', textTransform: 'uppercase',
        }}>Supporting Artifacts</div>
        <div style={{ flex: 1, height: '1px', background: 'linear-gradient(90deg, rgba(207,185,145,0.2) 0%, transparent 100%)' }} />
      </div>

      {/* Filter Bar */}
      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px', marginBottom: '20px' }}>
        <button
          className="pv-filter"
          style={{
            padding: '6px 16px', borderRadius: '20px',
            fontSize: '11px', fontWeight: 600, cursor: 'pointer',
            border: !domainFilter ? '1px solid rgba(207,185,145,0.4)' : '1px solid rgba(75,85,99,0.25)',
            background: !domainFilter ? 'rgba(207,185,145,0.1)' : 'transparent',
            color: !domainFilter ? '#CFB991' : '#6B7280',
            transition: 'all 0.2s',
          }}
          onClick={() => setDomainFilter(null)}
        >All</button>
        {Object.values(competencyDomains).map((domain) => {
          const c = COMP_COLORS[domain.id];
          const active = domainFilter === domain.id;
          return (
            <button
              key={domain.id}
              className="pv-filter"
              style={{
                padding: '6px 16px', borderRadius: '20px',
                fontSize: '11px', fontWeight: 600, cursor: 'pointer',
                border: active ? `1px solid ${c}60` : '1px solid rgba(75,85,99,0.25)',
                background: active ? `${c}15` : 'transparent',
                color: active ? c : '#6B7280',
                transition: 'all 0.2s',
              }}
              onClick={() => setDomainFilter(active ? null : domain.id)}
            >{domain.title}</button>
          );
        })}
      </div>

      <div style={{
        display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(300px, 1fr))',
        gap: '14px', marginBottom: '36px',
      }}>
        {filtered.map((artifact, ai) => (
          <a
            key={artifact.id}
            href={artifact.pdfPath}
            target="_blank"
            rel="noopener noreferrer"
            className="pv-card"
            style={{
              padding: '18px 20px', borderRadius: '12px',
              background: 'linear-gradient(145deg, rgba(24,22,18,0.8) 0%, rgba(24,22,18,0.5) 100%)',
              border: '1px solid rgba(207,185,145,0.06)',
              transition: 'all 0.3s ease', textDecoration: 'none', color: 'inherit',
              display: 'block', animationDelay: `${ai * 50}ms`,
            }}
          >
            <div style={{
              fontSize: '9px', color: '#CFB991', textTransform: 'uppercase',
              letterSpacing: '2px', fontFamily: "'JetBrains Mono', monospace",
              marginBottom: '8px', opacity: 0.7,
            }}>{artifact.type}</div>
            <div style={{
              fontSize: '14px', fontWeight: 700, color: '#E2E8F0',
              marginBottom: '8px', lineHeight: 1.3,
            }}>{artifact.title}</div>
            <div style={{
              fontSize: '12px', color: '#9CA3AF', lineHeight: 1.6, marginBottom: '12px',
            }}>{artifact.description}</div>
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '4px' }}>
              {artifact.competencies.map((comp) => (
                <span key={comp} style={{
                  fontSize: '9px', padding: '2px 8px', borderRadius: '10px',
                  background: `${COMP_COLORS[comp]}12`,
                  border: `1px solid ${COMP_COLORS[comp]}25`,
                  color: COMP_COLORS[comp], fontWeight: 600,
                }}>
                  {competencyDomains[comp]?.title}
                </span>
              ))}
            </div>
          </a>
        ))}
      </div>

      {/* Technology Badges */}
      <div style={{
        display: 'flex', alignItems: 'center', gap: '12px', marginBottom: '12px',
      }}>
        <div style={{
          fontFamily: "'Cinzel', serif", fontSize: '15px', color: '#CFB991',
          letterSpacing: '3px', textTransform: 'uppercase',
        }}>Legacy — LDT Technology Badges</div>
        <div style={{ flex: 1, height: '1px', background: 'linear-gradient(90deg, rgba(207,185,145,0.2) 0%, transparent 100%)' }} />
      </div>
      <div style={{
        fontSize: '12px', color: '#6B7280', marginBottom: '20px', fontStyle: 'italic',
        borderLeft: '2px solid rgba(207,185,145,0.1)', paddingLeft: '12px',
      }}>
        The foundation — approved coursework that forms the bedrock upon which Trinity was built.
      </div>
      <div style={{
        display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(280px, 1fr))',
        gap: '10px', marginBottom: '24px',
      }}>
        {technologyBadges.map((badge) => (
          <a
            key={badge.id}
            href={badge.link}
            target="_blank"
            rel="noopener noreferrer"
            className="pv-tech"
            style={{
              display: 'flex', alignItems: 'center', gap: '12px',
              padding: '12px 16px', borderRadius: '10px',
              background: 'rgba(24,22,18,0.5)',
              border: '1px solid rgba(207,185,145,0.06)',
              transition: 'all 0.25s', textDecoration: 'none', color: 'inherit',
            }}
          >
            <div style={{
              width: '36px', height: '36px', borderRadius: '8px',
              background: 'linear-gradient(135deg, rgba(207,185,145,0.1), rgba(207,185,145,0.02))',
              border: '1px solid rgba(207,185,145,0.12)',
              display: 'flex', alignItems: 'center', justifyContent: 'center',
              fontSize: '16px', flexShrink: 0,
            }}>🏅</div>
            <div>
              <div style={{ fontSize: '12px', fontWeight: 700, color: '#E2E8F0' }}>{badge.title}</div>
              <div style={{ fontSize: '10px', color: '#6B7280', marginTop: '1px' }}>{badge.description}</div>
            </div>
          </a>
        ))}
      </div>
    </div>
  );
}

/* ═══════════════════════ BADGES TAB ═══════════════════════ */
function BadgesTab() {
  const [expandedBadge, setExpandedBadge] = useState(null);
  const { categories, badges } = portfolioData;

  const CATEGORY_META = {
    foundations: { icon: '⚙️', color: '#818cf8' },
    planning:    { icon: '🔍', color: '#3b82f6' },
    design:      { icon: '🎨', color: '#a855f7' },
    evaluation:  { icon: '📊', color: '#22c55e' },
  };

  return (
    <div>
      <div style={{
        fontSize: '13px', color: '#9CA3AF', marginBottom: '28px', lineHeight: 1.7,
        borderLeft: '3px solid rgba(207,185,145,0.15)', paddingLeft: '16px',
      }}>
        A systematic collection of artifacts demonstrating mastery of the Purdue LDT Competencies,
        organized by IBSTPI domain. Each badge represents <em style={{ color: '#CFB991' }}>verified competency</em>.
      </div>

      {categories.map((category) => {
        const meta = CATEGORY_META[category.id] || { icon: '📋', color: '#6B7280' };
        const categoryBadges = badges.filter(b => b.categoryId === category.id);
        return (
          <div key={category.id} style={{ marginBottom: '36px' }}>
            {/* Category Header */}
            <div style={{
              display: 'flex', alignItems: 'center', gap: '14px',
              marginBottom: '16px', paddingBottom: '14px',
              borderBottom: `1px solid ${meta.color}15`,
            }}>
              <div style={{
                width: '48px', height: '48px', borderRadius: '14px',
                background: `linear-gradient(135deg, ${meta.color}12, ${meta.color}04)`,
                border: `1px solid ${meta.color}20`,
                display: 'flex', alignItems: 'center', justifyContent: 'center',
                fontSize: '22px',
              }}>{meta.icon}</div>
              <div>
                <div style={{
                  fontSize: '18px', fontWeight: 700, color: '#E2E8F0',
                  fontFamily: "'Cinzel', serif",
                }}>{category.title}</div>
                <div style={{ fontSize: '12px', color: '#6B7280', marginTop: '2px', maxWidth: '500px' }}>
                  {category.description}
                </div>
              </div>
              <div style={{ marginLeft: 'auto' }}>
                <span style={{
                  fontSize: '9px', padding: '4px 10px', borderRadius: '20px',
                  background: `${meta.color}12`, border: `1px solid ${meta.color}25`,
                  color: meta.color, fontFamily: "'JetBrains Mono', monospace",
                  letterSpacing: '1px',
                }}>{categoryBadges.length} BADGES</span>
              </div>
            </div>

            {/* Badge Grid */}
            <div style={{
              display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(260px, 1fr))',
              gap: '10px',
            }}>
              {categoryBadges.map((badge, bi) => (
                <div key={badge.id}>
                  <div
                    className="pv-badge"
                    style={{
                      padding: '16px', borderRadius: '12px',
                      background: expandedBadge === badge.id
                        ? `linear-gradient(145deg, ${meta.color}08, rgba(24,22,18,0.8))`
                        : 'rgba(24,22,18,0.6)',
                      border: expandedBadge === badge.id
                        ? `1px solid ${meta.color}30`
                        : '1px solid rgba(207,185,145,0.06)',
                      cursor: 'pointer', transition: 'all 0.25s ease',
                      animation: `pv-fadeIn 0.3s ease-out ${bi * 40}ms both`,
                    }}
                    onClick={() => setExpandedBadge(expandedBadge === badge.id ? null : badge.id)}
                  >
                    <div style={{
                      display: 'flex', alignItems: 'center', justifyContent: 'space-between',
                      marginBottom: '4px',
                    }}>
                      <div style={{ fontSize: '13px', fontWeight: 700, color: '#E2E8F0' }}>
                        {badge.title}
                      </div>
                      <span style={{
                        fontSize: '10px', color: '#34d399',
                        fontFamily: "'JetBrains Mono', monospace",
                      }}>✓</span>
                    </div>
                    <div style={{
                      fontSize: '8px', textTransform: 'uppercase', letterSpacing: '2px',
                      color: '#34d399', fontFamily: "'JetBrains Mono', monospace",
                    }}>Verified</div>
                    {badge.description && (
                      <div style={{
                        fontSize: '11px', color: '#6B7280', marginTop: '6px',
                        lineHeight: 1.5, display: '-webkit-box', WebkitLineClamp: 2,
                        WebkitBoxOrient: 'vertical', overflow: 'hidden',
                      }}>
                        {badge.description}
                      </div>
                    )}
                  </div>

                  {/* Expanded detail */}
                  {expandedBadge === badge.id && (
                    <div style={{
                      padding: '18px 20px', borderRadius: '12px', marginTop: '6px',
                      background: `linear-gradient(145deg, ${meta.color}06, rgba(24,22,18,0.9))`,
                      border: `1px solid ${meta.color}20`,
                      animation: 'pv-fadeIn 0.2s ease-out',
                    }}>
                      <div style={{
                        display: 'flex', justifyContent: 'space-between', alignItems: 'center',
                        marginBottom: '10px',
                      }}>
                        <div style={{
                          fontSize: '15px', fontWeight: 700, color: '#CFB991',
                        }}>{badge.title}</div>
                        <button
                          onClick={(e) => { e.stopPropagation(); setExpandedBadge(null); }}
                          style={{
                            background: 'none', border: 'none', color: '#6B7280',
                            cursor: 'pointer', fontSize: '14px', padding: '2px 6px',
                          }}
                        >✕</button>
                      </div>
                      {badge.description && (
                        <div style={{
                          fontSize: '13px', color: '#D1D5DB', lineHeight: 1.7,
                          marginBottom: '14px',
                        }}>{badge.description}</div>
                      )}
                      {badge.artifacts && badge.artifacts.length > 0 && (
                        <div>
                          <div style={{
                            fontSize: '9px', color: '#6B7280', textTransform: 'uppercase',
                            letterSpacing: '2px', marginBottom: '10px',
                            fontFamily: "'JetBrains Mono', monospace",
                          }}>Evidence Artifacts</div>
                          {badge.artifacts.map((art, i) => (
                            <div key={i} style={{
                              padding: '10px 14px', marginBottom: '6px', borderRadius: '8px',
                              background: 'rgba(15,13,10,0.5)',
                              borderLeft: `2px solid ${meta.color}30`,
                            }}>
                              <div style={{ fontSize: '12px', fontWeight: 700, color: '#E2E8F0', marginBottom: '3px' }}>
                                {art.title}
                              </div>
                              {art.reflection && (
                                <div style={{
                                  fontSize: '11px', color: '#9CA3AF', fontStyle: 'italic', lineHeight: 1.5,
                                }}>"{art.reflection}"</div>
                              )}
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

/* ═══════════════════════ MAIN COMPONENT ═══════════════════════ */
export default function PortfolioView() {
  const [activeSubTab, setActiveSubTab] = useState('overview');

  return (
    <div style={{
      background: 'linear-gradient(180deg, #0f0d0a 0%, #181612 40%, #0f0d0a 100%)',
      fontFamily: "'Crimson Text', serif",
      color: '#E2E8F0',
    }}>
      {/* ── Header ── */}
      <div style={{
        textAlign: 'center', padding: '48px 24px 28px',
        position: 'relative', overflow: 'hidden',
      }}>
        {/* Gradient orb */}
        <div style={{
          position: 'absolute', top: '-60px', left: '50%', transform: 'translateX(-50%)',
          width: '400px', height: '200px', borderRadius: '50%',
          background: 'radial-gradient(ellipse, rgba(207,185,145,0.06) 0%, transparent 70%)',
          pointerEvents: 'none',
        }} />
        <div style={{
          fontSize: '11px', color: '#6B7280', fontFamily: "'JetBrains Mono', monospace",
          letterSpacing: '4px', textTransform: 'uppercase', marginBottom: '12px',
        }}>
          TRINITY ID AI OS
        </div>
        <div style={{
          fontSize: '28px', fontFamily: "'Cinzel', serif",
          letterSpacing: '4px', textTransform: 'uppercase',
          marginBottom: '12px',
          background: 'linear-gradient(135deg, #CFB991 0%, #E8D5A8 50%, #CFB991 100%)',
          backgroundSize: '200% 100%',
          animation: 'pv-shimmer 6s ease-in-out infinite',
          WebkitBackgroundClip: 'text', WebkitTextFillColor: 'transparent',
          backgroundClip: 'text',
        }}>
          The Graduation Portfolio
        </div>
        <div style={{
          fontSize: '14px', color: '#6B7280', fontStyle: 'italic',
          maxWidth: '460px', margin: '0 auto', lineHeight: 1.6,
        }}>
          Built by doing, not by reading. Every artifact is evidence of a completed station on the Iron Road.
        </div>
        <div style={{
          width: '60px', height: '2px', margin: '20px auto 0',
          background: 'linear-gradient(90deg, transparent, rgba(207,185,145,0.4), transparent)',
        }} />
      </div>

      {/* ── Sub-tab bar ── */}
      <div style={{
        display: 'flex', justifyContent: 'center', gap: '4px',
        padding: '12px 24px 16px',
        borderBottom: '1px solid rgba(207,185,145,0.06)',
      }}>
        {SUB_TABS.map((tab) => {
          const active = activeSubTab === tab.id;
          return (
            <button
              key={tab.id}
              style={{
                padding: '10px 28px', borderRadius: '10px',
                fontSize: '12px', fontFamily: "'Cinzel', serif",
                letterSpacing: '2px', cursor: 'pointer',
                border: active ? '1px solid rgba(207,185,145,0.25)' : '1px solid transparent',
                background: active
                  ? 'linear-gradient(145deg, rgba(207,185,145,0.1), rgba(207,185,145,0.03))'
                  : 'transparent',
                color: active ? '#CFB991' : '#4B5563',
                transition: 'all 0.25s ease',
                boxShadow: active ? '0 4px 16px rgba(0,0,0,0.2)' : 'none',
              }}
              onClick={() => setActiveSubTab(tab.id)}
              onMouseEnter={(e) => {
                if (!active) e.currentTarget.style.color = '#9CA3AF';
              }}
              onMouseLeave={(e) => {
                if (!active) e.currentTarget.style.color = '#4B5563';
              }}
            >
              {tab.label}
            </button>
          );
        })}
      </div>

      {/* ── Tab content ── */}
      <div style={{ padding: '28px 24px', maxWidth: '1100px', margin: '0 auto' }}>
        {activeSubTab === 'overview' && <OverviewTab />}
        {activeSubTab === 'evidence' && <EvidenceTab />}
        {activeSubTab === 'badges' && <BadgesTab />}
      </div>
    </div>
  );
}
