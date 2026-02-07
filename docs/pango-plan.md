# Plan: Integracion de Pango/PangoCairo

Fecha: 2026-02-07

## Objetivo
Incorporar `pango = "0.21.5"` y `pangocairo = "0.21.5"` para mejorar la calidad tipografica y el render de texto en el chart (etiquetas de ejes, tooltips, headers, markers, etc.). Esto habilita:
- Mejor layout de texto (kerning, ligaduras, shaping).
- Soporte robusto de fuentes y locales.
- Medicion precisa de texto para evitar cortes/solapes.
- Estilos avanzados (bold/italic) sin hacks.

## Beneficios esperados
- Labels y tooltips con alineacion mas precisa (menos overflow).
- Mejor soporte para simbolos, idiomas y caracteres especiales.
- Consistencia visual entre plataformas (GTK ya usa Pango internamente).
- Simplificacion de calculos manuales de ancho/alto de texto en el render de Cairo.

## Alcance tecnico
Pango se usa para layout de texto; PangoCairo permite renderizar ese layout en un `cairo::Context`.

## Cambios propuestos

### Fase 1 ? Dependencias y wrappers basicos
1. Agregar dependencias en `Cargo.toml`:
   - `pango = "0.21.5"`
   - `pangocairo = "0.21.5"`
2. Crear helper `src/chart/text.rs` con:
   - `measure_text(cr, text, font_desc, size) -> (width, height, baseline)`
   - `draw_text(cr, text, x, y, color, font_desc, size, align)`
3. Agregar tests de medicion (si aplica) o snapshot manual.

### Fase 2 ? Migrar render de ejes y overlays
1. Reemplazar mediciones actuales en:
   - `src/chart/core/render_axes.rs`
   - `src/chart/core/render_overlays.rs`
   - `src/chart/core/render_crosshair.rs`
2. Mantener la misma API p?blica; solo cambia el backend de layout.
3. Validar no regression visual (comparar con capturas previas).

### Fase 3 ? Tooltips y markers
1. Reemplazar layout de tooltips y labels con Pango.
2. Ajustar padding con medidas reales de texto (no aproximadas).
3. Mejorar clipping si el label excede el area.

### Fase 4 ? Localizacion y formatos
1. Integrar con `LocalizationOptions` (si se agrega luego).
2. Medir strings con formatos largos (fecha completa, numeros grandes).

## Consideraciones
- Pango agrega overhead; minimizar layouts repetidos (cachear `Layout` por frame).
- Medicion por frame puede ser costosa: cachear por string/estilo.
- Mantener compatibilidad con `ChartStyle::axis_font_size` (mapear a `pango::FontDescription`).

## Impacto en el proyecto
- Refactor de render de texto centralizado.
- Mejor consistencia tipografica y soporte multi?idioma.
- Menos bugs en alineacion de etiquetas y tooltips.

## Tareas sugeridas (future)
1. Definir helper `TextRenderer` con cache LRU simple.
2. Migrar un modulo a la vez (axes -> overlays -> tooltip -> markers).
3. Agregar ejemplo que use caracteres no ASCII para validar shaping.
