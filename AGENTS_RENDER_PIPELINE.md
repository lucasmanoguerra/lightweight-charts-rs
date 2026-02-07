# AGENTS_RENDER_PIPELINE.md

Cairo/Pango Render Pipeline & Performance Contract

## Purpose

Define the rendering architecture and performance rules for the chart engine.

This document exists to guarantee:

* 60–240+ FPS
* zero allocations per frame
* predictable latency
* minimal redraw work
* deterministic rendering

Rendering is the hottest path in the entire application.

Every micro-allocation or unnecessary draw call matters.

If something allocates during render, it is a bug.

# Core Philosophy

Rendering must be:

pure
stateless
local
cheap

Render should behave like:

```
pixels = f(context_snapshot, area_state)
```

No IO.
No locks.
No heavy math.
No allocations.

Just draw.

# Golden Rules

During render():

❌ no heap allocations
❌ no Vec::push
❌ no String formatting
❌ no locks
❌ no data cloning
❌ no indicator calculations
❌ no layout calculations

ONLY drawing.

# Frame Pipeline (MANDATORY)

Each frame MUST follow:

1. process events
2. update context snapshot
3. layout pass
4. update pass (areas cache)
5. render pass (draw only)

Never mix update and render.

Never compute during render.

# Rendering Architecture

Hierarchy:

Window
└── ChartView
└── Areas
└── render(cr)

Each Area renders ONLY its own rectangle.

No Area may draw outside its bounds.

# Clipping Rule (MANDATORY)

Every Area MUST clip:

```
cr.rectangle(rect.x, rect.y, rect.w, rect.h);
cr.clip();
```

Prevents overdraw and keeps draw cost bounded.

# Dirty Rect Strategy (CRITICAL)

Never redraw the full window if unnecessary.

Use dirty rectangles.

Each Area must:

* track when it changed
* mark itself dirty

ChartView:

* unions dirty rects
* redraw only those regions

Example:

```
if area.is_dirty() {
    dirty_regions.push(area.rect());
}
```

Benefits:

✓ huge FPS gains
✓ lower CPU
✓ better battery life

# Layer Model

Rendering must be layered.

Recommended order:

1. background
2. grid
3. candles
4. indicators
5. drawings/tools
6. crosshair
7. overlays/text

Never mix layers.

Benefits:

✓ caching
✓ partial redraw
✓ easier debugging

# Cairo Rules

Allowed:

✓ stroke
✓ fill
✓ prebuilt paths
✓ cached surfaces

Avoid:

✗ creating paths per element
✗ save/restore excessively
✗ per-candle state changes

Batch operations whenever possible.

# Batching Rule

Wrong:

for candle in candles:
draw rect

Correct:

build single path
draw once

Example:

```
let path = build_candle_path(&candles);
cr.append_path(&path);
cr.fill();
```

Batching is mandatory for large datasets.

# Text Rendering (Pango)

Text is expensive.

Rules:

✓ cache PangoLayout
✓ reuse fonts
✓ reuse glyphs
✓ update only when text changes

Never:

✗ create layout per frame
✗ format strings per frame

Correct:

layout.set_text(cached_string)

Wrong:

format!("Price: {}", price)

# Geometry Caching

Compute ONCE in update():

✓ candle x positions
✓ y transforms
✓ grid lines
✓ tick labels
✓ indicator paths

Store in Area state.

Render() uses cached values only.

Render must NOT compute transforms.

# Memory Rules

Preallocate:

✓ Vec capacity
✓ paths
✓ buffers

Reuse:

✓ surfaces
✓ layouts
✓ temporary arrays

Avoid:

✗ allocating per frame
✗ cloning large vectors

If profiler shows allocs → fix immediately.

# Surfaces & Caching

Use cached surfaces for:

✓ static grid
✓ background
✓ rarely changing layers

Example:

grid_surface: ImageSurface

Only redraw when scale/theme changes.

Then just:

```
cr.set_source_surface(grid_surface, 0.0, 0.0);
cr.paint();
```

Huge performance win.

# Data Access Rules

Access data as:

✓ slices
✓ contiguous memory

Avoid:

✗ hashmaps
✗ iterators with allocations
✗ boxing

Hot loops must be simple for CPU cache.

# Parallelism

Allowed:

✓ precompute geometry in workers
✓ indicators async
✓ path generation off-thread

Not allowed:

✗ drawing from multiple threads

Cairo rendering stays single-threaded.

Compute elsewhere, draw fast.

# FPS Targets

Minimum:

* 60 FPS

Target:

* 120 FPS

Ideal:

* 240 FPS on simple charts

Budget:

~16ms → 60fps
~8ms → 120fps
~4ms → 240fps

Render must typically stay under:

< 2ms

# Benchmarks (MANDATORY)

Each Area must benchmark:

✓ render()
✓ path building
✓ label layout

Using:

criterion

Track:

* time
* allocations

Render benches must show:

0 allocations

# Testing Rules

Must support:

✓ headless render to ImageSurface
✓ snapshot comparison
✓ pixel diff tests

Example:

render → save PNG → compare hash

Guarantees deterministic visuals.

# Agent Instructions (CRITICAL FOR CODEX)

When generating render code:

ALWAYS:

✓ precompute in update()
✓ cache geometry
✓ reuse buffers
✓ batch draw calls
✓ clip to bounds

NEVER:

✗ allocate inside render
✗ compute indicators
✗ format strings
✗ rebuild layouts
✗ clone big vectors

If you need to compute something:
→ move to update()

# Architecture Smells

If you see:

Vec::new() inside render
format! inside render
new PangoLayout per frame
drawing each candle individually
full window redraw each frame

You broke the pipeline.

# Definition of Done

Render change is valid only if:

✓ no allocations
✓ cached geometry
✓ dirty rect respected
✓ benchmarks pass
✓ FPS stable
✓ deterministic output

# Summary

Render is:

fast
pure
cached
local
zero-copy

Compute first.
Draw later.

Like a GPU mindset, but with Cairo.

If render does work, it's wrong.
