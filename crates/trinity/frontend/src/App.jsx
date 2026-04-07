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
import ChariotViewer from './components/ChariotViewer';
import PlayerHandbookElearning from './components/PlayerHandbookElearning';
import FieldManualViewer from './components/FieldManualViewer';
import JournalViewer from './components/JournalViewer';
import PortfolioView from './components/PortfolioView';
import ActivityBar from './components/ActivityBar';
import { useQuest } from './hooks/useQuest';
import { useBestiary } from './hooks/useBestiary';
import { useSSE } from './hooks/useSSE';
import { usePearl } from './hooks/usePearl';
// GlobalDeckOverlay removed — Hook Deck lives in CharacterSheet

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
  const [templates, setTemplates] = useState([]);

  React.useEffect(() => {
    fetch('/api/projects/community')
      .then(r => r.json())
      .then(d => { if (Array.isArray(d)) setTemplates(d); })
      .catch(() => {});
  }, []);

  const handleSubmit = () => {
    const subject = selectedSubject || custom.trim();
    if (subject) onSelect(subject, medium, vision.trim());
  };

  return (
    <div className="subject-picker">
      <div className="subject-picker__title">ID · Learning</div>
      <div className="subject-picker__subtitle">
        Welcome to the Iron Road. Choose a subject to begin a guided journey
        through the ADDIECRAPEYE framework to build something extraordinary.
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

      {/* Community Templates */}
      {templates.length > 0 && (
        <div className="subject-section" style={{ marginTop: '24px' }}>
          <div className="section-label" style={{ color: '#34d399' }}>🌍 FEATURED COMMUNITY TEMPLATES</div>
          <div className="subject-grid" style={{ gridTemplateColumns: '1fr 1fr' }}>
            {templates.map(tpl => (
              <div 
                key={tpl.id} 
                className={`subject-btn ${selectedSubject === tpl.name ? 'subject-btn--active' : ''}`}
                onClick={() => { setSelectedSubject(tpl.name); setCustom(''); setMedium('Simulation'); }}
                style={{ display: 'flex', flexDirection: 'column', alignItems: 'flex-start', padding: '12px' }}
              >
                <div style={{ fontWeight: 'bold', fontSize: '1.1rem' }}>{tpl.name}</div>
                <div style={{ fontSize: '0.8rem', opacity: 0.7, marginTop: '4px', textAlign: 'left' }}>
                  {tpl.archive_reason || "Starter module"}
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

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

      {/* Walkthrough Link */}
      <button
        className="see-what-btn"
        onClick={() => {
          // Dispatch chariot event — App.jsx listens for 'chariot:' prefix in tab changes
          window.dispatchEvent(new CustomEvent('trinity-walkthrough', { detail: 'PLAYERS_HANDBOOK.md' }));
        }}
      >
        📖 See What It Does
      </button>
    </div>
  );
}

export default function App() {
  const [activeTab, setActiveTab] = useState('ironroad');
  const [started, setStarted] = useState(false);
  const [viewPhase, setViewPhase] = useState(null);
  const [appMode, setAppMode] = useState('iron_road'); // iron_road | express | yardmaster
  const [setupComplete, setSetupComplete] = useState(false);
  const [isCheckingSetup, setIsCheckingSetup] = useState(true);
  const [chariotDoc, setChariotDoc] = useState(null);  // filename of chariot doc to view
  const { quest, phases, currentPhaseIndex, refetch } = useQuest();
  const { bestiary } = useBestiary();
  const { events, dismissEvent } = useSSE();
  const [inferenceModel, setInferenceModel] = React.useState('...');

  // Fetch active model name on mount and bypass wizard if the server is already configured
  React.useEffect(() => {
    const localState = localStorage.getItem('trinitySetupState');
    if (localState) {
      setSetupComplete(true);
    }

    fetch('/api/models/active')
      .then(r => r.json())
      .then(d => {
        if (d.healthy && !localState) {
          setSetupComplete(true);
        }
        setInferenceModel(d.model_name || d.name || '?');
        setIsCheckingSetup(false);
      })
      .catch(() => setIsCheckingSetup(false));
  }, []);

  // Listen for handoff events from PhaseWorkspace (Recycler → Pete workshop)
  React.useEffect(() => {
    const handleModeSwitch = (e) => {
      if (e.detail === 'yardmaster') {
        setAppMode('yardmaster');
        setActiveTab('yard');
      }
    };
    const handleWalkthrough = (e) => {
      if (e.detail) {
        setChariotDoc(e.detail);
        setActiveTab('chariot');
      }
    };
    window.addEventListener('trinity-mode', handleModeSwitch);
    window.addEventListener('trinity-walkthrough', handleWalkthrough);
    return () => {
      window.removeEventListener('trinity-mode', handleModeSwitch);
      window.removeEventListener('trinity-walkthrough', handleWalkthrough);
    };
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

  if (isCheckingSetup) {
    return (
      <div style={{ height: '100vh', width: '100vw', background: '#090a0f', display: 'flex', alignItems: 'center', justifyContent: 'center', color: '#B3C2D1', fontFamily: '"Inter", sans-serif', fontSize: '1.2rem', letterSpacing: '2px' }}>
        INITIALIZING NERVOUS SYSTEM...
      </div>
    );
  }

  if (!setupComplete) {
    return (
      <div style={{ height: '100vh', width: '100vw', background: '#090a0f', display: 'flex', alignItems: 'center', justifyContent: 'center', color: '#B3C2D1', fontFamily: '"Inter", sans-serif', fontSize: '1.2rem', letterSpacing: '2px' }}>
        AWAITING VLLM CONNECTION...
      </div>
    );
  }

  return (
    <div className={`iron-road-layout ${activeTab === 'ironroad' ? 'zen-mode' : ''}`}>
      <NavBar
        quest={quest}
        activeTab={activeTab}
        onTabChange={(tab) => {
          // Handle chariot doc requests: 'chariot:FILENAME.md'
          if (tab.startsWith('chariot:')) {
            setChariotDoc(tab.split(':')[1]);
            setActiveTab('chariot');
          } else {
            setActiveTab(tab);
          }
        }}
        onNewJourney={newJourney}
      />

      {activeTab === 'chariot' && chariotDoc ? (
        chariotDoc === 'PLAYERS_HANDBOOK.md' ? (
          <PlayerHandbookElearning onBack={() => setActiveTab('ironroad')} />
        ) : chariotDoc === 'ASK_PETE_FIELD_MANUAL.md' ? (
          <FieldManualViewer onBack={() => setActiveTab('ironroad')} />
        ) : (
          <ChariotViewer
            filename={chariotDoc}
            onBack={() => setActiveTab('ironroad')}
          />
        )
      ) : activeTab === 'portfolio' ? (
        <div style={{ gridColumn: '1 / -1', gridRow: 2, overflow: 'auto', display: 'flex', flexDirection: 'column', gap: '32px', paddingBottom: '40px' }}>
          <CharacterSheet />
          <PortfolioView />
        </div>
      ) : activeTab === 'scorecard' ? (
        <div style={{ gridColumn: '1 / -1', gridRow: 2, overflow: 'auto' }}>
          <QualityScorecard />
        </div>
      ) : activeTab === 'journal' ? (
        <div style={{ gridColumn: '1 / -1', gridRow: 2, overflow: 'auto' }}>
          <JournalViewer />
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
          <div style={{ display: activeTab === 'art' ? 'block' : 'none', gridColumn: '1 / -1', gridRow: 2, overflow: 'auto' }}>
            <ArtStudio />
          </div>

          {activeTab === 'ironroad' && (
            <>
              <PhaseWorkspace
                quest={quest}
                bestiary={bestiary}
                sseEvents={events}
                onDismissEvent={dismissEvent}
                onRefetch={refetch}
                viewPhase={viewPhase}
                allPhases={phases}
                onClearView={clearViewPhase}
              />
            </>
          )}
        </>
      ) : (
        <>
          <div className="full-span-tab">
            <SubjectPicker onSelect={startJourney} />
          </div>
        </>
      )}

      <ActivityBar />
      <footer className="footer">
        {/* Left: System Telemetry (Proximity: hardware status grouped together) */}
        <div className="footer-group footer-left">
          <span className="footer__label">TRINITY ID AI OS</span>
          <span className="sep">|</span>
          <span style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
            📡 <span style={{ color: 'var(--green)' }}>ONLINE</span>
          </span>
          <span className="sep">|</span>
          <span title="Active Inference Engine">🚂 {inferenceModel}</span>
        </div>

        {/* Center: Context (Alignment: Dead Center for current active work) */}
        <div className="footer-group footer-center">
          <span>{quest?.subject || 'No active subject'}</span>
          <span className="sep">|</span>
          <span>Phase: {quest?.phase || '—'}</span>
          <span className="sep">|</span>
          <span>XP: {quest?.xp_earned || 0}</span>
        </div>

        {/* Right: Actionable OS Modes (Proximity: clickable buttons isolated) */}
        <div className="footer-group footer-right">
          <div className="mode-toggle">
            {[['iron_road', '🚂', 'Iron Road', 'Iron Road (Guided Journey)'], ['express', '⚡', 'Express', 'Express Mode (Auto-generate)'], ['yardmaster', '🔧', 'Workshop', 'Yardmaster (System Tools)']].map(([m, icon, label, title]) => (
              <button
                key={m}
                title={title}
                className={`mode-toggle__btn ${appMode === m ? 'mode-toggle__btn--active' : ''}`}
                onClick={() => {
                  setAppMode(m);
                  if (m === 'yardmaster') setActiveTab('yard');
                  else if (m === 'iron_road') setActiveTab('ironroad');
                  else if (m === 'express') setActiveTab('express');
                  
                  fetch('/api/mode', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ mode: m }),
                  }).catch(() => {});
                }}
              >
                {icon} <span className="mode-toggle__label">{label}</span>
              </button>
            ))}
          </div>
        </div>
      </footer>
      <OnboardingTour />

    </div>
  );
}
