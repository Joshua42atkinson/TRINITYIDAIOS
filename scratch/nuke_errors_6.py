def replace_in_file(path, old, new):
    with open(path, 'r') as f: c = f.read()
    c = c.replace(old, new)
    with open(path, 'w') as f: f.write(c)

replace_in_file("crates/trinity/src/main.rs", ".expect(\"error while running tauri application\");", "// .expect(\"error while running tauri application\");")
