# AGENTS.md
Guía operativa para agentes automáticos y contribuciones de código.
Este proyecto es una plataforma de análisis financiero en tiempo real similar a TradingView.
Prioridades absolutas: simplicidad, modularidad, rendimiento y verificabilidad.

====================================================================
FILOSOFÍA GENERAL
====================================================================

- Piensa como ingeniero senior de sistemas.
- Prefiere soluciones simples, explícitas y medibles.
- El rendimiento y la mantenibilidad son más importantes que la “magia”.
- Ningún cambio es válido si no puede probarse o medirse.

====================================================================
FILOSOFÍA UNIX (OBLIGATORIA)
====================================================================

Cada módulo debe:
- hacer UNA sola cosa
- tener UNA responsabilidad
- exponer interfaces pequeñas y claras
- ser testeable de forma aislada

Reglas:
- Evitar “god modules”
- Evitar dependencias circulares
- Comunicación solo por traits/interfaces
- Dominio puro sin dependencias externas

Estructura esperada:

- core-types/      → tipos y modelos puros
- datafeed/        → ingestión de datos (IO/red)
- indicators/      → cálculos matemáticos puros
- renderer/        → GPU/dibujo
- layout/          → panes, escalas, sincronización
- tools/           → drawings/interacción
- app/ui/          → integración final

Los módulos matemáticos NO deben depender de UI o GPU.

====================================================================
REGLAS DE DISEÑO
====================================================================

- < 300 líneas por archivo (objetivo)
- funciones cortas (< 50 líneas)
- sin estado global mutable
- dependencias inyectadas (traits)
- errores tipados (no strings sueltos)

Si un módulo no puede testearse aisladamente → rediseñar.

====================================================================
RENDIMIENTO (CRÍTICO)
====================================================================

Objetivo:
- 60–144 FPS estables
- latencia mínima

Presupuesto por frame:
- CPU lógica < 5ms
- sin allocations en hot paths

Prohibido:
- Vec::push sin capacidad en loops críticos
- clones innecesarios
- locks en render loop

Preferir:
- preallocación
- buffers reutilizables
- slices
- SmallVec / arrays stack
- procesamiento por chunks
- algoritmos O(n)

====================================================================
CONCURRENCIA
====================================================================

- hilo principal solo render/UI
- tareas pesadas en workers
- paso de mensajes > locks
- evitar mutabilidad compartida

====================================================================
TESTING AUTOMÁTICO (OBLIGATORIO)
====================================================================

Cada feature nueva DEBE incluir automáticamente:

✅ unit tests
✅ integration tests si cruza módulos
✅ mocks para IO/red
✅ property tests para lógica matemática

Reglas:
- lógica financiera = siempre testeada
- cálculos determinísticos
- cubrir edge cases (NaN, overflow, gaps, datos vacíos)

Si no hay tests → la tarea está incompleta.

Herramientas sugeridas:
- cargo test
- proptest / quickcheck
- mocks con traits

====================================================================
BENCHMARKS AUTOMÁTICOS (OBLIGATORIO)
====================================================================

Toda pieza crítica debe incluir benchmarks:

- indicadores
- agregaciones
- render pipeline
- parsing de datos

Crear:
- microbenchmarks por función
- benchmarks con datasets grandes

Objetivo:
- detectar regresiones de rendimiento

Herramientas:
- criterion

Si un cambio degrada rendimiento → optimizar o justificar.

====================================================================
OBSERVABILIDAD
====================================================================

Agregar métricas cuando sea relevante:

- FPS
- latencia por frame
- uso de memoria
- tiempo de indicadores

Debe poder activarse/desactivarse por config.

====================================================================
CONFIGURACIÓN
====================================================================

Todo debe ser configurable en runtime:
- CLI
- env
- config files

Proveer defaults seguros.
Documentar impacto de cada opción.

====================================================================
FLUJO DE TRABAJO DEL AGENTE (OBLIGATORIO)
====================================================================

Antes de implementar:
1. Analizar arquitectura
2. Proponer diseño modular
3. Identificar impacto en rendimiento

Durante:
4. implementar código simple y desacoplado
5. evitar optimizaciones prematuras

Después:
6. crear tests
7. crear benchmarks si aplica
8. ejecutar build/tests
9. documentar decisiones

====================================================================
PROHIBICIONES
====================================================================

- mezclar UI con lógica matemática
- introducir dependencias innecesarias
- allocations en render loop
- código sin tests
- código crítico sin benchmark

====================================================================
DEFINICIÓN DE TAREA COMPLETA
====================================================================

Una tarea solo está completa si:
- compila
- tests pasan
- benchmarks existen (si aplica)
- rendimiento no empeora
- documentación actualizada
