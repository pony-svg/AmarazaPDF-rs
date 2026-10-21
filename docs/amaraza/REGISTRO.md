# Registro de ejecución del fork Amaraza PDF (Rust)

Reglas: `.agents/rules/00-amaraza.md`. Plan: `docs/amaraza/PLAN.md`.

Cada entrada anota la tarea, los comandos, el resultado y el SHA completo (40 caracteres), copiado de `git rev-parse` y comprobado con `git cat-file -t`. Un commit no puede contener su propio SHA: el de cada commit se anota en la entrada siguiente.

## Archivos del upstream modificados

Mapa para resolver conflictos en cada sincronización con el upstream (`ARS.n`). Una fila por cada archivo del upstream que el fork modifica, borra o renombra. Los archivos nuevos del fork (`docs/amaraza/`, `.agents/rules/00-amaraza.md`, `.claude/rules/amaraza.md`, `crates/amaraza-*`) no van aquí.

**Cómo localizar los cambios (ARS.4):** los números de línea de las filas son orientativos, de cuando se escribió cada fila; el upstream los desplaza en cada sincronización. Los enganches de AR0.3 se localizan con `git grep -n "ADR-0001" -- ':!docs'` y por el ancla de contenido de la tabla del §3 del ADR-0001; los demás cambios, por la cadena del fork que contienen (por ejemplo, `amaraza_identity::` o "Amaraza PDF").

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
| `crates/ui-egui/tests/ui_group/links.rs` (antes `crates/ui-egui/tests/links.rs`; el upstream lo movió en ARS.2) | Reescrita: ni Discord ni ArtCraft en pantalla, "Acerca de" con atribución y enlaces, comandos de Ayuda | Prueba del nuevo comportamiento | AR0.1 |
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
| `crates/ui-egui/src/dialogs.rs`, `xtask/src/screenshots.rs`, `crates/engine/src/links.rs`, `xtask/src/assets.rs`, `crates/ui-egui/tests/ui_group/links.rs` | Comentarios reformulados para no nombrar ArtCraft sin necesidad (las cadenas de las pruebas que prohíben la marca se quedan) | Búsqueda final más limpia | AR0.1 |
| `docs/brand/artcraft-{logo,logo-white,mark,mark-black}.{svg,png}` (8) | **Borrados** | Licencia de marca ArtCraft: un fork debe retirarlas | AR0.1 |
| `docs/brand/LICENSE-brand.txt` | Sustituido por la licencia de la marca Amaraza aprobada por el titular | Ídem | AR0.1 |
| `docs/images/pdfcraft-tools.png` | **Borrado** | Captura de la pantalla de inicio con la marca ArtCraft | AR0.1 |
| `ATTRIBUTION.toml`, `ATTRIBUTION.md` | Las 8 entradas ArtCraft pasan a 3 de `docs/brand/amaraza-logo.{svg,png,ico}`; fuera la entrada de la captura; `ATTRIBUTION.md` regenerado | Ídem | AR0.1 |
| `xtask/src/assets.rs` | `BRAND_LICENCE` = `LicenseRef-Amaraza-Trademark`; su prueba usa el logotipo Amaraza; nueva prueba que rechaza `LicenseRef-Storyteller-Trademark` | Que una sincronización no pueda volver a traer marcas ArtCraft | AR0.1 |
| `xtask/src/screenshots.rs` | Sin el texto de la marca ArtCraft en la pantalla de inicio | Ya no aparece | AR0.1 |
| `NOTICE` | Cabecera de Amaraza PDF con la atribución, aviso de archivos modificados (Apache-2.0 §4(b)) y marca Amaraza en lugar de la ArtCraft | Licencia Apache-2.0 y de marca | AR0.1 |
| `apps/pdfcraft/src/single_instance.rs` (nuevo en el upstream) | Socket de la instancia única en la carpeta de Amaraza PDF (`amaraza_identity::FOLDER_NAME`) en lugar de `%LOCALAPPDATA%\PdfCraft`; prueba `the_socket_is_in_amaraza_pdfs_own_folder` | No tocar carpetas de PdfCraft ni entregar archivos a un PdfCraft abierto | ARS.1 |
| `apps/pdfcraft-cli/build.rs` (nuevo en el upstream), `apps/pdfcraft-cli/tests/version_resource.rs` | Recursos de `pdfcraft-cli.exe`: `ProductName` "Amaraza PDF", `FileDescription` "Amaraza PDF, herramienta de línea de comandos", `CompanyName` "Amaraza" y el copyright de `pdfcraft.exe`; el nombre del archivo no cambia | Identidad (lo que muestra el Explorador) | ARS.1 |
| `apps/pdfcraft/build.rs`, `apps/pdfcraft/src/updates.rs`, `crates/ui-egui/src/chrome.rs`, `packaging/windows/{pdfcraft.wxs,package.ps1,test-msi.ps1}`, `packaging/linux/package.sh`, `packaging/linux/flatpak/io.github.pony_svg.AmarazaPDF.bundle.yml`, `packaging/freebsd/package.sh` | Conflictos de ARS.1: la identidad de AR0.1 más lo nuevo del upstream (manifiesto de Windows, estados HTTP de GitHub, pestañas que se encogen con el ancho reservado sin Discord, modelos de OCR junto a `AmarazaPDF.exe` y en `share/pdfcraft/models`) | Ídem AR0.1 | ARS.1 |
| `crates/ui-egui/src/i18n/{ar,it,pt-br}.tsv` | El cambio de AR0.1 en los catálogos nuevos o ampliados por el upstream: fuera Discord, ArtCraft, "PdfCraft web page" y "Part of"; "Welcome to PdfCraft" y "About PdfCraft" pasan a las fuentes en español; nueva "Amaraza PDF en GitHub" | Ídem AR0.1 | ARS.1 |
| `crates/ui-egui/tests/{ui,pickers,viewer}.rs`, `crates/ui-egui/src/i18n/mod.rs` (solo su módulo de pruebas) | Pruebas nuevas del upstream adaptadas: "Keyboard shortcuts" en lugar del botón de Discord, título de inicio y de ventana de Amaraza PDF, "Acerca de Amaraza PDF" en árabe | Pruebas del comportamiento del fork | ARS.1 |
| `Cargo.lock` | `amaraza-identity` 0.5.0 (la versión del workspace) | Merge de la versión 0.5.0 del upstream | ARS.1 |
| `ATTRIBUTION.toml`, `ATTRIBUTION.md` | `sha256` de los 15 catálogos; `ATTRIBUTION.md` regenerado | `cargo xtask assets` | ARS.1 |
| `crates/ui-egui/src/image_pick.rs` (nuevo en el upstream) | Carpeta del portapapeles en la de Amaraza PDF (`amaraza_identity::FOLDER_NAME`/`FOLDER_NAME_LOWER`) en lugar de `PdfCraft`; prueba `the_clipboard_folder_is_amaraza_pdfs_own` | La función borra archivos en esa carpeta: nunca en la de PdfCraft | ARS.2 |
| `crates/ui-egui/src/chrome.rs`, `crates/ui-egui/tests/ui.rs` | Barra de título nueva (controles de ventana propios) sin el botón de Discord ni su ancho reservado; la prueba de la banda de macOS no busca "Discord" | Licencia de marca ArtCraft | ARS.2 |
| `apps/pdfcraft-cli/src/main.rs`, `crates/ui-egui/src/dialogs.rs`, `apps/pdfcraft/src/main.rs` | `--version` y "Acerca de" con `build_info::version()` y la identidad de Amaraza, sin Discord ni la web de ArtCraft; sin `migrate_legacy_folders`; las pruebas permiten el icono genérico de documento y leen el `.desktop` del fork | Identidad | ARS.2 |
| `apps/pdfcraft/build.rs` | Recurso 2 = icono genérico de documento PDF del upstream (`assets/app-icon/pdfcraft-document.ico`); el icono de la app sigue siendo el logotipo de Amaraza | Asociaciones de archivos PDF del instalador | ARS.2 |
| `apps/pdfcraft/src/single_instance.rs` | Comportamiento portátil nuevo del upstream (socket `portable-…`), siempre en la carpeta de Amaraza PDF | Identidad | ARS.2 |
| `packaging/windows/pdfcraft.wxs`, `packaging/windows/installer-ui.wxs`, `packaging/windows/test-msi.ps1`, `packaging/windows/package.ps1` | Menú contextual nuevo "Editar con Amaraza PDF" (`AmarazaPDF.Edit`) con **GUID de componente propio** `73C1D61D-77D3-4DC6-974C-C5BF06C92F97`; `FileSearch` de "Amaraza PDF.lnk"; `DefaultIcon` `,-2` en el ProgId y en `Applications\AmarazaPDF.exe`; el zip portátil lleva `NOTICE` y `LICENSE-document.txt` | Identificadores propios del instalador | ARS.2 |
| `packaging/linux/io.github.pony_svg.AmarazaPDF.desktop`, `packaging/linux/package.sh`, `packaging/macos/package.sh`, `apps/pdfcraft/tests/winit_x11_dpi.rs` | `StartupWMClass` con el id del fork; etiqueta de compilación del upstream con los nombres `AmarazaPDF-*`; DMG con `hdiutil create` (volumen "Amaraza PDF") y comprobación de "Amaraza PDF.app" | Identidad | ARS.2 |
| `.github/workflows/packaging-lint.yml`, `.github/workflows/windows-arm64.yml` | Se conservan los del fork; no los flujos reutilizables `storytold/craft-common-actions@main` del upstream | Identidad y no depender de flujos externos sin versión fija | ARS.2 |
| `.github/workflows/freebsd.yml` | La versión del fork (la de `e9b9ee6`, autocontenida); el upstream la cambió sin conflicto en ARS.2 por `storytold/craft-common-actions@main` | Ídem; lo detectó la revisión D-13 de ARS.2/ARS.3 | ARS.3 |
| `.github/dependabot.yml` (nuevo en el upstream) | **Borrado** | Actualizaciones diarias automáticas de dependencias: en el fork, añadir o actualizar dependencias lo decide el titular (`00-amaraza.md` §6) | ARS.3 |
| `.github/pull_request_template.md` (nuevo en el upstream) | "Thanks for contributing to Amaraza PDF (a fork of PdfCraft)." | Identidad | ARS.3 |
| `.github/workflows/nix.yml` (nuevo en el upstream en ARS.2) | **Sin cambios**: se conserva tal cual. Se dispara con `push` y `pull_request` sobre `Cargo.lock`, sin `schedule` ni flujos de `craft-common-actions`; inerte mientras Actions siga desactivado en el fork | Revisar en AR0.5 si se activa Actions; lo señaló la segunda revisión independiente | ARS.4 |
| `packaging/macos/Info.plist.in` | `CFBundleTypeIconFile` del tipo de documento PDF = `AmarazaPDF` (el `.icns` que genera `package.sh`), no `PdfCraft`; entró sin conflicto en ARS.2. Las claves internas `PdfCraftVersion` y `PdfCraftBuildCommit` se conservan, como los nombres de crates | Identidad; lo señaló la segunda revisión independiente | ARS.4 |
| `crates/engine/src/links.rs` | Pruebas nuevas del upstream (documentos remotos) más la del fork; fuera la que exigía la web de ArtCraft | Licencia de marca | ARS.2 |
| `crates/ui-egui/src/i18n/{cs,el,kk,lt,pl,sl,tr}.tsv` | El tratamiento de AR0.1 (fuera Discord, ArtCraft y "Part of"; fuentes en español; "Amaraza PDF en GitHub"); `cs` había recuperado las entradas de Discord en la fusión | Licencia de marca | ARS.2 |
| `crates/ui-egui/Cargo.toml`, `Cargo.lock`, `ATTRIBUTION.*` | `craft-ui` del upstream junto a `amaraza-identity`; `amaraza-identity` 0.8.0; `sha256` de los 21 catálogos y entradas del icono de documento sin las de ArtCraft ni las del icono de PdfCraft | Merge | ARS.2 |
| `packaging/windows/pdfcraft.wxs`, `packaging/windows/test-msi.ps1` | Reconstruidos desde los del upstream con las sustituciones de identidad del fork (guiones de ARS.3, ver la sección ARS.3). Novedades: el `<Upgrade>` que detecta versiones previas usa el `UpgradeCode` del fork; componente del menú de imágenes con **GUID propio** `96D52A18-56D6-49F5-A0C9-00CD3790CD18`; búsquedas de registro de `AmarazaPDF.Edit` y `AmarazaPDF.CreatePdf` | Identificadores propios del instalador | ARS.3 |
| `apps/pdfcraft/src/settings.rs` (nuevo en el upstream), `apps/pdfcraft/src/main.rs` | Preferencias XDG en `~/.config/amarazapdf` (`FOLDER_NAME_LOWER`), no en la de PdfCraft, con sus pruebas; carpeta de datos y de registros de Amaraza PDF; sin la migración de PrintCraft; menú de la app en macOS con el nombre del producto | Nunca tocar carpetas de PdfCraft ni PrintCraft | ARS.3 |
| `apps/pdfcraft/src/single_instance.rs` | Socket de Linux y BSD en `$XDG_RUNTIME_DIR/amarazapdf`; la prueba del fork cubre Windows y Linux | No entregar archivos a un PdfCraft abierto | ARS.3 |
| `packaging/macos/Credits.rtf`, `packaging/macos/package.sh` | El panel "Acerca de" de macOS muestra la atribución en texto plano y los enlaces al fork y al upstream, sin la web ni el Discord de ArtCraft | Licencia de marca ArtCraft | ARS.3 |
| `crates/engine/Cargo.toml`, `crates/engine/src/links.rs`, `docs/development.md`, `crates/ui-egui/src/i18n/*.tsv` (con `vi`, nuevo), `Cargo.lock`, `ATTRIBUTION.*` | `craft-docpath` junto a `amaraza-identity`; pruebas nuevas del upstream más la del fork; carpetas de Amaraza en la documentación; tratamiento de marca de AR0.1 en los 22 catálogos; `amaraza-identity` 0.9.0 | Merge | ARS.3 |
| `xtask/src/layers.rs` | **E1** (l. 80–81): `amaraza-ext` en `TABLE` como `Layer(6)`, tras `engine`; marcador en el comentario | La comprobación de capas exige registrar cada crate | AR0.3 |
| `xtask/src/layers.rs` | **E2** (l. 102): primera arista de `SIDEWAYS`, `("amaraza-ext", "engine")`; marcador al final | `amaraza-ext` usa el motor desde su misma capa | AR0.3 |
| `crates/engine/src/lib.rs` | **E3** (l. 16): `pub mod amaraza;`; marcador al final | Enganche de ADR-0001; los crates `amaraza-*` no pueden registrar por su cuenta (capas) | AR0.3 |
| `crates/engine/src/commands.rs` | **E4** (l. 157): la lista literal pasa de `pub const COMMANDS` a `const UPSTREAM_COMMANDS`, sin otros cambios | Enganche de ADR-0001; los crates `amaraza-*` no pueden registrar por su cuenta (capas) | AR0.3 |
| `crates/engine/src/commands.rs` | **E5** (l. 347–349): nuevo `pub const COMMANDS` = los del upstream + `crate::amaraza::COMMANDS`, concatenados al compilar; marcador en el comentario. rustfmt lo deja en una línea (3 en vez de las 4 previstas) | Enganche de ADR-0001; los crates `amaraza-*` no pueden registrar por su cuenta (capas) | AR0.3 |
| `crates/engine/src/actions.rs` | **E6** (l. 11–12): variante `Step::Amaraza(crate::amaraza::Step)`, la primera del enum; marcador en el comentario | Enganche de ADR-0001; los crates `amaraza-*` no pueden registrar por su cuenta (capas) | AR0.3 |
| `crates/engine/src/actions.rs` | **E7** (l. 38–46): envoltorio `pub fn all()` (upstream + fork); el `all()` del upstream pasa a `fn upstream_all()`; marcador en el comentario | Enganche de ADR-0001; los crates `amaraza-*` no pueden registrar por su cuenta (capas) | AR0.3 |
| `crates/engine/src/actions.rs` | **E8** (l. 66, 84, 102–105 y 113–116): brazo `Step::Amaraza(p) => …` al principio de `id` y `label`; guarda `if let` antes del `match` de `arg` y `arg_mut`; un marcador en cada uno | Enganche de ADR-0001; los crates `amaraza-*` no pueden registrar por su cuenta (capas) | AR0.3 |
| `crates/engine/src/actions.rs` | **E9** (l. 172): brazo `Step::Amaraza(p) => return crate::amaraza::apply(…)` al principio del `match` de `apply_step`; marcador al final | Enganche de ADR-0001; los crates `amaraza-*` no pueden registrar por su cuenta (capas) | AR0.3 |
| `crates/automation/Cargo.toml` | **E10** (l. 13–14): dependencia interna `amaraza-ext = { path = … }`; marcador en el comentario | AR-D3 | AR0.3 |
| `crates/automation/src/lib.rs` | **E11** (l. 15): `mod amaraza;`; marcador al final | Enganche de ADR-0001; los crates `amaraza-*` no pueden registrar por su cuenta (capas) | AR0.3 |
| `crates/automation/src/lib.rs` | **E12** (l. 150): brazo con guarda `amaraza::PREFIX` al principio del `match` de `Automation::call`; marcador al final | Enganche de ADR-0001; los crates `amaraza-*` no pueden registrar por su cuenta (capas) | AR0.3 |
| `crates/automation/src/tools.rs` | **E13** (l. 116–124): envoltorio `pub fn tools()` (upstream + fork); el `tools()` del upstream pasa a `fn upstream_tools()`; marcador en el comentario | Enganche de ADR-0001; los crates `amaraza-*` no pueden registrar por su cuenta (capas) | AR0.3 |
| `crates/ui-egui/Cargo.toml` | **E14** (l. 14): dependencia interna `amaraza-ext`, junto a la de AR0.1; marcador al final | AR-D3 | AR0.3 |
| `crates/ui-egui/src/lib.rs` | **E15** (l. 27): `mod amaraza;`; marcador al final | Enganche de ADR-0001; los crates `amaraza-*` no pueden registrar por su cuenta (capas) | AR0.3 |
| `crates/ui-egui/src/commands.rs` | **E16** (l. 127): brazo con guarda `crate::amaraza::handles` al principio del `match id` de `execute_unguarded`; marcador al final | Enganche de ADR-0001; los crates `amaraza-*` no pueden registrar por su cuenta (capas) | AR0.3 |
| `crates/ui-egui/src/chrome.rs` | **E17** (l. 392): `crate::amaraza::main_menu(app, ui);` antes del menú Ayuda; marcador al final. Riesgo medio: la zona cambió 4 veces en el upstream | Enganche de ADR-0001; los crates `amaraza-*` no pueden registrar por su cuenta (capas) | AR0.3 |
| `crates/ui-egui/src/i18n/mod.rs` | **E18** (l. 178–179): `Catalog::parse` recibe `crate::amaraza::catalog_source(self.code, self.source)`; marcador en la línea anterior (con él al final, la línea pasaba de 150 columnas) | Capa de traducciones del fork (AR-D4) | AR0.3 |
| `ATTRIBUTION.toml`, `ATTRIBUTION.md` | **E19**: entrada de `crates/ui-egui/src/amaraza/i18n.tsv` tras las del logotipo Amaraza (marcador en `usage`); `ATTRIBUTION.md` regenerado (239 activos) | `cargo xtask assets`: un `.tsv` es un activo | AR0.3 |
| `Cargo.lock` | **E20**: paquete `amaraza-ext` y sus aristas desde `pdfcraft-automation` y `pdfcraft-ui-egui`; sin crates externos ni versiones nuevas. Archivo generado, sin marcador | AR-D3 | AR0.3 |
| `crates/create/Cargo.toml`, `crates/create/src/lib.rs` | Dependencia `amaraza-identity`; `Producer` del PDF usa `amaraza_identity::PRODUCT_NAME` | Metadatos generados propios (AR-D7, ADR-0002) | AR0.4 |
| `crates/sign/Cargo.toml`, `crates/sign/src/pdf.rs` | Dependencia `amaraza-identity`; `/Prop_Build /App /Name` usa `amaraza_identity::PRODUCT_NAME` | Identidad en firmas digitales y sellos de tiempo (AR-D7, ADR-0002) | AR0.4 |
| `crates/edit/Cargo.toml`, `crates/edit/src/added.rs` | Dependencia `amaraza-identity`; nombre de familia de la fuente moldeada usa `amaraza_identity::SHAPED_TEXT_FONT_FAMILY` (l. 711) y `mod amaraza_tests;` (l. 1152, marcador ADR-0001) | Metadatos generados propios (AR-D7, ADR-0002) | AR0.4 |
| `crates/preflight/Cargo.toml`, `crates/preflight/src/icc.rs` | Dependencia `amaraza-identity`; descripción del perfil ICC sRGB usa `amaraza_identity::ICC_PROFILE_DESC` | Metadatos generados propios (AR-D7, ADR-0002) | AR0.4 |
| `crates/automation/Cargo.toml`, `crates/automation/src/{comments,mcp}.rs` | Dependencia `amaraza-identity`; `DEFAULT_AUTHOR` = `amaraza_identity::PRODUCT_NAME`; servidor MCP `serverInfo.name = "amarazapdf"`, `serverInfo.title = "Amaraza PDF"` e instrucciones y herramientas | Identidad en servidor MCP y metadatos (AR-D7, ADR-0002) | AR0.4 |
| `crates/engine/src/commands.rs`, `crates/engine/src/links.rs` | Retirado el comando `help.upstream` ("PdfCraft on GitHub") de `COMMANDS` y de `LINKS`; `UPSTREAM` se mantiene para "Acerca de" | AR-D8, ADR-0002 | AR0.4 |
| `crates/ui-egui/src/dialogs.rs` | "Acerca de": atribución `BASED_ON` es un enlace interactivo diferido a `links::UPSTREAM` sin requerir comando de menú | AR-D8, ADR-0002 | AR0.4 |
| `crates/ui-egui/src/image_pick.rs` | Fallback de `safe_name` usa `amaraza_identity::FOLDER_NAME` (`"AmarazaPDF"`) | Identidad en archivos del portapapeles | AR0.4 |
| `crates/ui-egui/src/web_print.rs` | Título del HTML de impresión web usa `amaraza_identity::PRODUCT_NAME` | Identidad en impresión web | AR0.4 |
| `docs/images/pdfcraft-*.png` (12 archivos) | **Borrados**: capturas obsoletas con marca ArtCraft y botones de Discord | Licencia de marca y limpieza de activos | AR0.4 |
| `ATTRIBUTION.toml`, `ATTRIBUTION.md` | Retiradas las 12 entradas de capturas; activos pasan de 251 a 239 | `cargo xtask assets` | AR0.4 |
| `packaging/linux/io.github.pony_svg.AmarazaPDF.metainfo.xml.in` | Retirado bloque `<screenshots>` que referenciaba las capturas borradas | Coherencia de empaquetado | AR0.4 |
| `crates/ui-egui/tests/{ui.rs,signing.rs,ui_group/updates.rs,ui_group/links.rs}`, `crates/automation/tests/automation.rs` | Pruebas adaptadas al nuevo nombre de producto en mensajes de UI, nombre MCP y enlace de atribución | Coherencia con la identidad en textos y metadatos | AR0.4 |

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

