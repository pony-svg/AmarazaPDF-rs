# Plan del fork Amaraza PDF (Rust): prueba de concepto

**Estado:** vigente desde el 2026-10-09. Reglas: `.agents/rules/00-amaraza.md`. Registro: `docs/amaraza/REGISTRO.md`.

## Objetivo

Decidir con evidencia si Amaraza PDF pasa de Python (PDForte) a Rust sobre PdfCraft. Duración objetivo: unas dos semanas. Mientras tanto, PDForte sigue siendo el producto.

## Decisiones del titular

| ID | Fecha | Decisión |
|---|---|---|
| AR-D1 | 2026-10-09 | Fork **público** de `storytold/pdfcraft` en `pony-svg/AmarazaPDF-rs` |
| AR-D2 | 2026-10-09 | Licencia `MIT OR Apache-2.0` durante la prueba; se revisa al cerrarla |

## Estrategia de código

- **Lo nuevo va en crates propios** (`crates/amaraza-*`, `apps/amaraza` si hace falta). Así las actualizaciones del upstream entran con pocos conflictos.
- **Los archivos del upstream solo se modifican** por obligación de marca e identidad (AR0.1) o para correcciones mínimas registradas. Cuantos menos archivos del upstream se toquen, más barata es cada sincronización.
- La sincronización con el upstream solo se hace con autorización del titular, en una rama propia, y conserva la identidad Amaraza (ver "Sincronización con el upstream").
- `docs/amaraza/REGISTRO.md` lleva una tabla viva de **archivos del upstream modificados**, con archivo, motivo y tarea. Es el mapa para resolver conflictos en cada sincronización.

## Fase AR0: puesta a punto (bloquea cualquier push)

### AR0.0. Entorno y línea base
- Remotos: `origin` = fork; `upstream` = storytold con el push deshabilitado. Los configura el titular.
- Toolchain: `rustc --version` ≥ 1.90, con objetivo MSVC. Si `aws-lc-sys` pide CMake o NASM, se registra y se pide al titular.
- Línea base, sin cambios de código:
  - `cargo build --workspace`;
  - `cargo test --workspace`: número de pruebas, fallos ya existentes y tiempo;
  - `cargo xtask assets`.
- Se crea `docs/amaraza/REGISTRO.md` con la línea base y se hace commit de `.agents/rules/00-amaraza.md`, `.claude/rules/amaraza.md` y este plan.

### AR0.1. Retirar la marca ArtCraft y poner la identidad Amaraza
Es obligatorio por la licencia de marca del upstream y debe estar hecho antes del primer push.

**Concentra la identidad en un único módulo propio:** nombre, id de la app, carpetas, URL y logotipo. Los archivos del upstream solo deben llamarlo. Así, al sincronizar, los conflictos se reducen a unas pocas líneas.

1. **Marca.**
   - Retirar `docs/brand/artcraft-*` (SVG y PNG) y sus entradas en `ATTRIBUTION.toml`.
   - Sustituir `docs/brand/LICENSE-brand.txt` por la licencia de la marca Amaraza que apruebe el titular.
   - Añadir el logotipo de Amaraza cuando el titular confirme su origen.
   - En `xtask/src/assets.rs`: `BRAND_LICENCE` = `LicenseRef-Amaraza-Trademark` y actualizar la prueba `brand_licence_only_for_brand_logos`.
2. **Interfaz.**
   - `crates/ui-egui/src/widgets.rs`: `artcraft_logo` y `artcraft_mark` pasan a ser la marca Amaraza.
   - Actualizar sus usos en `home.rs` y `dialogs.rs`.
   - Quitar los enlaces a getartcraft.com y a su Discord: `crates/engine/src/links.rs`, `crates/engine/src/commands.rs`, `apps/pdfcraft-cli/src/main.rs`, con su prueba `crates/ui-egui/tests/links.rs`.
   - Revisar también `xtask/src/screenshots.rs`.
   - "Acerca de" debe decir "Basado en PdfCraft, del equipo ArtCraft".
