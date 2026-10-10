# Equivalencias PDForte → Amaraza PDF (Rust)

Tarea AR0.2 (`docs/amaraza/PLAN.md`), 2026-10-09.

Este documento clasifica cada regla de §3.1–§3.8 del `AGENTS.md` de PDForte y cada decisión D-01…D-13 (más RF-FIRMA-01). PDForte se usa como referencia funcional: sus reglas se citan por número y se resumen, sin traer su código, actas ni auditorías (§3 de las reglas del fork).

## Criterio y clases

- **En el código del original** (archivos de `storytold/pdfcraft`) mandan sus reglas (`AGENTS.md` y `CLAUDE.md` del upstream). Lo que el fork cambie en ellos se registra en la tabla de archivos modificados de `REGISTRO.md`.
- **En los crates `amaraza-*`** (y en cualquier código nuevo del fork) se aplica la más estricta de las dos reglas.

| Clase | Significado |
|---|---|
| **Original** | La cubre el original. Se indica dónde. |
| **Nueva** | Regla nueva de Amaraza. Las reglas A1…A15 se definen en la sección 3 y van a `.agents/rules/00-amaraza.md` §9; otras ya estaban escritas en las reglas o el plan del fork, y se indica dónde. |
| **Descartada** | No aplica. Se indica el motivo. |

Cuando el original cubre solo una parte, se dice en la columna "Dónde / motivo".

## Resumen

| Origen | Original | Nueva | Descartada | Total |
|---|---:|---:|---:|---:|
| §3.1–§3.8 (30 reglas) | 11 | 16 | 3 | 30 |
| D-01…D-13 y RF-FIRMA-01 | 6 | 6 | 2 | 14 |

Puntos donde PDForte es más estricto que PdfCraft y que piden una decisión del titular sobre el código del original: la sección 4.

## 1. Reglas de §3 del `AGENTS.md` de PDForte

### §3.1 Integridad del documento del usuario

| Ref. | Regla de PDForte (resumen) | Clase | Dónde / motivo |
|---|---|---|---|
| 3.1.a | Toda mutación pasa por un único servicio de publicación | **Original** | Toda edición pasa por el motor (`Session::apply_edit` y el historial de deshacer, `crates/engine/src/lib.rs`), y todo guardado por `write_atomically` (`crates/ui-egui/src/editing.rs`). Los crates `amaraza-*` mutan solo a través del motor (A1). |
| 3.1.b | Nunca escribir sobre el PDF del usuario: candidato en un temporal de la misma carpeta → **verificación** → sustitución | **Nueva (A1)** | Parcial en el original: `write_atomically` escribe en un temporal de la misma carpeta, hace `fsync` y renombra. Pero la verificación (reabrir desde los bytes, `mark_saved`) llega **después** de sustituir. |
| 3.1.c | La doble barrera de huella, la copia `rec_*` y la quiescencia son del servicio, y no se reimplementan | **Nueva (A2)** | El original no comprueba si el archivo cambió en disco desde que se abrió, ni guarda una copia del original antes de sustituirlo. |
| 3.1.d | Operaciones terminales (censura, firma en el sitio, compresión en el sitio, publicación de texto): frontera de historial, confirmación, aviso de que no se pueden deshacer y sin rollback automático ante un fallo crítico | **Nueva (A3)** | Parcial en el original. La censura pide confirmación ("se eliminan de forma permanente", `crates/ui-egui/src/redact_ui.rs`) y obliga a reescribir el archivo entero al guardar. Pero **no vacía el historial**: se puede deshacer en memoria (hasta 100 pasos, `MAX_UNDO`) mientras el documento sigue abierto. |
| 3.1.e | Sin fallbacks de composición: dependencias por constructor y composición solo en los contextos | **Descartada** | Es una regla de la inyección de dependencias de la arquitectura en Python. En Rust las dependencias se pasan de forma explícita por tipos, y las capas las vigila `cargo xtask layers`. |
| 3.1.f | Prohibido el código consciente de las pruebas en producción (shims, detectar mocks, identidades inventadas) | **Nueva (A4)** | El original usa `cfg!(test)` en código de producción para forzar el inglés en sus pruebas unitarias (`crates/ui-egui/src/i18n/mod.rs`). Ahí manda su regla. |