**B. Confinamiento de `--root` (1 fallo): falso positivo de entorno.** *Corregido en la revisión D-13 de AR0.2: la primera versión de este registro lo daba por un defecto real del upstream, y así lo dice también el mensaje del commit de AR0.0 (`61c281839a07f0baf12f29d9aba17b4288a3abd8`), que ya está publicado. Ningún mensaje de los commits de AR0.1 lo dice.*

- Prueba: `pdfcraft-automation --test hostile_args`, `no_tool_escapes_the_root`.
- Salida: `root confinement: 510 escape attempts over 140 tools, 495 refused, 15 leaks`.
- Qué pasa: la prueba intenta crear un enlace simbólico de directorio, `link-out`, que apunta fuera de la raíz, e ignora el error si no puede (`let _ = symlink_dir(…)`). Después espera que todas las herramientas rechacen `link-out/outside.pdf`. 15 herramientas la aceptan: `doc_save`, `page_insert_file`, `doc_create_multiple`, `doc_print`, `measure_export`, `sign_id_create`, `comments_summarize`, `doc_watermark`, `doc_background`, `accessibility_report`, `form_merge_data`, `doc_compare_report`, `doc_export_text`, `doc_reduce` y `doc_optimize`.
- Por qué es un falso positivo: en este equipo crear un enlace simbólico exige privilegios de administrador (no está activo el modo de desarrollador de Windows). El enlace no llega a existir, así que `link-out/outside.pdf` es una ruta normal **dentro** de la raíz, y aceptarla es lo correcto.
- Comprobaciones:
  - `New-Item -ItemType SymbolicLink` falla con "Se necesitan privilegios de administrador para esta operación".
  - La segunda red de la prueba (archivos creados, modificados o borrados fuera de la raíz) no encontró nada: las 15 filtraciones son solo "accepted".
- Donde se pueden crear enlaces (CI del upstream en Linux y macOS, o Windows con el modo de desarrollador), la prueba comprueba lo que dice. En este equipo la puerta de pruebas cuenta este fallo como de entorno. Hacer que la prueba se salte los casos de enlace cuando no se puede crear el enlace sería un cambio en el upstream: lo decide el titular (§6).

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
| `2f219c37c4734a00291dadf8ff186335b068d296` | Corrección de autoría: los 11 commits de `origin/main..ar0.1-identidad` reescritos con el autor del proyecto (ver "Corrección de autoría") y este registro con los SHA nuevos | Sin cambios de contenido: `git diff` entre el extremo anterior y el nuevo, vacío; fmt 0, clippy 0, assets ok |
| `6fcbb19089fc5deca6395e5cb1cc46fa316bae9b` | Reglas: §6 (reescribir solo commits locales no subidos y con autorización expresa) y §7.1 (comprobar `git config user.email` antes del primer commit) | Solo documentación; fmt 0, assets ok |

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
7. **Falso positivo de entorno `no_tool_escapes_the_root`** (ver AR0.0, fallo B; corregido en la revisión D-13 de AR0.2): aquí no se pueden crear enlaces simbólicos sin privilegios de administrador. No hay escritura fuera de `--root`.
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

### Revisión D-13 y evidencia del titular (2026-10-09)

