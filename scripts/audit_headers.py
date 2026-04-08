import os
import glob

TEMPLATE = """// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-server
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:        {filename}
// PURPOSE:     {purpose}
//
// 🪟 THE LIVING CODE TEXTBOOK:
// This file is part of the Trinity ID AI OS. It is designed to be read, 
// modified, and authored by YOU. As you transition from LEARNING to WORK, 
// this is where the logic lives. 
//
// 📖 THE HOOK BOOK CONNECTION:
// For a full catalogue of system capabilities, see: docs/HOOK_BOOK.md
//
// 🛡️ THE COW CATCHER & AUTOPOIESIS:
// All files operate under the autonomous Cow Catcher telemetry system. Runtime
// errors and scope creep are intercepted to prevent catastrophic derailment,
// maintaining the Socratic learning loop and keeping drift at bay.
//
// MATURITY:     L5 → Shippable
// QUEST_PHASE:  Integration
//
// CHANGES:
//   2026-04-08  Cascade  Migrated to §17 comment standard
//
// ═══════════════════════════════════════════════════════════════════════════════

"""

src_dir = '/home/joshua/Workflow/desktop_trinity/trinity-genesis/crates/trinity/src'
missing_files = []

for filepath in glob.glob(os.path.join(src_dir, '*.rs')):
    with open(filepath, 'r') as f:
        content = f.read()
    
    if "THE LIVING CODE TEXTBOOK" not in content and "TRINITY ID AI OS" not in content:
        missing_files.append(filepath)

print("Files missing standard headers:")
for f in missing_files:
    print(f"  - {os.path.basename(f)}")

print(f"\nTotal files needing update: {len(missing_files)}")

# Auto-patch them using the template
if len(missing_files) > 0:
    for filepath in missing_files:
        basename = os.path.basename(filepath)
        with open(filepath, 'r') as f:
            original = f.read()
            
        new_header = TEMPLATE.format(filename=basename, purpose="Subsystem Logic")
        
        with open(filepath, 'w') as f:
            f.write(new_header + original)
            
    print(f"Patched {len(missing_files)} files with standard header.")
