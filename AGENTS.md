# AGENTS – Lineamientos de Ingeniería de Software

- **Piensa como ingeniero/a de software**: prioriza claridad de diseño, mantenibilidad y evidencia de decisiones (por qué y con qué trade-offs).
- **Modularidad y archivos cortos**: divide en unidades pequeñas con responsabilidades únicas; evita “god files” y dependencias circulares; favorece interfaces claras.
- **Desac acoplamiento**: usa inyección de dependencias y límites explícitos entre capas (núcleo de dominio, UI, IO); minimiza estado global.
- **Alta configurabilidad**: expone opciones a nivel de runtime (CLI/env/archivos de config) con valores por defecto seguros; documenta cada opción.
- **Gestión de memoria**: evita copias innecesarias, reutiliza buffers, prealoca cuando se conoce el tamaño; mide y elimina hot spots de asignación.
- **Grandes volúmenes de datos**: usa estructuras y algoritmos O(n) o mejores, streaming/iteradores, chunking, y evita deserializaciones completas cuando no hagan falta.
- **Rendimiento 60+ FPS**: mantén el trabajo por frame bajo; mueve tareas pesadas a hilos/colas, usa batching y evita locking fino en el camino crítico de render.
- **Observabilidad**: agrega métricas ligeras y trazas para latencia/memoria/render; activa/desactiva por configuración.
- **Pruebas**: tests unitarios por módulo y escenarios de estrés para datos masivos y fps.
- **Documenta cada cambio**: registra decisiones y configuraciones relevantes junto con su impacto esperado.