- **Revisión D-13** de AR0.0 y AR0.1 (sesión distinta de la que implementó): aprobada, con una corrección obligatoria antes del push (la autoría; ver "Corrección de autoría").
- **Publicación:** el titular subió `main` a `origin`. Al empezar AR0.2 se comprobó que `main` == `origin/main` == `dbfd8249258b790f5f48d000ac1c0ea1d3b84f70` y que `git config user.email` es el correo del proyecto.
- **Evidencia real del titular** (captura de la aplicación en Windows, tomada por el titular):
  - título de la ventana "… — Amaraza PDF", con el icono de Amaraza;
  - sin el botón de Discord;
  - menú Ayuda con "Amaraza PDF en GitHub", "PdfCraft en GitHub" (la traducción al español de `help.upstream`), "Buscar actualizaciones…" y "Acerca de Amaraza PDF".
- **Carpetas:** se creó `%APPDATA%\AmarazaPDF`, y no hay carpetas de PdfCraft ni de PrintCraft.
- **Pendiente de evidencia:**
  - la captura del diálogo "Acerca de";
  - el MSI, que necesita WiX v5 (no instalado).

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

## AR0.2. Equivalencias PDForte → Rust (2026-10-09)

### Punto de partida

- `git config user.email`: el correo del proyecto (§7.1).
- `git fetch origin`: `main` == `origin/main` == `dbfd8249258b790f5f48d000ac1c0ea1d3b84f70` (`git cat-file -t` → `commit`). Ese commit es el que define AR0.2 a AR0.5 en el plan.
- Rama `ar0.2-equivalencias`, creada desde `main`.

### Qué se hizo

- `docs/amaraza/EQUIVALENCIAS.md` clasifica las 30 reglas de §3.1–§3.8 del `AGENTS.md` de PDForte y las 14 decisiones (D-01…D-13 y RF-FIRMA-01):

  | Origen | La cubre el original | Regla nueva de Amaraza | Descartada |
  |---|---:|---:|---:|
  | §3.1–§3.8 | 11 | 16 | 3 |
  | Decisiones | 6 | 6 | 2 |

  - **Criterio:** en el código del original mandan sus reglas; en el código del fork, la más estricta.
  - PDForte se cita por número de regla, sin traer código, actas ni auditorías (§3 de las reglas).
- `.agents/rules/00-amaraza.md` §9 recoge las reglas nuevas A1–A15.
- **Fuentes del original consultadas:**
  - guardar: `crates/ui-egui/src/editing.rs`;
  - firmados: `crates/engine/src/lib.rs`;
  - firmas: `crates/sign/src/pdf.rs` y `crates/sign/src/windows.rs`;
  - censura: `crates/ui-egui/src/redact_ui.rs`;
  - recuperación: `crates/ui-egui/src/recovery.rs`;
  - límites de render: `crates/render/src/raster.rs`;
  - ventana mínima y registros: `apps/pdfcraft/src/main.rs`;
  - capas: `xtask/src/layers.rs`;
  - licencias: `deny.toml`;
  - y `AGENTS.md` y `CLAUDE.md` del upstream.
- **Hallazgos en que PdfCraft es menos estricto que PDForte**, que quedan como decisiones del titular (EQUIVALENCIAS §4):
  - permite las ediciones incrementales de documentos firmados (D-03);
  - la recuperación escribe en claro y no purga por antigüedad (D-10);
  - al guardar verifica después de sustituir, y no hay copia ni barrera;
  - la censura conserva el historial de deshacer;
  - registra rutas completas;
  - la ventana mínima es de 820×520 (D-08);
  - no hay instancia única (D-12);
  - al listar certificados abre la clave en modo silencioso (A6; se comprueba con el token en AR1.2).
- No se modificó ningún archivo del upstream: la tabla de archivos modificados no cambia.

### Commits de AR0.2

| Commit | Contenido | Puertas |
|---|---|---|
| `647daf4e20d8e2cc526e287edc8ff145aae61a97` | AR0.1: evidencia del titular y SHA de los últimos commits (solo documentación) | fmt 0, assets ok |
| `4e98829054c96afe092420a4fa4fc91ce73194aa` | `docs/amaraza/EQUIVALENCIAS.md` (solo documentación) | Ver la fila siguiente |
| `2bcded2a68e5fd459c286715181b8d48168364e5` | Reglas A1–A15 (`.agents/rules/00-amaraza.md` §9) y este registro (solo documentación) | Puertas con `LANG=en_US.UTF-8` y `CARGO_BUILD_JOBS=6`, sobre el mismo código que `main`: fmt 0 (4 s), clippy 0 (8 s), assets ok (1 s, 235 activos); pruebas (305 s, portátil conectado a la corriente): **1582 pasadas, 1 fallida, 6 ignoradas**. El fallo es `no_tool_escapes_the_root`, el falso positivo de entorno de la línea base (ver AR0.0, fallo B); con el inglés pasan las 9 pruebas que dependen del idioma |
| `ece74322ada0423a28d6bf7ee0d84048db7818dd` | Correcciones de la revisión D-13 de AR0.2 (ver "Revisión D-13 de AR0.2: correcciones") | Ver esa sección |
| `fc6a8c993377698bf1db0ac0aa18a0e5196c1657` | Correcciones pendientes de la revisión D-13 (ver "Revisión D-13 de `ece7432`: correcciones pendientes") | Ver esa sección. **Dictamen D-13 sobre `fc6a8c9`: aprobado; AR0.2 cerrada.** Puertas del revisor en inglés: 1582 pasadas, 1 fallida (`no_tool_escapes_the_root`, de entorno) y 6 ignoradas, como la línea base. Fusionada en `main` con `e9b9ee622d4dbf9720ef0c62c823ddb3a54decb0` |

### Revisión D-13 de AR0.2: correcciones

- **`no_tool_escapes_the_root` es un falso positivo de entorno**, no un defecto del upstream. Está corregido en AR0.0 (fallo B, con la causa y las comprobaciones), en el pendiente 7 de AR0.1 y en la fila de puertas de AR0.2.
- **A7 aplicado de verdad en `amaraza-identity`:**
  - nuevo `crates/amaraza-identity/clippy.toml`: las 4 claves `allow-*-in-tests` de la raíz, porque clippy no fusiona configuraciones, más `cognitive-complexity-threshold = 15`;
  - en `src/lib.rs`, `#![warn(clippy::cognitive_complexity)]`: el lint es de nursery, y `[lints] workspace = true` impide `[lints.clippy]`.
- **Comprobación de A7:**
  - `cargo clippy -p amaraza-identity --all-targets -- -D warnings` detectó `the function has a cognitive complexity of (16/15)` en la prueba `the_other_platforms_packages_use_the_app_id`. Eso demuestra que se lee el umbral 15 del crate, no el 25 por defecto.
  - Se extrajo el recorrido de carpetas al ayudante `files_under`. Después: clippy con código 0 y `cargo test -p amaraza-identity` con 6 pruebas pasadas.
- `.agents/rules/00-amaraza.md` §9 queda más corto, con el mecanismo de A7, y `docs/amaraza/EQUIVALENCIAS.md` describe A7 igual.
- **Tamaño de los archivos de reglas** (límite: 12 000 caracteres):
  - `.agents/rules/00-amaraza.md`: 11 679 caracteres (antes de compactar §9, 12 820);
  - `.claude/rules/amaraza.md`: 178.
- SHA de `2bcded2a68e5fd459c286715181b8d48168364e5` anotado en su fila.
- **Puertas** (`LANG=en_US.UTF-8`, `CARGO_BUILD_JOBS=6`):
  - fmt 0 (4 s);
  - clippy 0 (11 s), también con `--all-targets` (16 s), que incluye las pruebas;
  - assets ok (235 activos);
  - pruebas (283 s): **1582 pasadas, 1 fallida, 6 ignoradas**. El fallo es `no_tool_escapes_the_root`, el falso positivo de entorno.

### Revisión D-13 de `ece7432`: correcciones pendientes

Dictamen sobre `ece74322ada0423a28d6bf7ee0d84048db7818dd`: aprobado con correcciones. Faltaban 4 correcciones de la revisión original de AR0.2, más una menor. Se aplican en un commit nuevo, sin reescribir `ece7432`.

1. **Las reglas heredadas, en su propio archivo.**
   - Nuevo `.agents/rules/01-amaraza-heredadas.md` (`trigger: always_on`). Lleva el criterio y A1–A15 con la redacción íntegra de `2bcded2a68e5fd459c286715181b8d48168364e5`, salvo cuatro cambios:
     - A4, aclarada (punto 2);
     - A7, con el mecanismo de `clippy.toml` y `#![warn(…)]`;
     - en A6 y A14, "§5" y "§6" pasan a "`00-amaraza.md` §5" y "§6", porque el archivo es otro.
   - `00-amaraza.md` §9 queda como una remisión, y `.claude/rules/amaraza.md` importa el archivo nuevo.
   - Referencias actualizadas: `crates/amaraza-identity/clippy.toml`, `crates/amaraza-identity/src/lib.rs`, `EQUIVALENCIAS.md` (2) y `PLAN.md` (AR0.2). Las menciones históricas de este registro no se tocan.
2. **A4**, en el archivo nuevo y en EQUIVALENCIAS §3: los módulos `#[cfg(test)]` están permitidos. Lo prohibido es que el código de producción cambie de comportamiento con `cfg!(test)`, detecte dobles de prueba o use identidades inventadas.
3. **EQUIVALENCIAS 3.1.d:** el original ya vacía el historial al firmar (`Session::mark_signed`) y al revertir (`Session::revert`), comprobado en `crates/engine/src/lib.rs`. El hueco es solo la censura.
4. **EQUIVALENCIAS D-07:** sin el nombre del repositorio privado de PDForte. `git grep` no lo encuentra en ningún archivo rastreado.
5. **Fallo B de AR0.0:** solo el mensaje del commit de AR0.0 lo daba por defecto real; ninguno de AR0.1 lo hacía (comprobado con `git log`).
- **Tamaño de los archivos de reglas** (límite: 12 000 caracteres):
  - `.agents/rules/00-amaraza.md`: 9 561;
  - `.agents/rules/01-amaraza-heredadas.md`: 4 069;
  - `.claude/rules/amaraza.md`: 223.
- **Puertas** (`LANG=en_US.UTF-8`, `CARGO_BUILD_JOBS=6`): `cargo fmt --check` 0; `cargo clippy --workspace -- -D warnings` 0; `cargo clippy -p amaraza-identity --all-targets -- -D warnings` 0; `cargo xtask assets` ok (235 activos). No se ejecutó la suite completa: solo cambian documentación y comentarios.

## ARS.1. Sincronización con el upstream (2026-10-10)

La hace la sesión revisora (D-13) y la revisa la sesión implementadora.

**Autorización del titular (2026-10-10):** integrar los cambios del upstream sin perder la identidad Amaraza, y aceptar sus dependencias si las licencias son permisivas. No incluye el push ni el merge a `main`.

### Punto de partida

- `main` = `dbfd8249258b790f5f48d000ac1c0ea1d3b84f70`. AR0.2 sigue en su rama, aprobada y sin fusionar.
- `git fetch upstream`: `upstream/main` = `5d590604867fdac7ed71c555c40477ae3331a60a` (`git cat-file -t` → `commit`), con la etiqueta `v0.5.0`. Son 94 commits desde la base común `90bb2f14ad4c60f84d900c1389e289e51a5ce7a6`.
- Worktree aparte, con su propio `target/`. Rama `sync/upstream-2026-10-10` creada desde `main`, y `git merge --no-ff upstream/main` (sin rebase).

### Conflictos (13)

| Archivo | Resolución |
|---|---|
| `.github/workflows/release.yml` | Borrado en el fork y modificado en el upstream: sigue borrado |
| `README.md` | El del fork |
| `ATTRIBUTION.toml` | Las entradas del upstream (catálogos nuevos `it` y `ar`) y los `sha256` recalculados de los 15 catálogos |
| `ATTRIBUTION.md` | Regenerado con `cargo xtask assets --write` |
| `apps/pdfcraft/build.rs` | Recursos de Amaraza PDF más el manifiesto de Windows del upstream |
| `apps/pdfcraft/src/updates.rs` | `User-Agent` `AmarazaPDF/…` más la lectura de los estados HTTP (límite de peticiones de GitHub) |
| `crates/ui-egui/src/chrome.rs` | La barra de pestañas nueva del upstream, con el ancho reservado de los dos botones de la derecha, sin Discord |
| `packaging/windows/{pdfcraft.wxs,package.ps1,test-msi.ps1}`, `packaging/linux/package.sh`, `packaging/linux/flatpak/io.github.pony_svg.AmarazaPDF.bundle.yml`, `packaging/freebsd/package.sh` | La identidad de AR0.1 más los modelos de OCR |

### Identidad en el código nuevo del upstream (sin conflicto)

- **Instancia única** (`apps/pdfcraft/src/single_instance.rs`, nuevo): el socket iba en `%LOCALAPPDATA%\PdfCraft`. Sin el cambio, Amaraza PDF crearía una carpeta de PdfCraft y podría entregar sus archivos a un PdfCraft abierto. Ahora va en la carpeta de Amaraza PDF, con una prueba de regresión.
- **Recursos de la CLI** (`apps/pdfcraft-cli/build.rs`, nuevo): el producto y el fabricante de Amaraza, como en `pdfcraft.exe`.
- **Catálogos** `ar`, `it` y `pt-br`: el mismo cambio que AR0.1 hizo en los otros 12.
- **Pruebas nuevas del upstream** que buscaban la identidad de PdfCraft (`ui`, `pickers`, `viewer` e `i18n`): adaptadas.

