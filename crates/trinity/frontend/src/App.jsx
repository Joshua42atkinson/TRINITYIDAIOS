import React, { useState } from 'react';
import NavBar from './components/NavBar';
import ChapterRail from './components/ChapterRail';
import PhaseWorkspace from './components/PhaseWorkspace';
import GameHUD from './components/GameHUD';
import ArtStudio from './components/ArtStudio';
import CharacterSheet from './components/CharacterSheet';
import Yardmaster from './components/Yardmaster';
import ExpressWizard from './components/ExpressWizard';
import OnboardingTour from './components/OnboardingTour';
import QualityScorecard from './components/QualityScorecard';
import { useQuest } from './hooks/useQuest';
import { useBestiary } from './hooks/useBestiary';
import { useSSE } from './hooks/useSSE';
import { usePearl } from './hooks/usePearl';

const SUBJECTS = [
  'Ecosystems', 'Physics', 'US History', 'Algebra',
  'Creative Writing', 'Chemistry', 'Computer Science', 'Art History',
];

const MEDIUMS = [
  { value: 'Game', icon: '🎮' },
  { value: 'Storyboard', icon: '🎬' },
  { value: 'Simulation', icon: '🔬' },
  { value: 'LessonPlan', icon: '📋' },
  { value: 'Assessment', icon: '📝' },
  { value: 'Book', icon: '📖' },
];

function SubjectPicker({ onSelect }) {
  const [custom, setCustom] = useState('');
  const [medium, setMedium] = useState('Game');
  const [vision, setVision] = useState('');
  const [selectedSubject, setSelectedSubject] = useState('');

  const handleSubmit = () => {
    const subject = selectedSubject || custom.trim();
    if (subject) onSelect(subject, medium, vision.trim());
  };

  return (
    <div className="subject-picker">
      <div className="subject-picker__title">THE IRON ROAD</div>
      <div className="subject-picker__subtitle">
        Choose your subject. Pete will guide you through ADDIECRAPEYE —
        the 12-chapter pedagogical framework for building gamified lessons.
      </div>

      {/* Subject Grid */}
      <div className="subject-grid">
        {SUBJECTS.map((s) => (
          <button
            key={s}
            className={`subject-btn ${selectedSubject === s ? 'subject-btn--active' : ''}`}
            onClick={() => { setSelectedSubject(s); setCustom(''); }}
          >
            {s}
          </button>
        ))}
      </div>

      {/* Custom Subject */}
      <div className="subject-custom">
        <input
          className="chat-input"
          placeholder="Or type your own subject..."
          value={custom}
          onChange={(e) => { setCustom(e.target.value); setSelectedSubject(''); }}
        />
      </div>

      {/* Medium Selector */}
      <div className="subject-section">
        <div className="section-label">DELIVERY MEDIUM</div>
        <div className="medium-grid">
          {MEDIUMS.map((m) => (
            <button
              key={m.value}
              className={`medium-btn ${medium === m.value ? 'medium-btn--active' : ''}`}
              onClick={() => setMedium(m.value)}
            >
              {m.icon} {m.value}
            </button>
          ))}
        </div>
      </div>

      {/* Vision */}
      <div className="subject-section">
        <div className="section-label">VISION (what should the output feel like?)</div>
        <input
          className="vision-input"
          placeholder="e.g. Students feel like they discovered Newton's laws themselves"
          value={vision}
          onChange={(e) => setVision(e.target.value)}
        />
      </div>

      {/* Begin Button */}
      <button
        className="chat-send begin-btn"
        onClick={handleSubmit}
        disabled={!selectedSubject && !custom.trim()}
      >
        BEGIN JOURNEY
      </button>
    </div>
  );
}

