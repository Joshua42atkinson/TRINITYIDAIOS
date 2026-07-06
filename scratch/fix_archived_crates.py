# Fix agent.rs type annotations
filepath = "/home/joshua/Workflow/TRINITYIDAIOS/crates/trinity/src/agent.rs"
with open(filepath, 'r') as f: content = f.read()
content = content.replace(".filter(|e| !e.is_empty())", ".filter(|e: &String| !e.is_empty())")
content = content.replace(".filter(|a| !a.is_empty())", ".filter(|a: &String| !a.is_empty())")
with open(filepath, 'w') as f: f.write(content)

# Fix main.rs type annotations
filepath = "/home/joshua/Workflow/TRINITYIDAIOS/crates/trinity/src/main.rs"
with open(filepath, 'r') as f: content = f.read()
content = content.replace("let sheet = state.player.character_sheet.read().await;", "let sheet: tokio::sync::RwLockReadGuard<'_, trinity_quest::CharacterSheet> = state.player.character_sheet.read().await;")
content = content.replace("let mut sheet = state.player.character_sheet.write().await;", "let mut sheet: tokio::sync::RwLockWriteGuard<'_, trinity_quest::CharacterSheet> = state.player.character_sheet.write().await;")
content = content.replace("experience.as_deref(),", "experience.as_deref().unwrap_or(\"\"),")
content = content.replace("audience.as_deref(),", "audience.as_deref().unwrap_or(\"\"),")
with open(filepath, 'w') as f: f.write(content)