### §3.2 Firmas digitales (D-03, RF-FIRMA-01)

| Ref. | Regla de PDForte (resumen) | Clase | Dónde / motivo |
|---|---|---|---|
| 3.2.a | Documentos firmados: toda mutación en el sitio se bloquea por defecto (`allow_with_consent` opcional); firmar sobre un firmado siempre se permite | **Nueva (A5)** | **El original es menos estricto.** Permite las ediciones que se guardan de forma incremental (los bytes firmados se conservan; la barra de firmas y la clasificación DocMDP muestran después los cambios) y rechaza solo las que exigen reescribir el archivo (`EditError::SignedRewrite`, `crates/engine/src/lib.rs`). Firmar un firmado también es incremental (`crates/sign/src/pdf.rs`). Decisión pendiente: sección 4. |
| 3.2.b | Enumerar `CurrentUser\MY`, incluidos tokens USB y virtuales, sin exportar nunca la clave privada; APIs de exportación prohibidas | **Original** | `crates/sign/src/windows.rs` enumera `CurrentUser\My` con `rustls-cng`. Ni en `crates/sign` ni en `vendor/` hay llamadas a `NCryptExportKey`, `CryptExportKey`, `PFXExportCertStore*` ni `NCRYPT_ALLOW_EXPORT_FLAG`. Con token, sin probar (AR1.2). Repetida en `00-amaraza.md` §5. |
| 3.2.c | Al listar certificados no se abre la clave privada (solo al firmar) | **Nueva (A6)** | **El original no lo cumple:** `identities()` abre la clave en modo silencioso mientras lista (`context.acquire_key(true)`, `crates/sign/src/windows.rs`). Ya está en `00-amaraza.md` §5 ("al listar certificados no se debe pedir PIN"); se comprueba con el token en AR1.2. |
| 3.2.d | La validación distingue integridad, validez criptográfica y confianza; nunca "válida" si la confianza no se evaluó | **Original** | `crates/sign/src/pdf.rs`: `Status::Valid` exige bytes intactos, identidad de confianza (`TrustStore`) y cambios posteriores permitidos; sin confianza es `Unknown`, y lo demás `Invalid`. Los cambios posteriores se clasifican (`Modification::None`, `Allowed` o `Disallowed`). |

### §3.3 Arquitectura y capas

| Ref. | Regla de PDForte (resumen) | Clase | Dónde / motivo |
|---|---|---|---|
| 3.3.a | Dominio solo con biblioteca estándar, la aplicación no importa la interfaz, puertos y adaptadores | **Original** | Las capas L0–L8 de `xtask/src/layers.rs` (`cargo xtask layers`): nada por debajo de L7 depende de egui, winit, eframe ni rfd, y `cos`, `filters`, `crypt` y `arlington` son independientes (`CLAUDE.md`, "Layering"). `amaraza-identity` está registrado como `Standalone(0)`. |
| 3.3.b | Funciones nuevas con complejidad ciclomática ≤ 15; no hacer crecer la ventana principal | **Nueva (A7)** | El original no fija límite de complejidad: su `clippy.toml` solo relaja reglas en las pruebas. Sus archivos centrales son grandes (por ejemplo, `crates/ui-egui/src/lib.rs`). En `amaraza-identity` el límite ya está aplicado y detectó una prueba de complejidad 16, que se refactorizó. |

### §3.4 `modules/` (motor heredado): evolución controlada

| Ref. | Regla de PDForte (resumen) | Clase | Dónde / motivo |
|---|---|---|---|
| 3.4 | Cada cambio en el motor heredado se declara (`CC-xx`), con prueba primero y lista blanca de una guardiana; nada nuevo en `modules/` | **Nueva (ya escrita)** | Su equivalente en el fork es el código del upstream. Ya lo cubren `PLAN.md` ("Estrategia de código": lo nuevo va en crates `amaraza-*`; los archivos del upstream solo se modifican por identidad o correcciones mínimas registradas) y la tabla de archivos modificados de `REGISTRO.md`. No hay guardiana automática: los puntos de enganche de AR0.3 la acotan. |

### §3.5 Errores, robustez y logs

