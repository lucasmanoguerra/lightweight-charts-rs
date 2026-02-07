# AGENTS_DATAFEED.md
Guía operativa para agentes que modifiquen o creen código del módulo de Datafeed.

Este módulo es responsable de:
- ingestión de datos en tiempo real (WebSocket/streaming)
- descarga histórica (REST/archivos)
- normalización de ticks/candles
- caching local
- distribución de datos al sistema

El datafeed es infraestructura crítica.
Debe ser confiable, determinístico y no bloquear el render.

====================================================================
PRINCIPIOS GENERALES
====================================================================

- IO aislado del dominio
- determinismo > conveniencia
- cero bloqueos en el hilo principal
- alta resiliencia a fallos de red
- reproducible para backtests

El datafeed NO debe depender de UI ni renderer.

====================================================================
FILOSOFÍA UNIX (OBLIGATORIA)
====================================================================

Separar en módulos pequeños:

- transport/   → websocket/http/files
- parser/      → decoding (json/binary)
- normalize/   → ticks → candles/series
- cache/       → almacenamiento local
- bus/         → distribución de eventos
- replay/      → backtesting / histórico offline

Cada módulo:
- una responsabilidad
- testeable aisladamente
- sin side effects ocultos

====================================================================
ARQUITECTURA ESPERADA
====================================================================

Flujo:

Transport → Parser → Normalizer → Cache → Event Bus → Consumers

Reglas:
- cada etapa transforma datos
- sin lógica mezclada
- comunicación por canales/mensajes
- sin estado global mutable

====================================================================
CONCURRENCIA (CRÍTICO)
====================================================================

Prohibido:
- bloquear el hilo principal
- locks largos
- operaciones de red síncronas

Obligatorio:
- async/await (tokio)
- workers dedicados
- paso de mensajes (mpsc/broadcast)
- backpressure controlado

El render thread nunca debe esperar datos.

====================================================================
MODELO DE DATOS
====================================================================

Los datos deben ser:

- inmutables
- copy-light
- representados con slices o referencias cuando sea posible
- sin strings dinámicos innecesarios

Preferir:
- structs compactos
- timestamps enteros
- enums pequeños
- Vec prealocado

Evitar:
- HashMap en hot paths
- parsing repetido
- clonaciones grandes

====================================================================
NORMALIZACIÓN
====================================================================

El dominio trabaja solo con tipos internos:

Ejemplo:
- Tick
- Trade
- Candle
- OrderBookSnapshot

Los formatos externos (JSON/exchange-specific) deben convertirse inmediatamente.

Regla:
❌ nunca propagar JSON crudo al resto del sistema

====================================================================
CACHING
====================================================================

Objetivos:
- reducir requests
- acelerar carga histórica
- soportar modo offline
- reproducibilidad para backtesting

Requisitos:
- cache local persistente (archivos/binario/columnar)
- lectura por chunks
- acceso O(1) por rango temporal
- no cargar todo en memoria

Preferir:
- formatos binarios
- compresión ligera
- mmap cuando sea posible

====================================================================
RESILIENCIA DE RED
====================================================================

El sistema debe tolerar:

- desconexiones
- reconexiones
- paquetes fuera de orden
- duplicados
- gaps

Obligatorio:
- reconexión automática con backoff exponencial
- re-suscripción automática
- deduplicación de eventos
- detección de huecos históricos

Nunca asumir conexión perfecta.

====================================================================
TESTING AUTOMÁTICO (OBLIGATORIO)
====================================================================

Cada feature nueva debe incluir:

✅ unit tests por módulo
✅ integration tests del pipeline completo
✅ mocks de transporte (fake websocket/http)
✅ tests de reconexión
✅ tests de datos corruptos
✅ tests de orden incorrecto
✅ tests de alto volumen

Ejemplos mínimos:
- 1M ticks procesados correctamente
- reconexión mantiene consistencia
- candles calculadas sin gaps
- parser tolera datos inválidos

Usar:
- traits para mockear IO
- datasets determinísticos

====================================================================
PROPERTY TESTING (OBLIGATORIO PARA CÁLCULOS)
====================================================================

Para normalización/aggregación:

- candles nunca negativas
- high >= max(open, close)
- low <= min(open, close)
- volumen acumulado consistente

Usar:
- proptest / quickcheck

====================================================================
BENCHMARKS AUTOMÁTICOS (OBLIGATORIO)
====================================================================

Crear benchmarks para:

- parsing
- agregación de ticks → candles
- lectura de cache
- throughput del pipeline

Objetivos:
- > 100k–1M eventos/segundo (referencial)
- detectar regresiones

Usar:
- criterion

Si un cambio empeora rendimiento → optimizar o justificar.

====================================================================
OBSERVABILIDAD
====================================================================

Agregar métricas:

- latencia de red
- eventos/segundo
- tamaño de cola
- uso de memoria cache
- tiempo de parsing

Todo configurable.

====================================================================
PROHIBICIONES
====================================================================

- mezclar IO con cálculos de dominio
- JSON en hot path
- locks largos
- allocaciones por evento
- dependencias a UI
- código sin tests
- código crítico sin benchmark

====================================================================
DEFINICIÓN DE TAREA COMPLETA
====================================================================

Una tarea en datafeed solo está completa si:

- compila
- tests pasan
- mocks creados
- benchmarks creados
- soporta reconexión
- no bloquea render
- rendimiento aceptable
- documentación actualizada
