# 🚂 Trinity ID AI OS — Quick Start Guide

> **Download → Install LM Studio → Run Trinity → Open Browser**
> Total time: ~15 minutes (including model download)

---

## What You Need

| Requirement | Why |
|-------------|-----|
| **Linux** (Ubuntu 22.04+, Fedora 38+) | Trinity is a native Rust binary |
| **16 GB+ RAM** | Minimum for small AI models |
| **LM Studio** | The AI brain that Trinity connects to |
| A browser | Chrome, Firefox, or Edge |

> **Windows/Mac?** Trinity is Linux-only right now. If you're on Windows, use WSL2 (Windows Subsystem for Linux).

---

## Step 1: Install LM Studio (The AI Brain)

LM Studio is a free, local AI model runner. Trinity uses it as its "brain."

1. Go to [https://lmstudio.ai](https://lmstudio.ai)
2. Download and install the Linux version
3. Open LM Studio

> **See [LM_STUDIO_SETUP.md](LM_STUDIO_SETUP.md) for recommended model settings for your hardware.**

---

## Step 2: Load a Model in LM Studio

In LM Studio:

1. Click **Search** in the left sidebar
2. Search for a model based on your RAM:
   - **8-16 GB RAM**: Search `phi-4-mini` or `qwen2.5-7b` (small, fast)
   - **24-32 GB RAM**: Search `mistral-small-24b` (recommended for teachers)
   - **64-128 GB RAM**: Search `mistral-small-4-119b` (full power)
3. Download the **Q4_K_M** quantization (best quality/size ratio)
4. Click **Load** on the downloaded model
5. Go to **Local Server** tab → Click **Start Server**
   - Verify it shows `Server running on port 1234`

---

## Step 3: Run Trinity

Extract the Trinity archive and run:

```bash
# 1. Extract the archive
tar -xzf trinity-v1.0-linux-x86_64.tar.gz
cd trinity

# 2. Run Trinity
./run-trinity.sh
```

You should see:
```
🚂 Trinity ID AI OS starting...
✅ Server running on http://localhost:3000
✅ LM Studio detected on port 1234
```

---

## Step 4: Open in Browser

Open your browser and go to:

```
http://localhost:3000
```

### What You'll See

| Tab | What It Is |
|-----|-----------|
| **Iron Road** 🚂 | The main workspace — chat with Pete, your AI teaching assistant |
| **Yardmaster** 🔧 | Advanced developer terminal for agentic workflows |
| **Art Studio** 🎨 | Creative tools (image/music/video generation) |
| **Character Sheet** | Your learning portfolio and profile |

### First Thing to Do

1. Click on the **Iron Road** tab
2. Type a subject you want to build a lesson for (e.g., "5th grade fractions")
3. Pete will guide you through the ADDIECRAPEYE instructional design process

---

## Troubleshooting

| Problem | Solution |
|---------|----------|
| "Trinity can't find an AI backend" | Make sure LM Studio is running with a model loaded and the server is started |
| Page is blank | Wait 5 seconds and refresh — the frontend may still be loading |
| "INFERENCE_OFFLINE" banner | LM Studio's server isn't started. Go to LM Studio → Local Server → Start Server |
| Slow responses | Your model may be too large for your RAM. Try a smaller model. |

---

## How It Works (30-Second Version)

Trinity is an AI-powered instructional design system. It helps teachers build educational games by walking them through a 12-phase process called **ADDIECRAPEYE**:

- **ADDIE** = Analysis → Design → Development → Implementation → Evaluation
- **CRAP** = Contrast → Repetition → Alignment → Proximity (visual design)
- **EYE** = Envision → Yoke → Evolve (reflection & polish)

Your AI mentor **Pete** guides you through each phase with Socratic questioning — he makes you think, not just click. Everything runs **100% locally on your machine** — no cloud, no data sharing, no API fees.

---

## More Resources

- **[LM_STUDIO_SETUP.md](LM_STUDIO_SETUP.md)** — Detailed LM Studio settings for your hardware
- **[INSTALL.md](INSTALL.md)** — Building from source (for developers)
- **[README.md](README.md)** — Full project overview
- **[PROFESSOR.md](PROFESSOR.md)** — Institutional evaluation guide

---

*Questions? Issues? Open an issue at [github.com/Joshua42atkinson/TRINITYIDAIOS](https://github.com/Joshua42atkinson/TRINITYIDAIOS)*