### Búsqueda de la marca

`git grep -niE "artcraft|getartcraft|storyteller"`, sin `vendor/`:
- Los mismos 13 archivos que al cerrar AR0.1.
- Uno nuevo, `xtask/src/macos_integration.rs` (2 líneas): `ai.storyteller.pdfcraft` en el plist de ejemplo de una prueba del diagnóstico de macOS del upstream. Es un archivo del upstream que el fork no toca y no se distribuye.
- Ningún nombre de archivo, y ningún catálogo con Discord ni ArtCraft.

### Dependencias

- 12 crates nuevos, ninguno quitado ni actualizado. Licencias leídas con `cargo metadata` (`cargo-deny` no está instalado):
  - `bp256` 0.14.0, `bp384` 0.14.0, `p521` 0.14.0, `ed25519` 3.0.0, `keccak` 0.2.2, `ripemd` 0.2.0, `sha3` 0.12.0 y `sponge-cursor` 0.1.0: MIT o Apache-2.0 (RustCrypto);
  - `curve25519-dalek` 5.0.0 y `ed25519-dalek` 3.0.0: BSD-3-Clause; `curve25519-dalek-derive` 0.1.1: MIT o Apache-2.0;
  - `winsafe` 0.0.29: MIT.
- `winsafe` es una dependencia directa de `apps/pdfcraft`, solo en Windows. Los criptográficos llegan a través de `pdfcraft-sign`.
- No hay APIs de exportación de claves (§5) ni `unsafe` en el código del workspace.

### Puertas (`LANG=en_US.UTF-8`, `CARGO_BUILD_JOBS=6`)

- `cargo fmt --check` 0; `cargo clippy --workspace -- -D warnings` 0 (y `--all-targets` en `pdfcraft-ui-egui`, `pdfcraft` y `pdfcraft-cli`); `cargo xtask assets` ok, con 238 activos.
- `cargo test --workspace --no-fail-fast` (2243 s, con la compilación en frío): 148 binarios, **1847 pasadas, 6 fallidas y 7 ignoradas**.
  - Las 3 que buscaban la identidad de PdfCraft se adaptaron y, repetidas, pasan: lib 197, `pickers` 34, `viewer` 22.
  - `no_tool_escapes_the_root`: el falso positivo de entorno (AR0.0, fallo B).
  - `perf` (`scrolling_a_500_page_document_stays_within_the_frame_budget`, unos 80 ms por fotograma): portátil en batería (del 25 % al 16 %). **Hay que repetirla con el equipo conectado a la corriente** antes de la revisión.
  - `recovery` (`saving_or_closing_clears_the_entry_and_nothing_is_written_for_clean_documents`): la prueba intermitente del Pendiente 9 de AR0.1. Pasa 5 de 5 veces por separado.
  - La ignorada nueva (`a_windows_store_identity_signs_with_its_issuers`) escribe en el almacén de certificados real y no se ejecuta (§6).

### Cambios del upstream que afectan a otros documentos (para la sesión implementadora)

- **EQUIVALENCIAS D-12 y A15:** el original ya tiene instancia única en Windows (`apps/pdfcraft/src/single_instance.rs`).
- **AR0.4:** hay mensajes nuevos que nombran PdfCraft:
  - el aviso "PdfCraft is drawing without a graphics processor…" (`apps/pdfcraft/src/main.rs`);
  - el error "the text recognition models are not installed (reinstall PdfCraft, or set PDFCRAFT_MODELS…)" (`crates/ocr/src/lib.rs`). Lo encontró la revisión de la sesión implementadora.
- La versión pasa a 0.5.0.

### Revisión D-13 (sesión implementadora, 2026-10-10)

- **Aprobada**, sin hallazgos bloqueantes. Puertas en el worktree de sincronización, con el portátil conectado a la corriente: 1852 pasadas, 1 fallida (`no_tool_escapes_the_root`, de entorno) y 7 ignoradas.
- La sesión revisora repitió `cargo test -p pdfcraft-ui-egui --test perf` con corriente: 2 pasadas en 9,8 s.

### Commits

| Commit | Contenido |
|---|---|
| `0664233c2c51aaa482a21ea78118a2f9cd83aacd` | Merge de `upstream/main` (`5d590604867fdac7ed71c555c40477ae3331a60a`) con los conflictos resueltos |
| `fb919ac9afeb1da3d2872b83f5bf01ad153793b2` | Este registro (solo documentación) |
| `e9b9ee622d4dbf9720ef0c62c823ddb3a54decb0` | Merge en `main` de AR0.2 (`fc6a8c993377698bf1db0ac0aa18a0e5196c1657`) y de esta sincronización, autorizado por el titular el 2026-10-10. Único conflicto: el final de este registro, donde se conservan las dos secciones |

## AR0.3. Puntos de enganche para los crates `amaraza-*` (2026-10-10)

### Punto de partida

- `git fetch origin`: `main` == `origin/main` == `e9b9ee622d4dbf9720ef0c62c823ddb3a54decb0` (`git cat-file -t` → `commit`), con AR0.2 y el upstream v0.5.0 (ARS.1) integrados.
- `git config user.email`: el correo del proyecto.
- Rama `ar0.3-enganches`, creada desde `main`. El worktree de la sincronización es del revisor y no se usa.
- `AGENTS.md` y `CLAUDE.md` del upstream no cambiaron con ARS.1.

### Puesta al día tras ARS.1 (primer commit)

- **Registro:** el SHA de `fc6a8c9`, con el dictamen de AR0.2, y el de `e9b9ee6` (el merge en `main` de ARS.1).
- **`00-amaraza.md` §0.3:** las reglas propias van en ese archivo y en `01-amaraza-heredadas.md`.
- **EQUIVALENCIAS:**
  - D-12 pasa a **Original**: desde v0.5.0, el original tiene instancia única en Windows (`apps/pdfcraft/src/single_instance.rs`), y el fork pone el socket en `%LOCALAPPDATA%\AmarazaPDF`.
  - A15 deja de ser una decisión pendiente; sale de la sección 4.
  - D-09 cuenta 15 catálogos (`ar` e `it` son nuevos en v0.5.0).
  - Resumen de las decisiones: 7 original, 5 nuevas, 2 descartadas.
- **`01-amaraza-heredadas.md` A15:** la cubre el original; el código del fork no la reimplementa ni cambia la carpeta del socket.
- **`PLAN.md` AR0.4:** a los 12 mensajes que dicen PdfCraft se suman los 3 nuevos de v0.5.0:
  - el aviso de GPU (`apps/pdfcraft/src/main.rs`);
  - el error de OCR "reinstall PdfCraft…" (`crates/ocr/src/lib.rs`);
  - su aviso en la interfaz (`crates/ui-egui/src/ocr_ui.rs`).

### Commits de AR0.3

| Commit | Contenido | Puertas |
|---|---|---|
| `0f2f47fabfe80692523d9086e6cc4a07cb1a9959` | Puesta al día tras ARS.1 (solo documentación) | En inglés, `CARGO_BUILD_JOBS=6`, a la corriente: fmt, clippy y assets en verde; `clippy -p amaraza-identity --all-targets` en verde. Pruebas: 1851 pasadas, 2 fallidas y 7 ignoradas. Las fallidas: `no_tool_escapes_the_root` (de entorno) y `panels_with_hundreds_of_items_stay_within_the_frame_budget` (`--test perf`). La prueba de rendimiento es intermitente con la máquina cargada: aislada, falló 3 veces y pasó 2, la última con las dos de `perf` en verde. El cambio solo toca documentación. **Dictamen D-13 sobre `0f2f47f`: aprobado.** Puertas del revisor: 1852 pasadas, 1 fallida y 7 ignoradas, como la línea base; `perf` pasó en su ejecución |
| `aef67104cc3600016ffed957b24a822559f08ddd` | ADR-0001, puntos de enganche (solo diseño, para la revisión D-13; ver "ADR-0001 (segundo commit)") | En inglés, `CARGO_BUILD_JOBS=6`, a la corriente: fmt, clippy y assets en verde. Pruebas: 1851 pasadas, 2 fallidas y 7 ignoradas. Las fallidas: `no_tool_escapes_the_root` (de entorno) y `saving_or_closing_clears_the_entry_and_nothing_is_written_for_clean_documents` (`--test recovery`), la intermitente del Pendiente 9 de AR0.1, que aislada pasó 5 de 5 veces. El cambio solo toca documentación. **Dictamen D-13 sobre `aef6710`: aprobado con correcciones menores.** El revisor verificó contra el código las anclas, las cuentas (12 archivos, 43 + 4 líneas), que `CommandSpec` es `Copy`, que `Catalog::parse` no presta, la cobertura i18n por `has()` y `catalog()`, que no hay `match` exhaustivos sobre `Step` fuera de `actions.rs`, y que `check_args`, `Needs` y la guarda actúan antes de los brazos del fork. Puertas del revisor: 1852 pasadas, 1 fallida y 7 ignoradas, como la línea base |
| `49d2fcbf86a5ebcb72a5e1601f03033e4088be9b` | ADR-0001 aceptado: correcciones de la revisión D-13 y decisiones AR-D3 a AR-D5 (solo documentación) | Ver "ADR-0001 aceptado (tercer commit)" |
| `94fdc7e07bb4fa61da6d61f3bbee15117eaa276c` | Fase 1, paso 1: crate `amaraza-ext` (API `hello::greeting` y sus 3 pruebas) y capas (E1, E2). `Cargo.lock` solo añade el paquete `amaraza-ext` | En inglés, `CARGO_BUILD_JOBS=6`, a la corriente: fmt, clippy, `clippy -p amaraza-ext --all-targets`, `xtask layers` (36 crates, sin violaciones) y assets en verde. Pruebas: 1855 pasadas, 1 fallida (`no_tool_escapes_the_root`, de entorno) y 7 ignoradas |
| `dcb0e5d8d42c96041b7b70ae1726cfb1a1fc9f68` | Fase 1, paso 2: declaración del fork en el motor (`crates/engine/src/amaraza.rs`: `COMMANDS` vacío por ahora, `Step::Hello`, `apply`, 3 pruebas) y enganches E3–E9. El comando `amaraza.hello` se declara en el paso 4, con su interfaz y sus traducciones, porque antes fallarían las 13 pruebas de catálogos y `every_registered_command_is_implemented`. rustfmt deja E5 en 3 líneas (el nuevo `COMMANDS` cabe en una), así que la fase 1 queda en 42 insertadas y 4 modificadas. `Cargo.lock` sin cambios | En inglés, `CARGO_BUILD_JOBS=6`, a la corriente: fmt, clippy y assets en verde. Pruebas: 1858 pasadas, 1 fallida (`no_tool_escapes_the_root`, de entorno) y 7 ignoradas |
| `d59bc9f0b8d2655b9444c2f7a9b30dab74aeeeb4` | Fase 1, paso 3: automatización. `crates/automation/src/amaraza.rs` (herramienta `amaraza_hello`, de solo lectura, sin enlace a comando hasta el paso 4) y enganches E10–E13; 5 pruebas de extremo a extremo en `crates/automation/tests/amaraza.rs` (con y sin documento, argumentos inválidos, orden de la tabla, el paso `amaraza_hello` por `action_run`), con raíz temporal que se borra (A11). Título y descripción de la herramienta en inglés, como la tabla del upstream (documentación para agentes); el `text` que devuelve, en español (AR-D5). `Cargo.lock`: solo la arista `pdfcraft-automation` → `amaraza-ext`. CLI: `pdfcraft-cli tools` lista `amaraza_hello` y `run --script` devuelve el saludo | En inglés, `CARGO_BUILD_JOBS=6`, a la corriente: fmt, clippy, `xtask layers` y assets en verde. Pruebas: 1863 pasadas, 1 fallida (`no_tool_escapes_the_root`, de entorno: 15 aceptaciones de `link-out/…`, las mismas que antes del cambio, con 142 herramientas en vez de 141) y 7 ignoradas |
| `70ddcad72a2a241636184232d387e9cb7e30324a` | Fase 1, paso 4: interfaz. Comando `amaraza.hello` declarado en el motor ("Hola, Amaraza", menú "Amaraza", sin atajo, icono `hand`) y enlazado a `amaraza_hello`. `crates/ui-egui/src/amaraza/` (ejecución, menú Amaraza, capa de traducciones `i18n.tsv` con filas "sin traducir" en los 15 idiomas, AR-D4) y enganches E14–E18 (E18: el marcador va en la línea anterior, porque con él la línea pasaba de 150 columnas; queda en 1 insertada y 1 modificada). E19: entrada de `i18n.tsv` en `ATTRIBUTION.toml` y `ATTRIBUTION.md` regenerado (239 activos). Pruebas nuevas: 5 de interfaz con kittest en `crates/ui-egui/tests/amaraza.rs` (comando, menú, paleta, selector de pasos, acciones guardadas), 3 unitarias de la capa de traducciones y 1 de automatización (`command_list` y `command_run`). `Cargo.lock`: solo la arista `pdfcraft-ui-egui` → `amaraza-ext`. **Evidencia sintética:** captura de Menú ▸ Amaraza ▸ "Hola, Amaraza" con `pdfcraft --control` en modo portátil (marcador temporal junto al ejecutable de `target/debug`) y un PDF sintético, fuera del repositorio (política de activos), `sha256` `b6c19c1c3e9e71b762e6d16f8aff9c9a5cad236c2e9e10e3cb80747fef37d6ad` | En inglés, `CARGO_BUILD_JOBS=6`, a la corriente: fmt, clippy, `xtask layers` y assets en verde. Pruebas: 1872 pasadas, 1 fallida (`no_tool_escapes_the_root`, de entorno, las mismas 15 aceptaciones) y 7 ignoradas. Las 13 `*_covers_commands_and_catalogue` y `every_registered_command_is_implemented` pasan con el comando del fork |
| `a88cb3993e98bf95a9e0c14a0708343d0de5a3bd` | Fase 1, paso 5: registro. Una fila por enganche (E1–E20) en "Archivos del upstream modificados", con su línea y su marcador `ADR-0001`, y el resumen de la fase 1 (solo documentación) | Ver "Fase 1 terminada (quinto commit)". **Dictamen D-13 sobre la fase 1 (`49d2fcb..a88cb39`): aprobada.** Puertas del revisor, a la corriente: fmt; clippy del workspace y `--all-targets` de `amaraza-ext`, `amaraza-identity`, engine, automation y ui-egui; `xtask layers` sin violaciones; assets (239); pruebas 1872/1/7 en 152 binarios. CLI comprobada: `amaraza_hello` por `run --script`, con error accionable para un documento inexistente. Enganches: 12 archivos, +47 −4, todos con `ADR-0001`. Observación no bloqueante: comentar por qué se ignora `remove_dir_all` en las pruebas (A10) |
| `1dbfdb5a9c06ce0edf4d6db4859be061f4c7c5ec` | Tras la revisión de la fase 1: comentarios A10 en los dos `remove_dir_all` descartados de `crates/automation/tests/amaraza.rs` (observación no bloqueante del revisor) | En inglés, `CARGO_BUILD_JOBS=6`, a la corriente: fmt, clippy, `xtask layers` y assets (239) en verde; pruebas de amaraza en verde (6 de automatización, 5 de ui-egui). Observación no bloqueante atendida |
| `039a7aaa2c84c29669142ae08dcfea05ac88ef2d` | Integración de `main` (ARS.2 y ARS.3) en `ar0.3-enganches`: resolución de 10 conflictos (7 enganches de código, E19 y este registro), 7 nuevos idiomas en `i18n.tsv` y recalculado sha256 en `ATTRIBUTION.toml` | En inglés, `CARGO_BUILD_JOBS=6`, a la corriente: fmt, clippy, `xtask layers` y assets (251) en verde |
| `8ead39c47626e8e59f05527d0981444648c4f804` | Registro de la integración de `main` en `ar0.3-enganches` (solo documentación) | Solo documentación. **Dictamen D-13 sobre la integración (`1dbfdb5…`, `039a7aa…`, `8ead39c4`): APROBADA; AR0.3 cerrada.** Ver la sección "Revisión D-13 de la integración de main y evidencia del titular" |
| `494f5c020e37366529ee7fd0e8054808a706dfd0` | Cierre de AR0.3: dictamen D-13 y evidencia real del titular (solo documentación) | Solo documentación. AR0.3 integrada en `main` con avance rápido y push autorizados por el titular el 2026-10-19 (`main` = `origin/main` = `494f5c0`) |

