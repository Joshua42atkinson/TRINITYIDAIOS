import { useState, useEffect } from 'react';

// Fallback phases — used only until the first /api/quest response arrives.
// The API now sends the canonical phases array from hero.rs::Phase.
const FALLBACK_PHASES = [
  { name: 'Analysis',       blooms: 'Remember',   icon: '🔍', group: 'ADDIE', circuit: 'Scope' },
  { name: 'Design',         blooms: 'Understand', icon: '📐', group: 'ADDIE', circuit: 'Scope' },
  { name: 'Development',    blooms: 'Apply',      icon: '🛠️', group: 'ADDIE', circuit: 'Scope' },
  { name: 'Implementation', blooms: 'Apply',      icon: '🚀', group: 'ADDIE', circuit: 'Scope' },
  { name: 'Evaluation',     blooms: 'Analyze',    icon: '📊', group: 'ADDIE', circuit: 'Scope' },
  { name: 'Contrast',       blooms: 'Analyze',    icon: '⊕',  group: 'CRAP',  circuit: 'Build' },
  { name: 'Repetition',     blooms: 'Evaluate',   icon: '⧉',  group: 'CRAP',  circuit: 'Build' },
  { name: 'Alignment',      blooms: 'Evaluate',   icon: '⌬',  group: 'CRAP',  circuit: 'Build' },
  { name: 'Proximity',      blooms: 'Create',     icon: '◈',  group: 'CRAP',  circuit: 'Build' },
  { name: 'Envision',       blooms: 'Evaluate',   icon: '👁️', group: 'EYE',   circuit: 'Ship' },
  { name: 'Yoke',           blooms: 'Create',     icon: '🔗', group: 'EYE',   circuit: 'Ship' },
  { name: 'Evolve',         blooms: 'Create',     icon: '🌱', group: 'EYE',   circuit: 'Ship' },
];

export function useQuest() {
  const [quest, setQuest] = useState(null);
  const [loading, setLoading] = useState(true);

  const fetchQuest = async () => {
    try {
      const res = await fetch('/api/quest');
      if (!res.ok) return;
      const data = await res.json();
      setQuest(data);
    } catch { /* silent */ }
    finally { setLoading(false); }
  };

  useEffect(() => {
    fetchQuest();
    const iv = setInterval(fetchQuest, 5000);
    return () => clearInterval(iv);
  }, []);

  // Use API phases (source of truth from hero.rs) or fallback before first fetch
  const phases = quest?.phases || FALLBACK_PHASES;

  // Use API phase_index (direct from backend) or fall back to string matching
  const currentPhaseIndex = quest?.phase_index ?? (quest
    ? phases.findIndex((p) => p.name === quest.phase)
    : 0);

  return {
    quest,
    loading,
    phases,
    currentPhaseIndex,
    refetch: fetchQuest,
  };
}
