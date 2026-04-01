import React, { useState, useEffect } from 'react';
import '../styles/hook_deck.css';

const HOOK_ICONS = {
  'Pearl': '🔮',
  'Coal': '🪨',
  'Steam': '💨',
  'Hook': '🪝',
  'Mirror': '🪞',
  'Compass': '🧭'
};

export default function GlobalDeckOverlay() {
  const [deck, setDeck] = useState([]);

  useEffect(() => {
    // Poll the Character Sheet / LDT Portfolio every 3 seconds to see XP/Levelling progress
    const fetchCharacter = () => {
      fetch('/api/character')
        .then(r => r.json())
        .then(data => {
            if (data?.ldt_portfolio?.hook_deck) {
                // sort by Level, then XP
                const cards = Object.values(data.ldt_portfolio.hook_deck)
                  .sort((a,b) => b.level - a.level || b.xp - a.xp);
                setDeck(cards);
            }
        })
        .catch(() => {});
    };
    
    fetchCharacter();
    const iv = setInterval(fetchCharacter, 3000);
    return () => clearInterval(iv);
  }, []);

  if (deck.length === 0) return null;

  return (
    <div className="global-deck-wrapper">
      {deck.map((card, i) => (
        <div 
          key={card.id} 
          className="hook-card"
          draggable="true"
          onDragStart={(e) => {
            // Encode the Sserde-compatible Command Payload directly into the drag event!
            const payload = {
              command: "CastHook",
              params: { hook: card.id }
            };
            e.dataTransfer.setData('application/json', JSON.stringify(payload));
            e.dataTransfer.setData('text/plain', `CastHook:${card.id}`);
            
            // Generate a subtle ghost for the drag
            const rect = e.target.getBoundingClientRect();
            e.dataTransfer.setDragImage(e.target, rect.width / 2, rect.height / 2);
          }}
          title={`${card.title} - Socratic Mechanic`}
        >
          <div className="hook-card__icon">{HOOK_ICONS[card.id] || '🃏'}</div>
          <div className="hook-card__title">{card.title}</div>
          <div className="hook-card__level">LVL {card.level}</div>
          <div className="hook-card__xp">{card.xp} / {(card.level * 100)} XP</div>
        </div>
      ))}
    </div>
  );
}