### ADR-0001 (segundo commit)

- `docs/amaraza/adr/0001-puntos-de-enganche.md`, solo diseño. La demostración no se implementa hasta que lo apruebe el revisor.
- Cubre, sobre el código de v0.5.0:
  - cómo registra un crate `amaraza-*` comandos, herramientas, menús y capas;
  - los pasos del asistente de acciones como enganche, con el diseño del paso "Firmar" por lote (A5, A6 y RF-FIRMA-01);
  - la demostración "hola Amaraza" como plan de verificación;
  - las alternativas, los riesgos y cómo se revierte.
- **Fase 1** (la demostración): 12 archivos del upstream, con 43 líneas insertadas y 4 modificadas, más `ATTRIBUTION.*` y `Cargo.lock`. La tabla línea a línea está en el §3 del ADR. Se copiará a "Archivos del upstream modificados" cuando se implemente.
- Se comprobaron en archivos fuera del repositorio, con rustc 1.99:
  - la concatenación `const` de `COMMANDS`: compila, clippy (`all` y `pedantic`) no da avisos y un `N` erróneo detiene la compilación;
  - la guarda `if let` antes del `match` de `Step::arg_mut`: el préstamo `&mut` compila y su prueba pasa.
- **Decisiones que se piden al titular** (§10 del ADR):
  - crear `crates/amaraza-ext`, con dependencias internas (`Cargo.lock` cambia solo con el paquete interno);
  - la política de las filas "sin traducir" en la capa de idiomas del fork;
  - para la fase 2: `pdfcraft-sign` y, si AR1.2 lo exige, `rustls-cng`.

### ADR-0001 aceptado (tercer commit)

- **Decisiones del titular (2026-10-10)** en `PLAN.md`:
  - AR-D3: se crea `crates/amaraza-ext`;
  - AR-D4: los textos del fork, en español en los 14 idiomas restantes;
  - AR-D5: identificadores del código en inglés.

  La fase 2 se decide al empezarla.
- **Correcciones del ADR:**
  1. **A6 en parte** (§4.5 y fila A6 de §6): `identities()` del upstream abre la clave en modo silencioso al listar (`crates/sign/src/windows.rs:80`). Cumplir A6 del todo exige listar sin abrirla, y se decide en AR1.2.
  2. **A7:** excepción explícita para los módulos de pegamento dentro de crates del upstream (umbral 25 de la raíz; clippy no admite un umbral por módulo), mitigada sin lógica con ramas en ellos.
  3. **Riesgos:**
     - R4 se amplía con `misc.action-wizard-manage`: la importación y exportación de acciones perdería los pasos del fork;
     - R9, nuevo: `misc.action-prompt-steps` puede solaparse con F1.
  4. **Identificadores en inglés:** `amaraza::Step`, `Sign`, `amaraza_ext::batch` (`Plan`, `Signer`, `FileOutcome`, `BatchError`, `plan`, `run`), `hello::greeting`, `amaraza.hello`, `amaraza_hello`, `amaraza_sign` y `amaraza_sign_batch`. §10 recoge AR-D3 a AR-D5.
- **Estado del ADR:** aceptado.
- **Puertas** (en inglés, `CARGO_BUILD_JOBS=6`, a la corriente): fmt, clippy y assets en verde. Pruebas: 1852 pasadas, 1 fallida (`no_tool_escapes_the_root`, de entorno) y 7 ignoradas, como la línea base.

### Fase 1 terminada (quinto commit)

- **Enganches en "Archivos del upstream modificados":** una fila por enganche, de E1 a E20. `git grep -n "ADR-0001"` los lista todos, salvo `Cargo.lock`, que es generado.
- **Diff contra `e9b9ee6`:** 12 archivos del upstream con 43 líneas insertadas y 4 modificadas, más `ATTRIBUTION.*` y `Cargo.lock`, como preveía el ADR. E5 quedó en 3 líneas y E18 ganó 1.
- **Código nuevo del fork:**
  - `crates/amaraza-ext`;
  - `crates/engine/src/amaraza.rs`;
  - `crates/automation/src/amaraza.rs` y `crates/automation/tests/amaraza.rs`;
  - `crates/ui-egui/src/amaraza/{mod.rs, i18n.tsv}` y `crates/ui-egui/tests/amaraza.rs`.
- **Pruebas nuevas:** 20. Por crate: 3 de `amaraza-ext`, 3 del motor, 6 de automatización y 8 de interfaz (5 con kittest y 3 de la capa de traducciones). La línea base pasa de 1852 a 1872 pasadas.
- **Evidencia sintética:** la captura de Menú ▸ Amaraza ▸ "Hola, Amaraza" (paso 4), fuera del repositorio.
- **Sin cambios:** `parity/`, `ROADMAP.md`, `AGENTS.md` y `CLAUDE.md`.
- **Puertas de este commit** (en inglés, `CARGO_BUILD_JOBS=6`, a la corriente): fmt, clippy y assets en verde. Pruebas: 1871 pasadas, 2 fallidas y 7 ignoradas. Las fallidas: `no_tool_escapes_the_root` (de entorno) y `saving_or_closing_clears_the_entry_and_nothing_is_written_for_clean_documents` (`--test recovery`), la intermitente del Pendiente 9 de AR0.1, que aislada pasó 5 de 5 veces. El commit solo toca documentación; con el mismo código, el paso 4 dio 1872 pasadas y 1 fallida.

### Integración de main tras ARS.2 y ARS.3 (primer merge del diseño)

- `git merge main` (sin rebase).
- 10 conflictos resueltos según ADR-0001 §3:
  - 7 enganches de código: E3 (`crates/engine/src/lib.rs`), E10 (`crates/automation/Cargo.toml`), E11 (`crates/automation/src/lib.rs`), E14 (`crates/ui-egui/Cargo.toml`), E15 (`crates/ui-egui/src/lib.rs`), E16 (`crates/ui-egui/src/commands.rs`) y E17 (`crates/ui-egui/src/chrome.rs`).
  - 12 enganches fusionaron sin conflicto automáticamente: E1, E2, E4, E5, E6, E7, E8, E9, E12, E13, E18, E20.
  - E19 (`ATTRIBUTION.toml`) y regenerado `ATTRIBUTION.md` (251 activos).
  - Este registro (`docs/amaraza/REGISTRO.md`).
- 7 idiomas nuevos de v0.8.0/v0.9.0 añadidos a `crates/ui-egui/src/amaraza/i18n.tsv` con filas "sin traducir" en español (AR-D4): `el`, `kk`, `lt`, `pl`, `sl`, `tr` y `vi`. Su sha256 recalculado en `ATTRIBUTION.toml`.
- Todos los enganches conservan su marcador `ADR-0001` (`git grep -n "ADR-0001"`).
- Puertas completas tras la integración: `cargo fmt --check` (0), `cargo clippy --workspace -- -D warnings` (0), `cargo clippy -p amaraza-identity -p amaraza-ext --all-targets -- -D warnings` (0), `cargo xtask assets` (251 activos ok), `cargo xtask layers` (39 crates ok). Pruebas del fork: `amaraza-identity` (6 de 6), `amaraza-ext` (3 de 3), `pdfcraft-engine -- amaraza` (4 de 4), `pdfcraft-automation --test amaraza` (6 de 6), `pdfcraft-ui-egui --lib amaraza` (5 de 5) y `pdfcraft-ui-egui --test amaraza` (5 de 5). Pruebas del motor (`engine --lib`: 286 de 286) y automatización (`automation --lib`: 36 de 36).
- La evidencia real de interfaz del titular (menú Amaraza ▸ "Hola, Amaraza") se toma sobre esta rama ya integrada (ver la sección siguiente).

### Revisión D-13 de la integración de main y evidencia del titular (AR0.3 cerrada)

- **Dictamen D-13 sobre la integración de `main` en `ar0.3-enganches` (`1dbfdb5…`, `039a7aa…`, `8ead39c47626e8e59f05527d0981444648c4f804`): APROBADA.**
- **Verificado por la sesión revisora:**
  - El merge integra `a78c2d199b917e5a9948ad57bf2ab169ce663dd9` (`main`, con ARS.2 y ARS.3) sin rebase.
  - Frente a `main`, los 12 archivos de enganche difieren en exactamente +47/−4 líneas (nada del upstream perdido).
  - `i18n.tsv` con 22×3 filas y sha256 correcto (`c2454977059e8fe9becfbaa8e39e84a0af0ca0828ede7ef3201883e5d8a59f79`).
  - `Cargo.lock` solo añade el paquete `amaraza-ext`.
- **Puertas completas del revisor (a la corriente):**
  - `cargo fmt --check`: 0.
  - `cargo clippy --workspace -- -D warnings` y `clippy --all-targets` de crates del fork y enganches: 0.
  - `cargo xtask layers`: 39 crates, sin violaciones.
  - `cargo xtask assets`: 251 activos ok.
  - `cargo test --workspace`: 177 binarios, **3796 pasadas, 0 fallidas y 16 ignoradas**. (En el fork las puertas de calidad incluyen `cargo test --workspace` completo, conforme a `00-amaraza.md` §7.4; la norma nueva del `CLAUDE.md` del upstream de limitar las pruebas a los crates tocados no la sustituye).
- **Evidencia real del titular (2026-10-19, app real en Windows, sobre esta rama):**
  - Ventana "… — Amaraza PDF" con el icono de Amaraza en el título y la barra de tareas.
  - Barra de título sin botón de Discord.
  - Interfaz en español.
  - El comando "Hola, Amaraza" muestra el aviso "Hola, Amaraza PDF 0.9.0 · 15 páginas" con un PDF del titular de 15 páginas abierto.
  - *Privacidad (`00-amaraza.md` §5):* la evidencia se registra estrictamente en texto; no se sube la captura de pantalla ni se cita el nombre, ruta ni contenido del documento del titular.
- **Dictamen D-13 final: AR0.3 APROBADA y CERRADA.**
- Pendiente: el revisor hará el avance rápido de `main` y el push a `origin`, con autorización expresa del titular.

