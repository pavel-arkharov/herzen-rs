# Herzen UI Design Brief

> This document is a comprehensive design spec for the Herzen desktop UI.
> Target stack: Tauri v2 + Svelte. Dark theme. Minimal, information-dense.
> The app has TWO window modes and FOUR main screens.

---

## Window Modes

### 1. Voice Orb (compact, always-accessible)

A small floating widget — the daily driver. Think macOS widget or Spotlight-like overlay.

```
┌─────────────────────────┐
│                         │
│       ◉  (orb)          │  ← Animated sphere, ~120x120px
│                         │
│   "Listening..."        │  ← Status text below orb
│                         │
└─────────────────────────┘
      ~200px wide
```

**Orb states and animations:**

| State | Visual | Animation |
|-------|--------|-----------|
| Idle | Soft glow, slow pulse | Gentle breathing (scale 0.98–1.02, 3s cycle) |
| Listening | Bright ring, surface ripples | Audio input waveform drives displacement on sphere surface. Louder = bigger ripples |
| Processing | Rotating inner patterns | Slow spiral rotation, particles orbiting |
| Speaking | Emanating concentric rings | Rings expand outward from orb, speed matches TTS cadence. Audio output drives ring intensity |
| Error | Red tint, shake | Brief horizontal shake, then fade to idle |
| Thinking | Subtle inner swirl | Abstract flowing patterns inside the sphere, like lava lamp |

**Orb interactions:**
- Click orb → toggle listening on/off
- Right-click → context menu (open full UI, settings, quit)
- Drag → reposition floating window
- Scroll on orb → adjust volume

**Below the orb:**
- Current status text: "Listening...", "Processing...", "Speaking...", idle shows nothing
- Last response (truncated, 1-2 lines, fades after 5s)
- Thin confidence bar during skill matching (shows match confidence in real-time)

**Future: Avatar Mode**
The orb can be replaced by a digital avatar in a later phase. The container stays the same size but renders a face/character instead of a sphere. The avatar's expressions map to the same states (idle, listening, speaking, thinking). This is a future feature — the orb is the v1 visual.

---

### 2. Full Dashboard (main window)

Opened from the orb's context menu or a keyboard shortcut. Has a sidebar + content area.

```
┌──────┬──────────────────────────────────────────────┐
│      │                                              │
│  ◉   │  [Content Area — switches by nav]            │
│      │                                              │
│ ──── │                                              │
│ 💬   │                                              │
│ Chat │                                              │
│      │                                              │
│ 🧠   │                                              │
│Models│                                              │
│      │                                              │
│ 🔧   │                                              │
│Work- │                                              │
│shop  │                                              │
│      │                                              │
│ 📊   │                                              │
│System│                                              │
│      │                                              │
│      │                                              │
│ ──── │                                              │
│ ⚙️   │                                              │
│      │                                              │
└──────┴──────────────────────────────────────────────┘
  64px              rest of window
```

**Sidebar (narrow, icon + label):**
- Mini orb at top (shows current state, click to toggle listening)
- Navigation: Chat, Models, Workshop, System
- Settings gear at bottom

---

## Screen 1: Chat

The conversation view. Shows the real-time flow of turns.

```
┌─────────────────────────────────────────────────────┐
│  Chat                                    [🎤] [⌨️]  │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌─ You ─────────────────────────────────────────┐  │
│  │ Turn off the bedroom lights                   │  │
│  │ 🎤 via voice · whisper-small · 340ms          │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
│  ┌─ Herzen ──────────────────────────────────────┐  │
│  │ Bedroom lights turned off.                    │  │
│  │ ⚡ skill:lights · keyword match · 0.95        │  │
│  │ 🔊 piper · calm · 280ms                      │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
│  ┌─ You ─────────────────────────────────────────┐  │
│  │ What's the weather like tomorrow?             │  │
│  │ ⌨️ via text                                   │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
│  ┌─ Herzen ──────────────────────────────────────┐  │
│  │ Tomorrow will be partly cloudy with a high    │  │
│  │ of 18°C. Light rain expected in the evening.  │  │
│  │ 🧠 qwen2.5-3b · 1.2s · 47 tok/s             │  │
│  │ 🔊 piper · neutral · 450ms                   │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
├─────────────────────────────────────────────────────┤
│  [  Type a message...                    ] [Send]   │
│  Mode: [Voice ▾] Model: [qwen2.5-3b ▾]             │
└─────────────────────────────────────────────────────┘
```

