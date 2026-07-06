import re
import sys

def remove_functions(filepath, funcs_to_remove):
    with open(filepath, 'r') as f:
        content = f.read()

    for func in funcs_to_remove:
        # Find the start of the function
        # Matches `fn name` or `async fn name` or `pub fn name` etc
        pattern = re.compile(r'(?:pub\s+)?(?:async\s+)?fn\s+' + re.escape(func) + r'\b.*?\{', re.DOTALL)
        
        while True:
            match = pattern.search(content)
            if not match:
                break
                
            start_idx = match.start()
            
            # Find the matching closing brace
            brace_count = 0
            end_idx = -1
            in_string = False
            escape = False
            
            for i in range(match.end() - 1, len(content)):
                c = content[i]
                
                if escape:
                    escape = False
                    continue
                    
                if c == '\\':
                    escape = True
                    continue
                    
                if c == '"':
                    in_string = not in_string
                    continue
                    
                if not in_string:
                    if c == '{':
                        brace_count += 1
                    elif c == '}':
                        brace_count -= 1
                        if brace_count == 0:
                            end_idx = i
                            break
                            
            if end_idx != -1:
                # Remove from start_idx to end_idx + 1
                content = content[:start_idx] + content[end_idx+1:]
            else:
                print(f"Could not find matching brace for {func}")
                break
                
    with open(filepath, 'w') as f:
        f.write(content)
    print("Done removing functions.")

funcs = [
    "gguf_model_path",
    "safetensor_model_path",
    "resolve_sidecar_role",
    "tool_avatar_pipeline",
    "tool_generate_music",
    "tool_generate_video",
    "tool_generate_mesh3d",
    "tool_blender_render",
    "tool_sidecar_status",
    "tool_sidecar_start",
    "tool_quest_status",
    "tool_quest_advance",
    "tool_cowcatcher_log",
    "tool_daydream_command",
    "tool_scaffold_bevy_game",
    "tool_project_archive",
    "tool_zombie_check"
]

remove_functions('crates/trinity/src/tools.rs', funcs)
