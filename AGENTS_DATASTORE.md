# AGENTS_DATASTORE.md

Market Data Storage, Memory Layout & Zero-Copy Contract

## Purpose

Define how all historical and real-time market data is stored in memory.

This document exists to guarantee:

* zero-copy access
* cache-friendly iteration
* fast slicing
* instant snapshots
* smooth scrolling with millions of candles
* deterministic behavior
* no locks during rendering

The DataStore is effectively an in-memory time-series database.

If DataStore is slow, everything is slow.

# Core Philosophy

Data must be:

contiguous
immutable
append-only
snapshot-based
zero-copy

Rendering must read slices only.

Never clone.
Never lock.
Never recompute.

# Golden Rules

DataStore MUST:

✓ be immutable snapshot
✓ be Arc-backed
✓ be contiguous memory
✓ allow O(1) slicing
✓ allow cheap cloning (Arc)

DataStore MUST NOT:

✗ use Mutex/RwLock in hot path
✗ use HashMap for candles
✗ allocate per frame
✗ clone big Vecs
✗ mutate while rendering

# Ownership Model

Datafeed thread
↓
builds new snapshot
↓
Arc<DataStore>
↓
ChartView swaps pointer
↓
Areas read-only

Single writer
Multiple readers

Lock-free.

# Snapshot Strategy (MANDATORY)

When new data arrives:

WRONG:

push into shared Vec

CORRECT:

create new Arc<DataStore>
swap pointer atomically

Example:

self.store = Arc::new(new_store);

Old readers continue safely.
New frame reads new snapshot.

Zero locks.

# Memory Layout (CRITICAL)

Use SoA (Struct of Arrays), NOT AoS.

Wrong (AoS):

```
struct Candle {
    time, open, high, low, close, volume
}

Vec<Candle>
```

Bad for cache.

Correct (SoA):

```
struct CandleSeries {
    time:   Arc<[i64]>,
    open:   Arc<[f32]>,
    high:   Arc<[f32]>,
    low:    Arc<[f32]>,
    close:  Arc<[f32]>,
    volume: Arc<[f32]>,
}
```

Benefits:

✓ cache-friendly
✓ SIMD friendly
✓ faster iteration
✓ smaller memory footprint
✓ easier slicing

# Access Pattern

Rendering must do:

```
let slice = &series.close[start..end];
```

NOT:

```
clone()
collect()
map()
```

Always slice, never copy.

# DataStore Structure

Example:

```
pub struct DataStore {
    pub candles: CandleSeries,
    pub indicators: IndicatorStore,
    pub metadata: Arc<Metadata>,
}
```

Keep it:

flat
simple
contiguous

# Indicator Storage

Indicators must also be SoA.

Wrong:

Vec<Vec<f32>>

Correct:

struct IndicatorSeries {
values: Arc<[f32]>,
}

Multiple indicators:

Vec<IndicatorSeries>

Each pane slices independently.

# Append Strategy

Datafeed builds using Vec during construction:

```
let mut open = Vec::with_capacity(n);
```

Then freeze:

```
open.into()
```

Convert to Arc<[T]>.

Never grow after publish.

# Zero-Copy Rules

Allowed:

✓ &slice
✓ Arc clone
✓ index math

Forbidden:

✗ to_vec()
✗ clone()
✗ collect()
✗ iterators allocating

If profiler shows memory copy → fix.

# Range Access (HOT PATH)

Must be O(1):

```
fn slice(&self, range: Range<usize>) -> CandleSlice
```

Where CandleSlice only contains references:

```
struct CandleSlice<'a> {
    open: &'a [f32],
    high: &'a [f32],
    low:  &'a [f32],
    close:&'a [f32],
}
```

Zero allocation.

# Capacity Rules

Preallocate large buffers.

Example:

Vec::with_capacity(5_000_000)

Avoid repeated reallocations.

Historical data is predictable.

# Compression Options (optional)

Allowed:

✓ f32 instead of f64
✓ delta encoding for timestamps
✓ separate volume storage
✓ packed structs

Not allowed:

✗ runtime decompression during render

Decompression must happen before snapshot.

# Threading Rules

Heavy tasks allowed off-thread:

✓ parsing
✓ indicator calculation
✓ resampling
✓ aggregation

Must finish before snapshot publish.

Render thread must NEVER compute indicators.

# Determinism Rule

Given same snapshot → same render.

Therefore:

✓ no time-based changes
✓ no mutable caches in DataStore

All changes require new snapshot.

# Testing Rules

DataStore must support:

✓ synthetic generators
✓ large datasets (10M+ candles)
✓ deterministic snapshots
✓ benchmarks

Must run headless.

# Benchmarks (MANDATORY)

Benchmark:

✓ slice()
✓ iteration speed
✓ snapshot swap
✓ memory footprint

Targets:

* slicing < 50ns
* iteration near memcpy speed
* zero allocations

# Agent Instructions (CRITICAL FOR CODEX)

When adding new data:

ALWAYS:

✓ add new array
✓ keep SoA layout
✓ use Arc<[T]>
✓ slice only
✓ mutate only during build

NEVER:

✗ add Vec that grows at runtime
✗ add HashMap
✗ add locks
✗ clone series for rendering

If something needs copying:
→ redesign.

# Architecture Smells

If you see:

Vec<Candle>
Mutex<Vec<_>>
clone() inside render
HashMap timestamp lookup
indicator recompute per frame

You broke the DataStore rules.

# Definition of Done

DataStore change is valid only if:

✓ immutable snapshot
✓ zero-copy slices
✓ no locks
✓ no render allocations
✓ contiguous memory
✓ benchmarks pass

# Summary

DataStore is:

in-memory database
immutable snapshot
SoA layout
Arc-backed
zero-copy
cache-friendly

Build once.
Freeze.
Read fast.

Never mutate live data.
