# Registro de ejecución del fork Amaraza PDF (Rust)

Reglas: `.agents/rules/00-amaraza.md`. Plan: `docs/amaraza/PLAN.md`.

Cada entrada anota la tarea, los comandos, el resultado y el SHA completo (40 caracteres), copiado de `git rev-parse` y comprobado con `git cat-file -t`. Un commit no puede contener su propio SHA: el de cada commit se anota en la entrada siguiente.

## Archivos del upstream modificados

Mapa para resolver conflictos en cada sincronización con el upstream (`ARS.n`). Una fila por cada archivo del upstream que el fork modifica, borra o renombra. Los archivos nuevos del fork (`docs/amaraza/`, `.agents/rules/00-amaraza.md`, `.claude/rules/amaraza.md`, `crates/amaraza-*`) no van aquí.

| Archivo | Cambio | Motivo | Tarea |
|---|---|---|---|
| `Cargo.lock` | Entrada local `amaraza-identity` (y su uso por los crates que la llaman); sin dependencias de terceros | Crate de identidad del fork | AR0.1 |
| `xtask/src/layers.rs` | `amaraza-identity` registrado como `Standalone(0)` en `TABLE` | La comprobación de capas exige registrar cada crate | AR0.1 |
| `crates/engine/Cargo.toml`, `crates/ui-egui/Cargo.toml` | Dependencia `amaraza-identity = { path = … }` | Identidad en un solo crate | AR0.1 |
| `crates/engine/src/links.rs` | Sin Discord, getartcraft.com ni página de la app; `GITHUB` = repositorio del fork; nuevos `UPSTREAM` y `BASED_ON`; prueba que impide enlaces o etiquetas de ArtCraft | Licencia de marca ArtCraft: solo atribución en texto plano | AR0.1 |
| `crates/engine/src/commands.rs` | Fuera `help.discord`, `help.app_page` y `help.website`; `help.github` → "Amaraza PDF en GitHub"; nuevo `help.upstream` ("PdfCraft on GitHub", al upstream); `help.about` → "Acerca de Amaraza PDF" | Ídem | AR0.1 |
| `crates/ui-egui/src/chrome.rs` | Sin el botón de Discord de la barra de título (y su ancho reservado) | Ídem | AR0.1 |
| `crates/ui-egui/src/home.rs` | Sin la tarjeta de la comunidad ArtCraft; título "Te damos la bienvenida a Amaraza PDF" | Ídem e identidad | AR0.1 |
| `crates/ui-egui/src/widgets.rs` | Fuera `artcraft_logo` y `artcraft_mark` (dejan de incrustarse las marcas); `community_links` → `project_links`; nuevo `amaraza_logo` | Ídem | AR0.1 |
| `crates/ui-egui/src/dialogs.rs` | "Acerca de": logotipo y nombre de Amaraza PDF, atribución `BASED_ON` y enlaces del proyecto, sin "Part of" + logotipo ArtCraft | Ídem | AR0.1 |
| `crates/ui-egui/src/i18n/{bg,cs,de,es,fr,hu,ja,ru,te,uk,zh-hans,zh-hant}.tsv` | Fuera las entradas de Discord y ArtCraft; "About PdfCraft" y "Welcome to PdfCraft" pasan a las nuevas fuentes en español; nueva "Amaraza PDF en GitHub". Las traducciones existentes se conservan, solo cambia el nombre del producto | Las pruebas de cobertura exigen traducir cada etiqueta de menú | AR0.1 |
| `ATTRIBUTION.toml` | `sha256` de los 12 catálogos | `cargo xtask assets` comprueba el hash | AR0.1 |
| `crates/ui-egui/tests/links.rs` | Reescrita: ni Discord ni ArtCraft en pantalla, "Acerca de" con atribución y enlaces, comandos de Ayuda | Prueba del nuevo comportamiento | AR0.1 |
| `crates/ui-egui/tests/ui.rs` | Busca el nuevo título de inicio | Ídem | AR0.1 |
| `apps/pdfcraft-cli/src/main.rs` | Sin la línea de Discord en `--help`; `--version` muestra el repositorio del fork y la atribución | Ídem | AR0.1 |
| `crates/automation/src/mcp.rs` | `websiteUrl` del servidor MCP = repositorio del fork | Ya no existe la página de getartcraft.com | AR0.1 |
| `apps/pdfcraft/Cargo.toml` | Dependencia `amaraza-identity` | Identidad en un solo crate | AR0.1 |
| `apps/pdfcraft/src/main.rs` | Icono = logotipo Amaraza en todas las plataformas; ajustes en `AmarazaPDF`; **fuera `migrate_legacy_folders`** (ya no mueve carpetas de PrintCraft/PdfCraft); nombre de la app, título de la ventana, `--version` y registro con "Amaraza PDF"; 2 pruebas (carpeta propia y recursos del `.exe`); `APP_ID` = `amaraza_identity::APP_ID` (commit del empaquetado) | Identidad; nunca tocar datos de otras apps | AR0.1 |
| `apps/pdfcraft/build.rs` | Recursos del `.exe`: icono `docs/brand/amaraza-logo.ico`, `ProductName`/`FileDescription` "Amaraza PDF", `CompanyName` "Amaraza", `OriginalFilename` `AmarazaPDF.exe`, `InternalName` `AmarazaPDF`; se conserva el copyright de PdfCraft | Identidad (lo que muestra el Explorador) | AR0.1 |
| `apps/pdfcraft/src/updates.rs` | Consulta las releases del fork (`LATEST_RELEASE_API`); `User-Agent` `AmarazaPDF/…`; la prueba rechaza las páginas de releases de PdfCraft | No ofrecer descargas de otra app | AR0.1 |
| `crates/ui-egui/src/updates.rs` | `RELEASES_PAGE` = releases del fork | Ídem | AR0.1 |
| `crates/ui-egui/src/lib.rs` | Título de la ventana "… — Amaraza PDF" | Identidad | AR0.1 |
| `crates/ui-egui/src/portable.rs` | Marcador `AmarazaPDF.portable` y carpeta `AmarazaPDFData` (desde `amaraza-identity`); `MARKERS` pasa de `[&str; 2]` a `&[&str]`; nueva prueba: los marcadores y la carpeta de PdfCraft no activan ni se tocan | Identificadores propios | AR0.1 |
| `crates/ui-egui/src/recovery.rs` | Carpeta de recuperación `AmarazaPDF` (`amarazapdf` en Linux); nueva prueba | Ídem | AR0.1 |
| `crates/ui-egui/src/sign_ui.rs` | Solo el comentario de la carpeta de identificadores digitales | Exactitud | AR0.1 |
| `crates/ui-egui/tests/accessibility.rs`, `crates/ui-egui/tests/viewer.rs` | Título esperado "… — Amaraza PDF" | Ídem | AR0.1 |
| `packaging/windows/pdfcraft.wxs` | `Name`/`Title` "Amaraza PDF", `Manufacturer` "Amaraza", **`UpgradeCode` nuevo** (`B023D7AF-E898-4990-9855-B539545099F3`) y GUID nuevos de los 3 componentes con GUID fijo; el `.exe` se instala como `AmarazaPDF.exe` en `Program Files\AmarazaPDF`; App Paths, ProgId `AmarazaPDF.Document`, `Software\AmarazaPDF\Capabilities`, `RegisteredApplications`, menú contextual `AmarazaPDF.CreatePdf` ("Crear PDF con Amaraza PDF…"), accesos directos y enlaces de Agregar o quitar programas del fork. Los Id internos de WiX (`PdfcraftExe`…) no cambian | Identificadores propios del instalador | AR0.1 |
| `packaging/windows/package.ps1` | Artefactos `AmarazaPDF-<versión>-windows-<arch>.msi` y `-portable.zip`; el zip lleva `AmarazaPDF.exe` y el marcador `AmarazaPDF.portable`; icono `docs\brand\amaraza-logo.ico` | Ídem | AR0.1 |
| `packaging/windows/portable.txt` → `packaging/windows/AmarazaPDF.portable` | **Renombrado**; texto en español con las carpetas de Amaraza | Marcador portátil propio | AR0.1 |
| `packaging/windows/test-msi.ps1` | Comprueba fabricante "Amaraza", nombre del producto, `UpgradeCode`, `AmarazaPDF.exe`, carpeta de instalación y el menú contextual nuevo | Verificar la identidad en el MSI compilado | AR0.1 |
| `packaging/windows/sign.ps1` | Descripción y URL de la firma Authenticode: Amaraza PDF y el repositorio del fork | Ídem (la firma no se usa en la prueba) | AR0.1 |
| `packaging/linux/ai.storyteller.pdfcraft.{desktop,metainfo.xml.in,mime.xml}` → `packaging/linux/io.github.pony_svg.AmarazaPDF.*` | **Renombrados**; nombre "Amaraza PDF", id, desarrollador "Amaraza", URL del fork (sin Discord), colores del logotipo | APP_ID propio | AR0.1 |
| `packaging/linux/flatpak/ai.storyteller.pdfcraft{,.bundle}.yml` → `packaging/linux/flatpak/io.github.pony_svg.AmarazaPDF{,.bundle}.yml` | **Renombrados**; id, rutas e iconos desde `docs/brand/` | Ídem | AR0.1 |
| `packaging/linux/package.sh`, `packaging/linux/flatpak-bundle.sh`, `packaging/linux/nfpm.yaml`, `packaging/freebsd/package.sh` | `APP_ID`, artefactos `AmarazaPDF-*`, paquete `amarazapdf`, fabricante y web del fork, icono hicolor (512 px y scalable) desde `docs/brand/`, documentación en `share/doc/amarazapdf`. Los binarios siguen llamándose `pdfcraft` y `pdfcraft-cli` | Ídem | AR0.1 |
| `packaging/macos/Info.plist.in`, `packaging/macos/package.sh` | Bundle `io.github.pony_svg.AmarazaPDF`, "Amaraza PDF.app" con ejecutable e icono `AmarazaPDF` (el `.icns` se genera al empaquetar desde el PNG con `sips` e `iconutil`), DMG `AmarazaPDF-*` sin fondo ni disposición personalizada | Ídem | AR0.1 |
| `assets/app-icon/` (16 archivos), `packaging/icons.sh`, `packaging/macos/dmg/` (5 archivos) | **Borrados**: el icono de PdfCraft, su generador y el fondo del DMG que lo mostraba | El icono de la app es el logotipo Amaraza | AR0.1 |
| `ATTRIBUTION.toml`, `ATTRIBUTION.md` | Fuera las 16 entradas de esos archivos (235 activos) | Ídem | AR0.1 |
| `xtask/src/gates.rs` | `macos_bundle_id_matches_app_id` comprueba el id nuevo en `amaraza-identity`, `Info.plist.in` y `main.rs` | La prueba fijaba el id del upstream | AR0.1 |
| `.github/workflows/packaging-lint.yml` | Rutas e id nuevos; filtro `docs/brand/**` en lugar de `assets/app-icon/**` | Coherencia (Actions no se activa) | AR0.1 |
| `.github/workflows/release.yml` | **Borrado** | Usaba los secretos de firma y publicación del upstream; el fork no publica binarios | AR0.1 |
| `.github/workflows/ci.yml` | Sin `schedule` (fuzzing nocturno) | No ejecutar nada programado en el fork | AR0.1 |
| `.github/workflows/windows-arm64.yml` | Sin `schedule`; la prueba de instalación busca "Amaraza PDF must be installed…", los accesos directos "Amaraza PDF.lnk" y `Program Files\AmarazaPDF\AmarazaPDF.exe` | Ídem y coherencia con el instalador | AR0.1 |
| `README.md` | **Sustituido** por el README propio de Amaraza PDF: qué es, estado de prueba de concepto, atribución "Basado en PdfCraft, del equipo ArtCraft", licencia, marca Amaraza y reglas del fork; sin logotipos, enlaces ni comunidad de ArtCraft | Plan AR0.1 §5; licencia de marca | AR0.1 |
| `docs/releasing.md` | Nota inicial: el proceso es el del upstream y el fork retiró `release.yml`; el id nuevo en lugar de `ai.storyteller.pdfcraft` | Coherencia | AR0.1 |
| `docs/development.md` | Carpetas de ajustes y registros, y marcador portátil de Amaraza PDF | Exactitud | AR0.1 |
| `crates/ui-egui/src/dialogs.rs`, `xtask/src/screenshots.rs`, `crates/engine/src/links.rs`, `xtask/src/assets.rs`, `crates/ui-egui/tests/links.rs` | Comentarios reformulados para no nombrar ArtCraft sin necesidad (las cadenas de las pruebas que prohíben la marca se quedan) | Búsqueda final más limpia | AR0.1 |
| `docs/brand/artcraft-{logo,logo-white,mark,mark-black}.{svg,png}` (8) | **Borrados** | Licencia de marca ArtCraft: un fork debe retirarlas | AR0.1 |
| `docs/brand/LICENSE-brand.txt` | Sustituido por la licencia de la marca Amaraza aprobada por el titular | Ídem | AR0.1 |
| `docs/images/pdfcraft-tools.png` | **Borrado** | Captura de la pantalla de inicio con la marca ArtCraft | AR0.1 |
| `ATTRIBUTION.toml`, `ATTRIBUTION.md` | Las 8 entradas ArtCraft pasan a 3 de `docs/brand/amaraza-logo.{svg,png,ico}`; fuera la entrada de la captura; `ATTRIBUTION.md` regenerado | Ídem | AR0.1 |
| `xtask/src/assets.rs` | `BRAND_LICENCE` = `LicenseRef-Amaraza-Trademark`; su prueba usa el logotipo Amaraza; nueva prueba que rechaza `LicenseRef-Storyteller-Trademark` | Que una sincronización no pueda volver a traer marcas ArtCraft | AR0.1 |
| `xtask/src/screenshots.rs` | Sin el texto de la marca ArtCraft en la pantalla de inicio | Ya no aparece | AR0.1 |
| `NOTICE` | Cabecera de Amaraza PDF con la atribución, aviso de archivos modificados (Apache-2.0 §4(b)) y marca Amaraza en lugar de la ArtCraft | Licencia Apache-2.0 y de marca | AR0.1 |

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

