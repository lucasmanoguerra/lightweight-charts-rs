# Propuesta de Librerias Adicionales (futuro)

Fecha: 2026-02-07

## Objetivo
Investigar librerias adicionales que podrian mejorar `lightweight-charts-rs` y documentar ventajas y areas de impacto. Este documento es para planificacion futura (no implica cambios inmediatos).

## Criterios de seleccion
- Aportar valor claro a performance, calidad visual, robustez o experiencia de usuario.
- Compatible con GTK4/Cairo y el enfoque desktop.
- No duplicar funcionalidades ya cubiertas por dependencias actuales.

## Librerias recomendadas (candidatas)

### 1) `pangocairo` y `pango` (tipografia avanzada)
- Ventaja: layout y rendering de texto de alta calidad (kerning, shaping, ligaduras), mejor medicion para labels y tooltips, soporte multi?idioma.
- Impacto: ejes, tooltips, overlays y markers.
- Nota: MSRV alto en versiones actuales (0.21.x requiere Rust 1.83+). Evaluar compatibilidad con la politica de version del proyecto. ?cite?turn0search0?turn0search2?turn0search8?

### 2) Libreria de indicadores tecnicos (ej. `finlib_ta` o `indexes_rs`)
- Ventaja: ampliar rapidamente el catalogo de indicadores (ADX, MFI, CCI, SAR, OBV, Williams %R, etc.).
- Impacto: mas indicadores para paneles secundarios y overlays.
- Nota: revisar API y licencia antes de adoptar; actualmente hay indicadores basicos en el repo. ?cite?turn0search5?turn0search3?

### 3) Cache LRU para layout y render de texto (`lru` / cache simple propia)
- Ventaja: evitar recalcular layout de texto en cada frame; mejora performance en datasets grandes.
- Impacto: tooltips, labels, ticks de ejes, markers.
- Nota: si se integra Pango, el cache es especialmente valioso.

### 4) `tracing` + `tracing-subscriber`
- Ventaja: instrumentacion estructurada para performance, debugging y timings de render.
- Impacto: core, rendering, data feed, interacciones.
- Nota: se puede habilitar via feature flags sin afectar builds release.

### 5) `rayon` (precomputos paralelos)
- Ventaja: acelerar calculos de indicadores y conflation/precomputation.
- Impacto: indicadores y preprocesamiento de datos.
- Nota: usar solo en tareas batch; evitar en el thread UI.

### 6) `smallvec` o `arrayvec`
- Ventaja: reducir allocations en paths calientes (p.ej. ticks y labels con tamanos peque?os).
- Impacto: render de ejes, arrays temporales de layout.

### 7) `ordered-float`
- Ventaja: ordenamiento estable y seguro de floats (si se requieren mapas/sets/ordenamientos)
- Impacto: util en indices, sorting de datos, caches de rango.

## No recomendadas (por ahora)
- HarfBuzz directo: Pango ya integra shaping y layout; ir directo a HarfBuzz + Cairo agrega complejidad sin necesidad para este proyecto en esta etapa. ?cite?turn0search4?

## Proximo paso sugerido
1. Priorizar 2 o 3 librerias para una prueba de concepto.
2. Definir criterios de exito (latencia de render, calidad visual, estabilidad).
3. Agendar implementacion por fases.
