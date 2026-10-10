# Registro de ejecución del fork Amaraza PDF (Rust)

Reglas: `.agents/rules/00-amaraza.md`. Plan: `docs/amaraza/PLAN.md`.

Cada entrada anota la tarea, los comandos, el resultado y el SHA completo (40 caracteres), copiado de `git rev-parse` y comprobado con `git cat-file -t`. Un commit no puede contener su propio SHA: el de cada commit se anota en la entrada siguiente.

## Archivos del upstream modificados

Mapa para resolver conflictos en cada sincronización con el upstream (`ARS.n`). Una fila por cada archivo del upstream que el fork modifica, borra o renombra. Los archivos nuevos del fork (`docs/amaraza/`, `.agents/rules/00-amaraza.md`, `.claude/rules/amaraza.md`, `crates/amaraza-*`) no van aquí.

| Archivo | Cambio | Motivo | Tarea |
|---|---|---|---|
| `Cargo.lock` | Entrada local `amaraza-identity` (y su uso por los crates que la llaman); sin dependencias de terceros | Crate de identidad del fork | AR0.1 |
| `xtask/src/layers.rs` | `amaraza-identity` registrado como `Standalone(0)` en `TABLE` | La comprobación de capas exige registrar cada crate | AR0.1 |

## AR0.0. Entorno y línea base (2026-10-09)

### Punto de partida

- Rama `main` en `90bb2f14ad4c60f84d900c1389e289e51a5ce7a6` (`git cat-file -t` → `commit`), igual que `origin/main`. Es el commit del upstream "Refs #142: decode outline titles with BOMs and raw UTF-8 consistently (#614)".
- `git status`: limpio, salvo los tres directorios nuevos del fork sin rastrear (`.agents/`, `.claude/`, `docs/amaraza/`).
- Remotos (`git remote -v`):
  - `origin` = `https://github.com/pony-svg/AmarazaPDF-rs.git` (fetch y push);
  - `upstream` = `https://github.com/storytold/pdfcraft.git` (fetch), con el push en `no-push`.

### Toolchain

| Componente | Versión |
|---|---|
| `rustc --version --verbose` | `rustc 1.99.0 (b940084d7 2026-09-28)`, host `x86_64-pc-windows-msvc`, LLVM 23.1.1 |
| `cargo --version` | `cargo 1.99.0 (5f94df478 2026-08-27)` |
| rustup | un solo toolchain: `stable-x86_64-pc-windows-msvc` (no hay `rust-toolchain` en el repositorio) |
| `rust-version` del workspace | 1.90: se cumple |
| MSVC | Visual Studio 2022 Community 17.14, herramientas MSVC 14.44.35207 |
| Windows SDK | 10.0.26100.0 |
| CMake | el que incluye Visual Studio 2022 (no hizo falta NASM) |
| Sistema | Windows 11 Enterprise 10.0.26300, idioma de pantalla `es-MX` |

El primer intento falló al enlazar (`linker 'link.exe' not found`): Visual Studio estaba instalado sin las herramientas de C++. El titular instaló la carga de trabajo "Desarrollo para el escritorio con C++" y se repitió la comprobación.

### Línea base, sin cambios de código

| Comando | Resultado | Duración |
|---|---|---|
| `cargo build --workspace` | Correcto: 449 crates compilados, 0 avisos | 23 min 38 s (perfil `dev`, compilación en frío) |
| `cargo test --workspace --no-fail-fast` | **1563 pasadas, 10 fallidas, 6 ignoradas**, en 127 binarios de prueba (incluidas las doc-tests). Código de salida 101 | 21 min 11 s en total, de ellos 18 min 15 s de compilación del perfil `test` |
| `cargo xtask assets` | `assets: ok (257 repository assets, 6 bundled by dependencies, 22 fetched at build time)` | 61 s |

- Se usó `--no-fail-fast` para contar todos los fallos: sin esa opción, cargo se detiene en el primer binario que falla. No se usaron `--ignored` ni `--include-ignored`.
- Después de las pruebas, `git status --short` solo muestra los tres directorios nuevos del fork: las pruebas no dejaron archivos en el repositorio.

