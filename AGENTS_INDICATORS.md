# AGENTS_INDICATORS.md
Guía operativa para agentes que implementen o modifiquen indicadores técnicos.

Este módulo contiene:
- indicadores técnicos (RSI, MACD, VWAP, EMA, etc.)
- transformaciones de series temporales
- agregaciones matemáticas
- cálculos financieros puros

Es el núcleo matemático del sistema.

Prioridades absolutas:
1) exactitud matemática
2) determinismo
3) rendimiento O(n)
4) cero asignaciones innecesarias
5) 100% testeable

====================================================================
FILOSOFÍA GENERAL
====================================================================

Los indicadores son funciones puras.

Deben:
- ser determinísticos
- no tener side effects
- no depender de IO/UI/red
- producir siempre el mismo resultado para la misma entrada

Regla:
Indicadores = matemática pura, no infraestructura.

Si depende de red/UI → está mal ubicado.

====================================================================
FILOSOFÍA UNIX
====================================================================

Cada indicador:
- hace una sola cosa
- archivo pequeño
- sin dependencias externas innecesarias
- testeable de forma aislada

Ejemplos:
- ema.rs
- rsi.rs
- macd.rs
- vwap.rs
- bollinger.rs

NO:
- indicators.rs gigante con todo mezclado

====================================================================
API ESTÁNDAR (OBLIGATORIA)
====================================================================

Todos los indicadores deben seguir interfaces simples:

Preferir:

- slices de entrada
- buffers de salida provistos por el caller
- sin asignaciones internas

Ejemplo:

fn rsi(input: &[f64], period: usize, out: &mut [f64])

NO:

fn rsi(data: Vec<f64>) -> Vec<f64>

Reglas:
- sin Vec internos
- sin clones
- sin heap allocations
- caller gestiona memoria

====================================================================
RENDIMIENTO (CRÍTICO)
====================================================================

Objetivos:
- O(n)
- single pass cuando sea posible
- sin allocaciones
- branch mínimo
- cache friendly

Prohibido:
- HashMap
- Box
- Strings
- iteradores que asignen
- colecciones intermedias grandes

Preferir:
- bucles for simples
- buffers prealocados
- rolling windows
- acumuladores
- SmallVec si es imprescindible

Si algo puede hacerse en 1 pasada → no hacer 2.

====================================================================
DETERMINISMO Y EXACTITUD
====================================================================

El resultado debe ser:

- determinístico
- reproducible
- independiente del hardware

Reglas:
- evitar paralelismo no determinístico
- evitar floats inconsistentes sin documentar
- documentar fórmulas usadas
- usar tipos consistentes (f64 o Decimal)

No mezclar precisión arbitrariamente.

====================================================================
NUMERICAL STABILITY
====================================================================

Evitar:
- overflow
- divisiones por cero
- acumulación de error flotante

Aplicar:
- checks de NaN
- clamps
- algoritmos estables (Welford para varianza, etc.)
- inicialización clara

Los indicadores nunca deben propagar NaN silenciosamente.

====================================================================
STREAMING (IMPORTANTE)
====================================================================

Soportar modo incremental:

- cálculo rolling
- actualización con último tick
- no recalcular toda la serie

Ejemplo:

update(last_value) → next_value

Esto es obligatorio para tiempo real.

====================================================================
TESTING AUTOMÁTICO (OBLIGATORIO)
====================================================================

Cada indicador debe incluir:

✅ unit tests básicos
✅ casos edge
✅ datasets conocidos
✅ comparación contra valores esperados

Casos mínimos:
- input vacío
- period > len
- NaN
- valores constantes
- datos extremos
- gaps

Nunca aceptar indicador sin tests.

====================================================================
PROPERTY TESTING (OBLIGATORIO)
====================================================================

Usar proptest / quickcheck.

Ejemplos de propiedades:

RSI:
- 0 ≤ rsi ≤ 100

EMA:
- output suavizado
- sin saltos mayores que input extremo

Bollinger:
- upper ≥ middle ≥ lower

VWAP:
- dentro del rango high/low ponderado

Propiedades matemáticas > tests manuales.

====================================================================
BENCHMARKS AUTOMÁTICOS (OBLIGATORIO)
====================================================================

Crear benchmarks con criterion:

Medir:
- tiempo con 10k, 100k, 1M velas
- modo streaming
- memoria

Objetivo:
- microsegundos por 10k puntos
- detectar regresiones

Si un cambio empeora >10% → optimizar o justificar.

====================================================================
PARALELISMO
====================================================================

Permitido:
- paralelizar grandes datasets offline

Prohibido:
- paralelismo que rompa determinismo
- locks

Tiempo real → single thread predecible.

Backtesting → paralelizable.

====================================================================
DEPENDENCIAS
====================================================================

Los indicadores:

NO deben depender de:
- UI
- renderer
- datafeed
- red
- filesystem

Solo:
- core-types
- std
- math crates ligeras

====================================================================
DOCUMENTACIÓN (OBLIGATORIA)
====================================================================

Cada indicador debe documentar:

- fórmula matemática
- referencia (libro/paper/TradingView/etc.)
- complejidad
- limitaciones

Ejemplo:

/// RSI (Wilder)
/// Complejidad: O(n)
/// Fuente: Wilder 1978
/// Rango: [0, 100]

====================================================================
PROHIBICIONES
====================================================================

- Vec nuevos por llamada
- asignaciones en loops
- IO
- estado global
- dependencias a UI/render
- indicadores sin tests
- indicadores sin benchmark
- lógica mezclada con caching

====================================================================
DEFINICIÓN DE TAREA COMPLETA
====================================================================

Un indicador está completo solo si:

- compila
- tests pasan
- property tests creados
- benchmarks creados
- O(n)
- zero allocations
- determinístico
- documentado

