# AGENTS_TOOLS.md
Guía operativa para agentes que implementen o modifiquen herramientas de dibujo (chart tools).

Este módulo incluye:
- líneas
- rayos
- rectángulos
- fibos
- canales
- texto
- marcadores
- mediciones
- selección / drag / resize

Las tools son un sistema de edición vectorial interactivo.

Prioridades:
1) interacciones fluidas (sin lag)
2) geometría determinística
3) separación estado/render
4) serializable
5) testeable sin UI

====================================================================
FILOSOFÍA GENERAL
====================================================================

Las tools NO son código de render.
Son modelos de datos + geometría.

Regla mental:
"Las tools describen qué dibujar, no cómo dibujarlo"

El renderer decide cómo pintar (Cairo).

====================================================================
FILOSOFÍA UNIX
====================================================================

Cada herramienta:
- hace una sola cosa
- archivo propio
- sin dependencias innecesarias
- testeable de forma aislada

Ejemplos:

- line_tool.rs
- rect_tool.rs
- fib_tool.rs
- text_tool.rs

NO:
- tools.rs gigante con todo mezclado

====================================================================
ARQUITECTURA OBLIGATORIA
====================================================================

Separar claramente:

- model/        → estado puro serializable
- geometry/     → cálculos matemáticos
- interaction/  → eventos mouse/teclado
- render/       → dibujo Cairo
- hit_test/     → detección de selección

Flujo:

Eventos → Update Model → Geometry → Render

Reglas:
- geometry no usa Cairo
- render no calcula lógica
- interaction no dibuja

====================================================================
MODELO DE DATOS (CRÍTICO)
====================================================================

Las tools deben:

- ser structs simples
- serializables (serde)
- determinísticas
- sin referencias a UI

Ejemplo:

struct LineTool {
    start: Point,
    end: Point,
    style: Style,
}

NO:
- Context
- Surface
- punteros a renderer
- closures
- estado oculto

El modelo debe poder guardarse/cargarse desde JSON/binario.

====================================================================
INTERACCIÓN
====================================================================

Eventos soportados:

- mouse_down
- mouse_move
- mouse_up
- drag
- hover
- key modifiers

Reglas:
- sin asignaciones por evento
- sin cálculos pesados por movimiento
- solo actualizar estado mínimo

Mouse move debe ser O(1).

Nunca recalcular geometría compleja en cada píxel.