export default function App() {
  const [activeTab, setActiveTab] = useState('ironroad');
  const [started, setStarted] = useState(false);
  const [viewPhase, setViewPhase] = useState(null);
  const [appMode, setAppMode] = useState('iron_road'); // iron_road | express | yardmaster
  const { quest, phases, currentPhaseIndex, refetch } = useQuest();
  const { bestiary } = useBestiary();
  const { events, dismissEvent } = useSSE();

  // Listen for handoff events from PhaseWorkspace (Recycler → Pete workshop)
  React.useEffect(() => {
    const handleModeSwitch = (e) => {
      if (e.detail === 'yardmaster') {
        setAppMode('yardmaster');
        setActiveTab('yard');
      }
    };
    window.addEventListener('trinity-mode', handleModeSwitch);
    return () => window.removeEventListener('trinity-mode', handleModeSwitch);
  }, []);

  const startJourney = async (subject, medium, vision) => {
    try {
      await fetch('/api/pearl', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ subject, medium, vision }),
      });
      refetch();
      setStarted(true);
    } catch (err) {
      console.error('Failed to start:', err);
    }
  };

  const newJourney = () => {
    setStarted(false);
  };

  const handlePhaseClick = async (idx, name) => {
    setViewPhase(name);
  };

  const clearViewPhase = () => setViewPhase(null);

  return (
    <div className="iron-road-layout">
      <NavBar
        quest={quest}
        activeTab={activeTab}
        onTabChange={setActiveTab}
        onNewJourney={newJourney}
      />

      {activeTab === 'creator' ? (
        <div style={{ gridColumn: '1 / -1', gridRow: 2, overflow: 'auto', padding: '40px 20px' }}>
          <div style={{
            maxWidth: '720px', margin: '0 auto',
            fontFamily: "'Inter', sans-serif", color: '#E2E8F0',
          }}>
            <div style={{
              textAlign: 'center', marginBottom: '32px',
              borderBottom: '1px solid rgba(207, 185, 145, 0.15)', paddingBottom: '24px',
            }}>
              <div style={{
                fontFamily: "'Cinzel', serif", fontSize: '28px', color: '#CFB991',
                letterSpacing: '3px', marginBottom: '8px',
              }}>
                JOSHUA ATKINSON
              </div>
              <div style={{ fontSize: '14px', color: '#9CA3AF', lineHeight: 1.6 }}>
                Marine Veteran · Father of Six · Graduate Student, Learning Design & Technology
                <br />Purdue University
              </div>
            </div>

            <div style={{
              fontSize: '15px', lineHeight: 1.8, color: '#CBD5E1', marginBottom: '32px',
            }}>
              <p style={{ marginBottom: '16px' }}>
                <em>"We are the stories we tell ourselves."</em>
                <br />And then I built a literal game engine to prove it.
              </p>
              <p style={{ marginBottom: '16px' }}>
                I built Trinity ID AI OS on a single machine in my house because I believe that the most
                powerful educational technology is the one that belongs to the learner — not to a corporation,
                not to a cloud, and not to an institution.
              </p>
              <p style={{ marginBottom: '16px' }}>
                Trinity is the product of three complete rebuilds, three operating systems, a decade of
                military service, seminary studies, bartending, fathering six children, and the persistent
                belief that the meek shall inherit the earth — because the meek are the ones who've done
                the hard work of knowing themselves.
              </p>
            </div>

            <div style={{
              display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px', marginBottom: '32px',
            }}>
              <a href="https://ldtatkinson.com" target="_blank" rel="noopener noreferrer"
                style={{
                  display: 'block', padding: '20px', borderRadius: '10px',
                  background: 'rgba(207, 185, 145, 0.06)', border: '1px solid rgba(207, 185, 145, 0.15)',
                  textDecoration: 'none', color: '#E2E8F0', transition: 'all 0.2s',
                }}
                onMouseEnter={(e) => e.currentTarget.style.borderColor = 'rgba(207, 185, 145, 0.4)'}
                onMouseLeave={(e) => e.currentTarget.style.borderColor = 'rgba(207, 185, 145, 0.15)'}
              >
                <div style={{ fontSize: '22px', marginBottom: '8px' }}>🎓</div>
                <div style={{ fontWeight: 600, marginBottom: '4px' }}>LDT Portfolio</div>
                <div style={{ fontSize: '12px', color: '#6B7280' }}>
                  LDTAtkinson.com — Academic portfolio with evidence-based artifacts
                </div>
              </a>
              <a href="/docs/PLAYERS_HANDBOOK.md" target="_blank" rel="noopener noreferrer"
                style={{
                  display: 'block', padding: '20px', borderRadius: '10px',
                  background: 'rgba(207, 185, 145, 0.06)', border: '1px solid rgba(207, 185, 145, 0.15)',
                  textDecoration: 'none', color: '#E2E8F0', transition: 'all 0.2s',
                }}
                onMouseEnter={(e) => e.currentTarget.style.borderColor = 'rgba(207, 185, 145, 0.4)'}
                onMouseLeave={(e) => e.currentTarget.style.borderColor = 'rgba(207, 185, 145, 0.15)'}
              >
                <div style={{ fontSize: '22px', marginBottom: '8px' }}>🎮</div>
                <div style={{ fontWeight: 600, marginBottom: '4px' }}>Player's Handbook</div>
                <div style={{ fontSize: '12px', color: '#6B7280' }}>
                  The philosophy behind Trinity — 15 chapters of why this game exists
                </div>
              </a>
            </div>

            <div style={{
              padding: '20px', borderRadius: '10px',
              background: 'rgba(207, 185, 145, 0.04)', border: '1px solid rgba(207, 185, 145, 0.1)',
            }}>
              <div style={{
                fontFamily: "'Cinzel', serif", fontSize: '12px', color: '#CFB991',
                letterSpacing: '2px', textTransform: 'uppercase', marginBottom: '12px',
              }}>
                Contact & Links
              </div>
              <div style={{ fontSize: '13px', lineHeight: 2, color: '#9CA3AF' }}>
                <div>📧 Portfolio: <a href="https://ldtatkinson.com" target="_blank" rel="noopener noreferrer" style={{ color: '#CFB991', textDecoration: 'none' }}>LDTAtkinson.com</a></div>
                <div>💻 Source: <a href="https://github.com/Joshua42atkinson/trinity-genesis" target="_blank" rel="noopener noreferrer" style={{ color: '#CFB991', textDecoration: 'none' }}>GitHub</a></div>
                <div>🔮 Framework: <a href="https://consciousframework.com" target="_blank" rel="noopener noreferrer" style={{ color: '#CFB991', textDecoration: 'none' }}>ConsciousFramework.com</a></div>
                <div>♻️ Recycler: <a href="https://greatrecycler.com" target="_blank" rel="noopener noreferrer" style={{ color: '#CFB991', textDecoration: 'none' }}>GreatRecycler.com</a></div>
              </div>
            </div>
          </div>
        </div>
      ) : activeTab === 'art' ? (
        <div style={{ gridColumn: '1 / -1', gridRow: 2, overflow: 'auto' }}>
          <ArtStudio />
        </div>
      ) : activeTab === 'character' ? (
        <div style={{ gridColumn: '1 / -1', gridRow: 2, overflow: 'auto' }}>
          <CharacterSheet />
        </div>
      ) : activeTab === 'scorecard' ? (
        <div style={{ gridColumn: '1 / -1', gridRow: 2, overflow: 'auto' }}>
          <QualityScorecard />
        </div>
      ) : activeTab === 'yard' || appMode === 'yardmaster' ? (
        <div className="full-span-tab">
          <Yardmaster />
        </div>
      ) : appMode === 'express' ? (
        <div className="full-span-tab">
          <ExpressWizard onComplete={(result) => {
            console.log('Express result:', result);
          }} />
        </div>
      ) : (started || quest?.subject) && quest?.subject !== '' ? (
        <>
          <ChapterRail
            phases={phases}
            currentPhaseIndex={currentPhaseIndex}
            completedPhases={quest?.completed_phases}
            onPhaseClick={handlePhaseClick}
          />

          <PhaseWorkspace
            quest={quest}
            sseEvents={events}
            onDismissEvent={dismissEvent}
            onRefetch={refetch}
            viewPhase={viewPhase}
            allPhases={phases}
            onClearView={clearViewPhase}
          />

          <GameHUD
            quest={quest}
            bestiary={bestiary}
            onRefetch={refetch}
          />
        </>
      ) : (
        <>
          <div className="full-span-tab">
            <SubjectPicker onSelect={startJourney} />
          </div>
        </>
      )}

      <footer className="footer">
        <span className="footer__label">TRINITY ID AI OS</span>
        <span className="sep">|</span>
        <span>{quest?.subject || 'No subject selected'}</span>
        <span className="flex-spacer" />
        <div className="mode-toggle">
          {[['iron_road', '🚂'], ['express', '⚡'], ['yardmaster', '🔧']].map(([m, icon]) => (
            <button
              key={m}
              className={`mode-toggle__btn ${appMode === m ? 'mode-toggle__btn--active' : ''}`}
              onClick={() => {
                setAppMode(m);
                fetch('/api/mode', {
                  method: 'POST',
                  headers: { 'Content-Type': 'application/json' },
                  body: JSON.stringify({ mode: m }),
                }).catch(() => {});
              }}
            >
              {icon}
            </button>
          ))}
        </div>
        <span className="sep">|</span>
        <span>Phase: {quest?.phase || '—'}</span>
        <span className="sep">|</span>
        <span>XP: {quest?.xp || 0}</span>
      </footer>
      <OnboardingTour />
    </div>
  );
}