### Fallos preexistentes (10, en 7 objetivos)

**A. Dependen del idioma de Windows (9 fallos, 6 objetivos de `pdfcraft-ui-egui`).**

Las pruebas de integración buscan etiquetas en inglés ("Menu", "OK", "Don't save", "Page 2"…), pero la interfaz sigue el idioma de pantalla de Windows, que aquí es `es-MX`. `cfg!(test)` solo fuerza el inglés en las pruebas unitarias del propio crate (`crates/ui-egui/src/i18n/mod.rs`), no en las de `tests/`.

| Objetivo | Pruebas |
|---|---|
| `--test commands` | `clear_recent_files_at_the_foot_of_open_recent_empties_the_list` |
| `--test date_format` | `a_date_the_pdf_cannot_hold_is_refused_with_a_warning_first`, `fill_and_sign_dates_follow_the_date_format_preference_and_it_is_remembered` |
| `--test editing` | `quitting_with_unsaved_changes_still_remembers_every_file` |
| `--test stamps` | `natural_image_stamps_keep_displayed_bounds_on_rotated_pages` |
| `--test ui` | `home_removes_one_recent_file_or_clears_them_all`, `narrow_tab_strips_accept_vertical_wheel_scrolling` |
| `--test viewer` | `a_short_page_gone_to_from_the_pages_panel_stays_current`, `pages_panel_thumbnails_have_a_context_menu` |

Comprobación: con `LANG=en_US.UTF-8`, el comando `cargo test -p pdfcraft-ui-egui --test commands --test date_format --test editing --test stamps --test ui --test viewer --no-fail-fast` pasa los 6 objetivos: 142 pruebas, 0 fallos (12 + 3 + 74 + 3 + 29 + 21).

En este equipo, la puerta `cargo test --workspace` se compara contra esta línea base. También se puede ejecutar con `LANG=en_US.UTF-8`, indicándolo en el registro.

**B. Confinamiento de `--root` en Windows (1 fallo real, no depende del entorno).**

- Prueba: `pdfcraft-automation --test hostile_args`, `no_tool_escapes_the_root`.
- Salida: `root confinement: 510 escape attempts over 140 tools, 495 refused, 15 leaks`.
- Qué pasa: la prueba crea un enlace simbólico de directorio, `link-out`, que apunta fuera de la raíz (aquí Windows lo permite). 15 herramientas aceptan la ruta `link-out/outside.pdf`: `doc_save`, `page_insert_file`, `doc_create_multiple`, `doc_print`, `measure_export`, `sign_id_create`, `comments_summarize`, `doc_watermark`, `doc_background`, `accessibility_report`, `form_merge_data`, `doc_compare_report`, `doc_export_text`, `doc_reduce` y `doc_optimize`.
- Es un defecto del upstream con efecto de seguridad: se puede escribir fuera de `--root` a través de un enlace. Queda pendiente que el titular decida si se corrige en el fork (como tarea propia) o se comunica al upstream (§6 de las reglas).

### Pruebas ignoradas (6, por diseño del upstream; no se ejecutaron)

| Prueba | Motivo indicado |
|---|---|
| `updates::tests::github_answers_with_the_latest_release` | needs network access |
| `corpus_parity` | needs corpus/ |
| `corpus_passwords` | needs corpus/ |
| `tests::probe_edit_latency_on_a_large_signed_document` | timing probe |
| `pdf::discovery_tests::real_file_discovery_cache_diagnostic` | needs PDFCRAFT_SIGNATURE_PROBE_SOURCE |
| `signing_with_windows_store_identities` | creates temporary certificates in the Windows Current User Personal store |

### Commit

`AR0.0: reglas, plan y línea base del fork Amaraza`, con `.agents/rules/00-amaraza.md`, `.claude/rules/amaraza.md`, `docs/amaraza/PLAN.md` y este registro. Su SHA se anota en la entrada de AR0.1.