| Ref. | Regla de PDForte (resumen) | Clase | Dónde / motivo |
|---|---|---|---|
| 3.5.a | Prohibido silenciar excepciones (`except: pass`) o capturarlas en general sin registrarlas | **Nueva (A10)** | Parcial en el original: `AGENTS.md` §4 obliga a devolver `Result` y propagar con `?`, sin `unwrap` ni `panic`, y con `catch_unwind` como red. Pero descarta errores en limpiezas (`let _ = std::fs::remove_file(…)`), lo que en PDForte sería un `except: pass`. |
| 3.5.b | Registrar con `event=… operation_id=… error_code=…` | **Nueva (A8)** | El original usa mensajes libres con el crate `log`. |
| 3.5.c | Nunca rutas completas, nombres de usuario, contenido de documentos ni secretos en los logs | **Nueva (A9)** | **El original registra rutas completas**, por ejemplo: el archivo de log (`apps/pdfcraft/src/main.rs`), la carpeta portátil (`crates/ui-egui/src/portable.rs`) y la fuente del sistema (`crates/ui-egui/src/system_fonts.rs`). El token del canal de control nunca se registra (`AGENTS.md` §3). Decisión pendiente: sección 4. |
| 3.5.d | Mensajes al usuario accionables y sin trazas técnicas; nada de diálogos modales desde hilos de trabajo | **Original** | `AGENTS.md` §3 ("errores sobre los que se pueda actuar: qué estaba mal y qué se permite"). egui dibuja en un único hilo: los trabajos en segundo plano devuelven resultados y es la interfaz quien los notifica. |
| 3.5.e | Toda entrada externa (PDF, imagen, hoja de cálculo, ruta) con límites de tamaño, píxeles y tiempo | **Original** | `AGENTS.md` §4 (asignaciones limitadas, recursión acotada, aritmética comprobada); `MAX_PIXELS` (64 Mpx) y `MAX_SIDE` en `crates/render/src/raster.rs`; fuzzing con `cargo xtask fuzz`. |

### §3.6 Privacidad y datos (D-02)

| Ref. | Regla de PDForte (resumen) | Clase | Dónde / motivo |
|---|---|---|---|
| 3.6.a | Nada institucional en código, pruebas, fixtures, corpus, capturas ni evidencias; solo PDF sintéticos | **Original** | `CLAUDE.md` ("Privacy") y `AGENTS.md` §2 (fixtures sintéticos). Reforzada por `00-amaraza.md` §5, porque el repositorio es público. |
| 3.6.b | Nunca escribir en la carpeta real de datos de la app; las pruebas la redirigen | **Nueva (A11)** | Parcial en el original: sus pruebas usan carpetas temporales (por ejemplo, `RecoveryStore::new(temp)`), pero no hay variable que redirija la carpeta de datos. Para ejecutar la app en desarrollo, modo portátil (`00-amaraza.md` §8). |
| 3.6.c | Sin temporales en la raíz del repositorio | **Nueva (ya escrita)** | `00-amaraza.md` §8 ("temporales en `%TEMP%` o en `target/`"). Las pruebas del original usan `std::env::temp_dir()`. |
| 3.6.d | No leer, copiar ni subir datos personales del perfil del usuario | **Original** | `AGENTS.md` §2 ("nunca capturar datos personales"). Reforzada por `00-amaraza.md` §5. |

### §3.7 Dependencias y licencias

| Ref. | Regla de PDForte (resumen) | Clase | Dónde / motivo |
|---|---|---|---|
| 3.7.a | No añadir ni actualizar dependencias sin el titular; declararlas y revisar su efecto en el empaquetado | **Nueva (ya escrita)** | `00-amaraza.md` §6 (incluido regenerar `Cargo.lock`). El original revisa las licencias con `deny.toml` y `cargo xtask deny`. |
| 3.7.b | Licencia AGPL-3.0: solo código compatible (MIT, BSD, Apache-2.0, LGPL, GPL/AGPL) | **Descartada** | El fork se publica como `MIT OR Apache-2.0` (AR-D2), y el original es más estricto: nunca enlaza código copyleft (`deny.toml`, `AGENTS.md` §2). Rige el original. |
| 3.7.c | Recursos (iconos, fuentes, imágenes) solo propios o con licencia abierta registrada | **Original** | `AGENTS.md` §1: `ATTRIBUTION.toml` y `cargo xtask assets`, iconos Lucide, y la marca Amaraza con `LicenseRef-Amaraza-Trademark`. |