- Rama `ar0.1-identidad`, creada desde `main` en `61c281839a07f0baf12f29d9aba17b4288a3abd8` (`git cat-file -t` → `commit`). Ese es el commit de AR0.0.

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

Comando, sobre `61c281839a07f0baf12f29d9aba17b4288a3abd8`:

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
| `63b25fed91709aa7809dba8bec3b13c6c921b4f6` | Decisiones del titular e inventario inicial (solo documentación) | fmt 0, clippy 0, assets ok; las pruebas no cambian respecto a la línea base |
| `f0785d3f9b68d86511760a331c53ff30d6b76948` | Crate `crates/amaraza-identity` (constantes y 3 pruebas), registrado en `xtask/src/layers.rs` | fmt 0, clippy 0, assets ok; pruebas: 1564 pasadas, 12 fallidas, 6 ignoradas. Son los 10 fallos de la línea base más los 2 de rendimiento (ver la nota) |
| `12df0d4b7ae2da8d81b48bba3558b703cfd421ca` | Interfaz y CLI sin enlaces ni comunidad de ArtCraft: botón de Discord, tarjeta de inicio, "Part of" + logotipo y comandos de Ayuda fuera; "Acerca de" con la atribución en texto plano; catálogos | fmt 0, clippy 0 (también `--all-targets`), assets ok; pruebas: 1565 pasadas, 10 fallidas (las de la línea base), 6 ignoradas; con `LANG=en_US.UTF-8`, los 6 objetivos dependientes del idioma pasan (142 pruebas). Una primera ejecución falló al compilar por artefactos incompletos en `target/` (`der`, `tiny_skia` sin `.rlib`); `cargo test --workspace --no-run` lo resolvió sin cambios y se repitieron las puertas |
| `7cb1db40c59f3f8ec6c2bcae2f371dff3aefde1c` | Marca: fuera los 8 archivos ArtCraft de `docs/brand/` y la captura `docs/images/pdfcraft-tools.png`; logotipo del titular (`docs/brand/amaraza-logo.{svg,png,ico}`) y licencia aprobada; `BRAND_LICENCE` = `LicenseRef-Amaraza-Trademark`; logotipo en "Acerca de" y en `amaraza-identity`; `NOTICE` | fmt 0, clippy 0, assets ok (251 activos); pruebas: 1567 pasadas, 10 fallidas (las de la línea base), 6 ignoradas; en inglés, los 6 objetivos dependientes del idioma pasan. Una primera ejecución falló al compilar por falta de memoria virtual (`os error 1455`, 16 GB de RAM con 12 trabajos en paralelo); desde aquí las puertas se ejecutan con `CARGO_BUILD_JOBS=6` |
| `1b35712bc6520ba2dc475bf9d6df4f11bea6aedc` | Identidad de la app: icono, título, nombre, `--version`, carpetas de ajustes y de recuperación, modo portátil, recursos del `.exe` y comprobación de actualizaciones desde `amaraza-identity`; fuera la migración de carpetas de PrintCraft | fmt 0, clippy 0 (también `--all-targets`), assets ok; pruebas: 1571 pasadas, 10 fallidas (las de la línea base), 6 ignoradas; en inglés, los 6 objetivos dependientes del idioma pasan (incluye los títulos de ventana de `--test viewer`) |
| `da6c45439c1524fc682aa3414f0dad6bfdac2d83` | Instalador de Windows y zip portátil con la identidad Amaraza (`UpgradeCode` y GUID nuevos, `AmarazaPDF.exe`, marcador `AmarazaPDF.portable`); `MSI_UPGRADE_CODE` y una prueba de coherencia en `amaraza-identity` | fmt 0, clippy 0, assets ok; pruebas: 1572 pasadas, 10 fallidas (las de la línea base), 6 ignoradas; los scripts de PowerShell se analizan sin errores y el XML de WiX es válido. **El MSI no se compiló**: WiX no está instalado en el equipo (instalarlo requiere autorización) |
| `a8eca388f5b658cb80e4647c94ac4466ec9f7b34` | `APP_ID` = `io.github.pony_svg.AmarazaPDF` y empaquetado de Linux, Flatpak, macOS y FreeBSD adaptado; fuera el icono de PdfCraft (`assets/app-icon/`), `packaging/icons.sh` y el fondo del DMG; prueba de coherencia del empaquetado en `amaraza-identity` | fmt 0, clippy 0, assets ok (235 activos); pruebas: 1573 pasadas, 10 fallidas (las de la línea base), 6 ignoradas; `bash -n` sin errores en los 4 scripts, YAML y XML válidos. **No se empaquetó en esas plataformas** (no hay Linux, macOS ni FreeBSD en este equipo) |
| `27d294b4f705c614b0f43b14fa67ef467d75d7dd` | CI: fuera `release.yml` y los `schedule` de `ci.yml` y `windows-arm64.yml`; la prueba ARM64 usa los nombres de Amaraza. Actions no se activa | fmt 0, clippy 0, assets ok; pruebas: 1573 pasadas, 10 fallidas (las de la línea base), 6 ignoradas; YAML válido |
| `cb99da959ccf3a77278d8e467aa0130761dd17e5` | README propio, notas en `docs/releasing.md` y `docs/development.md`, comentarios sin la marca | fmt 0, clippy 0, assets ok; pruebas: 1573 pasadas, 10 fallidas (las de la línea base), 6 ignoradas |
| `dbef6b626824382ea96d396f891ebaea6e3ad121` | Comprobación final y este registro (solo documentación) | Puertas finales sobre `cb99da959ccf3a77278d8e467aa0130761dd17e5` con `CARGO_BUILD_JOBS=6`: fmt 0 (4 s), clippy 0 (2 s), assets ok (1 s, 235 activos); pruebas (103 s): **1573 pasadas, 10 fallidas, 6 ignoradas**, en 129 binarios. Los 10 fallos son exactamente los de la línea base; las pruebas de rendimiento pasan. Con `LANG=en_US.UTF-8`, los 6 objetivos dependientes del idioma pasan (142 pruebas). Una ejecución completa anterior tuvo además el fallo intermitente de `--test recovery` (ver Pendiente) |
| *(siguiente)* | Corrección de autoría: los 11 commits de `origin/main..ar0.1-identidad` reescritos con el autor del proyecto (ver la sección siguiente) y este registro con los SHA nuevos | Sin cambios de contenido: `git diff` entre el extremo anterior y el nuevo, vacío |