3. **Identidad de la app** (`apps/pdfcraft/src/main.rs` y `crates/ui-egui/src/`):
   - `APP_ID` propio (lo decide el titular);
   - título de la ventana y nombre del ejecutable;
   - `eframe::storage_dir`, carpeta de recuperación en `%LOCALAPPDATA%` y marcadores y carpeta del modo portátil (`portable.rs`);
   - desactivar `migrate_legacy_folders` para que nunca toque carpetas de PdfCraft o PrintCraft;
   - icono de la app.
   - Si cambia `APP_ID`, mantener coherentes los archivos de empaquetado que lo usan, o declarar esas plataformas fuera de alcance en el README.
4. **Instalador** (`packaging/windows/pdfcraft.wxs`):
   - `Name`, `Manufacturer`, `UpgradeCode` (GUID nuevo, distinto del de PDForte), URLs, `INSTALLFOLDER`, ProgId, `RegisteredApplications` y accesos directos.
5. **Documentos.**
   - `README.md` propio: qué es, estado de prueba de concepto, atribución al upstream y licencia, sin marcas ArtCraft.
   - `NOTICE`: añadir la línea de archivos modificados (Apache-2.0 §4).
6. **CI.**
   - Retirar o desactivar `release.yml` (usa los secretos de firma del upstream).
   - Quitar los `schedule` de `ci.yml` y `windows-arm64.yml`.
   - No activar Actions sin autorización.
7. **Comprobación final.**
   - Buscar con `grep -rniE "artcraft|getartcraft|storyteller"`, excluyendo `vendor/` y `target/`. Solo pueden quedar:
     - el copyright de `LICENSE-*` y `NOTICE`;
     - la atribución en texto plano;
     - los campos de autor de `ATTRIBUTION.toml`.
   - Todas las puertas de calidad en verde.
   - La captura real del titular con la ventana y el instalador renombrados es evidencia del titular.

### AR0.2. Equivalencias PDForte → Rust
- Documento `docs/amaraza/EQUIVALENCIAS.md`. Clasifica cada regla de §3.1–§3.8 del `AGENTS.md` de PDForte y cada decisión D-01…D-13 en una de tres clases:
  - **la cubre el original** (dónde: archivo, prueba o regla del upstream);
  - **regla nueva de Amaraza**;
  - **descartada** (motivo).
- **Criterio:** en el código del original mandan sus reglas; en los crates `amaraza-*` se aplica la más estricta de las dos.
- Cubre como mínimo:
  - D-03: PDForte bloquea por defecto toda edición de documentos firmados, y PdfCraft permite las incrementales;
  - la publicación con verificación previa y copia `rec_*`;
  - las operaciones terminales;
  - los logs sin rutas;
  - la ventana mínima de 800×560;
  - la instancia única;
  - los snapshots cifrados;
  - la complejidad.
- PDForte se usa como referencia funcional: se citan sus reglas y decisiones por su número, sin traer su código, actas ni auditorías (§3 de las reglas).
- Las reglas nuevas se añaden a `.agents/rules/00-amaraza.md`.

### AR0.3. Puntos de enganche para los crates `amaraza-*`
- Un ADR en `docs/amaraza/adr/` con el diseño mínimo para registrar desde crates `amaraza-*`, tocando el upstream solo en el enganche:
  - comandos (`COMMANDS` en `crates/engine/src/commands.rs`);
  - herramientas (`tools()` en `crates/automation/src/tools.rs`);
  - menús y capas.
- Demostración: un comando "hola Amaraza" con API, herramienta y prueba de extremo a extremo (`AGENTS.md` §3).
- Cada enganche se anota en la tabla de archivos del upstream modificados de `REGISTRO.md`.

### AR0.4. Identidad en metadatos y textos (antes de AR1.2)
- Desde `amaraza-identity`:
  - el `Producer` de los PDF creados;
  - el nombre de la app en las firmas;
  - el autor por defecto de los comentarios;
  - el título de los trabajos de impresión;
  - el nombre del servidor MCP.
- Los 12 mensajes de la interfaz que dicen PdfCraft (lista en `REGISTRO.md`, "Pendiente"), con sus catálogos y los `sha256` de `ATTRIBUTION.toml`.
- Borrar las 12 capturas de `docs/images/` y sus entradas en `ATTRIBUTION.toml`.

### AR0.5. CI del fork (opcional, con autorización)
Es la antigua AR0.2.
- Un flujo mínimo en `windows-latest` con `fmt`, `clippy`, `test` y `xtask assets` en push y PR.

## Sincronización con el upstream (tarea `ARS.n`, a partir de AR0.1)

