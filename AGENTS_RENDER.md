# AGENTS_RENDER.md
Guía operativa para agentes que trabajen en el motor de render del chart.

Stack:
- cairo-rs (render 2D vectorial CPU)
- pango (texto y layout)
- render en tiempo real (60–144 FPS)

El render es un hot path crítico.
Cada milisegundo importa.

Prioridades:
1) FPS estables
2) baja latencia
3) cero stutter
4) mínima asignación de memoria
5) dibujo incremental (no full redraw)

====================================================================
FILOSOFÍA GENERAL
====================================================================

- Render determinístico y predecible
- El trabajo por frame debe ser mínimo
- Nunca recalcular lo que puede cachearse
- El render thread NO debe bloquearse

Regla mental:
"Si algo puede precomputarse, no debe hacerse por frame"

====================================================================
ARQUITECTURA (OBLIGATORIA)
====================================================================

Separar responsabilidades:

- scene/       → estado visual puro (datos a dibujar)
- layout/      → cálculo de geometría (ejes, panes, escalas)
- primitives/  → velas, líneas, grid, texto, tools
- text/        → wrappers de pango
- cache/       → surfaces, paths, layouts reutilizables
- renderer/    → orquestador final

Flujo:

Data → Layout → Scene → Render

Reglas:
- layout ≠ draw
- draw ≠ lógica de negocio
- renderer solo pinta, no calcula datos

====================================================================
REGLAS DE RENDIMIENTO (CRÍTICAS)
====================================================================

Objetivos:
- 60–144 FPS estables
- < 5ms CPU por frame
- cero allocations en el hot path

Prohibido en render loop:
- crear Strings
- crear Vec nuevos
- HashMap
- parsing
- IO
- locks largos
- cálculos pesados

Preferir:
- buffers prealocados
- SmallVec/arrays stack
- reutilizar structs
- arenas/memory pools

====================================================================
DIBUJO INCREMENTAL (OBLIGATORIO)
====================================================================

NO redibujar todo el chart cada frame.

Implementar:
- dirty rectangles
- invalidación por regiones
- redibujar solo panes afectados
- cache de background (grid/ejes)

Reglas:
- grid estático → cacheado
- texto estático → cacheado
- indicadores históricos → cacheables
- solo el último tramo cambia por tick

Full redraw solo si:
- resize
- zoom grande
- cambio de tema

====================================================================
CACHING (OBLIGATORIO)
====================================================================

Cachear agresivamente:

- cairo::ImageSurface
- cairo::Path
- pango::Layout
- geometrías precomputadas
- texto repetido (precios, labels)

Reglas:
- nunca crear Layout por frame
- nunca recalcular paths repetidos
- reutilizar Context/Surface cuando sea posible

Ejemplos:
✔ cachear velas históricas en una surface
✔ cachear grid
✔ cachear texto del eje Y

====================================================================
TEXTO (PANGO) — MUY COSTOSO
====================================================================

Pango es una de las operaciones más lentas.

Reglas estrictas:

- layouts creados UNA vez
- reutilizar pango::Layout
- no formatear strings por frame
- usar buffers reutilizables
- agrupar dibujo de texto

Evitar:
- set_text() por tick
- medir texto constantemente

Preferir:
- cache por precio/label
- atlas de texto si es posible

====================================================================
GEOMETRÍA Y LAYOUT
====================================================================

Separar cálculo geométrico del render.

Layout:
- calcular posiciones
- escalas
- transformaciones
- bounds

Render:
- solo dibuja primitivas ya calculadas

Nunca mezclar ambas cosas.

El layout solo se recalcula cuando:
- zoom
- scroll
- resize
- cambio de datos históricos

NO por frame.

====================================================================
PRIMITIVAS
====================================================================

Cada primitiva debe:

- ser stateless
- recibir contexto + datos precomputados
- no asignar memoria
- no mutar estado global

Ejemplos:
- CandleRenderer
- LineRenderer
- VolumeRenderer
- GridRenderer
- TextRenderer

====================================================================
CONCURRENCIA
====================================================================

Permitido:
- layout en worker threads
- precomputar geometría async

Prohibido:
- dibujar fuera del render thread
- compartir Context entre hilos
- locks en render loop

Comunicación por:
- canales / mensajes
- snapshots inmutables

====================================================================
TESTING AUTOMÁTICO (OBLIGATORIO)
====================================================================

Crear:

✅ unit tests de layout/geom
✅ tests de escalas (price → pixel)
✅ tests de panes sincronizados
✅ tests de clipping
✅ tests de zoom/scroll edge cases

La lógica matemática debe ser 100% testeable sin Cairo.

Nunca depender de rendering real para tests.

====================================================================
BENCHMARKS AUTOMÁTICOS (OBLIGATORIO)
====================================================================

Crear benchmarks para:

- layout de 10k–100k velas
- render de primitivas
- texto Pango
- composición de surfaces
- tiempo total de frame

Objetivo:
- detectar regresiones de FPS

Usar:
- criterion

Regla:
Si un cambio empeora >10% → optimizar o justificar.

====================================================================
OBSERVABILIDAD
====================================================================

Agregar métricas:

- frame time
- FPS
- tiempo de layout
- tiempo de texto
- tiempo de dibujo
- asignaciones por frame

Configurable ON/OFF.

====================================================================
PROHIBICIONES
====================================================================

- lógica de negocio dentro del renderer
- asignaciones en hot path
- crear Layouts por frame
- redibujar todo sin necesidad
- locks largos
- dependencias a datafeed directo
- código sin benchmarks
- código sin tests

====================================================================
DEFINICIÓN DE TAREA COMPLETA
====================================================================

Una tarea de render solo está completa si:

- compila
- tests pasan
- benchmarks creados
- FPS no empeora
- no hay allocations nuevas en hot path
- caching implementado
- documentación actualizada
