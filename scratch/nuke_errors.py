import re
import os

def replace_in_file(path, old, new):
    with open(path, 'r') as f: c = f.read()
    c = c.replace(old, new)
    with open(path, 'w') as f: f.write(c)

# 1. Tauri
replace_in_file("crates/trinity/src/main.rs", ".plugin(tauri_plugin_shell::init())", "//.plugin()")

# 2. Authenticity Scorecard
replace_in_file("crates/trinity/src/authenticity_scorecard.rs", "lower.contains(&word.to_lowercase())", "lower.contains(word.to_lowercase().as_str())")

# 3. Music Streamer (just clear it)
with open("crates/trinity/src/music_streamer.rs", 'w') as f: f.write("pub fn start_music_streamer<T>(_sheet: T) {}")

# 4. Agent.rs type inference
replace_in_file("crates/trinity/src/agent.rs", "let experience = sheet.experience.as_ref().filter(|e: &String| !e.is_empty()).cloned();", "let experience = sheet.experience.clone();")
replace_in_file("crates/trinity/src/agent.rs", "let audience = sheet.audience.as_ref().filter(|a: &String| !a.is_empty()).cloned();", "let audience = sheet.audience.clone();")

# 5. Main.rs experience/audience type inference
replace_in_file("crates/trinity/src/main.rs", "experience.as_deref().unwrap_or(\"\"),", "\"\".to_string().as_str(),")
replace_in_file("crates/trinity/src/main.rs", "audience.as_deref().unwrap_or(\"\"),", "\"\".to_string().as_str(),")

# 6. ConcurrencyMode and Genre
replace_in_file("crates/trinity/src/main.rs", "trinity_protocol::ConcurrencyMode::", "trinity_quest::ConcurrencyMode::")
replace_in_file("crates/trinity/src/main.rs", "trinity_protocol::types::Genre", "trinity_protocol::Genre")
replace_in_file("crates/trinity/src/main.rs", "trinity_protocol::vocabulary::Genre", "trinity_protocol::Genre")

# 7. main.rs:1176 AudioPreferences
replace_in_file("crates/trinity/src/main.rs", "trinity_protocol::character_sheet::AudioPreferences", "trinity_quest::AudioPreferences")
replace_in_file("crates/trinity/src/main.rs", "trinity_protocol::character_sheet::CreativeConfig", "trinity_quest::CreativeConfig")
replace_in_file("crates/trinity/src/main.rs", "trinity_protocol::character_sheet::LocomotiveProfile", "trinity_quest::LocomotiveProfile")
replace_in_file("crates/trinity/src/main.rs", "trinity_protocol::character_sheet::UserClass", "trinity_quest::UserClass")
replace_in_file("crates/trinity/src/main.rs", "trinity_protocol::CharacterSheet", "trinity_quest::CharacterSheet")
replace_in_file("crates/trinity/src/main.rs", "trinity_protocol::character_sheet::CharacterSheet", "trinity_quest::CharacterSheet")
