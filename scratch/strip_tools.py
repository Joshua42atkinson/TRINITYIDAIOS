import re
import sys

with open('crates/trinity/src/tools.rs', 'r') as f:
    content = f.read()

# 1. Remove tools from get_tool_list
tools_to_remove = [
    '"quest_status"',
    '"cowcatcher_log"',
    '"quest_advance"',
    '"scaffold_bevy_game"',
    '"avatar_pipeline"',
    '"daydream_command"',
    '"generate_mesh3d"',
    '"blender_render"',
    '"zombie_check"',
    '"sidecar_start"',
    '"sidecar_status"',
    '"project_archive"',
    '"generate_music"',
    '"generate_video"'
]

for tool in tools_to_remove:
    # Regex to match the ToolInfo struct definition for the specific tool name
    # e.g., ToolInfo { name: "quest_status".into(), ... },
    pattern = r'[ \t]*ToolInfo\s*\{\s*name:\s*' + tool + r'\.into\(\)[^\}]+\},\n'
    content = re.sub(pattern, '', content)

# 2. Remove match arms in execute_tool_raw
# e.g., "quest_status" => tool_quest_status().await,
match_arms_to_remove = [
    r'[ \t]*"quest_status"\s*=>\s*tool_quest_status\(\)\.await,?\n',
    r'[ \t]*"quest_advance"\s*=>\s*tool_quest_advance\(params\)\.await,?\n',
    r'[ \t]*"cowcatcher_log"\s*=>\s*tool_cowcatcher_log\(\)\.await,?\n',
    r'[ \t]*"scaffold_bevy_game"\s*=>\s*tool_scaffold_bevy_game\(params\)\.await,?\n',
    r'[ \t]*"daydream_command"\s*=>\s*tool_daydream_command\(params\)\.await,?\n',
    r'[ \t]*"project_archive"\s*=>\s*tool_project_archive\(params\)\.await,?\n',
    r'[ \t]*"generate_mesh3d"\s*=>\s*tool_generate_mesh3d\(params\)\.await,?\n',
    r'[ \t]*"blender_render"\s*=>\s*tool_blender_render\(params\)\.await,?\n',
    r'[ \t]*"zombie_check"\s*=>\s*tool_zombie_check\(params\)\.await,?\n',
    r'[ \t]*"sidecar_start"\s*=>\s*tool_sidecar_start\(params\)\.await,?\n',
    r'[ \t]*"sidecar_status"\s*=>\s*tool_sidecar_status\(\)\.await,?\n',
    r'[ \t]*"avatar_pipeline"\s*=>\s*tool_avatar_pipeline\(params\)\.await,?\n',
    r'[ \t]*"generate_music"\s*=>\s*tool_generate_music\(params\)\.await,?\n',
    r'[ \t]*"generate_video"\s*=>\s*tool_generate_video\(params\)\.await,?\n',
]

for pattern in match_arms_to_remove:
    content = re.sub(pattern, '', content)

with open('crates/trinity/src/tools.rs', 'w') as f:
    f.write(content)

print("Tool list and match arms updated.")
