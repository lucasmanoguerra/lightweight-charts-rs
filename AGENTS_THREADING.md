# AGENTS_THREADING.md

Concurrency, Workers & Snapshot Publishing Contract

## Purpose

Define how concurrency and multithreading are used safely in the chart engine.

This system must guarantee:

* zero blocking in render thread
* deterministic behavior
* no locks in hot paths
* safe parallelism
* smooth 60–240 FPS
* reproducible bugs

Threading exists ONLY to move heavy work off the render path.

If render ever waits → architecture is wrong.

# Core Philosophy

Render must be sacred.

Nothing blocks render.
Nothing locks render.
Nothing computes inside render.

All heavy work happens elsewhere.

Mental model:

Render thread = real-time system
Workers = background batch processors

# Golden Rules

Render thread MUST NEVER:

✗ block
✗ lock Mutex/RwLock
✗ wait on channels
✗ allocate large memory
✗ compute indicators
✗ parse data
✗ fetch network

Render must only:

✓ read snapshot
✓ update areas
✓ draw

# Thread Roles (MANDATORY)

Separate responsibilities strictly.

1. Render Thread

---

Responsibilities:

✓ events
✓ context update
✓ layout
✓ render

Must be:

single-threaded
lock-free
fast

Never does heavy work.

2. Datafeed Thread(s)

---

Responsibilities:

✓ websocket
✓ REST historical fetch
✓ parsing
✓ buffering

Produces:

Raw market events

3. Worker Pool

---

Responsibilities:

✓ indicators
✓ resampling
✓ aggregation
✓ path precompute
✓ heavy math

Produces:

Computed results

4. Snapshot Publisher

---

Responsibilities:

✓ build immutable DataStore
✓ publish Arc snapshot
✓ atomic swap

No other threads allowed to mutate shared data.

# Communication Model

ALL communication uses:

✓ channels
✓ events
✓ Arc snapshots

NEVER:

✗ shared mutable state
✗ locks across threads

# Architecture Diagram

Datafeed
↓ channel
Workers
↓ compute
Snapshot Builder
↓ Arc<DataStore>
ChartView (render thread)
↓
Areas

One-way flow only.

No back pressure into render.

# Snapshot Publishing (CRITICAL)

Workers NEVER mutate live data.

Correct:

1. compute new Vec
2. build new DataStore
3. wrap in Arc
4. atomic swap

Example:

```
let new_store = Arc::new(store);
self.current_store.store(new_store);
```

Render sees new pointer next frame.

No locks.
No waiting.
No copying.

# Channel Rules

Use:

✓ crossbeam::channel
✓ tokio mpsc
✓ lock-free queues

Avoid:

✗ std::sync::mpsc (slow)
✗ blocking recv in render
✗ synchronous requests

Render may only:

try_recv() (non-blocking)

# Worker Pool Rules

Use:

✓ rayon
✓ tokio tasks
✓ fixed thread pool

Never:

✗ spawn unlimited threads
✗ spawn per candle
✗ create threads inside render

Workers must be long-lived.

# Indicator Computation

Indicators MUST:

✓ compute off-thread
✓ output arrays
✓ publish snapshot

Never:

✗ compute inside render()
✗ compute inside update()

Flow:

data → worker → compute → snapshot → render reads

# Determinism Rule

Given same:

* input events
* datafeed stream

Must produce same output.

Therefore:

✓ no time-dependent logic
✓ no thread timing dependencies
✓ no shared mutable state
✓ no race conditions

Snapshots make state deterministic.

# Locking Policy

Allowed:

✓ Mutex in datafeed
✓ Mutex inside worker internals

Forbidden:

✗ locks inside render
✗ locks inside Areas
✗ locks in DataStore access

If you need lock in render:
→ redesign architecture

# Memory Rules

Prefer:

✓ Arc
✓ atomics
✓ channels
✓ copy-on-write

Avoid:

✗ shared Vec mutation
✗ interior mutability
✗ RefCell across threads

Everything shared must be immutable.

# Backpressure Strategy

If workers lag:

Allowed:

✓ drop old frames
✓ coalesce updates
✓ keep latest only

Not allowed:

✗ blocking render until done

Real-time rendering always wins.

# Task Granularity

Workers should:

✓ batch work
✓ process chunks

Avoid:

✗ micro tasks
✗ per-candle tasks

Too many small tasks = overhead.

# Performance Targets

Render thread:

< 2 ms budget

Worker threads:

any time acceptable

If render spikes → profiling required.

# Testing Rules

Threading must support:

✓ deterministic replay
✓ single-thread fallback
✓ mock channels
✓ stress tests

Must run headless.

Bugs must be reproducible.

# Agent Instructions (CRITICAL FOR CODEX)

When adding heavy computation:

ALWAYS:

✓ move to worker
✓ communicate via channel
✓ publish snapshot

NEVER:

✗ compute inside render
✗ add Mutex around shared data
✗ wait for worker result

If you need result immediately:
→ redesign to async snapshot

# Architecture Smells

If you see:

Mutex in Area
RwLock in DataStore
recv() blocking in render
thread::sleep in render
indicator compute in update()

You broke threading rules.

# Definition of Done

Threading change is valid only if:

✓ render remains lock-free
✓ no blocking calls
✓ snapshot based
✓ deterministic
✓ workers isolated
✓ stress tested

# Summary

Threading exists to:

protect render

Compute elsewhere.
Publish snapshot.
Render reads only.

Never wait.
Never lock.
Never block.

Render must always stay real-time.
