use std::process::Command;
use std::fs;
use std::path::PathBuf;

fn get_workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap().to_path_buf()
}

fn validate_template(genre: &str) {
    let workspace_root = get_workspace_root();
    let template_dir = workspace_root.join(format!("templates/bevy_{}", genre));
    
    assert!(template_dir.exists(), "Template directory missing for genre: {}", genre);

    // Create temp directory for validation using std::env and uuid
    let test_id = uuid::Uuid::new_v4();
    let game_dir = std::env::temp_dir().join(format!("trinity_test_{}_{}", genre, test_id));
    let src_dir = game_dir.join("src");
    let assets_dir = game_dir.join("assets");
    
    fs::create_dir_all(&src_dir).unwrap();
    fs::create_dir_all(&assets_dir).unwrap();

    let template_files = [
        ("Cargo.toml.template", "Cargo.toml"),
        ("src/main.rs.template", "src/main.rs"),
        ("src/game_state.rs.template", "src/game_state.rs"),
        ("src/player.rs.template", "src/player.rs"),
        ("src/ui.rs.template", "src/ui.rs"),
        ("src/config.rs.template", "src/config.rs"),
    ];
    
    let specific_files = match genre {
        "sorting" => vec![("src/drag_drop.rs.template", "src/drag_drop.rs")],
        "quiz_rpg" => vec![("src/dialogue.rs.template", "src/dialogue.rs")],
        "timeline" => vec![("src/timeline.rs.template", "src/timeline.rs")],
        "simulation" => vec![("src/simulation.rs.template", "src/simulation.rs")],
        _ => vec![],
    };

    let mut all_files = template_files.to_vec();
    all_files.extend(specific_files);

    for (template_name, output_name) in all_files {
        let template_path = template_dir.join(template_name);
        if template_path.exists() {
            let content = fs::read_to_string(&template_path).unwrap();
            let processed = content
                .replace("{{GAME_NAME}}", &format!("test_game_{}", genre))
                .replace("{{GAME_TITLE}}", "Test Game")
                .replace("{{GAME_DESCRIPTION}}", "A test game")
                .replace("{{SUBJECT}}", "Testing")
                .replace("{{AUTHOR}}", "Trinity System")
                .replace("{{VOCABULARY_ENTRIES}}", "VocabEntry { word: \"test\".to_string(), definition: \"test\".to_string() },")
                .replace("{{LEARNING_OBJECTIVES}}", "\"test objective\".to_string(),");
            
            fs::write(game_dir.join(output_name), processed).unwrap();
        }
    }

    // Run cargo check to verify template structural integrity
    // NOTE: We don't fetch crates fully online to save CI time, 
    // but we use --offline if we want. We'll let it resolve local workspace.
    // To share target cache, we point it to the main workspace target!
    let target_dir = workspace_root.join("target").join("template_checks");
    let output = Command::new("cargo")
        .args(["check", "--target-dir", target_dir.to_str().unwrap()])
        .current_dir(&game_dir)
        .output()
        .unwrap();

    // Clean up
    let _ = fs::remove_dir_all(&game_dir);

    if !output.status.success() {
        panic!("Compilation failed for {}: {}", genre, String::from_utf8_lossy(&output.stderr));
    }
}

// These tests are marked as ignored by default because they take significant time
// (downloading/checking Bevy) and shouldn't block the standard short CI loops.
// They are manually triggered during major template updates:
// cargo test -p trinity -- tests::template_validation --ignored

#[test]
#[ignore = "Slow template compilation test"]
fn test_exploration_template() {
    validate_template("exploration");
}

#[test]
#[ignore = "Slow template compilation test"]
fn test_sorting_template() {
    validate_template("sorting");
}

#[test]
#[ignore = "Slow template compilation test"]
fn test_quiz_rpg_template() {
    validate_template("quiz_rpg");
}

#[test]
#[ignore = "Slow template compilation test"]
fn test_timeline_template() {
    validate_template("timeline");
}

#[test]
#[ignore = "Slow template compilation test"]
fn test_simulation_template() {
    validate_template("simulation");
}