### §3.8 Interfaz

| Ref. | Regla de PDForte (resumen) | Clase | Dónde / motivo |
|---|---|---|---|
| 3.8.a | Solo tokens del tema: nada de colores literales fuera del tema (con prueba guardiana) | **Nueva (A12)** | El original tiene tokens (`theme::Tokens`), pero 26 archivos de `crates/ui-egui/src` fuera de `theme.rs` usan colores literales (por ejemplo, la barra de firmas de `sign_ui.rs`). |
| 3.8.b | Controles con nombre accesible, orden de tabulación y atajo | **Original** | AccessKit (eframe con `accesskit`); los widgets declaran su nombre (`widget_info`) y las pruebas kittest los buscan por él; atajos en Ayuda ▸ Atajos de teclado. El lector de pantalla se evalúa en AR1.3. |
| 3.8.c | Todo debe caber en una ventana de 800×560 (D-08) | **Nueva (A13)** | El tamaño mínimo del original es 820×520 (`apps/pdfcraft/src/main.rs`): 20 px más ancho y 40 px más bajo. Se evalúa en AR1.3; decisión pendiente en la sección 4. |
| 3.8.d | Textos de la interfaz en español, incluidos los diálogos estándar | **Original** | Catálogo `es.tsv` con prueba de cobertura (`crates/ui-egui/src/i18n`); la interfaz sigue el idioma de Windows, y los diálogos de archivos son los del sistema (rfd). Los textos nuevos de Amaraza van en español (`00-amaraza.md` §8). |
| 3.8.e | Cada herramienta nueva del visor decide si entra en el modo continuo | **Descartada** | Regla específica del visor Qt de PDForte. PdfCraft tiene otro visor, con sus propias disposiciones (continua, dos páginas, una página) y sus pruebas (`crates/ui-egui/tests/viewer.rs`). |

## 2. Decisiones D-01…D-13 y RF-FIRMA-01

