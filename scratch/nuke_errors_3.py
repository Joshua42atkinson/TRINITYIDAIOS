import re

def replace_in_file(path, old, new):
    with open(path, 'r') as f: c = f.read()
    c = c.replace(old, new)
    with open(path, 'w') as f: f.write(c)


# 1. Creative.rs PortfolioArtifact and VisualStyle
with open("crates/trinity/src/creative.rs", 'r') as f: c = f.read()
c = c.replace("trinity_protocol::character_sheet::PortfolioArtifact", "trinity_quest::PortfolioArtifact")
c = c.replace("trinity_protocol::character_sheet::VisualStyle::", "trinity_quest::VisualStyle::")
c = c.replace("trinity_protocol::character_sheet::MusicStyle::", "trinity_quest::MusicStyle::")
with open("crates/trinity/src/creative.rs", 'w') as f: f.write(c)

# 2. Authenticity Scorecard word type inference
replace_in_file("crates/trinity/src/authenticity_scorecard.rs", "let lw = word.to_lowercase();", "let lw: String = word.to_string().to_lowercase();")

