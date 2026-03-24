# TRINITY Task Queue — March 24, 2026

**Source**: `docs/MATURATION_MAP.md`  
**Updated**: 11:42 AM ET  
**Session**: Red Hat Hardening + Identity Split + Physics Engine

---

## Completed This Session ✅

| # | Task | Tier | Status |
|---|------|:----:|:------:|
| 1 | Bind Trinity to `127.0.0.1:3000` | T1 | ✅ |
| 2 | Restrict CORS to allowlist | T1 | ✅ |
| 3 | Harden Caddyfile — block all dangerous API routes | T1 | ✅ |
| 4 | Create `edge_guard.rs` middleware (33 blocked prefixes) | T1 | ✅ |
| 5 | Rate limiter: 60 req/min per IP for tunnel traffic | T1 | ✅ |
| 6 | Write 8 edge guard unit tests | T1 | ✅ |
| 7 | Create `trinity.service` systemd unit | T2 | ✅ |
| 8 | Create `cloudflared.service` systemd unit | T2 | ✅ |
| 9 | Create `llama-server.service` systemd unit | T2 | ✅ |
| 10 | Enable all 3 services for boot | T2 | ✅ |
| 11 | Rebuild LDTAtkinson portfolio (`npm run build`) | T2 | ✅ |
| 12 | Edge Guard: redirect tunnel `/` → `/portfolio/` | T2 | ✅ |
| 13 | Slim health endpoint for tunnel traffic | T2 | ✅ |
| 14 | **Create `PlayerContext` + `ProjectContext` structs** | T3.5 | ✅ |
| 15 | **Migrate `character_sheet` (24 sites, 7 files)** | T3.5 | ✅ |
| 16 | **Migrate `game_state`, `bestiary`, `app_mode`** | T3.5 | ✅ |
| 17 | **Migrate `conversation_history`, `book`, `book_updates`, `session_id`** | T3.5 | ✅ |
| 18 | **Remove legacy flat fields from AppState** | T3.5 | ✅ |
| 19 | **Wire RLHF → Shadow/Steam/Friction (Soft Spot 5)** | SS5 | ✅ |
| 20 | **Add `process_shadow` endpoint** | SS5 | ✅ |
| 21 | **Add `consecutive_negatives` field + `recalculate_vulnerability()`** | SS7 | ✅ |
| 22 | **Wire track friction reduction on phase advance (Soft Spot 6)** | SS6 | ✅ |
| 23 | **Brakeman MVP — edge_guard blocks tools for tunnel traffic** | SS10 | ✅ |
| 24 | Update `CONTEXT.md` | — | ✅ |

---

## Remaining

| # | Task | Tier | Notes | Status |
|---|------|:----:|-------|:------:|
| 25 | React UI 4-part identity split | T3.5-2 | Frontend awareness of Player/Project | ⬜ |
| 26 | Synchronize Bible (Car 1.6 App State) | T3.5-3 | Documentation alignment | ⬜ |
| 27 | Cloudflare Zero Trust config | T3 | Dashboard — manual, not code | ⬜ |
| 28 | `AppMode::Demo` variant | T3 | Auto-detect for tunnel visitors | ⬜ |
| 29 | RLHF → PEARL alignment scores (Soft Spot 8) | SS8 | Requires PearlPhase mapping | ⬜ |
| 30 | Knowledge Tracing skills update (Soft Spot 9) | SS9 | After portfolio artifact vault | ⬜ |
| 31 | 2D ART Gallery bypass (Soft Spot 11) | SS11 | Deferred to presentation polish | ⬜ |

---

## Stats

- **Tier 1**: ✅ Complete (4/4 acceptance)
- **Tier 2**: ✅ Complete (4/4 acceptance)
- **Tier 3.5 Backend**: ✅ Complete (identity split done)
- **Soft Spots 5,6,7,10**: ✅ Wired
- **Tests**: 205 passing, 0 failures
- **Server**: Healthy, release binary live

**Total tasks**: 31 | **Complete**: 24 | **Remaining**: 7
