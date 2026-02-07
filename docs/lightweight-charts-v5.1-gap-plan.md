# Lightweight Charts v5.1 Gap Analysis y Plan de Implementacion (Desktop)

Fecha: 2026-02-07

## Alcance
Este documento compara el proyecto `lightweight-charts-rs` con la libreria **TradingView Lightweight Charts v5.1** para identificar funcionalidades faltantes y proponer un plan de implementacion futuro. El objetivo es paridad funcional en **desktop** (GTK4), no compatibilidad web.

## Gap Analysis (resumen)

### 1) Tipos de series y API de series
**Lo que v5.1 ofrece:** Area, Bar, Baseline, Candlestick, Histogram, Line y Custom Series (plugins). Referencia: docs oficiales de series soportadas y API de series. ?cite?turn1search0?turn1search1?
**Estado actual (repo):** Candlestick, Line, Histogram.
**Faltante:**
- Area series.
- Bar series.
- Baseline series.
- Custom series (plugin API).

### 2) Sistema de plugins y primitives
**Lo que v5.1 ofrece:** Plugins con Custom Series y Primitives (series/pane), y cambios donde Watermark y Series Markers pasan a plugins. Referencias: docs de plugins y release notes v5.1. ?cite?turn0search3?turn0search6?turn0search0?
**Estado actual:** No existe arquitectura de plugins/primitives.
**Faltante:**
- API para registrar/detachar primitives (series y pane).
- Watermark como pane primitive.
- Series markers como plugin (o abstraccion equivalente).

### 3) Conflation (optimizaci?n de performance)
**Lo que v5.1 agrega:** Data Conflation con opciones `enableConflation`, `conflationThresholdFactor`, `precomputeConflationOnInit`, `precomputeConflationPriority` y soporte para reducers personalizados. Referencias: release notes v5.1 y opciones de `TimeScaleOptions`. ?cite?turn0search0?turn0search5?
**Estado actual:** No existe conflation.
**Faltante:**
- Conflation a nivel de time scale/series y pipeline de render.
- Configuracion de conflation y precomputo.

### 4) Crosshair
**Lo que v5.1 agrega:** `doNotSnapToHiddenSeriesIndices` en `CrosshairOptions`. ?cite?turn0search0?
**Estado actual:** Crosshair sin ese flag.
**Faltante:**
- Opcion para evitar snap a series ocultas.

### 5) Formateo y localizacion
**Lo que v5.1 ofrece:** `LocalizationOptionsBase` / `LocalizationOptions` con `priceFormatter`, `tickmarksPriceFormatter`, `timeFormatter`, etc., y `base` en `PriceFormatCustom` (alternativa a `minMove`). ?cite?turn0search2?turn0search4?turn0search1?
**Estado actual:**
- Formato de precios basico (`precision` + `min_move`) y formatos simples de tooltip/time label.
**Faltante:**
- API de localizacion y formatters avanzados.
- `base` en price format.

### 6) API de series (extras de v5.x)
**Lo que v5.1 agrega/expone:** `priceLines()` en `ISeriesApi` (release notes v5.1). ?cite?turn0search0?
**Estado actual:**
- Tenemos `create_price_line`, pero no listado/consulta.
**Faltante:**
- `priceLines()` (o equivalente para listar/inspeccionar price lines).

## Plan de Implementacion (futuro)

### Fase 0 ? Base de API y compatibilidad
1. Definir modelo interno de series generico (serie + tipo + data) para soportar Area/Bar/Baseline.
2. Agregar `priceLines()` (o equivalente) a la API actual de series.
3. Agregar `PriceFormat::Base` y wiring en formateo de precios.
4. Agregar estructura de `LocalizationOptions` y callbacks de formatters.

### Fase 1 ? Nuevos tipos de series
1. Implementar Area series (render, opciones, leyenda y tooltip).
2. Implementar Bar series (OHLC bar). Reutilizar Candle/Bar data.
3. Implementar Baseline series (linea + relleno positivo/negativo).
4. Agregar ejemplos y tests visuales basicos (render deterministico).

### Fase 2 ? Conflation (performance)
1. Definir modelo de agregacion por serie (open/high/low/close, min/max, etc.).
2. Implementar conflation en pipeline de render cuando `bar_spacing < threshold`.
3. Agregar opciones de configuracion y precomputo.
4. Medir performance con datasets grandes (100k+ puntos).

### Fase 3 ? Plugins/Primitives
1. Dise?ar interfaces equivalentes a `ISeriesPrimitive` y `IPanePrimitive`.
2. Implementar lifecycle (attach/detach) y update pipeline.
3. Migrar markers a plugin interno (mantener API amigable).
4. Agregar watermark como pane primitive.

### Fase 4 ? Crosshair y localizacion avanzada
1. Agregar `doNotSnapToHiddenSeriesIndices` en opciones de crosshair.
2. Wirear formatters en price scale y time scale.

### Fase 5 ? Paridad fina y QA
1. Validar comportamiento de `fitContent` y offsets cuando se adopte conflation.
2. Alinear ticks/labels con opciones avanzadas.
3. Documentar API final y actualizar ejemplos.

## Notas
- La libreria de referencia es web; adaptaremos el rendering a GTK4 manteniendo la logica y semantica.
- Algunas mejoras (plugins, conflation) requieren cambios estructurales en el core.