**Nota sobre las pruebas de rendimiento** (`pdfcraft-ui-egui --test perf`: `scrolling_a_500_page_document_stays_within_the_frame_budget` y `panels_with_hundreds_of_items_stay_within_the_frame_budget`):
- Miden el tiempo medio por fotograma contra un presupuesto fijo de 40 ms. En Windows no pueden leer la carga del sistema, así que nunca se saltan.
- En la línea base pasaron (7,4 s). Desde las 20:00 fallan siempre, también aisladas (dos repeticiones: unos 60 ms por fotograma). En ese momento el portátil funcionaba con batería (26 %, descargándose, plan "Equilibrado").
- El crate de identidad no lo usa todavía ningún otro crate, así que no puede afectarlas. Se tratan como dependientes del entorno y se repiten con el equipo conectado a la corriente antes de cerrar AR0.1.
- Resultado: pasaron en todas las ejecuciones completas siguientes (enlaces, marca, identidad de la app, instalador, empaquetado, CI, documentación y puertas finales), con el equipo ya cargando.

### Comprobación final (sobre `cb99da959ccf3a77278d8e467aa0130761dd17e5`)

**Búsqueda de la marca** (el mismo comando del inventario inicial):

```
grep -rniE "artcraft|getartcraft|storyteller" --exclude-dir=vendor --exclude-dir=target --exclude-dir=.git .
```