## ARS.2. Sincronización con el upstream hasta v0.8.0 (2026-10-14 a 2026-10-19)

La hace la sesión revisora (D-13) y la revisa la sesión implementadora. **Autorización del titular (2026-10-14):** "haz el update del repo original", conservando la identidad Amaraza; dependencias aceptadas si su licencia es permisiva. No incluye el push ni el merge a `main`.

### Punto de partida

- `main` = `e9b9ee622d4dbf9720ef0c62c823ddb3a54decb0`. AR0.3 sigue en su rama (`ar0.3-enganches`), pendiente de la evidencia de interfaz del titular, así que sus enganches se adaptan cuando la implementadora integre `main`.
- `upstream/main` = `9931329b9fae0b05373b4dc3ac7c58541f0891a4` (`git cat-file -t` → `commit`), la etiqueta `v0.8.0`: 373 commits desde `5d590604867fdac7ed71c555c40477ae3331a60a` (v0.6.0, v0.7.1 y v0.8.0).
- Rama `sync/upstream-2026-10-14` desde `main`, en el worktree de sincronización; `git merge --no-ff upstream/main`.
- El 2026-10-14 la red bloqueaba `static.crates.io`; la compilación y las puertas se hicieron el 2026-10-19, con otra red.

### Conflictos (22) e identidad

- Borrados del fork que siguen borrados: `.github/workflows/release.yml`, `assets/app-icon/README.md`, `packaging/icons.sh` y `packaging/macos/dmg/README.md`. README del fork.
- CI: se conservan los flujos del fork. El upstream pasa a flujos reutilizables `storytold/craft-common-actions@main`, sin versión fija y con su identidad.
- Detalle por archivo: filas ARS.2 de "Archivos del upstream modificados".
- **Identidad que entraba sin conflicto**:
  - la carpeta del portapapeles (`image_pick.rs`, nuevo) era `%LOCALAPPDATA%\PdfCraft\Clipboard` y borra archivos en ella;
  - el componente nuevo del menú contextual del instalador traía el GUID del upstream;
  - `StartupWMClass` del `.desktop` llegaba con `ai.storyteller.pdfcraft` por la detección de renombrados;
  - `cs.tsv` recuperó las entradas de Discord, y llegaron 6 catálogos nuevos sin tratar (`el`, `kk`, `lt`, `pl`, `sl` y `tr`);
  - pruebas nuevas del upstream buscaban el botón de Discord o el `.desktop` de PdfCraft.
- **Icono de documento PDF:** se conserva el genérico del upstream (`assets/app-icon/pdfcraft-document.*`: contorno de Lucide con la etiqueta "PDF", ISC y MIT o Apache-2.0) para las asociaciones de archivos. No es una marca; un icono de documento con la paleta de Amaraza queda como decisión del titular.

### Búsqueda de la marca

`git grep -niE "artcraft|getartcraft|storyteller"`, sin `vendor/`: los mismos archivos que tras ARS.1 (incluido el ejemplo de `xtask/src/macos_integration.rs`), ninguno nuevo; ningún nombre de archivo; ningún catálogo con Discord ni ArtCraft.

### Dependencias

- **Repositorios git nuevos de storytold**, fijados por commit: `craft-libs` (`craft-ui` y `craft-print`, MIT o Apache-2.0, sin marcas ni activos incrustados) y `winprint.rs` (BSD-3-Clause).
- **`winprint`** puede descargar binarios de PDFium (de `bblanchon/pdfium-binaries`, verificados con sigstore) en su script de compilación, pero solo con su característica `pdfium`, que `craft-print` apaga (`default-features = false`). Sus dependencias de compilación (`reqwest`, `quinn`, `tokio`, `sigstore-*`, `tar`) se compilan igualmente; no se descarga nada.
- **Licencias de los crates nuevos** (leídas con `cargo metadata`): MIT, Apache-2.0, BSD-3-Clause, ISC o combinaciones, salvo `webpki-root-certs` (CDLA-Permissive-2.0, datos de certificados raíz), que solo entra en el script de compilación de `winprint` y nunca en el programa (excepción documentada en `deny.toml` del upstream).
- `Cargo.lock`: `amaraza-identity` pasa a 0.8.0.

### Puertas (`LANG=en_US.UTF-8`, `CARGO_BUILD_JOBS=6`)

- `cargo fmt --check` 0; `cargo clippy --workspace -- -D warnings` 0 (267 s); `cargo xtask layers` sin violaciones (36 crates); `cargo xtask assets` ok (248 activos).
- `cargo test --workspace --no-fail-fast` (1578 s, con la compilación en frío): 155 binarios, **3057 pasadas, 1 fallida y 15 ignoradas**. La fallida era la guarda `the_windows_installer_is_amarazas` de `amaraza-identity`, que prohibía cualquier `app-icon` en `package.ps1`; se limitó al icono de PdfCraft y pasa (6 de 6). `no_tool_escapes_the_root` y las dos pruebas de rendimiento pasan.

### Pendiente para la sesión implementadora

- **AR0.3:** al integrar `main`, el upstream cambió 11 de los 12 archivos de enganche, y la capa de traducciones del fork necesita filas para `el`, `kk`, `lt`, `pl`, `sl` y `tr`.
- **AR0.4:** v0.8.0 trae muchos mensajes nuevos que nombran PdfCraft, el nombre de fuente "PdfCraft Shaped Text" (`crates/edit/src/added.rs`) y el nombre por defecto "PdfCraft" de las imágenes copiadas (`image_pick.rs`).

### Commits

| Commit | Contenido |
|---|---|
| `a8d513fe5196a79ba7e0064175104ec7858bf1c1` | Merge de `upstream/main` (`9931329b9fae0b05373b4dc3ac7c58541f0891a4`, v0.8.0) con los conflictos resueltos |
| `75b4d6b0b41bcef8ab7e8ba63e37b0a20b3ea7fe` | Este registro (solo documentación) |

## ARS.3. Sincronización con el upstream hasta v0.9.0 + 205 commits (2026-10-19)

Mismo encargo y autorización que ARS.2: el upstream siguió avanzando mientras ARS.2 esperaba la red, así que se hizo en un segundo paso sobre ARS.2 ya en verde. La revisa la sesión implementadora. Sin push ni merge a `main`.

### Punto de partida

- Base: el registro de ARS.2, `75b4d6b0b41bcef8ab7e8ba63e37b0a20b3ea7fe`.
- `upstream/main` = `43550c448db3ba91753f638e3843c0da036bbd60` (`git cat-file -t` → `commit`): la etiqueta `v0.9.0` (`645016ad`, 2026-10-17) más 205 commits; 454 commits desde v0.8.0. *Corregido en la revisión D-13: la primera versión de esta sección y los mensajes de los commits `4d7ccc92` y `1c733c3d` lo llamaban "v0.9.0"; los mensajes no se reescriben.*
- Rama `sync/upstream-2026-10-19`, en el worktree de sincronización; `git merge --no-ff`. La lista de conflictos se previó antes con `git merge-tree` sobre un commit temporal sin referencia.

### Conflictos (25) e identidad

- **Catálogos (14):** el upstream insertó una línea junto a "Welcome to PdfCraft", que el fork había renombrado. Se tomó el lado del upstream y se volvió a pasar el tratamiento de marca de AR0.1 a los 22 catálogos (`vi` es nuevo).
- **Instalador:** el upstream reorganizó los menús contextuales (componente propio para el de imágenes, conservación de la elección del usuario al actualizar) y añadió un `<Upgrade>` de solo detección con **su** `UpgradeCode`. En lugar de resolver bloque a bloque, `pdfcraft.wxs` y `test-msi.ps1` se reconstruyeron desde los del upstream aplicando la lista cerrada de sustituciones del fork, cada una con su recuento comprobado (nombres, fabricante, `UpgradeCode`, GUID de componentes, ProgId, claves, textos, URL, accesos directos). Resultado: sin restos de PdfCraft, Learning Machines, getartcraft, Discord ni GUID del upstream, salvo los Id internos de WiX.
- **Identidad que entraba sin conflicto:**
  - `settings.rs` (nuevo) guardaba y migraba las preferencias en `~/.config/pdfcraft`;
  - la instancia única de Linux (nueva) ponía el socket en `$XDG_RUNTIME_DIR/pdfcraft`;
  - `main.rs` volvía a traer `migrate_legacy_folders`, `eframe::storage_dir("PdfCraft")` para los registros y el menú de macOS "PdfCraft";
  - `Credits.rtf` (macOS) enlazaba a getartcraft.com y al Discord de ArtCraft.
- `README.md`: el del fork. `links.rs`: pruebas nuevas del upstream y la del fork. `docs/development.md`: carpetas de Amaraza y sin la migración de PrintCraft.

### Búsqueda de la marca

Los mismos archivos que tras ARS.2, más `packaging/macos/Credits.rtf`, que contiene solo la atribución en texto plano ("Basado en PdfCraft, del equipo ArtCraft"). Ningún nombre de archivo; ningún catálogo con Discord ni ArtCraft.

### Dependencias

- Crates nuevos: `craft-docpath`, `craft-quit` (de `craft-libs`), `crossbeam-channel`, `fontconfig-parser`, `keyboard-types` y `muda`, todos MIT o Apache-2.0. Ninguno quitado.
- De la revisión nueva `82a24b0` de `craft-libs` solo se usa `craft-docpath`; sus crates `ai-core` (con valores de ArtCraft en pruebas) y `cms` (con perfiles ICC incrustados) no forman parte del grafo.
- `craft-print` sigue usando `winprint` con `default-features = false`: no se descarga PDFium.

### Puertas (`LANG=en_US.UTF-8`, `CARGO_BUILD_JOBS=6`, a la corriente)

- `cargo fmt --check` 0; `cargo clippy --workspace -- -D warnings` 0; `cargo clippy -p amaraza-identity --all-targets -- -D warnings` 0; `cargo xtask layers` sin violaciones (38 crates); `cargo xtask assets` ok (250 activos).
- `cargo test --workspace --no-fail-fast` (1165 s): 173 binarios, **3776 pasadas, 0 fallidas y 16 ignoradas**. Pasan las guardas del fork (instalador, empaquetado, socket, portapapeles, `.desktop`), `no_tool_escapes_the_root` y las dos de rendimiento.
- Una primera ejecución detectó una llave de más en `main.rs` (resolución del conflicto de `settings_dir`); se corrigió antes del commit.

### Cambios del upstream que afectan al fork (para la sesión implementadora)

- `CLAUDE.md` del upstream añade normas: demostrar que una prueba falla revirtiendo el arreglo, revisar todos los bloques fusionados al integrar, y limitar compilaciones y pruebas a los crates tocados (`-p <crate> -j 2`). En el fork siguen mandando las puertas completas de `00-amaraza.md` §7.
- **AR0.3** (al integrar `main`): el upstream tocó 11 de los 12 archivos de enganche; la capa de traducciones del fork necesita filas para `el`, `kk`, `lt`, `pl`, `sl`, `tr` y `vi`.
- **AR0.4:** la lista de mensajes que nombran PdfCraft sigue creciendo.

### Commits

| Commit | Contenido |
|---|---|
| `4d7ccc92872894f84958694cf37d7508e15fd041` | Merge de `upstream/main` (`43550c448db3ba91753f638e3843c0da036bbd60`, v0.9.0 + 205 commits) con los conflictos resueltos |
| `1c733c3d8121ffbe3bdcd45ddd94884bc204fa7d` | Este registro (solo documentación) |
| `a78c2d199b917e5a9948ad57bf2ab169ce663dd9` | Correcciones de la revisión D-13 (ver la sección siguiente) |

### Revisión D-13 de ARS.2 y ARS.3 (sesión implementadora, 2026-10-19)

- **Aprobadas con correcciones**, ninguna bloqueante. Comprobaciones estáticas correctas: GUID del instalador y `<Upgrade>`, carpetas del portapapeles, de las preferencias y del socket, `Credits.rtf`, catálogos, búsqueda de la marca y licencias.
- Puertas de la implementadora en el worktree de sincronización, a la corriente: 3773 pasadas, 3 fallidas y 16 ignoradas. Las fallidas (las dos de `--test perf` y `attachment_save_preserves_bytes_metadata_and_selected_destination`) pasan por separado (5 de 5 y 3 de 3). En la ejecución de la sesión revisora pasaron las 3776.
- Hallazgos, todos confirmados por la sesión revisora contra el repositorio y corregidos en el commit siguiente:
  - `.github/workflows/freebsd.yml` pasó sin conflicto (ya en ARS.2) a `storytold/craft-common-actions@main`, aunque el registro decía que se conservaban los flujos del fork: vuelve la versión del fork;
  - `.github/dependabot.yml` (nuevo, diario): borrado;
  - la plantilla de PR decía "PdfCraft";
  - ARS.3 llamaba "v0.9.0" a `43550c4`, que es v0.9.0 + 205 commits: corregido en este registro (los mensajes de commit no se reescriben);
  - la cabecera de `apps/pdfcraft/src/single_instance.rs` mostraba las rutas de PdfCraft.

## ARS.4. Sincronización con el upstream (2026-10-19)

La hace la sesión revisora (D-13) y la revisa la sesión implementadora. **Autorización del titular (2026-10-19):** empezar ARS.4 antes de implementar AR0.4, con los arreglos de `main` que señaló la segunda revisión independiente. No incluye el push ni el merge a `main`.

### Punto de partida

