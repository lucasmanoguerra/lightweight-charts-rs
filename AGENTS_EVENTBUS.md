# AGENTS_EVENTBUS.md

EventBus & Communication Contract

## Purpose

Define the ONLY allowed communication mechanism between Areas and ChartView.

This system enforces:

* decoupling
* modularity
* testability
* deterministic behavior

No Area may talk directly to another Area.
All communication MUST go through events.

EventBus is the single source of truth for interaction.

# Core Philosophy

UNIX style:

Programs do not call each other.
They send messages.

Areas are independent processes.
Events are pipes.

If two modules need direct calls, the architecture is wrong.

# Golden Rule

Area → emits events
ChartView → processes events
ChartView → updates state
Areas → read updated state next frame

Never:

Area → Area
Area → ChartView direct mutation
Area → shared mutable state

# Event Flow (MANDATORY)

Per frame:

1. Input arrives
2. ChartView forwards input to Areas
3. Areas return Vec<ChartEvent>
4. ChartView collects events
5. ChartView processes events sequentially
6. Context updated
7. Areas update() with new state
8. Render

Events are ALWAYS processed centrally.

# Event Properties

Events MUST be:

✓ immutable
✓ small (copyable or Arc)
✓ deterministic
✓ serializable (for replay/tests)
✓ side-effect free

Events MUST NOT:

✗ contain references
✗ contain large payloads
✗ contain UI objects
✗ contain closures
✗ contain pointers to areas

# Event Categories

Separate events by responsibility.

## Input Events (user actions)

MouseMove
MouseDown
MouseUp
Wheel
KeyPress
DragStart
DragMove
DragEnd

Produced by:
→ system

Consumed by:
→ Areas

## Chart Events (area intentions)

ZoomX
ScrollX
ZoomY
AutoScale
ToggleLog
CursorMoved
ToolSelected
AddIndicator
RemoveIndicator
AddPane
RemovePane
RequestRedraw

Produced by:
→ Areas

Consumed by:
→ ChartView

## System Events (internal)

DataUpdated
LayoutChanged
ThemeChanged
FrameTick

Produced by:
→ ChartView/Datafeed

Consumed by:
→ Areas

# Event Definitions

Example:

```
#[derive(Clone, Debug)]
pub enum ChartEvent {
    ZoomX(f32),
    ScrollX(f32),
    ZoomY(PaneId, f32),

    AutoScale(PaneId, bool),
    ToggleLog(PaneId, bool),

    ToolSelected(ToolId),

    AddPane(PaneKind),
    RemovePane(PaneId),

    AddIndicator(PaneId, IndicatorId),
    RemoveIndicator(PaneId, IndicatorId),

    CursorMoved(Point),

    RequestRedraw,
}
```

Rules:

* small data only
* identifiers, not references
* pure data

# Area Contract

handle_event must:

```
fn handle_event(
    &mut self,
    input: &InputEvent
) -> SmallVec<[ChartEvent; 4]>;
```

Rules:

✓ return events only
✓ no side effects outside area
✓ no context mutation
✓ no calling ChartView

Area expresses INTENT, not action.

# ChartView Contract

ChartView must:

✓ own event queue
✓ process events sequentially
✓ mutate context
✓ trigger layout/redraw

Pseudo:

```
for event in queue {
    match event {
        ZoomX(d) => context.time_scale.zoom(d),
        ScrollX(d) => context.time_scale.scroll(d),
        AddPane(k) => self.add_pane(k),
    }
}
```

Never:

* call area methods
* perform rendering
* perform heavy compute

# Determinism Rule

Same input sequence must produce same result.

Therefore:

✓ no random inside events
✓ no time.now()
✓ no IO inside handlers

All external effects must be injected.

# Performance Rules

Event system must:

* allocate zero or near-zero
* use SmallVec or preallocated Vec
* batch events
* avoid dynamic dispatch if possible

Events are hot path.

# Serialization Rule (IMPORTANT)

Events must support:

✓ logging
✓ replay
✓ testing
✓ debugging

Implement:

```
serde::Serialize
serde::Deserialize
```

This enables:

* backtesting
* input replay
* deterministic bug reproduction

# Testing Rules

Must support:

✓ replay recorded events
✓ simulate interactions headless
✓ fuzz testing events
✓ property tests

Example:

```
replay(events.json)
assert(final_state)
```

# Agent Instructions (CRITICAL FOR CODEX)

When adding new behavior:

NEVER:

* add direct area references
* share mutable state
* call other modules directly

ALWAYS:

* define new ChartEvent variant
* emit event
* handle inside ChartView

If you need cross communication:
→ create event
→ do not bypass system

# Definition of Done

Event change is complete only if:

✓ new enum variant added
✓ handled in ChartView
✓ no direct calls added
✓ serializable
✓ tests updated
✓ deterministic

# Architecture Smell Detection

If you see:

* Rc<RefCell<Area>>
* &mut ChartView inside Area
* global static state
* cross imports between areas

You broke the EventBus rules.

# Summary

EventBus is:

the only communication channel

Areas speak with events.
ChartView listens.
State updates.
Next frame renders.

Like UNIX pipes.

Loose coupling always.
