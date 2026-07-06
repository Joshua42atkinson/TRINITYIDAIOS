import re

def replace_in_file(path, old, new):
    with open(path, 'r') as f: c = f.read()
    c = c.replace(old, new)
    with open(path, 'w') as f: f.write(c)


# 1. Main.rs tauri and IntentPosture
replace_in_file("crates/trinity/src/main.rs", ".run(tauri::generate_context!())", "// .run(tauri::generate_context!())")
replace_in_file("crates/trinity/src/main.rs", "trinity_protocol::character_sheet::IntentPosture", "trinity_quest::IntentPosture")

# 2. rlhf_api.rs
replace_in_file("crates/trinity/src/rlhf_api.rs", "trinity_protocol::character_sheet::ShadowStatus", "trinity_quest::ShadowStatus")

# 3. character_sheet.rs
replace_in_file("crates/trinity/src/character_sheet.rs", "trinity_protocol::CharacterSheet", "trinity_quest::CharacterSheet")

# 4. character_api.rs
replace_in_file("crates/trinity/src/character_api.rs", "trinity_protocol::character_sheet::PortfolioArtifact", "trinity_quest::PortfolioArtifact")
replace_in_file("crates/trinity/src/character_api.rs", "trinity_protocol::character_sheet::SkillType", "trinity_quest::SkillType")

# 5. authenticity_scorecard.rs
replace_in_file("crates/trinity/src/authenticity_scorecard.rs", "trinity_protocol::CharacterSheet", "trinity_quest::CharacterSheet")
with open("crates/trinity/src/authenticity_scorecard.rs", 'r') as f: lines = f.readlines()
for i in range(len(lines)):
    if "for word in sheet.vaam_profile.word_weights.keys()" in lines[i]:
        lines[i+1] = "        let lw: String = format!(\"{}\", word).to_lowercase(); if lower.contains(&lw) {\n"
with open("crates/trinity/src/authenticity_scorecard.rs", 'w') as f: f.writelines(lines)

# 6. quests.rs
replace_in_file("crates/trinity/src/quests.rs", "trinity_protocol::SkillType", "trinity_quest::SkillType")

# 7. voice.rs
replace_in_file("crates/trinity/src/voice.rs", "trinity_protocol::character_sheet::VoiceEmotion", "trinity_quest::VoiceEmotion")

# 8. agent.rs
replace_in_file("crates/trinity/src/agent.rs", "trinity_protocol::CharacterSheet", "trinity_quest::CharacterSheet")

