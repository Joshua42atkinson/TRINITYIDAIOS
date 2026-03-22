# UNIFIED UI PLAN — Single Page Trinity
## Goal: One screen to rule them all

---

## Problem
- 4 separate HTML pages with no visual continuity
- Yardmaster chat can't show generated files/images
- User can't browse files from the web UI
- No way to see ART output without leaving the chat
- Iron Road book is isolated from the creative tools
- Non-technical Purdue evaluators need ONE simple interface

## Solution: Single Scrollable Page

```
┌─────────────────────────────────────────────────────────────┐
│  TRINITY ID AI OS                            ● Mistral 256K │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌─────────────────────────────────────────────────────────┐│
│  │  IRON ROAD (Pete)                                       ││
│  │  ┌─────────────────┐  ┌──────────────────────────────┐ ││
│  │  │  ADDIECRAPEYE    │  │  Book / Quest View           │ ││
│  │  │  Phase Tracker   │  │  Current chapter, objectives │ ││
│  │  │  12 stations     │  │  Coal/Steam/XP bars          │ ││
│  │  │  Progress dots   │  │  Creep bestiary sidebar      │ ││
│  │  └─────────────────┘  └──────────────────────────────┘ ││
│  └─────────────────────────────────────────────────────────┘│
│                                                              │
│  ┌─────────────────────────────────────────────────────────┐│
│  │  ART STUDIO (Preview + Gallery)                         ││
│  │  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐   ││
│  │  │ Latest Image  │ │ Avatar Card  │ │ 3D Preview   │   ││
│  │  │ (from ComfyUI)│ │ (portrait +  │ │ (GLB viewer) │   ││
│  │  │              │ │  stats)      │ │              │   ││
│  │  └──────────────┘ └──────────────┘ └──────────────┘   ││
│  │  Audio player: [▶ theme.wav]  [▶ voice_sample.wav]     ││
│  └─────────────────────────────────────────────────────────┘│
│                                                              │
│  ┌─────────────────────────────────────────────────────────┐│
│  │  YARDMASTER (Chat + Files)                              ││
│  │  ┌───────────────────────┐  ┌────────────────────────┐ ││
│  │  │  Chat                 │  │  File Browser           │ ││
│  │  │  [📊][🎭][🖼️][📜][🔧]│  │  📁 crates/            │ ││
│  │  │                       │  │  📁 assets/avatars/     │ ││
│  │  │  User: build oxtapus  │  │  📁 quests/board/       │ ││
│  │  │  Trinity: I can create │  │  📁 docs/               │ ││
│  │  │  that! Want me to go  │  │                          │ ││
│  │  │  ahead?               │  │  Click file → view in   │ ││
│  │  │  User: do it          │  │  editor panel or image  │ ││
│  │  │  Trinity: ▶ avatar... │  │  preview                │ ││
│  │  │                       │  │                          │ ││
│  │  │  [What do you need?]  │  │                          │ ││
│  │  └───────────────────────┘  └────────────────────────┘ ││
│  └─────────────────────────────────────────────────────────┘│
│                                                              │
│  ┌─────────────────────────────────────────────────────────┐│
│  │  STATUS BAR                                             ││
│  │  LLM: ● Mistral 256K | ComfyUI: ● | Voice: ● | PG: ●  ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

## Key Design Decisions

1. **Scroll-based navigation** — Iron Road at top, ART in middle, Yardmaster at bottom
   - Anchors: `#ironroad`, `#art`, `#yardmaster`
   - Sticky nav with 3 section buttons

2. **ART preview is INLINE** — generated images/audio appear in the ART section automatically
   - When avatar_pipeline completes → portrait appears in ART gallery
   - When generate_image completes → image appears in ART preview
   - Audio player for theme.wav and voice_sample.wav

3. **File browser is REAL** — click to view/edit files
   - Shows: crates/, assets/avatars/, quests/, docs/
   - Click .rs/.md/.json → opens in inline editor (code-area textarea)
   - Click .png → shows inline image preview
   - Click .wav → plays in audio player

4. **Consent gates in chat** — Trinity asks before acting
   - "I'll create an Oxtapus avatar with steampunk style. Want me to go ahead?"
   - User types "yes" / "do it" / "go ahead" → tool executes
   - Results appear in ART section automatically

5. **Visual continuity** — one dark theme, one font stack, consistent spacing
   - Zed-inspired dark palette (already in dev.html CSS)
   - JetBrains Mono for code, Inter for text
   - Yardmaster green (#4EC9B0) as accent throughout

## Implementation Plan (Next Session)

### Phase 1: Merge HTML (2-3 hours)
- Create `/static/index.html` as the unified page
- Port Iron Road book view from `ironroad.html`
- Port ART preview section (new — image gallery + audio player)
- Port Yardmaster chat from `dev.html`
- Add file browser panel (real directory listing via API)
- Add sticky nav with section anchors
- Keep old pages as legacy redirects

### Phase 2: Wire ART Preview (1-2 hours)
- When avatar_pipeline completes → auto-refresh ART gallery
- Serve generated images via `/api/creative/gallery`
- Add audio player component for .wav files
- Add image lightbox for portrait viewing

### Phase 3: File Browser (1-2 hours)
- Click directory → expand tree (lazy load via list_dir API)
- Click file → load into inline viewer
- .rs/.md/.toml → code editor with syntax highlighting
- .png/.jpg → image preview
- .wav → audio player
- .json → formatted JSON view

### Phase 4: Polish (1 hour)
- Responsive layout for different screen sizes
- Loading states (spinner while LLM thinks)
- Keyboard shortcuts (Ctrl+Enter to send, Escape to cancel)
- Session persistence (scroll position, active focuses)

## Non-Goals (this iteration)
- Bevy 3D viewport in browser (future L3)
- Real-time collaborative editing
- Terminal emulator in browser
- Drag-and-drop file upload

---

*Created: March 19, 2026*
*For: Next session UI refactor*