## AR0.1. Retirar la marca ArtCraft y poner la identidad Amaraza (2026-10-09)

- Rama `ar0.1-identidad`, creada desde `main` en `2a6fa369a938d43f6f18ac8ff46d04791f5577bc` (`git cat-file -t` → `commit`). Ese es el commit de AR0.0.

### Decisiones del titular (2026-10-09)

| Tema | Decisión |
|---|---|
| Id de la app (`APP_ID`) | `io.github.pony_svg.AmarazaPDF` |
| Ejecutable y carpetas de datos | `AmarazaPDF`. El ejecutable se renombra **solo al empaquetar** (instalador y zip portátil); `cargo` sigue produciendo `pdfcraft.exe` |
| Fabricante | `Amaraza` |
| Logotipo | `app_icon.svg`, `app_icon.png` (512 px) y `app_icon.ico` (8 tamaños) del titular, que confirmó que son obra suya |
| Licencia de la marca | Borrador en español aprobado tal cual (`docs/brand/LICENSE-brand.txt`, `LicenseRef-Amaraza-Trademark`) |
| Empaquetado de Linux, Flatpak, macOS y FreeBSD | Se adapta al nuevo id. No se prueba en esas plataformas durante la prueba de concepto |
| Dónde va la identidad | Un crate propio, `crates/amaraza-identity`, sin dependencias. Alcance: app, interfaz e instalador |
| Fuera de este alcance (pendiente) | El `Producer` de los PDF creados, el nombre de la app en las firmas y el autor por defecto de los comentarios siguen diciendo "PdfCraft" |

### Inventario inicial (búsqueda de la comprobación final, antes de cambiar nada)

Comando, sobre `2a6fa369a938d43f6f18ac8ff46d04791f5577bc`:

```
grep -rniE "artcraft|getartcraft|storyteller" --exclude-dir=vendor --exclude-dir=target --exclude-dir=.git .
```

Resultado: **302 líneas en 53 archivos** (`vendor/` no tiene ninguna). La columna cuenta líneas que coinciden (`grep -c`).

| Líneas | Archivo |
|---:|---|
| 6 | `.agents/rules/00-amaraza.md` |
| 6 | `.github/workflows/packaging-lint.yml` |
| 1 | `.github/workflows/release.yml` |
| 1 | `AGENTS.md` |
| 23 | `ATTRIBUTION.md` |
| 81 | `ATTRIBUTION.toml` |
| 1 | `LICENSE-MIT` |
| 2 | `NOTICE` |
| 33 | `README.md` |
| 1 | `ROADMAP.md` |
| 1 | `apps/pdfcraft-cli/src/main.rs` |
| 4 | `apps/pdfcraft/src/main.rs` |
| 1 | `assets/app-icon/LICENSE.txt` |
| 4 | `assets/app-icon/README.md` |
| 2 | `crates/engine/src/commands.rs` |
| 8 | `crates/engine/src/links.rs` |
| 2 | `crates/ui-egui/src/dialogs.rs` |
| 2 | `crates/ui-egui/src/home.rs` |
| 2–3 cada uno | `crates/ui-egui/src/i18n/{bg,cs,de,es,fr,hu,ja,ru,te,uk,zh-hans,zh-hant}.tsv` (12 archivos, 35 en total) |
| 9 | `crates/ui-egui/src/widgets.rs` |
| 6 | `crates/ui-egui/tests/links.rs` |
| 8 | `docs/amaraza/PLAN.md` |
| 14 | `docs/brand/LICENSE-brand.txt` |
| 2 cada uno | `docs/brand/artcraft-{logo,logo-white,mark,mark-black}.svg` |
| 4 | `docs/releasing.md` |
| 1 | `packaging/freebsd/package.sh` |
| 1 | `packaging/icons.sh` |
| 1 | `packaging/linux/ai.storyteller.pdfcraft.desktop` |
| 6 | `packaging/linux/ai.storyteller.pdfcraft.metainfo.xml.in` |
| 2 | `packaging/linux/flatpak-bundle.sh` |
| 5 | `packaging/linux/flatpak/ai.storyteller.pdfcraft.bundle.yml` |
| 7 | `packaging/linux/flatpak/ai.storyteller.pdfcraft.yml` |
| 1 | `packaging/linux/nfpm.yaml` |
| 2 | `packaging/linux/package.sh` |
| 1 | `packaging/macos/Info.plist.in` |
| 3 | `packaging/windows/pdfcraft.wxs` |
| 5 | `xtask/src/assets.rs` |
| 2 | `xtask/src/gates.rs` |
| 2 | `xtask/src/screenshots.rs` |