- `main` = `origin/main` = `494f5c020e37366529ee7fd0e8054808a706dfd0` (AR0.3 cerrada).
- `upstream/main` = `ec0a965074f376a5bf610435948e4e5184bf3299` (`git cat-file -t` → `commit`): 21 commits desde `43550c448db3ba91753f638e3843c0da036bbd60`.
- Rama `sync/upstream-2026-10-19-ars4` desde `main`, en el worktree de sincronización; `git merge --no-ff`.

### Primera sincronización con los enganches de AR0.3 en `main`

- **0 conflictos.** El upstream tocó 3 archivos de enganche (+13 −5 líneas en total) sin chocar con los enganches; los 16 archivos con el marcador `ADR-0001` siguen intactos.
- **Sin identidad nueva del upstream:** ningún literal nuevo con PdfCraft, Discord ni ArtCraft en el código; catálogos sin marca; sin cambios en dependencias (`Cargo.lock`), flujos de CI ni empaquetado.

### Arreglos de `main` (segunda revisión independiente)

- `packaging/macos/Info.plist.in`: el icono del tipo de documento PDF (`CFBundleTypeIconFile`) pasa de `PdfCraft` a `AmarazaPDF`. Había entrado sin conflicto en ARS.2 y la sesión revisora no lo detectó.
- `.github/workflows/nix.yml`: anotado en la tabla; se conserva, inerte mientras Actions siga desactivado.
- Tabla "Archivos del upstream modificados": `crates/ui-egui/tests/links.rs` pasa a `crates/ui-egui/tests/ui_group/links.rs`, y una nota explica que los números de línea son orientativos y que los enganches se localizan por el marcador `ADR-0001` y su ancla de contenido.

### Puertas (`LANG=en_US.UTF-8`, `CARGO_BUILD_JOBS=6`, a la corriente)

- Sobre el merge: `cargo fmt --check` 0; `cargo clippy --workspace -- -D warnings` 0; `cargo clippy -p amaraza-ext -p amaraza-identity --all-targets -- -D warnings` 0; `cargo xtask layers` sin violaciones (39 crates); `cargo xtask assets` ok (251 activos).
- `cargo test --workspace --no-fail-fast` (1303 s): 177 binarios, **3846 pasadas, 0 fallidas y 16 ignoradas**.

### Commits

| Commit | Contenido |
|---|---|
| `0ec9ea9a3534b87496b2e067444b21895ed6a46d` | Merge de `upstream/main` (`ec0a965074f376a5bf610435948e4e5184bf3299`), sin conflictos |
| `4e26c44f9e7db8b7363f7a370b5cf4071a151d75` | Arreglos de `main` y este registro |

### Revisión D-13 de ARS.4 (sesión implementadora y segundo revisor, 2026-10-19)

- **Dictámenes D-13:** APROBADA por ambos revisores (el implementador y el segundo revisor independiente), sin hallazgos. Verificados: `git show --stat` de `0ec9ea9` y `4e26c44`; los 16 archivos con `ADR-0001` intactos; `CFBundleTypeIconFile` corregido a `AmarazaPDF` en `packaging/macos/Info.plist.in`; sin marcas prohibidas ni literales de usuario con "PdfCraft" en el código nuevo del upstream; `Cargo.lock` idéntico a `main` (0 cambios).
- **Puertas de la implementadora (a la corriente):** fmt 0, clippy 0, layers ok (39 crates), assets ok (251), `cargo test --workspace` (177 binarios, 3846 pasadas, 0 fallidas, 16 ignoradas).
- **Push a origin/main:** autorizado por el titular el 2026-10-19 con avance rápido (`main` = `origin/main` = `4e26c44f9e7db8b7363f7a370b5cf4071a151d75`).

## AR0.4. Identidad en metadatos y textos (2026-10-19)

### Punto de partida

- `main` = `origin/main` = `4e26c44f9e7db8b7363f7a370b5cf4071a151d75` (`git cat-file -t` → `commit`), con ARS.4 cerrada e integrada con avance rápido y push autorizados por el titular.
- Rama `ar0.4-identidad`, integrada con `main` mediante `git merge --no-ff`.
- `git config user.email`: el correo del proyecto.

### Commits de AR0.4

| Commit | Contenido | Puertas |
|---|---|---|
| `39d3457f62ae0f4a07169e3e13a9a28d4659edf4` | ADR-0002: diseño de identidad en metadatos y textos (solo documentación, para revisión D-13) | En inglés, `CARGO_BUILD_JOBS=6`, a la corriente: fmt, clippy, `xtask layers` y assets (251) en verde |
| `87cf27683ae012812e6330f7fb26c07782eb808a` | ADR-0002: correcciones de la primera revisión D-13 (bloqueantes y no bloqueantes atendidos; solo documentación) | En inglés, `CARGO_BUILD_JOBS=6`, a la corriente: fmt, clippy, `xtask layers` y assets (251) en verde |
| `df963731413658f44f495948ea135c605cb8f681` | ADR-0002: segunda corrección (decisiones AR-D6 a AR-D8, dinamismo sin lista fija, guarda en código de UI, enlace en Acerca de y retiro de help.upstream; solo documentación) | En inglés, `CARGO_BUILD_JOBS=6`, a la corriente: fmt, clippy, `xtask layers` y assets (251) en verde |
| `0036048ad4c57f23efd5367e6a415a781986a81e` | Integración de `main` (ARS.4) en `ar0.4-identidad` con resolución de conflicto en REGISTRO.md | En inglés, `CARGO_BUILD_JOBS=6`, a la corriente: fmt, clippy, `xtask layers` (39) y assets (251) en verde |
| `1134a56a3af4ec3b68745b01962f38a87de7a7e1` | AR0.4: Identidad en metadatos y textos (ADR-0002 implementado: 8 metadatos desde `amaraza-identity`, traducciones dinámicas en memoria vía E18, retiro de `help.upstream` con enlace diferido en Acerca de, borrado de 12 capturas obsoletas y guardas contra regresiones) | En inglés, `CARGO_BUILD_JOBS=6`, a la corriente: fmt 0, clippy workspace 0, clippy fork crates 0, `xtask layers` (39 crates) ok, `xtask assets` (239 activos) ok, `cargo test --workspace`: 177 binarios, 3852 pasadas, 0 fallidas, 16 ignoradas. **Dictamen D-13: APROBADO CON CORRECCIONES** |
| *(siguiente)* | AR0.4: correcciones de la revisión D-13 (prueba de fuente moldeada con escrituras mixtas y demostración en rojo, pruebas extraídas a `crates/{create,sign}/tests/amaraza.rs` y `crates/edit/src/added/amaraza_tests.rs`, escáner de UI reforzado con lista de excepciones, ADR-0002 aceptado y nota sobre screenshots) | En inglés, `CARGO_BUILD_JOBS=6`, a la corriente |

### ADR-0002 (primer commit: 39d3457f62ae0f4a07169e3e13a9a28d4659edf4)

- `docs/amaraza/adr/0002-identidad-metadatos-y-textos.md`, solo diseño. No se implementa hasta la aprobación del titular y del revisor D-13.
- Cubre:
  1. **Metadatos desde `amaraza-identity` (L0 standalone):** los 7 puntos exactos identificados en el código:
     - `Producer` de los PDF creados (`crates/create/src/lib.rs:54`);
     - nombre de la app en firmas digitales (`crates/sign/src/pdf.rs:1835` y `:2008`);
     - autor por defecto de comentarios (`crates/automation/src/comments.rs:15`, `DEFAULT_AUTHOR`);
     - título de los trabajos de impresión en la web (`crates/ui-egui/src/web_print.rs:131`);
     - nombre y título del servidor MCP (`crates/automation/src/mcp.rs:134`);
     - nombre de familia de la fuente moldeada (`crates/edit/src/added.rs:711`);
     - nombre por defecto de recortes de imagen del portapapeles (`crates/ui-egui/src/image_pick.rs:77`).
     Total en código del upstream para metadatos: 8 líneas modificadas en 7 archivos, con dependencias a L0.
  2. **Mensajes de la interfaz (mecanismo con pocos enganches):**
     - Inventario: 23 cadenas fuente únicas en inglés que nombran "PdfCraft" y 457 líneas traducidas en los 22 catálogos TSV.
     - Mecanismo: aprovechar el enganche E18 existente (`catalog_source` en `crates/ui-egui/src/amaraza/mod.rs`):
       - para los 22 idiomas con catálogo (`code != "en"`): sustitución en memoria de `"PdfCraft"` por `"Amaraza PDF"` en la columna de traducción (`cols[2]`), preservando la clave fuente en inglés (`cols[1]`) y las atribuciones autorizadas (`"Basado en PdfCraft"`). Las llamadas `tl!(...)` del upstream siguen encontrando la clave sin modificar una sola línea de los 22 catálogos TSV;
       - para inglés (`code == "en"`): inyectar las 23 filas que mapean cada clave fuente a su versión con "Amaraza PDF" para que `Catalog::parse` pueble `c.plain` del inglés.
     - Cero líneas modificadas en los 22 catálogos `.tsv` y cero líneas adicionales en archivos `.rs` del upstream para los textos de la interfaz. Riesgo de conflicto en futuras sincronizaciones: cero.
  3. **Borrado de activos obsoletos:**
     - 12 capturas de `docs/images/` que muestran la marca ArtCraft y botones de Discord, junto con sus 12 bloques `[[asset]]` en `ATTRIBUTION.toml`.
  4. **Guardas automáticas (pruebas):**
     - Pruebas sintéticas que fallen si vuelve "PdfCraft" a los metadatos generados (`Producer`, firmas, comentarios, fuentes moldeadas, MCP) o a los textos de la interfaz.

### Primera revisión D-13 del ADR-0002 y correcciones aplicadas (87cf27683ae012812e6330f7fb26c07782eb808a)

- **Dictamen de la revisión:** APROBADO CON CORRECCIONES.
- **Correcciones bloqueantes integradas en ADR-0002:**
  1. **Dependencia en `crates/automation`:** se declara en §2.2 y §6 la adición de `amaraza-identity = { path = "../amaraza-identity" }` en `crates/automation/Cargo.toml` (L7 → L0) y la arista correspondiente en `Cargo.lock`.
  2. **Adaptación de pruebas existentes:** declaradas en §5.1 y computadas en §6 las 6 pruebas que romperían con el cambio (`ui.rs:114`, `updates.rs:58, 102`, `signing.rs:536`, `automation.rs:1300`, `image_pick.rs:251` y `amaraza/mod.rs:91`).
  3. **Regla de sustitución y respaldo en `catalog_source`:**
     - Excepción por clave de origen: `cols[1] == "PdfCraft on GitHub"`.
     - Respaldo para las 7 claves incompletas en el upstream: inyección automática de filas de respaldo sin duplicados.
     - La prueba de guarda de §5.2 verifica `tr(lang, clave)` en todos los idiomas del catálogo, no solo las entradas cargadas.
  4. **Alcance y delimitación:**
     - Se incluyen las metaherramientas MCP en `crates/automation/src/mcp.rs` (l. 206, 207, 220, 224, 225, 236: títulos, descripciones y anotaciones).
     - Se documenta que el texto de `OcrError::NoModels` (`crates/ocr/src/lib.rs:33`) es la clave exacta de los catálogos de la UI y no se modifica en `crates/ocr`.
     - Se declaran los errores internos en inglés del motor y de la CLI fuera del alcance de la prueba de concepto.
- **Correcciones no bloqueantes integradas:**
  - Etiqueta sRGB de perfil ICC PDF/A: `sRGB (Amaraza PDF)` en `crates/preflight/src/icc.rs:51`.
  - La prueba de guarda de `/Producer` usa `pdfcraft_cos` (no `lopdf`).
  - `websiteUrl` retirado de la tabla de cambios porque `links::GITHUB` ya apunta a `amaraza_identity::REPOSITORY`.
  - Unificación de nombre para recortes de imagen del portapapeles a `FOLDER_NAME` (`"AmarazaPDF"`).
  - Añadida prueba de guarda para `packaging/macos/Info.plist.in` (`CFBundleTypeIconFile`).

### Segunda revisión D-13 del ADR-0002 y decisiones del titular AR-D6 a AR-D8 (2026-10-19)

- **Dictamen de la revisión:** APROBADO CON CORRECCIONES.
- **Decisiones formales del titular adoptadas el 2026-10-19 (incorporadas en PLAN.md y ADR-0002 §8):**
  - **AR-D6:** mecanismo en memoria vía E18 aprobado para sustitución en traducciones, aceptando la limitación en idiomas con declinación (hu, sl, tr…); en español e inglés queda bien.
  - **AR-D7:** servidor MCP `amarazapdf` (título `Amaraza PDF`), fuente `Amaraza PDF Shaped Text` y perfil ICC `sRGB (Amaraza PDF)`.
  - **AR-D8:** se QUITA "PdfCraft en GitHub" (`help.upstream`) del menú Ayuda y de la paleta. Restricción (`00-amaraza.md` §1): "Acerca de" debe seguir mostrando "Basado en PdfCraft, del equipo ArtCraft" CON enlace a `https://github.com/storytold/pdfcraft`. En "Acerca de" (`dialogs.rs:1175`) la propia frase de atribución `BASED_ON` se presenta como un enlace interactivo directo (`Link`) que al pulsar abre `links::UPSTREAM` mediante `app.open_url`. README y NOTICE no cambian. Al no existir el comando en menús ni paleta, la excepción por clave `"PdfCraft on GitHub"` en `catalog_source` **desaparece por completo**.