- Antes: 302 líneas en 53 archivos, y 22 archivos rastreados con la marca en el nombre.
- Después: **35 líneas en 12 archivos sin contar este registro** (64 en 13 contándolo, medido antes de redactar esta sección), **y ningún nombre de archivo** (`git ls-files | grep -iE "artcraft|storyteller"` no devuelve nada). Ninguna línea es un logotipo, un enlace al sitio o a la comunidad de ArtCraft, ni un identificador del upstream.

| Archivo | Líneas | Qué queda | Clase |
|---|---:|---|---|
| `LICENSE-MIT` | 1 | Copyright "ArtCraft Team and the PdfCraft contributors" | Permitida por el plan (copyright) |
| `NOTICE` | 3 | Atribución "Basado en PdfCraft, del equipo ArtCraft", el copyright y la frase de que las marcas ArtCraft no forman parte de Amaraza PDF | Permitida (copyright y atribución); la tercera es una declaración de no afiliación en texto plano |
| `README.md` | 2 | La atribución en texto plano y la frase de no afiliación | Permitida (atribución) y declaración de no afiliación |
| `crates/amaraza-identity/src/lib.rs` | 5 | La constante `BASED_ON` (la atribución) y las listas de cadenas prohibidas de sus pruebas | Atribución y guardas de prueba |
| `docs/brand/LICENSE-brand.txt` | 3 | El apartado "Relación con PdfCraft" de la licencia aprobada por el titular | Texto aprobado por el titular (no afiliación) |
| `crates/engine/src/links.rs`, `crates/ui-egui/tests/links.rs`, `xtask/src/assets.rs` | 1 + 2 + 2 | Pruebas que fallan si vuelve un enlace, una etiqueta o un activo de la marca | Guardas de prueba: necesitan la palabra literal |
| `AGENTS.md` | 1 | La excepción de marca del upstream (§1.2) | Archivo del upstream que las reglas del fork prohíben editar (§0.3); §2 de las reglas la sustituye |
| `ROADMAP.md` | 1 | Entrada histórica del upstream: "bundle id `ai.storyteller.pdfcraft`" (2026-10-07) | Historial del upstream; las reglas (§2) piden no tocar `ROADMAP.md` sin una tarea que lo pida |
| `.agents/rules/00-amaraza.md`, `docs/amaraza/PLAN.md`, `docs/amaraza/REGISTRO.md` | 6 + 8 + el registro | Las reglas, el plan y este registro, que tratan de la marca | Documentos propios del fork |

