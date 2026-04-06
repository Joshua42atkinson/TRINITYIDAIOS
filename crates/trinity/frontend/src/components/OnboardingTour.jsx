import React, { useState, useEffect } from 'react';

/**
 * OnboardingTour — 3-step tooltip overlay for first-time users.
 * Uses localStorage to show only once per browser.
 * Pure CSS tooltips with arrow pointers, no external library.
 */

const TOUR_STEPS = [
  {
    target: '#chat-input',
    title: '💬 Talk to Pete',
    message: "This is where you chat with Pete, your AI conductor. He'll guide you through building your instructional game — just type naturally.",
    position: 'top',
  },
  {
    target: '#objectives-toggle',
    title: '📜 Quest Objectives',
    message: "These are your quest goals. Complete each one to advance through the 12 stations of the Iron Road — from Analysis to Evolve.",
    position: 'bottom',
  },
  {
    target: '#export-quiz-btn',
    title: '📦 Export Your Creation',
    message: "When you're ready, export your work as an HTML5 quiz or adventure game. Hand it to your students — no installation needed.",
    position: 'bottom',
  },
];

const LS_KEY = 'trinity_onboarding_done';

export default function OnboardingTour() {
  const [step, setStep] = useState(-1); // -1 = not started
  const [pos, setPos] = useState({ top: 0, left: 0 });

  useEffect(() => {
    // Check localStorage — only show once
    if (localStorage.getItem(LS_KEY)) return;

    // Delay start so the page renders first
    const timer = setTimeout(() => setStep(0), 1200);
    return () => clearTimeout(timer);
  }, []);

  useEffect(() => {
    if (step < 0 || step >= TOUR_STEPS.length) return;

    const s = TOUR_STEPS[step];
    const el = document.querySelector(s.target);
    if (!el) return;

    const rect = el.getBoundingClientRect();
    const scrollY = window.scrollY || window.pageYOffset;
    const scrollX = window.scrollX || window.pageXOffset;

    if (s.position === 'top') {
      setPos({
        top: rect.top + scrollY - 10,
        left: rect.left + scrollX + rect.width / 2,
      });
    } else {
      setPos({
        top: rect.bottom + scrollY + 10,
        left: rect.left + scrollX + rect.width / 2,
      });
    }

    // Highlight the target element
    el.style.position = 'relative';
    el.style.zIndex = '10001';
    el.style.boxShadow = '0 0 0 4px rgba(207, 185, 145, 0.5), 0 0 20px rgba(207, 185, 145, 0.3)';
    el.style.borderRadius = '8px';

    return () => {
      el.style.zIndex = '';
      el.style.boxShadow = '';
      el.style.borderRadius = '';
    };
  }, [step]);

  const advance = () => {
    const next = step + 1;
    if (next >= TOUR_STEPS.length) {
      setStep(-1);
      localStorage.setItem(LS_KEY, 'true');
    } else {
      setStep(next);
    }
  };

  const dismiss = () => {
    setStep(-1);
    localStorage.setItem(LS_KEY, 'true');
  };

  if (step < 0 || step >= TOUR_STEPS.length) return null;

  const current = TOUR_STEPS[step];
  const isAbove = current.position === 'top';

  return (
    <>
      {/* Backdrop */}
      <div
        className="onboarding-backdrop"
        style={{
          position: 'fixed',
          inset: 0,
          background: 'rgba(0, 0, 0, 0.6)',
          zIndex: 10000,
        }}
        onClick={dismiss}
      />

      {/* Tooltip */}
      <div
        className="onboarding-tooltip"
        style={{
          position: 'absolute',
          top: isAbove ? pos.top : pos.top,
          left: pos.left,
          transform: isAbove
            ? 'translate(-50%, -100%)'
            : 'translate(-50%, 0)',
          zIndex: 10002,
          background: '#14141e',
          border: '1px solid rgba(207, 185, 145, 0.4)',
          borderRadius: '12px',
          padding: '1.25rem 1.5rem',
          maxWidth: '320px',
          boxShadow: '0 8px 32px rgba(0, 0, 0, 0.5), 0 0 40px rgba(207, 185, 145, 0.1)',
          animation: 'onboardingFadeIn 0.4s ease-out',
        }}
      >
        <div style={{ color: '#cfb991', fontWeight: 700, fontSize: '1rem', marginBottom: '0.5rem' }}>
          {current.title}
        </div>
        <div style={{ color: '#c0baa8', fontSize: '0.875rem', lineHeight: 1.6, marginBottom: '1rem' }}>
          {current.message}
        </div>
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <span style={{ color: '#666', fontSize: '0.75rem' }}>
            {step + 1} of {TOUR_STEPS.length}
          </span>
          <div style={{ display: 'flex', gap: '0.5rem' }}>
            <button
              onClick={dismiss}
              style={{
                background: 'transparent',
                border: '1px solid #333',
                borderRadius: '6px',
                color: '#888',
                padding: '0.35rem 0.75rem',
                fontSize: '0.8rem',
                cursor: 'pointer',
              }}
            >
              Skip
            </button>
            <button
              onClick={advance}
              style={{
                background: 'linear-gradient(135deg, #8b6914, #cfb991)',
                border: 'none',
                borderRadius: '6px',
                color: '#0a0a0f',
                padding: '0.35rem 0.75rem',
                fontSize: '0.8rem',
                fontWeight: 600,
                cursor: 'pointer',
              }}
            >
              {step === TOUR_STEPS.length - 1 ? 'Done' : 'Next →'}
            </button>
          </div>
        </div>

        {/* Arrow */}
        <div
          style={{
            position: 'absolute',
            width: 0,
            height: 0,
            ...(isAbove
              ? {
                  bottom: '-8px',
                  left: '50%',
                  transform: 'translateX(-50%)',
                  borderLeft: '8px solid transparent',
                  borderRight: '8px solid transparent',
                  borderTop: '8px solid rgba(207, 185, 145, 0.4)',
                }
              : {
                  top: '-8px',
                  left: '50%',
                  transform: 'translateX(-50%)',
                  borderLeft: '8px solid transparent',
                  borderRight: '8px solid transparent',
                  borderBottom: '8px solid rgba(207, 185, 145, 0.4)',
                }),
          }}
        />
      </div>

      <style>{`
        @keyframes onboardingFadeIn {
          from { opacity: 0; transform: translate(-50%, ${isAbove ? 'calc(-100% + 10px)' : '-10px'}); }
          to { opacity: 1; transform: translate(-50%, ${isAbove ? '-100%' : '0'}); }
        }
      `}</style>
    </>
  );
}