| Decisión | En PDForte (resumen) | Clase | Dónde / motivo |
|---|---|---|---|
| D-01 | Licencia AGPL-3.0 | **Descartada** (durante la prueba) | Durante la prueba la sustituye AR-D2 (`MIT OR Apache-2.0`). Se revisa al cerrarla (AR1.4). |
| D-02 | Proyecto personal del titular; nada institucional; autoría unificada | **Nueva (ya escrita)** | `00-amaraza.md` §5 (más estricta: repositorio público) y §7.1 (correo del proyecto antes del primer commit; corrección de autoría del 2026-10-09 en `REGISTRO.md`). |
| D-03 | PDF firmados: bloqueo por defecto, consentimiento opcional, firmar siempre permitido | **Nueva (A5)** | Ver 3.2.a: el original permite las ediciones incrementales. Decisión pendiente: sección 4. |
| D-04 | PDF→Word con `pdf2docx` y `opencv-python-headless` | **Original** (la función) | El original exporta a Word de forma nativa (`crates/export`: "Export a PDF ▸ Word, HTML, RTF"). La elección de bibliotecas Python no aplica. Fuera del alcance de la prueba (`PLAN.md`). |
| D-05 | MSI con WiX, por máquina, `UpgradeCode` fijo, instalación silenciosa, rutas de datos estables | **Original** | `packaging/windows/pdfcraft.wxs` (WiX v5, `perMachine`, `MajorUpgrade`), con el `UpgradeCode` propio de Amaraza (`amaraza_identity::MSI_UPGRADE_CODE`, AR0.1). |
| D-06 | Sin firma Authenticode en el piloto; reevaluar antes de la GA | **Original** | `packaging/windows/sign.ps1` firma solo si hay secretos; sin ellos produce artefactos sin firmar con un aviso. El MSI firmado está fuera del alcance de la prueba. |
| D-07 | Repositorio privado `pony-svg/AmarazaPDF`; CI con GitHub Actions en `windows-latest` | **Descartada** | El repositorio del fork es público (AR-D1). La CI es AR0.5 (opcional, con autorización). |
| D-08 | Ventana mínima de 800×560 | **Nueva (A13)** | Ver 3.8.c. |
| D-09 | Internacionalización diferida (solo botones estándar de Qt) | **Original** (la supera) | 13 catálogos con pruebas de cobertura (`crates/ui-egui/src/i18n`), y la interfaz sigue el idioma de Windows. |
| D-10 | Snapshots cifrados con AES-GCM y clave solo en memoria; purga de restos a los 30 días | **Nueva (A14)** | **El original es menos estricto:** la recuperación (`crates/ui-egui/src/recovery.rs`) escribe la copia de trabajo **sin cifrar** en `%LOCALAPPDATA%\AmarazaPDF\Recovery`, salvo los PDF que ya estaban cifrados. No purga por antigüedad: cada entrada se borra al guardar, descartar o cerrar, y se ofrece recuperarla al arrancar. El historial de deshacer vive solo en memoria. Decisión pendiente: sección 4. |
| D-11 | Autovalidaciones `--validate-*` ocultas tras `AMARAZA_DIAGNOSTICS=1` | **Original** (equivalente) | Los canales de diagnóstico y control del original están desactivados por defecto (`AGENTS.md` §3): `--control` (bucle local, token nuevo en cada arranque, archivo solo del propietario) y el servidor MCP, que solo arranca con `pdfcraft-cli mcp`. No hay `--validate-*`. |
| D-12 | Instancia única: los PDF adicionales se abren en pestañas | **Nueva (A15)** | **El original no la tiene** en Windows ni en Linux: cada apertura lanza otro proceso. Sí tiene pestañas, y en macOS recibe los archivos en la app abierta (`apps/pdfcraft/src/apple_events.rs`). Decisión pendiente: sección 4. |
| D-13 | Revisor IA estricto, CI obligatoria y evidencia real del titular | **Nueva (ya escrita)** | `00-amaraza.md` §7.7 (revisión D-13 y evidencia del titular). La CI es AR0.5. |
| RF-FIRMA-01 | Firmar con `CurrentUser\MY` (incluidos tokens) sin exportar la clave privada | **Original** | Ver 3.2.b. La parte "no abrir la clave al listar" es 3.2.c (A6), y la prueba con el token del titular es AR1.2. |

## 3. Reglas nuevas de Amaraza

Se aplican al código del fork (crates `amaraza-*`, `apps/amaraza` si existe y los enganches de AR0.3). En el código del original manda su regla hasta que el titular decida otra cosa (sección 4). Van a `.agents/rules/00-amaraza.md` §9.