**Binario:** `cargo build -p pdfcraft -p pdfcraft-cli`; búsqueda de cadenas en `target/debug/pdfcraft.exe`:
- `getartcraft` 0, `discord.gg` 0, `aria-label="ArtCraft"` 0, `artcraft-mark` 0, `storyteller` 0, `Learning Machines` 0, `storytold/pdfcraft/releases` 0.
- `ArtCraft` 1, que es la atribución "Basado en PdfCraft, del equipo ArtCraft" del diálogo "Acerca de".
- Propiedades del `.exe`: `ProductName` y `FileDescription` "Amaraza PDF", `CompanyName` "Amaraza", `OriginalFilename` `AmarazaPDF.exe`, `InternalName` `AmarazaPDF`, `LegalCopyright` "Amaraza PDF, basado en PdfCraft. Copyright (c) the PdfCraft contributors. MIT OR Apache-2.0.".
- `pdfcraft.exe --version` → `Amaraza PDF 0.4.0`.

### Pendiente (fuera del alcance de AR0.1 o sin poder comprobarse aquí)

1. **Textos heredados que nombran PdfCraft:** 12 mensajes de la interfaz, con sus traducciones en 12 catálogos. Por ejemplo, "PdfCraft {v} is available.", "PdfCraft didn't shut down normally…", "Files you open in PdfCraft appear here…", los avisos de formularios XFA y de envío de formularios, y los del almacén de certificados. Lista completa: las entradas de `crates/ui-egui/src/i18n/es.tsv` cuya fuente contiene "PdfCraft", salvo "PdfCraft on GitHub", que es el enlace de atribución. Es trabajo de traducción, una tarea aparte (§8).
2. **Metadatos que siguen diciendo PdfCraft** (fuera del alcance que eligió el titular): el `Producer` de los PDF creados (`crates/create/src/lib.rs`), el nombre de la app en las firmas (`crates/sign/src/pdf.rs`), el autor por defecto de los comentarios (`crates/automation/src/comments.rs`), el título de los trabajos de impresión (`crates/print/src/spool.rs`) y el nombre del servidor MCP (`crates/automation/src/mcp.rs`).
3. **Capturas de `docs/images/`:** muestran la interfaz de PdfCraft con su botón de Discord. Ya no las usa el README. Para regenerarlas hace falta `cargo xtask screenshots` (necesita Chrome para el PDF de demostración).
4. **Nombres de binario:** en Linux, macOS y FreeBSD el ejecutable sigue siendo `pdfcraft`, y la CLI es `pdfcraft-cli` en todas las plataformas.
5. **Instalador sin compilar:** WiX v5 no está instalado. El MSI y su `test-msi.ps1` no se han ejecutado.
6. **Empaquetado de Linux, Flatpak, macOS y FreeBSD:** no se ha probado (solo `bash -n`, YAML y XML).
7. **Defecto del upstream `no_tool_escapes_the_root`** (escritura fuera de `--root` a través de un enlace simbólico en Windows): el titular decide si se corrige en el fork o se comunica al upstream.
8. **9 pruebas de interfaz dependen del idioma de Windows:** se podría fijar el idioma en sus arneses (tarea propia del fork) o comunicarlo al upstream.
9. **Prueba intermitente** `saving_or_closing_clears_the_entry_and_nothing_is_written_for_clean_documents` (`--test recovery`): falló una vez en la penúltima ejecución completa y pasó en las otras 10 ejecuciones completas y en 8 repeticiones aisladas. Usa su propia carpeta temporal, no la carpeta de recuperación que cambia AR0.1.
10. `LEGACY_STORAGE_KEY` (`"printcraft"`) sigue leyendo esa clave dentro del propio `app.ron` de Amaraza. Nunca existe y no toca carpetas de otra app.
11. Un comentario de `.github/workflows/ci.yml` todavía menciona `release.yml`, y su trabajo de fuzzing conserva la condición `schedule` (inofensiva sin `schedule`).

