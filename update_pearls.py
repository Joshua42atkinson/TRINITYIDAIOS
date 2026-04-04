import os

docs = {
    "App_Chrome.md": "Use the Chrome navigation bar to switch between the 3 main operating modes: ID (Iron Road Learning), AI (Art Studio Generation), and OS (Yardmaster Power Tools).",
    "ArtStudio.md": "Click the ART Studio tab to access ComfyUI/MusicGPT. Use this when you need an image or audio component for your lesson. Type a prompt, hit generate, and watch the coal/steam economy fuel the render.",
    "CharacterSheet.md": "Click the Character/Portfolio tab to view your LDT Portfolio. Show stakeholders this dashboard to prove QM score alignment, progression, and the artifact vault.",
    "ChariotViewer.md": "Click the Docs or Library button to open the Chariot Viewer. Use this to read the fundamental doctrine (Fancy Bible, Field Manual) without leaving the application.",
    "ExpressWizard.md": "Select the Express tab for a 10-minute generation. Use this when you do not want to play the full 12-phase game and simply need a Quick Lesson Plan exported immediately.",
    "GameHUD.md": "Located on the side/top of the screen. Look here to track the Locomotive Economy (Coal burning, Steam rising) and your semantic creeps taming status in real-time.",
    "HookBook.md": "Access your Hook Deck inventory when a Scope Creep appears. Drag and drop a Hook Card (like Socratic Interview) to tame anomalies and bring conversations back into scope.",
    "JournalViewer.md": "Click the Journal icon to open the reflection log. When you enter the EYE phases, use this to document your pedagogical maturation and complete the required graduation artifacts.",
    "PhaseWorkspace.md": "This is the core chat interface. Select one of the 12 ADDIECRAPEYE phases on the left, read the 3 checkboxes at the top, and chat with Pete to complete each objective.",
    "PortfolioView.md": "Similar to the Character Sheet, this vault shows your exported artifacts. Use this to view completed HTML packages or game design documents ready for download.",
    "QualityScorecard.md": "Trigger this from the Yardmaster or end-of-phase export. Show stakeholders the 0-100 evaluation of their lesson plan against Quality Matters rubrics.",
    "SetupWizard.md": "This is the very first screen. Type in the subject (PEARL) you want to teach and select the medium. This seeds Pete's initial Socratic state.",
    "YardBrowser.md": "Use the Yard Browser to visually inspect the internal node/graph artifacts being compiled in the backend while you chat.",
    "Yardmaster.md": "Click the OS (Work) tab. Use this multi-turn agentic terminal to direct Pete to read files, rewrite scripts, or compile documents if you prefer a power-user experience."
}

base_dir = "docs/pearls"

for file_name, how_to in docs.items():
    path = os.path.join(base_dir, file_name)
    if os.path.exists(path):
        with open(path, "r") as f:
            content = f.read()
        
        if "### HOW IT WORKS" not in content:
            appendix = "\n\n### HOW IT WORKS (User Action)\n*The Presentation 'How': What the user actually does.*\n- **Action:** " + how_to + "\n- **Why:** This demystifies the theoretical 'why' into a direct, clickable interaction that drives the system forward.\n"
            with open(path, "a") as f:
                f.write(appendix)
            print(f"Updated {file_name}")
