# Backlog UX/Professional - Prioridades y Dependencias

Fecha: 2026-02-07

## Objetivo
Convertir la lista de mejoras UX/profesionales en un backlog priorizado con dependencias, esfuerzo estimado y entregables.

## Leyenda
- Prioridad: P0 (critico), P1 (alto), P2 (medio), P3 (bajo)
- Esfuerzo: S (peque?o), M (medio), L (grande), XL (muy grande)
- Dependencias: requerimientos tecnicos previos

## Backlog Prioritizado

### P0 ? Base estructural
1. **Sistema de primitives/overlays**
   - Esfuerzo: L
   - Dependencias: none
   - Entregables:
     - API para registrar primitives en panel/serie.
     - Ciclo de vida: attach, detach, update, render.

2. **Modelo de layout multi?panel estable**
   - Esfuerzo: M
   - Dependencias: none
   - Entregables:
     - Layout manager con grid/stack.
     - Persistencia basica de paneles.

### P1 ? Herramientas de dibujo basicas
3. **Toolbar de drawing tools (v1)**
   - Esfuerzo: M
   - Dependencias: primitives
   - Entregables:
     - Trendline
     - Horizontal line
     - Vertical line
     - Ray

4. **Edicion interactiva de anchors**
   - Esfuerzo: M
   - Dependencias: primitives, toolbar v1
   - Entregables:
     - Drag handles
     - Snap a OHLC y time scale

5. **Persistencia de dibujos (save/load)**
   - Esfuerzo: M
   - Dependencias: primitives
   - Entregables:
     - Serializacion JSON de dibujos
     - Load/restore por panel

### P1 ? UX / Profesionalizacion inmediata
6. **Temas predefinidos y tipografia**
   - Esfuerzo: M
   - Dependencias: none (ideal con Pango)
   - Entregables:
     - Dark/Light/High?Contrast
     - Hook de tipografia

7. **Tooltips profesionales**
   - Esfuerzo: M
   - Dependencias: primitives (opcional)
   - Entregables:
     - Layout consistente
     - Formato flexible

### P2 ? Herramientas avanzadas
8. **Fibonacci Retracement/Extension**
   - Esfuerzo: L
   - Dependencias: toolbar v1, anchors editables

9. **Channel / Rectangle / Box**
   - Esfuerzo: M
   - Dependencias: toolbar v1

10. **Position tool (Long/Short)**
    - Esfuerzo: L
    - Dependencias: primitives, formatter de PnL

### P2 ? Funcionalidades de usuario
11. **Watchlist + Busqueda**
    - Esfuerzo: M
    - Dependencias: none

12. **Alertas basicas**
    - Esfuerzo: M
    - Dependencias: none

13. **Export PNG/SVG**
    - Esfuerzo: S
    - Dependencias: none

### P3 ? Extras y polish
14. **Multi?cursor / sync crosshair**
    - Esfuerzo: M
    - Dependencias: layout multi?panel

15. **Animaciones suaves**
    - Esfuerzo: S
    - Dependencias: none

16. **Preset de indicadores**
    - Esfuerzo: S
    - Dependencias: sistema de indicadores configurables

## Dependencias clave
- **Primitives/overlays** es el habilitador principal de drawing tools.
- **Persistencia** depende de un modelo de datos estable por dibujo.
- **Tipografia avanzada** se beneficia de Pango/PangoCairo.

## Sugerencia de entrega por fases
1. P0 completo (primitives + layout).
2. P1 drawing tools basicas + persistencia.
3. P1 UX profesional (temas + tooltips).
4. P2 avanzadas (Fibo/position tools).

## Notas
- Ajustar prioridades segun feedback de usuarios.
- Una vez exista primitives, se puede iterar rapido en nuevas herramientas.
