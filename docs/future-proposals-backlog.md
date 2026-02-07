# Backlog Prioritario (Top 5 Propuestas)

Fecha: 2026-02-07

## Objetivo
Seleccionar y priorizar las propuestas mas valiosas para el proyecto, con dependencias y esfuerzo estimado.

## Leyenda
- Prioridad: P0 (critico), P1 (alto), P2 (medio)
- Esfuerzo: S (peque?o), M (medio), L (grande)

## Top 5 Propuestas Prioritarias

### 1) Sincronizacion multi?chart (time scale + crosshair)
- Prioridad: P1
- Esfuerzo: M
- Dependencias: modularizacion de areas + layout estable
- Entregables:
  - `TimeScaleSyncGroup` compartido
  - Crosshair sincronizado
  - Toggle UI para link/unlink

### 2) Playback de historico
- Prioridad: P1
- Esfuerzo: M
- Dependencias: data feed modular y control de time scale
- Entregables:
  - Barra de tiempo con play/pause
  - Velocidad configurable
  - Modo step?by?step

### 3) Exportacion PNG/SVG
- Prioridad: P1
- Esfuerzo: S
- Dependencias: none
- Entregables:
  - Exportar snapshot del chart
  - Opcion de exportar solo panel visible

### 4) Command Palette
- Prioridad: P2
- Esfuerzo: M
- Dependencias: actions registry
- Entregables:
  - Buscador de acciones
  - Atajos rapidos

### 5) Alertas basicas
- Prioridad: P2
- Esfuerzo: M
- Dependencias: none
- Entregables:
  - Alertas de precio (cross/above/below)
  - Notificacion desktop + log

## Dependencias clave
- Modularizacion y layout estable habilitan sincronizacion multi?chart.
- Data feed modular facilita playback y replay.

## Siguientes pasos sugeridos
1. Implementar export PNG/SVG (rapido y visible al usuario).
2. Preparar sync multi?chart (TimeScaleSyncGroup).
3. Disenar UI para playback.