| Regla | Contenido | Origen |
|---|---|---|
| **A1** | Ningún crate `amaraza-*` escribe el PDF del usuario por su cuenta: muta a través del motor y guarda por un único camino. El candidato se escribe en un temporal de la misma carpeta, **se verifica antes de sustituir** (volviéndolo a abrir con el motor) y solo después reemplaza al original. | 3.1.a, 3.1.b |
| **A2** | Antes de sustituir: comprobar que el archivo en disco no cambió desde que se abrió (tamaño, fecha y huella), y conservar una copia del original hasta que la nueva versión esté verificada. Un único ayudante compartido, nunca reimplementado en cada función. | 3.1.c |
| **A3** | Operaciones terminales (censura, firma en el sitio, compresión en el sitio y similares): confirmación previa que avisa de que no se puede deshacer, y vacían el historial de deshacer. Un fallo crítico no se revierte en silencio: se avisa y se conserva la copia de A2. | 3.1.d |
| **A4** | Sin código consciente de las pruebas en producción: nada de `cfg!(test)`, de detectar dobles de prueba ni de identidades inventadas. Las pruebas inyectan lo que necesitan por parámetros. | 3.1.f |
| **A5** | Documentos con firmas activas: toda mutación se rechaza por defecto con un error accionable. Solo con el consentimiento explícito del usuario, si se implementa esa preferencia, y diciendo cuántas firmas se verán afectadas. Firmar sobre un documento firmado siempre se permite (de forma incremental, con aviso). | 3.2.a, D-03 |
| **A6** | Al listar certificados no se abre la clave privada ni se pide PIN; la clave solo se abre al firmar. Nunca exportarla. | 3.2.b, 3.2.c, RF-FIRMA-01 |
| **A7** | Complejidad ≤ 15 por función, también en las pruebas. Cada crate `amaraza-*` lleva su propio `clippy.toml`: clippy no fusiona configuraciones, así que repite las 4 claves `allow-*-in-tests` de la raíz y añade `cognitive-complexity-threshold = 15`. Además activa el lint, que es de nursery, con `#![warn(clippy::cognitive_complexity)]`, porque `[lints] workspace = true` impide añadir `[lints.clippy]` en el `Cargo.toml`. Ya está aplicado en `amaraza-identity`. No hacer crecer los archivos grandes del upstream: lo nuevo entra por los enganches de AR0.3. | 3.3.b |
| **A8** | Los registros llevan `event=…` y, si hay error, `error_code=…` (y `operation_id=…` cuando exista ese identificador). | 3.5.b |
| **A9** | En los registros, nada de rutas completas (como mucho, el nombre del archivo o una ruta relativa a una carpeta conocida), nombres de usuario, contenido de documentos ni secretos. | 3.5.c |
| **A10** | Ningún `Result` se descarta en silencio: se propaga, se registra o lleva un comentario que diga por qué se puede ignorar. | 3.5.a |
| **A11** | Las pruebas nunca escriben en las carpetas reales de la app (`%APPDATA%\AmarazaPDF`, `%LOCALAPPDATA%\AmarazaPDF`): usan carpetas temporales con limpieza garantizada. | 3.6.b |
| **A12** | Colores solo de los tokens del tema (`theme::Tokens`): nada de `Color32` literales. | 3.8.a |
| **A13** | Toda pantalla y diálogo nuevos se pueden usar en una ventana de 800×560. | 3.8.c, D-08 |
| **A14** | Ninguna copia de un documento se escribe en claro en disco: los snapshots se cifran (AES-GCM, clave solo en memoria) y los restos se purgan a los 30 días. Si hace falta una dependencia nueva, se pide al titular (§6). | D-10 |
| **A15** | Instancia única: una segunda apertura entrega sus archivos a la app ya abierta, que los abre en pestañas. Se implementa solo si el titular lo decide (sección 4). | D-12 |

Reglas de PDForte que ya estaban escritas en el fork y no cambian: D-02 (§5 y §7.1), D-13 (§7.7), dependencias (§6), temporales (§8), estrategia de código y tabla de archivos modificados (3.4, `PLAN.md` y `REGISTRO.md`).

## 4. Decisiones pendientes del titular (código del original)

Según el criterio, en el código del original manda su regla. Estos son los puntos en que PDForte es más estricto y en que conviene decidir si el producto adopta la regla de PDForte, aunque haya que tocar archivos del upstream (con un enganche de AR0.3 o un cambio mínimo registrado).

| Tema | Hoy en el original | Regla de PDForte | Archivos que habría que tocar |
|---|---|---|---|
| D-03, firmados | Permite las ediciones incrementales | Bloqueo por defecto | Comprobación en el camino de edición del motor (`crates/engine/src/lib.rs`) y un aviso en la interfaz |
| D-10, recuperación | Copia de trabajo sin cifrar; sin purga por antigüedad | Cifrada (AES-GCM) y purga a 30 días | `crates/ui-egui/src/recovery.rs`; posible dependencia criptográfica directa (§6) |
| 3.1.b/3.1.c, guardar | Escritura atómica; verificación después de sustituir; sin copia ni comprobación de cambios externos | Verificar antes, copia `rec_*` y barrera de huella | `save_doc_to` y `write_atomically` (`crates/ui-egui/src/editing.rs`) |
| 3.1.d, censura | Confirmación y reescritura completa, pero el historial se conserva | Vacía el historial | Camino de la censura (`crates/ui-egui/src/redact_ui.rs` y el motor) |
| 3.5.c, logs | Registra rutas completas | Sin rutas | Los mensajes de registro citados en 3.5.c |
| D-08, ventana | Mínimo 820×520 | 800×560 | `apps/pdfcraft/src/main.rs` (y comprobar la interfaz en AR1.3) |
| D-12, instancia única | Un proceso por apertura | Una sola instancia con pestañas | `apps/pdfcraft/src/main.rs` (nuevo mecanismo de comunicación entre procesos) |
