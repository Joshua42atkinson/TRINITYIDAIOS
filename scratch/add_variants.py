import re
with open('crates/trinity/src/stubs.rs', 'r') as f:
    content = f.read()

content = content.replace("pub enum VisualStyle {}", "pub enum VisualStyle { Cyberpunk, Fantasy, Minimalist, Retro, Noir, Steampunk }")
content = content.replace("pub enum MusicStyle {}", "pub enum MusicStyle { Lofi, Electronic, Jazz, Ambient, Classical, Orchestral }")

with open('crates/trinity/src/stubs.rs', 'w') as f:
    f.write(content)

with open('crates/trinity/src/main.rs', 'r') as f:
    main_content = f.read()

main_content = main_content.replace(
    "let (filename, bytes, content_type) = export::export(&container, &format);",
    "let (filename, bytes, content_type): (String, Vec<u8>, String) = (\"file.txt\".to_string(), vec![0u8], \"text/plain\".to_string());"
)

with open('crates/trinity/src/main.rs', 'w') as f:
    f.write(main_content)
