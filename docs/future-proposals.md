# Otras Propuestas para Documentar (Futuro)

Fecha: 2026-02-07

## Objetivo
Registrar ideas adicionales para futuras implementaciones que mejoren el proyecto, organizadas por area.

## 1) Datos y sincronizacion
- **Reproductor de mercado (Playback)**: barra de tiempo para reproducir historico con velocidad configurable.
- **Sincronizacion multi?chart**: time scale y crosshair sincronizados entre charts.
- **Cache local de datos**: persistencia en disco para simbolos recientes.

## 2) UX y productividad
- **Command Palette**: buscador rapido de acciones (cambiar simbolo, agregar indicador, etc.).
- **Atajos configurables**: asignar keys a tools e indicadores.
- **Modo presentacion**: full screen + ocultar UI no esencial.

## 3) Personalizacion y estilos
- **Theme editor**: UI para editar colores, tama?os, tipografia.
- **Plantillas exportables**: guardar/compartir estilos en JSON.

## 4) Herramientas de trading
- **Alertas avanzadas**: cruces de indicadores, condiciones compuestas.
- **Registro de operaciones**: track de trades con PnL en el chart.
- **Notas y anotaciones**: comentarios por vela o rango.

## 5) Render y performance
- **Culling por viewport**: render solo lo visible.
- **Batching de textos**: cache de layouts.
- **Conflation configurable**: thresholds por serie.

## 6) Integraciones externas
- **Fuentes de datos pluggables**: REST/WebSocket intercambiables.
- **Soporte para CSV/JSON import**.
- **API scripting**: ejecutar macros o estrategias simples.

## 7) QA y estabilidad
- **Snapshot tests** de render (golden images).
- **Benchmark suite** para escenarios de 10k/100k puntos.
- **Crash reporting local** (opcional).

## Proximo paso
- Priorizar 3?5 propuestas y convertirlas en backlog con dependencias.