**Key elements:**
- Each turn shows metadata inline: input method, model used, latency, tokens/sec
- Skill matches show the skill name, match method, confidence score
- TTS info: provider, style, generation time
- Toggle between voice input and text input mode
- Model selector dropdown (quick switch without going to Models screen)
- Streaming: LLM responses appear token-by-token

---

## Screen 2: Models

The LLM playground. Load, unload, configure, and monitor models.

```
┌─────────────────────────────────────────────────────┐
│  Models                          Memory: 4.2 / 10GB │
│                                  [Load Model ▾]     │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌─ qwen2.5-3b ─────────────────── LOADED ───────┐ │
│  │ Role: dialog (default)                         │ │
│  │ File: ~/.herzen/models/qwen2.5-3b-q4_k_m.gguf │ │
│  │ Memory: 2.1 GB · GPU layers: 99 · Ctx: 4096   │ │
│  │                                                │ │
│  │ Temperature: [====●=====] 0.7                  │ │
│  │ Top-P:       [======●===] 0.9                  │ │
│  │ Max tokens:  [===●======] 512                  │ │
│  │                                                │ │
│  │ Throughput: 42 tok/s avg                       │ │
│  │                            [Unload] [Set Default]│
│  └────────────────────────────────────────────────┘ │
│                                                     │
│  ┌─ phi-3-mini ──────────────────── LOADED ───────┐ │
│  │ Role: intent                                   │ │
│  │ Memory: 1.8 GB · GPU layers: 99 · Ctx: 2048   │ │
│  │ Throughput: 68 tok/s avg                       │ │
│  │                            [Unload] [Configure]│ │
│  └────────────────────────────────────────────────┘ │
│                                                     │
│  ┌─ multilingual-e5-small ─────── LOADED ─────────┐ │
│  │ Role: embeddings (skills matching)             │ │
│  │ Memory: 120 MB · ONNX Runtime                  │ │
│  │                                      [Unload]  │ │
│  └────────────────────────────────────────────────┘ │
│                                                     │
│  ── Available (not loaded) ─────────────────────── │ │
│                                                     │
│  ○ gemma-2b-it-q4      1.2 GB   dialog    [Load]  │ │
│  ○ qwen2.5-1.5b-q4     0.9 GB   dialog    [Load]  │ │
│  ○ tinyllama-1.1b      0.6 GB   intent    [Load]  │ │
│                                                     │
├─────────────────────────────────────────────────────┤
│  Memory Budget                                      │
│  ████████████████░░░░░░░░  4.2 / 10 GB available   │
│  [qwen2.5-3b: 2.1] [phi-3: 1.8] [e5: 0.1] [free] │
└─────────────────────────────────────────────────────┘
```

**Key elements:**
- **Memory bar** at top and bottom — always visible, shows total budget
- Each loaded model shows: name, role, file path, RAM usage, GPU layers, context size
- **Live sliders** for temperature, top_p, max_tokens — changes apply immediately
- **Throughput** stats from recent turns
- **Load Model dropdown** — shows available GGUF files from models directory
- **Unload** button with memory reclaim preview ("Unloading will free 2.1 GB")
- Available (not loaded) section shows models in the registry with one-click load
- **Role assignment** — each model has a role dropdown: dialog, intent, summarizer, embeddings, custom

**Model roles explained in the UI:**
- `dialog` — main conversational model, responds to user
- `intent` — classifies user input for skill routing
- `summarizer` — compacts conversation context
- `embeddings` — produces vectors for semantic skill matching (ONNX, not LLM)
- `custom` — user-defined purpose

