# Separacion de Areas Interconectadas (Modularizacion)

Fecha: 2026-02-07

## Objetivo
Definir una arquitectura de areas visuales independientes pero interconectadas para un chart desktop. Cada seccion mantiene limites claros y comparte datos para render coherente.

## Secciones base
1. **Chart (Candlestick/Main Series)**
2. **Barra de herramientas de dibujo**
3. **TimeScale**
4. **PriceScale**
5. **Pane de indicadores** (pueden existir varios; se apilan verticalmente)
6. **Zona de botones** (A: AutoScale, L: Logaritmica) que afectan solo al PriceScale

## Layout base (1 indicador)

-----------------------------------------------------------------
|  |                                                       |    |
|  |                                                       |    |
|  |                                                       |    |
|  |                         1                             |    |
|  |                                                       |    |
|2 |                                                       |    |
|  |                                                       | 4  |
|  |                                                       |    |
|  |-------------------------------------------------------|    |
|  |                         5                             |    |
|  |                                                       |    |
|  |------------------------------------------------------------|
|  |                         3                             | 6  |
-----------------------------------------------------------------

## Layout con multiples indicadores (apilados)

-----------------------------------------------------------------
|  |                                                       |    |
|  |                                                       |    |
|  |                         1                             |    |
|  |                                                       |    |
|2 |                                                       |    |
|  |-------------------------------------------------------|    |
|  |                         5                             | 4  |
|  |                                                       |    |
|  |-------------------------------------------------------|    |
|  |                         5                             |    |
|  |                                                       |    |
|  |------------------------------------------------------------|
|  |                         3                             | 6  |
-----------------------------------------------------------------

## Layout multi?chart (2 charts lado a lado)

-------------------------------------------------------------------
|  |                        |    |  |                        |    |
|  |                        |    |  |                        |    |
|  |            1           |    |  |            1           |    |
|  |                        |    |  |                        |    |
|2 |                        |    |2 |                        |    |
|  |------------------------|    |  |                        |    |
|  |            5           | 4  |  |                        | 4  |
|  |                        |    |  |                        |    |
|  |------------------------|    |  |------------------------|    |
|  |            5           |    |  |            5           |    |
|  |                        |    |  |                        |    |
|  |-----------------------------|  |-----------------------------|
|  |            3           | 6  |  |            3           | 6  |
-------------------------------------------------------------------

## Principios de modularizacion
- Cada seccion define su propio rectangulo (bounds) y solo dibuja dentro de el.
- Todas las secciones comparten un **modelo de datos** (series, escalas, opciones).
- Eventos de interaccion se enrutan por seccion (hit?testing por bounds).
- Secciones se pueden reordenar o desactivar sin romper el resto.

## Interconexion de datos (alto nivel)
- **Chart (1)** consume series principales y estado de crosshair.
- **Indicator Panes (5)** consumen series derivadas (RSI, MACD, etc.) y comparten TimeScale.
- **TimeScale (3)** se alimenta de la escala temporal global y de `visible_range`.
- **PriceScale (4)** depende del rango de precio visible del panel principal.
- **Toolbar (2)** modifica estado de drawings/primitives y escucha seleccion.
- **Botones (6)** mutan solo el PriceScale del panel al que pertenecen.

## Consideraciones
- Los panes de indicadores no se superponen; se apilan con separadores claros.
- El TimeScale siempre es unico por chart; los indicadores referencian su escala.
- En multi?chart, cada chart mantiene sus secciones, con opcion de sincronizar TimeScale.

## Proximo paso
- Mapear estas secciones a structs y modulos (p.ej. `ui/sections/*`).
- Definir API de layout para calcular bounds por seccion.
- Definir protocolo de eventos (mouse, drag, wheel) con hit?testing.
