# Propuesta tecnica: Areas modulares + flujo de eventos

Fecha: 2026-02-07

## Objetivo
Traducir el esquema de areas interconectadas en una propuesta tecnica concreta de modulos, structs y flujo de eventos para el render y la interaccion.

## Modulos sugeridos

### `src/ui/sections/`
- `chart_section.rs`      (Area 1)
- `draw_toolbar_section.rs` (Area 2)
- `time_scale_section.rs` (Area 3)
- `price_scale_section.rs` (Area 4)
- `indicator_pane_section.rs` (Area 5)
- `price_scale_controls_section.rs` (Area 6)

### `src/ui/layout/`
- `layout_engine.rs` (calculo de bounds)
- `layout_model.rs`  (modelo de filas/columnas/panes)

### `src/ui/events/`
- `event_router.rs` (hit?testing y dispatch)
- `gesture_state.rs`

## Structs base

### `SectionBounds`
```rust
pub struct SectionBounds {
    pub rect: Rect,
    pub id: SectionId,
}
```

### `SectionId`
```rust
pub enum SectionId {
    DrawToolbar,
    ChartMain,
    IndicatorPane(usize),
    TimeScale,
    PriceScale,
    PriceScaleControls,
}
```

### `ChartLayout`
```rust
pub struct ChartLayout {
    pub toolbar: Rect,
    pub main_panel: Rect,
    pub indicator_panels: Vec<Rect>,
    pub time_scale: Rect,
    pub price_scale: Rect,
    pub price_scale_controls: Rect,
}
```

### `Section` trait
```rust
pub trait Section {
    fn id(&self) -> SectionId;
    fn set_bounds(&mut self, bounds: Rect);
    fn bounds(&self) -> Rect;
    fn render(&self, ctx: &RenderContext);
    fn hit_test(&self, x: f64, y: f64) -> bool;
    fn handle_event(&mut self, event: &UiEvent, ctx: &mut RenderContext);
}
```

## RenderContext (datos compartidos)
```rust
pub struct RenderContext {
    pub chart_state: ChartState,
    pub time_scale: TimeScaleState,
    pub price_scale: PriceScaleState,
    pub indicators: IndicatorStore,
    pub drawings: DrawingStore,
    pub theme: Theme,
}
```

## Flujo de render (alto nivel)
1. `LayoutEngine` calcula bounds de cada seccion.
2. Se setean bounds en cada `Section`.
3. Render secuencial por seccion (orden fijo).

Orden sugerido:
1. DrawToolbar
2. ChartMain
3. IndicatorPanes (en orden)
4. TimeScale
5. PriceScale
6. PriceScaleControls

## Flujo de eventos (hit?testing)

```
Input (mouse/touch) -> EventRouter
  -> hit_test: seccion con bounds que contiene (x,y)
  -> dispatch: section.handle_event(event)
  -> si la seccion altera datos compartidos => invalidar render
```

### Reglas de routing
- `DrawToolbar` captura clicks en herramientas.
- `ChartMain` captura pan/zoom/crosshair.
- `IndicatorPanes` capturan hover y crosshair sincronizado.
- `PriceScale` captura zoom vertical y drag.
- `TimeScale` captura zoom horizontal y drag.
- `PriceScaleControls` captura toggles (AutoScale/Log).

## Sincronizacion entre secciones
- `TimeScale` actualiza `visible_range`; todos los panes usan ese rango.
- `PriceScale` principal actualiza `price_range` en `ChartMain`.
- Indicadores usan su propio `PriceScaleState`, pero comparten `TimeScaleState`.
- `DrawToolbar` escribe en `DrawingStore` que luego se renderiza en `ChartMain`.

## Multi?chart
- Cada chart tiene su propio `ChartLayout` y `RenderContext`.
- Opcional: un `TimeScaleSyncGroup` para sincronizar rangos entre charts.

## Notas de implementacion
- Evitar que una seccion haga draw fuera de sus bounds (clip por Cairo).
- `RenderContext` debe ser inmutable en render y mutable en eventos.
- Reutilizar `Rect` y `PanelId` actuales donde aplique.

## Proximos pasos
1. Crear modulos y stubs de `Section`.
2. Implementar `LayoutEngine` basico (padding + alturas fijas).
3. Mover render actual a `ChartMainSection`.
4. Integrar `EventRouter`.
