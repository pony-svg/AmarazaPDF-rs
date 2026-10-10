---
trigger: always_on
---

# Amaraza PDF (Rust): reglas del fork

Este repositorio es `pony-svg/AmarazaPDF-rs`, un **fork público** de `storytold/pdfcraft` (PdfCraft, del equipo ArtCraft). El producto se llama **Amaraza PDF** y su titular es Juan Diego Peraza Amado. Fase actual: **prueba de concepto** para decidir si Amaraza PDF pasa de Python (PDForte) a Rust. Plan y tareas: `docs/amaraza/PLAN.md`. Registro de ejecución: `docs/amaraza/REGISTRO.md`.

## 0. Precedencia y lectura obligatoria

1. Este archivo manda en el fork.
2. `AGENTS.md` y `CLAUDE.md` del upstream siguen vigentes, salvo lo que §2 sustituye. Al empezar cada sesión, **léelos completos con la herramienta de lectura de archivos**, porque pueden llegar truncados como reglas.
3. No edites `AGENTS.md` ni `CLAUDE.md` del upstream: cada cambio en ellos crea conflictos al sincronizar. Las reglas propias van en este archivo.

## 1. Identidad y marca (obligación legal)

- Nombre visible del producto: **Amaraza PDF**. Nunca uses "PdfCraft" ni "ArtCraft" como nombre de la ventana, del ejecutable, del instalador, de la carpeta de datos, del identificador de la app, ni en la publicidad.
- Las marcas ArtCraft (`docs/brand/` original) **no** tienen licencia MIT ni Apache. Su licencia (`docs/brand/LICENSE-brand.txt` del upstream) obliga a retirarlas de cualquier versión modificada. No pueden quedar en ningún archivo publicado ni en ningún binario, tampoco incrustadas con `include_bytes!`.
- Atribución permitida, en texto plano: "Basado en PdfCraft, del equipo ArtCraft", con enlace a `https://github.com/storytold/pdfcraft`. Va en README, en "Acerca de" y en `NOTICE`.
- La marca Amaraza (nombre y logotipo del titular) va en `docs/brand/` con la licencia `LicenseRef-Amaraza-Trademark`, nunca MIT ni Apache. El texto de esa licencia lo aprueba el titular. Sin su confirmación del origen del logotipo, no se añade.
- **Identificadores propios.** No reutilices nunca los del upstream ni los de PDForte: id de la app, `UpgradeCode` del MSI (GUID nuevo, fijo para siempre y distinto del de PDForte), fabricante, carpeta de datos, carpeta de recuperación, marcadores del modo portátil, ProgId y URL.
- **Datos de otras aplicaciones.** El fork nunca lee, mueve ni migra carpetas de PdfCraft, PrintCraft ni PDForte. `migrate_legacy_folders` no puede apuntar a ellas.
- Los nombres internos se mantienen durante la prueba para facilitar la sincronización: crates `pdfcraft-*`, variables `PDFCRAFT_*`, módulos.

## 2. Reglas del upstream que se sustituyen

| Upstream | En este fork |
|---|---|
| `AGENTS.md` §1.2, excepción de marca ArtCraft | §1 de este archivo (marca Amaraza, `LicenseRef-Amaraza-Trademark`) |
| `CLAUDE.md`, "Start every session here" (`plan/STATUS.md`, `plan/execution-plan.md`, `plan/architecture.md`) | `plan/` no existe aquí; es local del equipo storytold. Usa `docs/amaraza/PLAN.md` y `docs/amaraza/REGISTRO.md`. No inventes el contenido de `plan/` |
| `CLAUDE.md`, "update ROADMAP.md every session" | No toques `ROADMAP.md` ni `parity/` salvo que una tarea AR lo pida |
| `CLAUDE.md`, "Don't stop to ask" | Pregunta al titular en los casos de §6 |
| Commits `M1.4: …` | `ARx.y: …` (por ejemplo `AR0.1: identidad Amaraza`), para no confundirlos con los hitos del upstream |
| Créditos de contribuidores (`contributors/`, craftrules) | No aplica. No edites `contributors/` |
| Enlaces a `craftrules`, `../photocraft`, `../craft-fonts` | Son internos del upstream o no existen aquí. No supongas su contenido |

Todo lo demás del upstream se aplica completo:
- activos y atribución (§1), con `cargo xtask assets`;
- sala limpia (§2): nada de Adobe y no copiar código GPL/AGPL;
- control por agentes (§3) y "nunca cerrarse" (§4): sin `unwrap` ni `panic`, sin `unsafe`, entradas hostiles;
- capas, fidelidad, solo Rust y puertas de calidad.

## 3. Relación con PDForte (Amaraza PDF en Python)

- PDForte es **privado** y tiene licencia AGPL-3.0. Este fork es **público** y tiene licencia MIT o Apache-2.0.
- Puedes usar PDForte como referencia **funcional**: requisitos, decisiones D-01…D-13, RF-FIRMA-01 y lo que comprueban sus pruebas. **No copies su código.** Reescribe en Rust a partir del requisito. Un traslado literal necesita la autorización expresa del titular, anotada en el commit.
- Nunca traigas al fork actas, auditorías, capturas, rutas ni documentos de PDForte.
- No modifiques PDForte desde una sesión de este repositorio.

## 4. Licencia (decisión del titular del 2026-10-09)

