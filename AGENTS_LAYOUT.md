# AGENTS_LAYOUT.md

Layout & Pane Architecture Contract

## Purpose

Define strict rules for how UI areas (panes/sections) are structured, composed, and connected inside the chart engine.

This project follows a UNIX philosophy:

* each section does ONE thing well
* sections are isolated
* sections communicate only through shared state/events
* no direct coupling between areas
* everything is replaceable

If a feature cannot be implemented as an independent Area, the design is wrong.

# Core Principles

1. Composition over inheritance
2. No global state
3. No cross-area calls
4. Layout is declarative
5. Rendering is local
6. State is shared through context only
7. Every visual block = Area

# Mental Model

Workspace
├── ChartView
│     ├── ToolbarArea
│     ├── MainPane
│     ├── IndicatorPane*
│     ├── TimeScaleArea
│     ├── PriceScaleArea
│     └── ControlsArea
├── ChartView
└── ChartView

A ChartView is ONLY a coordinator.
It never renders directly.

# Area Contract (MANDATORY)

Every visual section MUST implement:

```
trait Area {
    fn layout(&mut self, rect: Rect);
    fn update(&mut self, ctx: &ChartContext);
    fn render(&self, cr: &cairo::Context);
    fn handle_event(&mut self, event: &Event);
}
```

Rules:

* layout() → compute geometry only
* update() → read shared state only
* render() → draw only
* handle_event() → emit events only

NEVER mix responsibilities.

# Hard Rules (DO NOT BREAK)

❌ Areas must NOT call other Areas
❌ Areas must NOT access global state
❌ Areas must NOT modify ChartView directly
❌ Areas must NOT perform layout outside layout()
❌ ChartView must NOT render graphics

✅ Areas communicate ONLY through events
✅ Areas read shared data ONLY via ChartContext
✅ Areas are swappable modules

# Communication Model

Shared state (read-only):

```
ChartContext {
    time_transform
    price_transform
    visible_range
    cursor
    data_series
    theme
}
```

Events (write):

```
enum ChartEvent {
    ZoomX(f32),
    ScrollX(f32),
    ZoomY(f32),
    AutoScale(bool),
    ToggleLog(bool),
    ToolSelected(ToolId),
    AddIndicator(IndicatorId),
    RemovePane(PaneId),
}
```

Flow:

Area → emits event
ChartView → updates state
ChartView → triggers redraw
All Areas → read updated context

Never Area → Area.

# Layout System

Layout is tree-based and declarative.

Supported containers:

* Stack (vertical)
* Split (horizontal/vertical)
* Fixed (absolute size)
* Fill (flex grow)

Example:

```
Split(H)
  ├── ChartView
  └── ChartView
```

Inside ChartView:

```
Stack(V)
  ├── MainPane (flex)
  ├── IndicatorPane (flex)
  ├── IndicatorPane (flex)
  └── TimeScale (fixed)
```

Rules:

* layout must be pure
* no drawing inside layout
* sizes must be deterministic
* panes must be resizable

# Pane Types

MainPane

* candles
* overlays
* tools
* crosshair

IndicatorPane

* indicators only
* independent price scale

ToolbarArea

* tool selection only

TimeScaleArea

* x axis only

PriceScaleArea

* y axis only

ControlsArea

* autoscale/log buttons only

If new feature mixes responsibilities → split it.

# Multi-Chart Rules

Charts must be fully isolated.

Charts may share:

* datafeed
* cache

Charts must NOT share:

* transforms
* cursor
* layout state

Synchronization must use explicit events only.

# Performance Rules

Each Area:

* must render only its own rect
* must not allocate during render
* must cache text/layout when possible
* must be benchmarkable independently

Rendering must be incremental when possible.

# Testing Rules

Every Area must support:

* unit tests (logic)
* snapshot tests (render)
* benchmarks

Areas must be instantiable without the full app.

# Agent Instructions (IMPORTANT FOR CODEX)

When adding a new feature:

1. Create a new Area module
2. Implement Area trait
3. Do not modify other areas
4. Use events/context only
5. Register it in ChartView layout tree

If you need to modify multiple areas,
the architecture is wrong → refactor.

# Definition of Done

A layout feature is complete only if:

* compiles independently
* has no cross imports
* works inside any ChartView
* can be removed without breaking others
* passes tests
* has benchmarks

# Summary

Everything is:

small
isolated
replaceable
composable

Like UNIX pipes, but for UI.

If it's not modular, it's not allowed.
