# AGENTS_CONTEXT.md

Shared State, Ownership & Mutability Contract

## Purpose

Define how shared data is stored, owned, mutated and accessed across the system.

ChartContext is the ONLY allowed shared state.

It exists to:

* share data safely
* avoid global state
* guarantee determinism
* enable zero-copy rendering
* prevent cross-area coupling

If something needs to be shared and is NOT in ChartContext,
the architecture is wrong.

# Core Philosophy

UNIX mindset:

Areas = processes
Context = read-only filesystem snapshot

Processes read.
Only the supervisor writes.

Areas NEVER mutate shared state.

# Golden Rule

ChartView → mutates
Areas → read-only

Never the opposite.

# Mental Model

Per frame:

1. ChartView updates context
2. Context becomes immutable snapshot
3. Areas read snapshot
4. Render happens
5. Next frame → new snapshot

No mid-frame mutation allowed.

# Hard Rules (MANDATORY)

Areas MUST NOT:

✗ mutate context
✗ store &mut ChartContext
✗ use interior mutability (RefCell/Mutex) on shared data
✗ own shared data
✗ perform IO
✗ cache references across frames

Areas MUST:

✓ treat context as immutable
✓ copy only small values
✓ borrow slices
✓ be stateless or locally stateful only

# Context Properties

ChartContext MUST be:

✓ read-only for Areas
✓ cheap to clone (Arc)
✓ snapshot based
✓ deterministic
✓ thread-safe
✓ zero-copy for big data

ChartContext MUST NOT:

✗ allocate per frame
✗ contain heavy logic
✗ contain references to Areas
✗ contain renderer objects

# Ownership Model

Data ownership:

Datafeed
↓
DataStore (Arc)
↓
ChartView
↓
ChartContext snapshot
↓
Areas read only

Single writer:
→ ChartView

Multiple readers:
→ Areas

# Recommended Types

Use:

✓ Arc<T>
✓ Arc<[T]>
✓ &[T]
✓ Small structs (Copy)
✓ immutable collections

Avoid:

✗ Rc
✗ RefCell
✗ Mutex (hot path)
✗ clone-heavy Vec
✗ interior mutability

If you need Mutex in render path → redesign.

# ChartContext Structure

Example:

```
pub struct ChartContext {
    pub time_scale: TimeTransform,
    pub price_scales: Arc<[PriceTransform]>,
    pub visible_range: Range<usize>,
    pub cursor: CursorState,
    pub theme: Arc<Theme>,
    pub data: Arc<DataStore>,
    pub settings: Arc<Settings>,
}
```

Rules:

* small values inline
* big values behind Arc
* no &mut

# DataStore Rules

DataStore contains:

✓ candles
✓ volumes
✓ indicators results
✓ cached calculations

Must be:

* immutable snapshot
* append-only or replaced atomically

Example:

```
Arc<[Candle]>
```

Never:

✗ Vec mutated while rendering
✗ locks during draw
✗ copying entire history

# Snapshot Strategy

When new data arrives:

OLD Arc<DataStore>
replaced by
NEW Arc<DataStore>

Cheap pointer swap.

Areas automatically read new snapshot next frame.

Benefits:

✓ zero locks
✓ zero copies
✓ thread safe
✓ deterministic

# Mutation Flow

Correct:

Area emits ChartEvent
ChartView handles event
ChartView mutates state
New context snapshot created
Areas read next frame

Wrong:

Area directly changes transform
Area writes into DataStore
Area stores &mut reference

# Zero-Copy Rules

For large series:

Correct:

```
&ctx.data.candles[range]
```

Wrong:

```
ctx.data.candles.clone()
```

Rendering must NEVER allocate big buffers.

# Lifetime Rules

Areas must NOT:

✗ store references to context fields across frames

Because:

Context may be replaced every frame.

Instead:

✓ copy small scalars
✓ store indices
✓ recompute cheap data

# Threading Rules

Allowed:

✓ background datafeed threads
✓ indicator workers
✓ snapshot swap with Arc

Not allowed:

✗ sharing mutable structures
✗ locks in render path

All heavy compute must finish BEFORE snapshot publish.

# Performance Rules

Context operations must be:

* O(1) access
* zero allocation
* cache friendly
* contiguous memory

Prefer:

✓ Vec
✓ slices
✓ struct of arrays (SoA)

Avoid:

✗ hashmaps in hot path
✗ linked lists
✗ pointer chasing

# Testing Rules

ChartContext must support:

✓ manual construction
✓ fake DataStore
✓ deterministic snapshots
✓ headless tests

Areas must work with mock context.

# Agent Instructions (CRITICAL FOR CODEX)

When adding shared data:

1. add to ChartContext
2. make immutable
3. wrap large data in Arc
4. mutate only in ChartView
5. never give &mut to Areas

If you think:
"I'll just pass &mut"
→ STOP
→ redesign with events

# Architecture Smells

If you see:

RefCell
Mutex in render
static mut
global variables
areas sharing references
clones of big vectors

You broke the Context rules.

# Definition of Done

Context change is valid only if:

✓ read-only for Areas
✓ Arc for big data
✓ no interior mutability
✓ no locks in render
✓ zero-copy access
✓ deterministic

# Summary

ChartContext is:

immutable snapshot
single writer
many readers
zero-copy
safe
predictable

Like a read-only filesystem.

Areas read.
ChartView writes.

Nothing else.
