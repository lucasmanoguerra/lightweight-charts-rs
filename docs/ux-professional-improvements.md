# Ideas de Mejoras UX/Profesional (Futuro)

Fecha: 2026-02-07

## Objetivo
Listar mejoras funcionales y de experiencia de usuario para hacer el proyecto mas profesional y agradable, con enfoque desktop. Este documento es para planificacion futura.

## 1) Barra de herramientas de dibujo y analisis tecnico
Implementar un toolbar lateral o superior con herramientas de analisis financiero:
- Trendline (linea de tendencia con snapping a OHLC).
- Horizontal/Vertical line.
- Ray/Extended line.
- Channel (parallel lines).
- Fibonacci Retracement/Extension.
- Rectangle/Box (zonas de soporte/resistencia).
- Price Range / Date Range / Date & Price Range.
- Position tool (Long/Short) con PnL, riesgo/beneficio y stop/TP.

Requisitos tecnicos:
- Sistema de primitives/overlays por panel.
- Handle/anchors editables.
- Snap a series y time scale.
- Persistencia de dibujos (serialize/deserialize).

## 2) Gestor de layouts y workspaces
- Guardar y cargar configuraciones de chart (series, indicadores, estilos, escalas).
- Layouts multi?panel (2, 3 o 4 charts en grid).
- Plantillas de layout (day trading, swing, cripto).

## 3) Sistema de alertas
- Alertas de precio (cross, above/below).
- Alertas de indicadores (RSI > 70, MACD cross).
- Notificaciones desktop y logging.

## 4) Watchlist y simbolos favoritos
- Sidebar con watchlist.
- Busqueda con autocompletado.
- Switch rapido de simbolo/timeframe.

## 5) Exportacion y reportes
- Exportar imagen PNG/SVG del chart.
- Exportar datos (CSV/JSON) del rango visible.
- Reporte rapido con estadisticas (rendimiento, volatilidad, drawdown).

## 6) Mejoras visuales y profesionalizacion
- Temas predefinidos (dark, light, high?contrast).
- Tipografia avanzada (Pango).
- Mejoras en tooltip y crosshair.
- Watermark, logo y branding.
- Animaciones suaves en pan/zoom (opcional).

## 7) Interacciones avanzadas
- Multi?cursor y crosshair sincronizado entre panels.
- Snap inteligente a niveles clave (high/low, cierre, VWAP).
- Bloqueo de escala/tiempo.
- Modo presentacion (pantalla completa, ocultar UI).

## 8) Indicadores profesionales
- Ampliar catalogo: VWAP, ADX, CCI, MFI, ATR, SAR, OBV, Williams %R.
- Indicadores personalizados (UI para parametros).
- Guardar presets.

## 9) Rendimiento y estabilidad
- Conflation para datasets grandes.
- Cache de layout y texto.
- Perfilado y trazas con `tracing`.

## 10) Accesibilidad y usabilidad
- Atajos de teclado configurables.
- Modo alto contraste.
- Navegacion por teclado en controles.

## Propuesta de fases (alto nivel)
1. Primitives + toolbar basico (trendline, horizontal line).
2. Persistencia de dibujos + export de imagen.
3. Watchlist + alertas basicas.
4. Indicadores avanzados + presets.
5. UX polish (temas, tipografia, animaciones).

## Notas
- Varias mejoras dependen de un sistema de primitives y de un modelo de layout mas estructurado.
- Priorizar aquellas que agregan valor inmediato al usuario (drawing tools y alertas).
