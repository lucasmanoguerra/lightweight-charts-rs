# AGENTS_AREA_TEMPLATE.md

Standard Template for Creating New UI Areas

## Purpose

This file defines the canonical template that MUST be used
when creating any new Area (pane/section/module) inside the chart engine.

All UI sections MUST follow this structure.

If your Area deviates from this template, the design is wrong.

# Philosophy

Each Area must:

* do ONE thing only
* be fully isolated
* be replaceable
* not know about other areas
* be testable headless
* render only its rect
* communicate only through events

UNIX style:
small tools connected by pipes.

# Hard Rules (MANDATORY)

An Area MUST NOT:

✗ access global state
✗ call another Area
✗ mutate ChartContext
✗ fetch network/datafeed
✗ allocate heavily inside render()
✗ perform layout outside layout()
✗ contain business logic unrelated to its role

An Area MUST:

✓ implement Area trait
✓ emit events only
✓ read state from context only
✓ be independently testable
✓ include tests
✓ include benchmarks

# Folder Structure

Every Area must follow:

src/areas/<area_name>/

```
mod.rs
area.rs
state.rs
events.rs
render.rs
layout.rs
tests.rs
benches.rs
```

## Responsibilities by file

area.rs
orchestration only

state.rs
local state only

layout.rs
geometry only

render.rs
cairo/pango drawing only

events.rs
event handling only

tests.rs
unit + integration tests

benches.rs
criterion benchmarks

# Area Trait (MANDATORY)

All areas MUST implement:

```
pub trait Area {
    fn id(&self) -> AreaId;

    fn layout(&mut self, rect: Rect);

    fn update(&mut self, ctx: &ChartContext);

    fn render(&self, cr: &cairo::Context);

    fn handle_event(&mut self, event: &InputEvent) -> Vec<ChartEvent>;
}
```

# Method Contracts

## layout(rect)

ONLY:

* compute geometry
* cache sizes

NEVER:

* render
* allocate heavy memory
* read context

## update(ctx)

ONLY:

* read shared state
* update internal caches

NEVER:

* emit events
* mutate ctx

## render(cr)

ONLY:

* draw using cairo/pango
* use cached values

NEVER:

* allocate
* compute heavy math
* fetch data

## handle_event(event)

ONLY:

* interpret input
* return ChartEvents

NEVER:

* modify other areas
* modify context directly

# Minimal Template Code

area.rs

---

pub struct TemplateArea {
id: AreaId,
rect: Rect,
state: TemplateState,
}

impl Area for TemplateArea {
fn id(&self) -> AreaId {
self.id
}

```
fn layout(&mut self, rect: Rect) {
    self.rect = rect;
    layout::compute(&mut self.state, rect);
}

fn update(&mut self, ctx: &ChartContext) {
    self.state.update(ctx);
}

fn render(&self, cr: &cairo::Context) {
    render::draw(cr, &self.state, self.rect);
}

fn handle_event(&mut self, event: &InputEvent) -> Vec<ChartEvent> {
    events::handle(&mut self.state, event)
}
```

}

---

# State Rules

state.rs may contain:

✓ cached values
✓ layout metrics
✓ hover state
✓ local UI state

Must NOT contain:

✗ shared transforms
✗ global data
✗ business logic from other modules

# Rendering Rules (cairo/pango)

* draw only inside rect
* clip to bounds
* reuse text layouts
* reuse paths
* avoid allocations
* avoid string formatting per frame

Cache:

✓ PangoLayout
✓ Paths
✓ glyphs
✓ precomputed coordinates

# Performance Rules

Areas must:

* run < 0.1ms typical
* allocate zero memory during render
* avoid locks
* avoid clones
* preallocate buffers

If render allocates → fix immediately.

# Testing Rules

tests.rs must include:

✓ unit tests for logic
✓ event handling tests
✓ layout tests
✓ snapshot render tests

Area must compile without full app.

# Benchmarks

benches.rs must measure:

✓ layout()
✓ update()
✓ render()

Using:

criterion

Targets:

* stable time
* zero allocations
* predictable scaling

# Agent Instructions (FOR CODEX)

When creating a new Area:

1. Copy this template
2. Rename files
3. Implement only local responsibility
4. Do not import other areas
5. Communicate via events only
6. Add tests
7. Add benchmarks

If feature requires touching multiple areas:
→ architecture is wrong
→ refactor first

# Definition of Done

A new Area is complete only if:

✓ implements Area trait
✓ isolated module
✓ no cross imports
✓ tests pass
✓ benches run
✓ zero render allocations
✓ documented responsibility

# Summary

Area = tiny program

input → local state → draw → emit events

Nothing more.

If it's bigger than that, split it.
