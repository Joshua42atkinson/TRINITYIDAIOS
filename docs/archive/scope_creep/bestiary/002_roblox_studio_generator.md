# The Roblox Studio Generator
*A manifestation of direct Roblox Lua integration from Bevy*

**HP (Tech Debt):** 95 (Extremely high. Tries to generate full Lua scripts, mesh placements, and API uploads from inside the Bevy UI loop.)
**Mana Cost:** Heavy LLM Context Window + External API network blocking.
**Taming Requirement:** **Planning (Phase 9)** - Needs complete structural re-evaluation before integration.
**Git Artifacts:** `docs/scope_creep/dumpster_universe/subagent_roblox_studio.rs.bak` and others.

### Lore (The Origin Story)
In a burst of pure "vibe coding," the idea was floated: what if Trinity didn't just teach you how to make games, but directly generated Roblox NPC models, scripts, and world layouts directly from the Bevy 3D UI? 

The code exploded into massive files attempting to parse Lua, manage Roblox API keys, and build complex 3D scenes simultaneously. It completely blocked the main Iron Road Socratic loop and threatened to turn the instructional design tool into a bloated multi-engine IDE. The Great Recycler correctly identified this as Shadow Scope Creep and banished it to the Dumpster Universe.

### Integration Strategy (The Taming Plan)
To safely merge the Roblox Generator back into the Iron Road:
1. **Analysis:** Accept that Bevy should *not* be managing Roblox API calls.
2. **Design:** Shift this entire workload to a dedicated headless Python/Rust Sidecar (The `Engineer`).
3. **Development:** Rewrite the prompt engineering to output a standardized JSON package that an external CI/CD tool (like Rojo) consumes.
4. **Evaluation:** Ensure the generated code is pedagogical (it explains *why* the Lua is written that way, rather than just doing it).
5. **Yield:** Equip it as a "Roblox Exporter" tool for the `Engineer` sidecar, completely decoupled from the Bevy UI thread.