---

## Screen 3: Workshop

The skill builder and debugger. Two-panel layout.

```
┌───────────┬─────────────────────────────────────────┐
│           │                                         │
│ Skills    │  Skill Editor: lights                   │
│ ────────  │  ─────────────────────                  │
│ ● lights  │                                         │
│ ● weather │  Name: [lights                 ]        │
│ ● summary │  Description: [Control lights  ]        │
│ ○ translate│  Priority: [10]   [Enabled ✓]          │
│           │                                         │
│           │  ┌─ INTENT SLOTS ──────────────────┐    │
│ [+ New]   │  │                                 │    │
│           │  │  action (required)          [▾]  │    │
│           │  │  ┌──────────────────────────┐   │    │
│           │  │  │ turn_on:                 │   │    │
│           │  │  │   en: turn on, switch on │   │    │
│           │  │  │   ru: включи, зажги, вруби│  │    │
│           │  │  │ turn_off:                │   │    │
│           │  │  │   en: turn off, kill     │   │    │
│           │  │  │   ru: выключи, гаси      │   │    │
│           │  │  └──────────────────────────┘   │    │
│           │  │                                 │    │
│           │  │  target (required)          [▾]  │    │
│           │  │  ┌──────────────────────────┐   │    │
│           │  │  │ light:                   │   │    │
│           │  │  │   en: light, lights, lamp│   │    │
│           │  │  │   ru: свет, лампу        │   │    │
│           │  │  └──────────────────────────┘   │    │
│           │  │                                 │    │
│           │  │  location (optional)        [▾]  │    │
│           │  │  bedroom, kitchen, living room   │    │
│           │  │                                 │    │
│           │  │                    [+ Add Slot] │    │
│           │  └─────────────────────────────────┘    │
│           │                                         │
│           │  ┌─ SEMANTIC EXAMPLES ─────────────┐    │
│           │  │ • turn off the lights           │ [x]│
│           │  │ • выключи свет                  │ [x]│
│           │  │ • kill the lights               │ [x]│
│           │  │ • погаси лампу на кухне         │ [x]│
│           │  │                                 │    │
│           │  │ [+ Add example]  Threshold: 0.85│    │
│           │  └─────────────────────────────────┘    │
│           │                                         │
│           │  ┌─ ACTIONS ──────────────────────┐     │
│           │  │ 1. [homeassistant ▾]           │     │
│           │  │    entity: [light.{{location}}]│     │
│           │  │    service: [{{action}}       ]│     │
│           │  │                     [+ Action] │     │
│           │  └────────────────────────────────┘     │
│           │                                         │
│           │  ┌─ CONFIDENCE GATES ─────────────┐     │
│           │  │ Auto-execute: [====●====] 0.90 │     │
│           │  │ Confirm above:[===●=====] 0.70 │     │
│           │  │ Destructive: [✓]               │     │
│           │  └────────────────────────────────┘     │
│           │                                         │
│           │  ┌─ RESPONSE ─────────────────────┐     │
│           │  │ Template: [{{action}} lights.]  │     │
│           │  │ Confirm:  [{{action}} lights?]  │     │
│           │  │ TTS: [piper ▾] Style: [calm ▾]  │     │
│           │  └────────────────────────────────┘     │
│           │                                         │
├───────────┴─────────────────────────────────────────┤
│  TEST BENCH                                         │
│  ┌────────────────────────────────────────────────┐ │
│  │ гаси свет в спальне                           │ │
│  └────────────────────────────────────────────────┘ │
│                                                     │
│  Match Results:                                     │
│  ✅ lights     0.95  keyword  action=turn_off       │
│  │                            target=light          │
│  │                            location=bedroom      │
│  │              decision: CONFIRM (destructive)     │
│  │              response: "Turn off bedroom lights?"│
│  ○  weather    0.08  none                           │
│  ○  summary    0.03  none                           │
│  ○  translate  0.12  partial  (no required slots)   │
│                                                     │
│  [▶ Execute]  [Show Semantic Scores]  [Show Debug]  │
│                                                     │
│  ┌─ Debug Panel (collapsed by default) ───────────┐ │
│  │ Language detected: ru                          │ │
│  │ Keyword scan: 3 slots filled in 0.02ms         │ │
│  │ Semantic: skipped (keywords sufficient)         │ │
│  │ Confidence: 0.95 (keyword full match)           │ │
│  │ Gate: 0.95 >= auto_execute(0.90) BUT            │ │
│  │       destructive=true → CONFIRM                │ │
│  └────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────┘
```