### Evidencia que debe aportar el titular

- **App real (Windows):** `cargo run -p pdfcraft` (o el `.exe` instalado) y una captura con la barra de título de Windows visible que muestre:
  - el título "Amaraza PDF" y, con un documento abierto, "documento.pdf — Amaraza PDF";
  - el logotipo Amaraza en la barra de tareas y en Alt+Tab;
  - la pantalla de inicio sin la tarjeta de comunidad, y la barra superior sin el botón de Discord;
  - Ayuda ▸ "Acerca de Amaraza PDF" con el logotipo, "Basado en PdfCraft, del equipo ArtCraft" y los dos enlaces;
  - Ayuda ▸ "Amaraza PDF en GitHub", que abre el repositorio del fork;
  - Ayuda ▸ "Buscar actualizaciones…", que consulta las releases del fork (hoy no hay ninguna).
- **Carpetas:** después de usar la app existen `%APPDATA%\AmarazaPDF` y, tras editar un documento, `%LOCALAPPDATA%\AmarazaPDF\Recovery`; no se crea ni se modifica nada en `%APPDATA%\PdfCraft` ni en `%LOCALAPPDATA%\PdfCraft`. Con un archivo `AmarazaPDF.portable` junto al `.exe`, todo va a `AmarazaPDFData`.
- **Propiedades del `.exe`** (clic derecho ▸ Propiedades ▸ Detalles): nombre del producto "Amaraza PDF", compañía "Amaraza", nombre de archivo original `AmarazaPDF.exe` y el icono Amaraza en el Explorador.
- **Instalador** (requiere WiX v5 y autorización para instalarlo): `pwsh packaging/windows/package.ps1 -Arch x64` debe producir `AmarazaPDF-0.4.0-windows-x64.msi`, y `test-msi.ps1` debe pasar. Al instalarlo, comprobar:
  - "Amaraza PDF" en Agregar o quitar programas, con el fabricante "Amaraza" y los enlaces al repositorio del fork;
  - `C:\Program Files\AmarazaPDF\AmarazaPDF.exe`;
  - los accesos directos "Amaraza PDF" en el menú Inicio y el escritorio;
  - "Abrir con ▸ Amaraza PDF" en un PDF, y "Crear PDF con Amaraza PDF…" en una imagen;
  - que un PdfCraft instalado en paralelo **no** se desinstala (`UpgradeCode` distinto);
  - que el zip portátil trae `AmarazaPDF.exe` y `AmarazaPDF.portable`.

