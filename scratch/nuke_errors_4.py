import re

def replace_in_file(path, old, new):
    with open(path, 'r') as f: c = f.read()
    c = c.replace(old, new)
    with open(path, 'w') as f: f.write(c)


# 1. agent.rs
replace_in_file("crates/trinity/src/agent.rs", "trinity_protocol::character_sheet::ShadowStatus", "trinity_quest::ShadowStatus")

# 2. character_api.rs
replace_in_file("crates/trinity/src/character_api.rs", "trinity_protocol::CharacterSheet", "trinity_quest::CharacterSheet")

# 3. conductor_leader.rs
replace_in_file("crates/trinity/src/conductor_leader.rs", "trinity_protocol::QmRubricEvaluator", "trinity_quest::QmRubricEvaluator")

# 4. authenticity_scorecard.rs
with open("crates/trinity/src/authenticity_scorecard.rs", 'r') as f: lines = f.readlines()
for i in range(len(lines)):
    if "for word in sheet.vaam_profile.word_weights.keys()" in lines[i]:
        lines[i+1] = "        let lw = word.to_string().to_lowercase(); if lower.contains(&lw) {\n"
with open("crates/trinity/src/authenticity_scorecard.rs", 'w') as f: f.writelines(lines)