Archivos rastreados cuyo **nombre** contiene la marca (`git ls-files | grep -iE "artcraft|storyteller"`), 22 en total:
- `docs/brand/artcraft-{logo,logo-white,mark,mark-black}.{svg,png}` (8);
- `assets/app-icon/hicolor/{16,24,32,48,64,128,256,512}x…/apps/ai.storyteller.pdfcraft.png` y `hicolor/scalable/apps/ai.storyteller.pdfcraft.svg` (9);
- `packaging/linux/ai.storyteller.pdfcraft.{desktop,metainfo.xml.in,mime.xml}` y `packaging/linux/flatpak/ai.storyteller.pdfcraft{,.bundle}.yml` (5).

Además, la identidad de PdfCraft aparece sin la palabra "ArtCraft": el título de la ventana y el nombre de la app ("PdfCraft"), `eframe::storage_dir("PdfCraft")`, `%LOCALAPPDATA%\PdfCraft\Recovery`, los marcadores portátiles `portable.txt` y `PdfCraft.portable`, la carpeta `PdfCraftData`, `migrate_legacy_folders` (que mueve las carpetas de PrintCraft), el icono `assets/app-icon/pdfcraft.*`, los recursos del `.exe` (`apps/pdfcraft/build.rs`), la comprobación de actualizaciones contra `storytold/pdfcraft` (`apps/pdfcraft/src/updates.rs`) y el instalador (`Name`, `Manufacturer`, `UpgradeCode`, GUID de componentes, ProgId `PdfCraft.Document`, `RegisteredApplications`, accesos directos).

### Commits de AR0.1

Cada commit pasa las cuatro puertas (`cargo fmt --check`, `cargo clippy --workspace -- -D warnings`, `cargo test --workspace --no-fail-fast` y `cargo xtask assets`). Los fallos de prueba se comparan con la línea base de AR0.0.

| Commit | Contenido | Puertas |
|---|---|---|
| `36312494802acec1e78514121ed0fc2d31063ebb` | Decisiones del titular e inventario inicial (solo documentación) | fmt 0, clippy 0, assets ok; las pruebas no cambian respecto a la línea base |
| *(siguiente)* | Crate `crates/amaraza-identity` (constantes y 3 pruebas), registrado en `xtask/src/layers.rs` | fmt 0, clippy 0, assets ok; pruebas: 1564 pasadas, 12 fallidas, 6 ignoradas. Son los 10 fallos de la línea base más los 2 de rendimiento (ver la nota) |

**Nota sobre las pruebas de rendimiento** (`pdfcraft-ui-egui --test perf`: `scrolling_a_500_page_document_stays_within_the_frame_budget` y `panels_with_hundreds_of_items_stay_within_the_frame_budget`):
- Miden el tiempo medio por fotograma contra un presupuesto fijo de 40 ms. En Windows no pueden leer la carga del sistema, así que nunca se saltan.
- En la línea base pasaron (7,4 s). Desde las 20:00 fallan siempre, también aisladas (dos repeticiones: unos 60 ms por fotograma). En ese momento el portátil funcionaba con batería (26 %, descargándose, plan "Equilibrado").
- El crate de identidad no lo usa todavía ningún otro crate, así que no puede afectarlas. Se tratan como dependientes del entorno y se repiten con el equipo conectado a la corriente antes de cerrar AR0.1.