**Key Workshop features:**
- Left panel: skill list with enabled/disabled indicators, drag to reorder priority
- Right panel: full skill editor with collapsible sections
- **Slot editor**: add/remove canonical values and keyword aliases per language
- **Semantic examples**: add/remove, each tagged with language, shows embedding distance
- **Action builder**: dropdown for type (homeassistant, llm, http, shell), dynamic fields
- **Test Bench** (bottom): type any input in any language, see real-time match scores across ALL skills
- **Debug Panel**: shows exact matching pipeline trace — which layer matched, timing, why confidence landed where it did
- **Execute button**: run the skill's action pipeline live and see the result
- All changes auto-save to the skill's TOML file

---

## Screen 4: System

Resource monitoring and health dashboard.

```
┌─────────────────────────────────────────────────────┐
│  System                                    Uptime: 2h│
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌─ Memory ──────────────────────────────────────┐  │
│  │ System: 12.4 / 16.0 GB                       │  │
│  │ ████████████████████░░░░░  77%                │  │
│  │                                               │  │
│  │ Herzen total: 4.3 GB                          │  │
│  │  ├ qwen2.5-3b    2.1 GB  ████████░░           │  │
│  │  ├ phi-3-mini    1.8 GB  ███████░░░           │  │
│  │  ├ e5-small      120 MB  █░░░░░░░░            │  │
│  │  ├ whisper        280 MB ██░░░░░░░            │  │
│  │  └ runtime         40 MB ░░░░░░░░░            │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
│  ┌─ Latency (last 10 turns) ─────────────────────┐  │
│  │ STT:      ██░░░░░░  340ms avg                 │  │
│  │ Matching:  ░░░░░░░    2ms avg                 │  │
│  │ LLM:      ████░░░░  820ms avg (42 tok/s)      │  │
│  │ TTS:      ██░░░░░░  280ms avg                 │  │
│  │ Total:    ██████░░  1.44s avg                  │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
│  ┌─ Processes ───────────────────────────────────┐  │
│  │ ● herzend         running   40 MB   PID 1234 │  │
│  │ ● whisper.cpp     idle      280 MB  PID 1235 │  │
│  │ ○ tts-piper       stopped   —                 │  │
│  │ ○ tts-xtts (py)   stopped   —       [Start]  │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
│  ┌─ Skill Activity ──────────────────────────────┐  │
│  │ lights       ████████████   42 triggers today │  │
│  │ weather      ████           12 triggers today │  │
│  │ summary      ██              6 triggers today │  │
│  │ (dialog)     ████████████████ 58 fallthroughs │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## Design Principles

1. **Dark theme only** (for now). Background: #0f0f0f. Surfaces: #1a1a1a. Borders: #2a2a2a. Text: #e0e0e0. Accent: warm amber #f5a623.

2. **Information density over whitespace.** This is a power-user tool, not a consumer app. Show data, don't hide it behind clicks.

3. **Real-time everything.** WebSocket-driven. Model stats, memory, latencies, orb state — all live-updating, no manual refresh.

4. **The orb is the brand.** It's the persistent visual identity. When a future avatar replaces it, the container and interaction model stay the same.

5. **Progressive disclosure.** Test Bench shows summary by default, Debug Panel expands on click. Model cards show key stats, Configure expands full controls.

6. **Keyboard-first.** Cmd+K → command palette. Cmd+1/2/3/4 → switch screens. Space → toggle listening. Escape → close panels.

---

## Component Architecture (Svelte)

```
src/
├── lib/
│   ├── stores/
│   │   ├── connection.ts    # WebSocket to herzen-server
│   │   ├── models.ts        # Model pool state (loaded, memory, stats)
│   │   ├── chat.ts          # Conversation history
│   │   ├── skills.ts        # Skill list + match results
│   │   ├── system.ts        # Memory, latency, process health
│   │   └── orb.ts           # Orb animation state (idle/listening/speaking/...)
│   ├── components/
│   │   ├── Orb.svelte                # Animated sphere (WebGL or CSS)
│   │   ├── Sidebar.svelte            # Navigation sidebar
│   │   ├── ChatMessage.svelte        # Single turn bubble
│   │   ├── ModelCard.svelte          # Loaded model card with controls
│   │   ├── MemoryBar.svelte          # Segmented memory usage bar
│   │   ├── SkillEditor.svelte        # Full skill editor form
│   │   ├── SlotEditor.svelte         # Keyword alias editor per slot
│   │   ├── TestBench.svelte          # Input + match results + debug
│   │   ├── LatencyChart.svelte       # Per-turn latency breakdown
│   │   └── ConfidenceSlider.svelte   # Dual-thumb slider for gates
│   └── api/
│       ├── client.ts          # REST client to herzen-server
│       └── ws.ts              # WebSocket event handler
├── routes/
│   ├── +layout.svelte         # Sidebar + orb wrapper
│   ├── chat/+page.svelte
│   ├── models/+page.svelte
│   ├── workshop/+page.svelte
│   └── system/+page.svelte
├── orb-window/
│   └── +page.svelte           # Standalone orb floating window
└── app.css                    # Global styles, dark theme tokens
```

---

## WebSocket Events (herzen-server → frontend)

```typescript
// Server pushes these events over WS
type ServerEvent =
  | { type: "orb_state"; state: "idle" | "listening" | "processing" | "speaking" | "error" }
  | { type: "audio_level"; level: number }  // 0.0-1.0, drives orb ripple intensity
  | { type: "turn_start"; turn_id: string; method: "voice" | "text" }
  | { type: "stt_result"; turn_id: string; text: string; model: string; duration_ms: number }
  | { type: "skill_match"; turn_id: string; results: SkillMatchResult[] }
  | { type: "llm_token"; turn_id: string; token: string }  // streaming
  | { type: "llm_done"; turn_id: string; model: string; tokens: number; duration_ms: number }
  | { type: "tts_start"; turn_id: string; provider: string; style: string }
  | { type: "tts_done"; turn_id: string; duration_ms: number }
  | { type: "model_status"; models: ModelStatus[] }
  | { type: "memory_update"; system_total: number; system_used: number; herzen_breakdown: Record<string, number> }
  | { type: "error"; message: string }
```

---

## Orb Technical Notes

**Option A: CSS-only orb** (simpler, good enough for v1)
- Radial gradient sphere with CSS animation for breathing/pulse
- `box-shadow` rings for speaking state
- `backdrop-filter: blur()` for glass effect
- Audio levels drive CSS custom properties (`--ripple-intensity`)

**Option B: WebGL/Three.js orb** (richer, more expressive)
- Sphere geometry with vertex displacement driven by audio FFT
- Custom shader: noise-based surface distortion
- Particle system for processing/thinking states
- Smooth transitions between states via shader uniforms

Recommend: **Start with CSS orb (Option A)**, upgrade to WebGL later when the avatar work begins. The CSS orb is fast to implement and the visual difference is small on a 120px element.

---

## Future: Avatar Mode

The orb container is designed to be swappable. When avatar mode is implemented:

- Same floating window, same size constraints
- Orb.svelte swaps for Avatar.svelte
- Avatar receives the same state events (idle/listening/speaking)
- LLM drives expression/gesture selection via a separate "avatar controller" prompt
- Lip sync driven by TTS audio output
- Tech: likely Three.js with a rigged 3D model (ReadyPlayerMe or custom)

This is Phase 5+ work. The architecture just needs to not block it.