- Durante la prueba, todo el código propio se publica con licencia `MIT OR Apache-2.0`, igual que el upstream. No añadas cabeceras AGPL ni cambies `LICENSE-*`.
- Conserva `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE` y `ATTRIBUTION.*` con todos sus avisos de terceros: Foxit y Adobe CMap (BSD-3), fuentes OFL, Lucide (ISC).
- `NOTICE` debe decir que los archivos fueron modificados (Apache-2.0 §4).
- Cambiar la licencia lo decide el titular al cerrar la prueba.

## 5. Repositorio público: privacidad (equivale a D-02, más estricta)

- Todo lo que se sube es público y permanente, aunque luego se borre.
- **Prohibido en el repositorio, en las pruebas, en las capturas, en los registros y en los mensajes de commit:**
  - documentos, nombres o datos institucionales o de terceros;
  - rutas del perfil (`C:\Users\<usuario>`) y nombres de usuario;
  - certificados reales, archivos `.pfx` o `.p12`, PIN, claves y tokens.
- PDF de prueba: solo sintéticos, generados por la prueba o por las herramientas del upstream. El corpus personal del titular se queda **fuera** del repositorio.
- Certificados de prueba: solo efímeros y creados por la prueba, con limpieza garantizada. Las pruebas automáticas no usan el almacén real ni el token del titular.
- **Firma (RF-FIRMA-01):**
  - nunca exportar la clave privada;
  - prohibidos `NCryptExportKey`, `CryptExportKey`, `PFXExportCertStore*` y `NCRYPT_ALLOW_EXPORT_FLAG`;
  - al listar certificados no se debe pedir PIN.
- Antes de cada commit, revisa `git diff --cached` buscando rutas, nombres y datos personales.

## 6. Pregunta siempre al titular antes de…

- hacer push, crear etiquetas o releases, publicar binarios o activar GitHub Actions;
- sincronizar con el upstream (merge o *Sync fork*). Es la tarea `ARS.n` y sigue el procedimiento del plan: rama propia, merge sin rebase, conservar la identidad Amaraza y volver a buscar la marca ArtCraft;
- ejecutar pruebas con `--ignored` o `--include-ignored`, porque algunas escriben certificados temporales en el almacén real de Windows o en el llavero;
- añadir o actualizar dependencias de Cargo, incluido regenerar `Cargo.lock`;
- instalar herramientas en el equipo (rustup, Build Tools, CMake, NASM…);
- borrar, mover o sobrescribir archivos no rastreados (`??`). Prohibidos `rm -rf`, `Remove-Item -Recurse -Force` y `git clean` sin confirmación expresa. Antes, lista el contenido. Recuerda el incidente del 2026-10-07;
- cambiar la licencia, la marca o los identificadores fijos (`UpgradeCode`, id de la app);
- abrir PR o issues en `storytold/pdfcraft`;
- permitir `unsafe` (el upstream lo prohíbe; la excepción la firma el titular).

Nunca: `push --force`, reescribir historial ya publicado; reescribir commits locales no subidos solo con autorización expresa del titular, ni push a `upstream`.

## 7. Protocolo de cada sesión

1. **Orientar.**
   - `git status`, `git log --oneline -5`, rama actual y `git remote -v`. Debe ser `origin` = `pony-svg/AmarazaPDF-rs` y `upstream` = `storytold/pdfcraft`, con el push deshabilitado.
   - Antes del primer commit, comprobar que `git config user.email` es el correo del proyecto.
   - Leer `docs/amaraza/PLAN.md` y la última entrada de `docs/amaraza/REGISTRO.md`.
2. **Planificar** una tarea `ARx.y` pequeña: archivos que se tocan, cómo se prueba y cómo se revierte.
3. **Implementar.** Primero la prueba que falla; después el cambio. Cada defecto corregido deja una prueba de regresión sintética.
4. **Verificar:**
   - `cargo fmt --check`
   - `cargo clippy --workspace -- -D warnings`
   - `cargo test --workspace`
   - `cargo xtask assets`

   Adjunta la salida real. Nada es "verde" sin el comando ejecutado. Si un fallo ya existía antes de tu cambio, demuéstralo con la línea base registrada.
5. **Registrar** en `docs/amaraza/REGISTRO.md`: tarea, comandos, resultado y SHA completo de 40 caracteres. Copia el SHA de `git rev-parse` y compruébalo con `git cat-file -t`; nunca de memoria.
6. **Commit:** una tarea `ARx.y` por commit, solo en verde, con rutas explícitas en `git add`. Si hay agentes en paralelo, usa un `CARGO_TARGET_DIR` propio.
7. **Revisión (D-13):** una sesión distinta (el revisor) revisa el diff y la evidencia antes de dar la tarea por hecha. Lo que las pruebas no cubren (interfaz, token, instalador) necesita evidencia real del titular. Las capturas generadas se marcan como "evidencia sintética".

## 8. Windows

- Toolchain: Rust estable ≥ 1.90 (`rust-version` del workspace) con objetivo MSVC y Visual Studio Build Tools con C++. `aws-lc-sys` puede pedir CMake y NASM. Si falta algo, detente y pídelo (§6).
- Para abrir la app durante el desarrollo, usa el modo portátil (marcador junto al ejecutable) o la carpeta de datos propia de Amaraza. Nunca las de PdfCraft ni las de PDForte.
- Temporales en `%TEMP%` o en `target/`, nunca en la raíz del repositorio.
- Los textos nuevos de la interfaz de Amaraza van en español. Traducir la interfaz heredada es una tarea aparte, que no se mezcla con otras.
