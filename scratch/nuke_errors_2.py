import re

def replace_in_file(path, old, new):
    with open(path, 'r') as f: c = f.read()
    c = c.replace(old, new)
    with open(path, 'w') as f: f.write(c)

# 1. tauri::Builder::default()
replace_in_file("crates/trinity/src/main.rs", "tauri::Builder::default()", "// tauri::Builder::default()")

# 2. Main.rs experience/audience type inference fix
replace_in_file("crates/trinity/src/main.rs", "\" \".to_string().as_str(),", "None,")
replace_in_file("crates/trinity/src/main.rs", "\" \".to_string().as_str(),", "None,")
replace_in_file("crates/trinity/src/main.rs", "\"\"\".to_string().as_str(),", "None,")
replace_in_file("crates/trinity/src/main.rs", "\"\"\".to_string().as_str(),", "None,")
replace_in_file("crates/trinity/src/main.rs", "\" \",", "None,")

with open("crates/trinity/src/main.rs", 'r') as f: c = f.read()
c = c.replace('experience.as_deref().unwrap_or(""),', 'experience.as_deref(),')
c = c.replace('audience.as_deref().unwrap_or(""),', 'audience.as_deref(),')
c = c.replace('"".to_string().as_str(),', 'None,')
with open("crates/trinity/src/main.rs", 'w') as f: f.write(c)

# 3. Authenticity Scorecard
replace_in_file("crates/trinity/src/authenticity_scorecard.rs", "if lower.contains(word.to_lowercase().as_str()) {", "let lw = word.to_lowercase(); if lower.contains(lw.as_str()) {")

# 4. Creative.rs MusicStyle
with open("crates/trinity/src/creative.rs", 'r') as f: c = f.read()
c = c.replace("trinity_protocol::character_sheet::MusicStyle::", "trinity_quest::MusicStyle::")
c = c.replace("MusicStyle::MusicStyle::", "MusicStyle::")
with open("crates/trinity/src/creative.rs", 'w') as f: f.write(c)