El upstream avanza muy rápido, a veces más de 60 commits al día. Conviene sincronizar a menudo y en pasos pequeños: cada semana o antes de empezar una función nueva. Cada sincronización necesita autorización del titular.

1. `git fetch upstream` y anotar el SHA de `upstream/main`.
2. Crear la rama `sync/upstream-AAAA-MM-DD` desde `main` y ejecutar `git merge upstream/main`.
   - Se hace con **merge**: nunca rebase de ramas publicadas ni `push --force`.
3. Resolver los conflictos conservando la identidad Amaraza. Usar como guía la tabla de archivos modificados de `REGISTRO.md`.
   - Tras resolver conflictos en los catálogos i18n (`crates/ui-egui/src/i18n/*.tsv`), recalcular sus `sha256` en `ATTRIBUTION.toml`.
4. **Volver a buscar la marca**, porque el código nuevo del upstream puede traer marcas o enlaces de ArtCraft. Usar la búsqueda de la comprobación final de AR0.1 y retirar lo nuevo antes de cerrar.
5. Listar las dependencias nuevas o actualizadas (diff de `Cargo.lock`) y sus licencias. Si `cargo-deny` está instalado, ejecutar `cargo deny check licenses`.
6. Pasar las puertas de calidad completas.
   - En este equipo, las puertas se ejecutan con `LANG=en_US.UTF-8`, porque 9 pruebas de interfaz dependen del idioma de Windows, y se anota en `REGISTRO.md`.
7. Registrar en `REGISTRO.md` los SHA del upstream integrado y del merge, los conflictos y las dependencias.
8. Revisión (D-13). Después, merge a `main` y push con autorización del titular.

## Fase AR1: prueba de concepto

### AR1.1. Fidelidad y robustez del motor
- Abrir un corpus local, **fuera del repositorio**, formado por PDF sintéticos y personales del titular sin datos de terceros: escaneados, firmados, dañados y grandes (unos 200 MiB).
- Métricas:
  - tasa de apertura;
  - errores y cierres;
  - tiempo hasta la primera página;
  - memoria máxima;
  - comparación visual con PDForte (MuPDF).
- El corpus y las capturas no se suben. En el informe solo van los números.

### AR1.2. Firma con el token del titular (RF-FIRMA-01)
- **Punto de partida:** PdfCraft ya firma con `CurrentUser\MY` por CNG sin exportar la clave (`crates/sign/src/windows.rs`, crate `rustls-cng`). Su ROADMAP indica que está probado con claves de software y que faltan las tarjetas inteligentes, PKCS #11, el sellado de tiempo (B-T) y LTV.
- **Prueba manual del titular** con su token USB o virtual:
  1. ¿Aparece el certificado en la lista?
  2. ¿Firma? ¿Windows pide el PIN en su propio diálogo?
  3. ¿El PDF valida en un validador externo?
- **Observación:** la enumeración abre la clave en modo silencioso (`acquire_key(true)`). RF-FIRMA-01 en PDForte pide no abrir la clave al listar. Hay que comprobar si con el token esto pide PIN, excluye el certificado o funciona bien.
- Si falla, se hace un prototipo en un crate propio y se documenta la causa.

### AR1.3. Interfaz
- Una pantalla mínima con la identidad Amaraza, en español:
  - con DPI al 125 %;
  - dentro de una ventana de 800×560 (como D-08);
  - con el lector de pantalla (AccessKit).
- Valoración del titular de egui frente a la interfaz de PDForte.

### AR1.4. Informe de decisión
- `docs/amaraza/DECISION_AR1.md`: métricas, evidencia, riesgos y recomendación. Lo revisa el revisor (D-13).

## Criterios para seguir (deben cumplirse todos)

1. La fidelidad de renderizado es aceptable en el corpus.
2. La firma con el token funciona.
3. egui es aceptable para los usuarios del titular.
4. El titular se siente capaz de mantener Rust con ayuda de la IA.

Si no se cumplen, PDForte sigue con su plan MP en Python. Como mucho, se aportan mejoras a PdfCraft.

## Fuera de alcance de la prueba

- Traducción completa de la interfaz heredada.
- PDF a Word y Office a PDF.
- Rotulado por lote.
- MSI firmado.
- Publicar binarios.