## Corrección de autoría (2026-10-09)

**Motivo:** el correo del autor no era el del proyecto. Los 11 commits locales y no subidos de `origin/main..ar0.1-identidad` llevaban como autor y committer el correo global de este equipo. El fork es público, así que se corrigió antes de cualquier push. El titular autorizó expresamente reescribir esos commits, solo para esta corrección (revisión D-13 de AR0.0 y AR0.1).

**Qué se hizo:**
1. `git config --local user.name "Juan Diego Peraza Amado"` y `git config --local user.email` con el correo del proyecto.
2. Con el árbol limpio, en `ar0.1-identidad`, se anotó el extremo (`65e4c29fff0c5abe3e0e72b74215825a8f36edef`) y se ejecutó `git rebase -r origin/main --exec "git commit --amend --no-edit --reset-author"`.
3. `git branch -f main` al nuevo commit de AR0.0.

**Comprobaciones:**
- `git log --format="%ae %ce" origin/main..ar0.1-identidad` (11 líneas) y `git log --format="%ae %ce" origin/main..main` (1 línea): solo el correo del proyecto, como autor y como committer.
- `git diff 65e4c29fff0c5abe3e0e72b74215825a8f36edef ar0.1-identidad`: vacío (0 bytes). Solo cambiaron autor, committer y fechas, no el contenido.
- `main` apunta a `61c281839a07f0baf12f29d9aba17b4288a3abd8` (AR0.0), cuyo padre es `origin/main` (`90bb2f14ad4c60f84d900c1389e289e51a5ce7a6`).
- El correo anterior no aparece en ningún archivo rastreado ni en ningún mensaje de commit.

