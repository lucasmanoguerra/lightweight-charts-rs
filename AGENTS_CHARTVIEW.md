# AGENTS_CHARTVIEW.md

ChartView Orchestrator Architecture Contract

## Purpose

Define the responsibilities and limits of ChartView.

ChartView is NOT a renderer.
ChartView is NOT a data processor.
ChartView is NOT a business logic container.

ChartView is ONLY:

* state coordinator
* event router
* area container
* lifecycle manager

Think of ChartView like:
→ a window manager
→ or a UNIX process supervisor

# Philosophy

ChartView must be:

* dumb
* predictable
* side-effect free (except routing)
* easily replaceable

If ChartView starts "doing work", the design is wrong.

All real work belongs to:

* Areas
* Datafeed
* Indicators
* Renderer

# High Level Model

ChartView
├── Areas (UI modules)
├── ChartContext (shared state)
├── EventBus
└── LayoutTree

Flow:

Input → Areas → Events → ChartView → update state → Areas render

ChartView never talks Area ↔ Area directly.

# Responsibilities (ONLY THESE)

ChartView MUST:

✓ hold shared state (ChartContext)
✓ own area list
✓ manage layout tree
✓ route events
✓ trigger redraws
✓ manage pane lifecycle
✓ manage synchronization (zoom/scroll)
✓ connect to datafeed (read-only updates)

ChartView MUST NOT:

✗ render graphics
✗ calculate indicators
✗ perform heavy computations
✗ fetch network data
✗ contain drawing logic
✗ directly modify Area internals
✗ contain business logic

# Core Structures

## ChartContext (shared read-only state)

struct ChartContext {
time_scale: TimeTransform,
price_scales: Vec<PriceTransform>,
visible_range: Range,
cursor: CursorState,
theme: Theme,
data_store: Arc<DataStore>,
}

Rules:

* Areas read only
* ChartView mutates
* no interior mutability inside Areas

## EventBus

enum ChartEvent {
ZoomX(f32),
ScrollX(f32),
ZoomY(PaneId, f32),
CursorMoved(Point),

```
AddPane(PaneKind),
RemovePane(PaneId),

AddIndicator(PaneId, IndicatorId),

ToolSelected(ToolId),

AutoScale(PaneId, bool),
ToggleLog(PaneId, bool),

Redraw,
```

}

Rules:

* Areas emit
* ChartView handles
* never Area → Area

## Area Registry

struct ChartView {
areas: Vec<Box<dyn Area>>,
layout: LayoutTree,
context: ChartContext,
event_queue: Vec<ChartEvent>,
}

Areas must be dynamically pluggable.

# Lifecycle

Per frame loop:

1. poll input
2. dispatch events to areas
3. collect emitted events
4. mutate context
5. recompute layout (if needed)
6. call update(ctx) for all areas
7. call render() for all areas

Strict order required.

Never render before state is stable.

# Event Handling Rules

Pattern:

for event in event_queue:
handle(event)

handle() must:

* update state only
* never call Area methods directly
* schedule redraw if necessary

Example:

ZoomX(delta)
→ update time_scale
→ mark dirty

Never:

ZoomX → call main_pane.zoom()

# Pane Management

ChartView is responsible for:

✓ create pane
✓ destroy pane
✓ reorder panes
✓ resize panes

Areas must NOT create themselves.

Example:

AddIndicatorPane:
→ create IndicatorPane
→ insert into layout
→ attach price scale
→ register area

# Synchronization Rules

Transforms must be single source of truth.

Shared:

* time transform

Independent:

* price transform per pane

Never duplicate transforms inside Areas.

# Data Flow

Datafeed → DataStore → ChartView → Areas

Rules:

* DataStore is immutable snapshot
* Areas never fetch
* Areas never own data

This allows:

✓ caching
✓ replay
✓ testing
✓ deterministic renders

# Performance Rules

ChartView must:

* avoid allocations per frame
* reuse vectors
* batch events
* skip layout if unchanged
* redraw only dirty areas

Heavy tasks must be offloaded.

# Testing Rules

ChartView must support:

✓ headless tests
✓ mock Areas
✓ deterministic event playback
✓ layout tests
✓ event routing tests

Must compile without renderer.

# Agent Instructions (CRITICAL FOR CODEX)

When modifying behavior:

NEVER:

* put logic inside ChartView

ALWAYS:

* move logic into Area or module

When adding feature:

1. create Area
2. emit events
3. update state in ChartView
4. avoid cross dependencies

If you feel forced to:

* import another area
* share mutable state
* add special case

→ refactor architecture first

# Definition of Done

A ChartView change is correct only if:

✓ no rendering code added
✓ no business logic added
✓ no area imports
✓ compiles independently
✓ event driven only
✓ deterministic
✓ testable

# Mental Check

If ChartView disappeared,
Areas should still compile.

If Areas disappeared,
ChartView should still compile.

If not → coupling exists.

# Summary

ChartView is:

router
container
state holder

Nothing else.

Small brain.
Big coordinator.

UNIX style.
