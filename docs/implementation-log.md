# Implementation Log (feat/docs-impl)

- 2026-02-07: Actualizamos dependencias a las últimas versiones compatibles y adaptamos el código a los cambios de API (usvg/resvg, ureq, tungstenite).
- 2026-02-07: Agregamos `do_not_snap_to_hidden_series_indices` en `CrosshairOptions` con su setter público; el crosshair ignora series sin datos cuando se habilita.
- 2026-02-07: Configuramos el entorno Windows con MSYS2/GTK4/pkgconf para permitir `cargo check --all --all-features`.