**SHA anterior → SHA nuevo** (cada SHA nuevo comprobado con `git cat-file -t` → `commit`; este registro ya usa los nuevos):

| Anterior | Nuevo | Commit |
|---|---|---|
| `2a6fa369a938d43f6f18ac8ff46d04791f5577bc` | `61c281839a07f0baf12f29d9aba17b4288a3abd8` | AR0.0: reglas, plan y línea base del fork Amaraza |
| `36312494802acec1e78514121ed0fc2d31063ebb` | `63b25fed91709aa7809dba8bec3b13c6c921b4f6` | AR0.1: decisiones del titular e inventario inicial de la marca |
| `af1edb863173a83d081868955192497cfcb11e8f` | `f0785d3f9b68d86511760a331c53ff30d6b76948` | AR0.1: crate amaraza-identity con la identidad de Amaraza PDF |
| `7bf38ea5538e1f706f45637e7a155b07f20334a9` | `12df0d4b7ae2da8d81b48bba3558b703cfd421ca` | AR0.1: interfaz y CLI sin enlaces ni comunidad de ArtCraft |
| `5a91dba9768ae222e80406357df8c2f42d3a76cb` | `7cb1db40c59f3f8ec6c2bcae2f371dff3aefde1c` | AR0.1: marca Amaraza en lugar de la marca ArtCraft |
| `3ecef8519b8dadc86ed9a72af8ef5c330f4790cd` | `1b35712bc6520ba2dc475bf9d6df4f11bea6aedc` | AR0.1: identidad de la app (título, carpetas, modo portátil, icono y .exe) |
| `fca0f5058fd11a70046e54087fa4dc8a5ae25e21` | `da6c45439c1524fc682aa3414f0dad6bfdac2d83` | AR0.1: instalador de Windows y zip portátil con la identidad Amaraza |
| `dfce1d4b646f5ec9e8db93166a56df70ab954e5e` | `a8eca388f5b658cb80e4647c94ac4466ec9f7b34` | AR0.1: APP_ID propio y empaquetado de Linux, Flatpak, macOS y FreeBSD |
| `538746aad3c33d40ad6076fb4b4abe3d4e0d600e` | `27d294b4f705c614b0f43b14fa67ef467d75d7dd` | AR0.1: CI del fork sin publicación ni tareas programadas |
| `cab1c1bd1330ab64d2949085a790e118d4738d47` | `cb99da959ccf3a77278d8e467aa0130761dd17e5` | AR0.1: README propio y documentación sin la marca ArtCraft |
| `65e4c29fff0c5abe3e0e72b74215825a8f36edef` | `dbef6b626824382ea96d396f891ebaea6e3ad121` | AR0.1: comprobación final y registro |

Los mensajes de commit no se editaron (`--no-edit`): el del commit de la comprobación final cita `cab1c1bd1330`, que corresponde a `cb99da959ccf3a77278d8e467aa0130761dd17e5`. Las puertas que este registro atribuye a un commit se ejecutaron sobre su árbol anterior, idéntico al nuevo.