- **Correcciones bloqueantes integradas en ADR-0002:**
  1. **Integridad:** formalización de AR-D6, AR-D7 y AR-D8 con fecha y contenido exacto; estado del ADR fijado en "propuesto (segunda corrección)".
  2. **Robustez ante sincronizaciones:**
     - Claves dinámicas: no se usa una lista fija de 23 claves; se descubren dinámicamente escaneando las fuentes de los catálogos embebidos.
     - Nueva guarda `ui_code_scan_has_no_unhandled_pdfcraft`: escanea el código fuente de la UI (`crates/ui-egui/src/`) extrayendo literales de `tl!`, `tl_ctx!`, `notify_tr`, `notify_fmt` y verifica con `tr(lang, &key)` en los 23 idiomas que ninguna salida devuelve "PdfCraft", alertando de inmediato si el upstream añade mensajes nuevos en futuras versiones.
- **Correcciones no bloqueantes integradas:**
  - Respaldo único y determinista para claves ausentes: inglés con "Amaraza PDF" (idéntico al fallback hacia inglés del upstream).
  - Declaración explícita de elementos fuera de alcance y sus motivos en §3.3:
    - marcador técnico `%PdfCraft` en flujos de edición (`crates/edit/src/lib.rs:330–334`, para reconocer y envolver ediciones previas);
    - `<title>PdfCraft</title>` en `apps/pdfcraft-web/index.html` (destino web fuera de la prueba de concepto);
    - mensajes de registro técnico (`log::info!`, etc., diagnósticos internos);
    - errores internos en inglés de bajo nivel en enums del motor y de la CLI;
    - clave de texto de `OcrError::NoModels` (clave de catálogo traducida en UI).
  - Tabla de archivos modificados en §7 actualizada con AR-D8 y recuento exacto: **33 líneas modificadas/insertadas/retiradas en 21 archivos Rust del upstream** (incluyendo 5 Cargo.toml y 6 archivos de prueba). Cero líneas en los 22 catálogos TSV y cero en código de producción de la UI para traducción.
- **Estado:** diseño aprobado en revisión D-13 (2026-10-19).

### Implementación de AR0.4 (cuarto commit)

- **ADR-0002 implementado según el diseño aprobado** (`df963731413658f44f495948ea135c605cb8f681`) y las 5 correcciones menores autorizadas:
  1. **Metadatos desde `amaraza-identity` (L0 standalone):**
     - Añadida dependencia `amaraza-identity = { path = "../amaraza-identity" }` en `crates/create/Cargo.toml`, `crates/sign/Cargo.toml`, `crates/edit/Cargo.toml`, `crates/preflight/Cargo.toml` y `crates/automation/Cargo.toml`.
     - `Producer` de PDF creados: `amaraza_identity::PRODUCT_NAME` en `crates/create/src/lib.rs:54`.
     - Nombre de la app en firmas y sellos de tiempo: `amaraza_identity::PRODUCT_NAME` en `crates/sign/src/pdf.rs:1835` y `:2008`.
     - Autor por defecto de comentarios: `DEFAULT_AUTHOR = amaraza_identity::PRODUCT_NAME` en `crates/automation/src/comments.rs:15`.
     - Servidor MCP: `serverInfo.name = amaraza_identity::MCP_SERVER_NAME` (`"amarazapdf"`), `serverInfo.title = amaraza_identity::PRODUCT_NAME` (`"Amaraza PDF"`), instrucciones "Amaraza PDF edits PDFs." y títulos/descripciones/anotaciones de metaherramientas ("Find Amaraza PDF tools", "Run any Amaraza PDF tool") en `crates/automation/src/mcp.rs`.
     - Familia de fuente de texto moldeado: `amaraza_identity::SHAPED_TEXT_FONT_FAMILY` (`"Amaraza PDF Shaped Text"`) en `crates/edit/src/added.rs:711`.
     - Perfil ICC sRGB en PDF/A: `amaraza_identity::ICC_PROFILE_DESC` (`"sRGB (Amaraza PDF)"`) en `crates/preflight/src/icc.rs:51`.
     - Recortes de imagen del portapapeles: `amaraza_identity::FOLDER_NAME` (`"AmarazaPDF"`) en `crates/ui-egui/src/image_pick.rs:77`.
     - Título de impresión web: `amaraza_identity::PRODUCT_NAME` en `crates/ui-egui/src/web_print.rs:131`.
  2. **Retirada de `help.upstream` y enlace en "Acerca de" (AR-D8):**
     - Retirado el comando `help.upstream` de `COMMANDS` en `crates/engine/src/commands.rs`.
     - Retirado `help.upstream` de `LINKS` en `crates/engine/src/links.rs` y comentarios actualizados indicando que `LINKS` solo contiene comandos con entrada de menú propia, mientras que `UPSTREAM` se enlaza desde la atribución en "Acerca de".
     - En "Acerca de" (`crates/ui-egui/src/dialogs.rs`), la atribución `BASED_ON` se implementa como enlace interactivo directo (`egui::Link`) con el patrón diferido de `dialogs.rs`: bandera `open_upstream = true;` en el bucle de dibujo y `app.open_url(links::UPSTREAM)` fuera de él junto a `link_command`.
  3. **Mecanismo de traducción en memoria vía E18 (`catalog_source`):**
     - Detección dinámica de todas las claves fuente que contienen `"PdfCraft"` inspeccionando los catálogos embebidos (`all_pdfcraft_keys()`).
     - Para los 22 idiomas con catálogo (`code != "en"`): reemplazo en memoria de `"PdfCraft"` por `"Amaraza PDF"` en la columna traducida; inyección de respaldo en inglés con "Amaraza PDF" para claves no presentes en el catálogo respectivo.
     - Para inglés (`code == "en"`): inyección de filas de equivalencia que pueblan `c.plain` con "Amaraza PDF".
     - Cero líneas modificadas en los 22 catálogos TSV (la clave `"PdfCraft on GitHub"` queda como fila huérfana en ellos sin tocar los archivos).
  4. **Borrado de activos obsoletos:**
     - 12 capturas borradas de `docs/images/` y de `ATTRIBUTION.toml`.
     - `ATTRIBUTION.md` regenerado con `cargo xtask assets --write` (los activos pasan de 251 a 239).
     - Retirado el bloque `<screenshots>` de `packaging/linux/io.github.pony_svg.AmarazaPDF.metainfo.xml.in`.
  5. **Pruebas de guardas contra regresiones y adaptación:**
     - 7 pruebas existentes adaptadas: `ui.rs:114`, `ui_group/updates.rs:58, 102`, `signing.rs:536`, `automation.rs:1300`, `image_pick.rs:251`, `amaraza/mod.rs:91` y `ui_group/links.rs:35-62`.
     - 6 pruebas nuevas de guarda:
       - `ui_code_scan_has_no_unhandled_pdfcraft` en `crates/ui-egui/tests/amaraza.rs` (escanea `crates/ui-egui/src` y `apps/pdfcraft/src` y verifica con `tr(lang, &key)` en los 23 idiomas que ninguna traducción contenga "PdfCraft");
       - `producer_is_amaraza_pdf` en `crates/create/src/tests.rs`;
       - `signature_app_name_is_amaraza_pdf` en `crates/sign/tests/pdf.rs`;
       - `default_comment_author_is_amaraza_pdf` y `mcp_server_info_is_amaraza_pdf` en `crates/automation/tests/amaraza.rs`;
       - `shaped_font_family_is_amaraza_pdf` en `crates/edit/src/tests.rs`;
       - `the_other_platforms_packages_use_the_app_id` comprueba la ausencia de "PdfCraft" en `packaging/macos/Info.plist.in` en `crates/amaraza-identity/src/lib.rs`.
- **Puertas de calidad (`LANG=en_US.UTF-8`, `CARGO_BUILD_JOBS=6`, a la corriente):**
  - `cargo fmt --check`: 0.
  - `cargo clippy --workspace -- -D warnings`: 0.
  - `cargo clippy -p amaraza-identity -p amaraza-ext --all-targets -- -D warnings`: 0.
  - `cargo xtask layers`: 39 crates, 0 violaciones.
  - `cargo xtask assets`: 239 activos del repositorio ok.
  - `cargo test --workspace --no-fail-fast`: 177 binarios, **3852 pasadas, 0 fallidas, 16 ignoradas** (+6 pasadas frente a la línea base de ARS.4).

### Correcciones de la revisión D-13 de AR0.4 (quinto commit)

- **Dictamen D-13 sobre `1134a56a3af4ec3b68745b01962f38a87de7a7e1`:** APROBADO CON CORRECCIONES.
- **Correcciones implementadas:**
  1. **Prueba de fuente moldeada con escrituras mixtas (`shaped_font_family_is_amaraza_pdf`):**
     - Sustituida la comprobación anterior por una prueba que construye un documento en memoria con clústeres de escrituras mezcladas (árabe y hebreo en una misma fuente Type 3 con contornos sintéticos `boxed(0.5)`), alcanzando el brazo `None` de `type3_shaped`.
     - Comprueba en el diccionario `/FontDescriptor` resultante:
       - `/FontFamily` = `"Amaraza PDF Shaped Text"` (`amaraza_identity::SHAPED_TEXT_FONT_FAMILY`), sin mención a `"PdfCraft"`;
       - `/FontName` = `"AmarazaPDFShapedText"` (sin espacios, derivado de `family.replace(' ', "")`).
     - **Demostración en rojo:** revirtiendo temporalmente `crates/edit/src/added.rs:711` a `"PdfCraft Shaped Text"`, la prueba falló con:
       ```
       thread 'added::amaraza_tests::shaped_font_family_is_amaraza_pdf' panicked at crates\edit\src\added\amaraza_tests.rs:44:5:
       assertion `left == right` failed
         left: "PdfCraft Shaped Text"
        right: "Amaraza PDF Shaped Text"
       ```
       Restaurado a `amaraza_identity::SHAPED_TEXT_FONT_FAMILY`, pasa en verde.
  2. **Pruebas del fork fuera de archivos grandes del upstream (A7):**
     - `producer_is_amaraza_pdf` trasladada a `crates/create/tests/amaraza.rs`. `crates/create/src/tests.rs` queda 100% idéntico al upstream.
     - `signature_app_name_is_amaraza_pdf` trasladada a `crates/sign/tests/amaraza.rs` (con sus propios ayudantes `fixture()`, `open()`, `opts()`, `data()`). `crates/sign/tests/pdf.rs` queda 100% idéntico al upstream.
     - `shaped_font_family_is_amaraza_pdf` ubicada en submódulo propio `crates/edit/src/added/amaraza_tests.rs`, declarado en `crates/edit/src/added.rs:1152` con una sola línea `#[cfg(test)] mod amaraza_tests; // Enganche ADR-0001`. `crates/edit/src/tests.rs` queda 100% idéntico al upstream.
  3. **Escáner de código de UI reforzado (`ui_code_scan_has_no_unhandled_pdfcraft`):**
     - En `crates/ui-egui/tests/amaraza.rs`, el escáner ahora examina todo literal de cadena que contenga `"PdfCraft"` fuera de módulos `#[cfg(test)]` y fuera de macros de log (`log::`, `tracing::`).
     - Detecta tanto llamadas directas (`tl!`, `notify_tr`, `notify_fmt`) como cadenas asignadas a variables intermedias (como `let template = "…PdfCraft…"` en `crates/ui-egui/src/lib.rs:1979–1981`).
     - Incorpora `ALLOWLIST_EXCEPTIONS` con la excepción explícita `"the running PdfCraft didn't confirm"` (mensaje de error interno en `single_instance.rs:311`, no visible en la UI).
     - Verifica que cada una de las cadenas detectadas (18 cadenas de UI) se traduzca sin `"PdfCraft"` en los 23 idiomas de la interfaz mediante `tr(lang, &key)`.
  4. **ADR-0002 y `catalog_source`:**
     - `docs/amaraza/adr/0002-identidad-metadatos-y-textos.md`: Estado actualizado a `aceptado (D-13, 2026-10-19); implementado en AR0.4`.
     - `crates/ui-egui/src/amaraza/mod.rs`: comentario de `catalog_source` actualizado; ayudante unificado `append_fallback_row` compartido entre `english_catalog` y `transform_catalog`.
  5. **Nota sobre capturas de pantalla:**
     - **`cargo xtask screenshots` no debe ejecutarse en el fork**: regeneraría `docs/images/` con capturas que contienen marcas, logotipos o la atribución *"PdfCraft screenshot"* del upstream.
- **Puertas de calidad (`LANG=en_US.UTF-8`, `CARGO_BUILD_JOBS=6`, a la corriente):**
  - `cargo fmt --check`: 0.
  - `cargo clippy --workspace -- -D warnings`: 0.
  - `cargo clippy -p amaraza-identity -p amaraza-ext --all-targets -- -D warnings`: 0.
  - `cargo xtask layers`: 39 crates, 0 violaciones.
  - `cargo xtask assets`: 239 activos del repositorio ok.
  - `cargo test --workspace --no-fail-fast`: 179 binarios, **3852 pasadas, 0 fallidas, 16 ignoradas** (dos binarios nuevos de integración para las pruebas extraídas de `create` y `sign`).
