# eLearning Template Guide — Trinity ID AI OS
## Based on Local-AI-Architect (Purdue University Student Project)

---

## Template Location

The reference eLearning template is at:
```
~/Elearning/local-ai-architect-elearning/
```

## Stack
- **Framework**: React 18 + Vite
- **Styling**: TailwindCSS with glassmorphism
- **Icons**: Lucide React
- **Routing**: React Router DOM v6

## Structure (ADDIE-Aligned)

```
src/
├── App.jsx                 # Navigation + progress bar + layout
├── pages/
│   ├── HomePage.jsx        # Course overview (ADDIE: Analysis output)
│   ├── ModuleOne.jsx       # Module 1: Setup (ADDIE: Design)
│   ├── ModuleTwo.jsx       # Module 2: Logic (ADDIE: Development)
│   ├── ModuleThree.jsx     # Module 3: Implement (ADDIE: Implementation)
│   ├── SandboxEmbed.jsx    # Interactive sandbox (Constructivism)
│   ├── KnowledgeCheck.jsx  # Quiz/Assessment (ADDIE: Evaluation)
│   └── Documentation.jsx   # Reference docs
├── index.css               # Global styles
└── main.jsx                # Entry point
```

## How Trinity Uses This Template

When a user completes the ADDIECRAPEYE Analysis + Design phases via voice or chat,
Trinity scaffolds a new eLearning project using this template:

1. **Analysis** → Trinity interviews SME, defines learning objectives
2. **Design** → Trinity maps objectives to modules using Backward Design
3. **Development** → Trinity generates:
   - `HomePage.jsx` with course overview + objectives
   - `Module[N].jsx` for each learning unit (from Bloom's-aligned content)
   - `KnowledgeCheck.jsx` with criterion-referenced assessments
   - `SandboxEmbed.jsx` with interactive practice (Action Mapping step 3)
4. **Implementation** → `npm run build` → deploy to server
5. **Evaluation** → QM Rubric automated check on generated content

## Scaffolding Command (Future)

```bash
# Trinity will generate this via the agent tool system
trinity scaffold elearning \
  --title "Local AI Architecture" \
  --modules 3 \
  --objectives objectives.json \
  --style glassmorphism \
  --output ~/projects/new-elearning/
```

## Key Design Patterns from the Template

### Progress Bar (Course Completion Tracking)
```jsx
<div className="h-1 w-full bg-slate-800/50">
  <div className="h-full bg-gradient-to-r from-emerald-500 via-fuchsia-500 to-amber-500"
       style={{ width: getProgressWidth() }} />
</div>
```

### Glassmorphism Navigation
```jsx
<nav className="glass-nav z-50 relative">
  {navItems.map(item => (
    <Link className={`px-4 py-2 rounded-xl ${getActiveStyles(item.path)}`}>
      {item.icon} {item.name}
    </Link>
  ))}
</nav>
```

### Module Structure (Repeatable Pattern)
Each module follows: Introduction → Content → Activity → Summary
This maps to Gagné's Nine Events of Instruction.

---

*This template is the base for all Trinity-generated eLearning content.*
*Extend with additional modules, assessments, and interactive elements as needed.*
