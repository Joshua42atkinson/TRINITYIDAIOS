import React, { useState } from 'react';

export default function SetupWizard({ onComplete }) {
  const [step, setStep] = useState(1);
  const [engine, setEngine] = useState('lm_studio'); // lm_studio | ollama | custom
  const [customIp, setCustomIp] = useState('http://127.0.0.1:11434/v1/chat/completions');
  const [isTesting, setIsTesting] = useState(false);
  const [errorMsg, setErrorMsg] = useState('');

  const handleTestConnection = async () => {
    setIsTesting(true);
    setErrorMsg('');
    try {
      // We send the desired configuration to the Rust backend to save/test
      const payload = {
        backend: engine,
        custom_url: engine === 'custom' ? customIp : null
      };

      const res = await fetch('/api/config/setup', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      });
      
      // We always save their preference so the dynamic Ignite Furnace button knows what to boot,
      // even if the model isn't actively running right now.
      localStorage.setItem('trinitySetupState', JSON.stringify({ engine, customIp: engine === 'custom' ? customIp : null }));
      onComplete();
    } catch (err) {
      setErrorMsg(err.message);
    } finally {
      setIsTesting(false);
    }
  };

  return (
    <div style={styles.wizardContainer}>
      <div style={styles.wizardBox}>
        <div style={styles.logoRow}>
          <div style={styles.logoIcon}>🚂</div>
          <h1 style={styles.title}>TRINITY ID AI OS</h1>
        </div>

        {step === 1 && (
          <div style={styles.stepContent}>
            <h2 style={{ color: '#E2E8F0', marginTop: 0 }}>The Body Meets The Mind</h2>
            <p style={{ color: '#94A3B8', lineHeight: '1.6' }}>
              Welcome to Trinity. The system core (the "Brain") has successfully initialized its memory pathways. 
              Before you can begin your journey on the Iron Road, you must plug in an AI engine (the "Mind"). 
              Trinity is entirely agnostic—you can use any generative mind you already have installed on your laptop.
            </p>
            <button style={styles.primaryButton} onClick={() => setStep(2)}>
              Proceed to neural link
            </button>
          </div>
        )}

        {step === 2 && (
          <div style={styles.stepContent}>
            <h2 style={{ color: '#E2E8F0', marginTop: 0 }}>Plug In Your Mind</h2>
            <p style={{ color: '#94A3B8', marginBottom: '24px' }}>
              Which inference engine are you running in the background?
            </p>

            <div style={styles.radioGroup}>
              <label style={engine === 'lm_studio' ? styles.radioLabelActive : styles.radioLabel}>
                <input 
                  type="radio" 
                  checked={engine === 'lm_studio'} 
                  onChange={() => setEngine('lm_studio')}
                  style={{ marginRight: '10px' }}
                />
                <div style={{ display: 'flex', flexDirection: 'column' }}>
                  <span style={{ fontWeight: 'bold' }}>LM Studio (Local)</span>
                  <span style={{ fontSize: '0.8rem', opacity: 0.7 }}>Port 1234 — Seamless API</span>
                </div>
              </label>

              <label style={engine === 'ollama' ? styles.radioLabelActive : styles.radioLabel}>
                <input 
                  type="radio" 
                  checked={engine === 'ollama'} 
                  onChange={() => setEngine('ollama')}
                  style={{ marginRight: '10px' }}
                />
                <div style={{ display: 'flex', flexDirection: 'column' }}>
                  <span style={{ fontWeight: 'bold' }}>Ollama (Local)</span>
                  <span style={{ fontSize: '0.8rem', opacity: 0.7 }}>Port 11434 — Lightweight</span>
                </div>
              </label>

              <label style={engine === 'custom' ? styles.radioLabelActive : styles.radioLabel}>
                <input 
                  type="radio" 
                  checked={engine === 'custom'} 
                  onChange={() => setEngine('custom')}
                  style={{ marginRight: '10px' }}
                />
                <div style={{ display: 'flex', flexDirection: 'column' }}>
                  <span style={{ fontWeight: 'bold' }}>Custom Endpoint</span>
                  <span style={{ fontSize: '0.8rem', opacity: 0.7 }}>llama-server, any OpenAI-compatible, or remote</span>
                </div>
              </label>
            </div>

            {engine === 'custom' && (
              <input 
                type="text" 
                value={customIp}
                onChange={e => setCustomIp(e.target.value)}
                style={styles.customInput}
                placeholder="http://192.168.1.100:8080/v1/chat/completions"
              />
            )}

            {errorMsg && (
              <div style={{ color: '#EF4444', padding: '12px', background: '#451A1A', borderRadius: '4px', marginTop: '16px' }}>
                {errorMsg}
              </div>
            )}

            <div style={{ display: 'flex', gap: '16px', marginTop: '32px' }}>
              <button style={styles.secondaryButton} onClick={() => setStep(1)}>Back</button>
              <button 
                style={styles.primaryButton} 
                onClick={handleTestConnection}
                disabled={isTesting}
              >
                {isTesting ? 'Initiating handshake...' : 'Test Connection & Boot'}
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

const styles = {
  wizardContainer: {
    position: 'fixed',
    top: 0, left: 0, right: 0, bottom: 0,
    backgroundColor: '#0F172A',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    zIndex: 9999,
    fontFamily: '"SF Pro Display", -apple-system, sans-serif'
  },
  wizardBox: {
    background: '#1E293B',
    border: '1px solid #334155',
    borderRadius: '12px',
    padding: '40px',
    width: '100%',
    maxWidth: '600px',
    boxShadow: '0 25px 50px -12px rgba(0, 0, 0, 0.5)'
  },
  logoRow: {
    display: 'flex',
    alignItems: 'center',
    gap: '16px',
    marginBottom: '32px',
    borderBottom: '1px solid #334155',
    paddingBottom: '24px'
  },
  logoIcon: {
    fontSize: '3rem'
  },
  title: {
    color: '#F8FAFC',
    margin: 0,
    fontSize: '1.5rem',
    fontWeight: '600',
    letterSpacing: '1px'
  },
  stepContent: {
    animation: 'fadeIn 0.5s ease-out'
  },
  radioGroup: {
    display: 'flex',
    flexDirection: 'column',
    gap: '12px'
  },
  radioLabel: {
    display: 'flex',
    alignItems: 'center',
    padding: '16px',
    background: '#0F172A',
    border: '1px solid #334155',
    borderRadius: '8px',
    cursor: 'pointer',
    color: '#E2E8F0',
    transition: 'all 0.2s ease'
  },
  radioLabelActive: {
    display: 'flex',
    alignItems: 'center',
    padding: '16px',
    background: '#1E293B',
    border: '2px solid #38BDF8',
    borderRadius: '8px',
    cursor: 'pointer',
    color: '#F8FAFC',
    boxShadow: '0 0 10px rgba(56, 189, 248, 0.2)'
  },
  customInput: {
    marginTop: '16px',
    width: '100%',
    padding: '12px 16px',
    background: '#0F172A',
    border: '1px solid #334155',
    borderRadius: '6px',
    color: '#F8FAFC',
    fontFamily: 'monospace',
    outline: 'none'
  },
  primaryButton: {
    flex: 1,
    background: '#0284C7',
    color: 'white',
    border: 'none',
    padding: '14px 24px',
    borderRadius: '6px',
    fontSize: '1rem',
    fontWeight: '500',
    cursor: 'pointer',
    transition: 'background 0.2s ease',
    width: '100%'
  },
  secondaryButton: {
    background: '#334155',
    color: 'white',
    border: 'none',
    padding: '14px 24px',
    borderRadius: '6px',
    fontSize: '1rem',
    cursor: 'pointer'
  }
};
