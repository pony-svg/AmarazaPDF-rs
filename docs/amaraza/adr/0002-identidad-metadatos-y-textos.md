# ADR-0002. Identidad en metadatos y textos (AR0.4)

- **Estado:** propuesto para revisión D-13.
- **Tarea:** AR0.4 (`docs/amaraza/PLAN.md`).
- **Fecha:** 2026-10-19.
- **Base:** `494f5c0` (`main`, con upstream v0.9.0 + 205 commits y AR0.3 cerrada).
- **Decisiones requeridas del titular:** §7.

---

## 1. Contexto

AR0.1 estableció la marca e identidad visual de Amaraza PDF (nombre del ejecutable, de la ventana, del instalador, carpetas de datos de la app y modo portátil, icono y logotipos del titular). AR0.3 estableció los puntos de enganche (ADR-0001) para que los crates `amaraza-*` extiendan el motor, la automatización y la interfaz sin conflictos al sincronizar con el upstream.

No obstante, en el motor, las herramientas y la interfaz persisten menciones a "PdfCraft" que:
1. Quedan escritas en los **metadatos de los documentos generados o modificados**:
   - el campo `/Producer` en el diccionario `/Info` de los PDF nuevos;
   - el nombre de la aplicación firmante en las firmas digitales (`/Prop_Build /App /Name`);
   - el autor por defecto de los comentarios y anotaciones en las herramientas de automatización;
   - el título en los trabajos de impresión en el navegador web;
   - el nombre y título del servidor MCP;
   - el nombre de familia de las fuentes generadas para texto moldeado (`/FontFamily (PdfCraft Shaped Text)`);
   - el nombre por defecto de los recortes de imágenes copiados al portapapeles.
2. Aparecen en **mensajes de la interfaz** para el usuario:
   - con v0.9.0, existen 23 cadenas fuente únicas en inglés que contienen "PdfCraft";
   - traducidas en los 22 catálogos TSV del upstream (`es`, `de`, `fr`, `ja`, etc.), suman **457 líneas** con "PdfCraft" en la columna de traducción.
3. Permanecen como **activos visuales obsoletos**:
   - 12 capturas de pantalla de la interfaz de PdfCraft en `docs/images/`, registradas en `ATTRIBUTION.toml`, que muestran la marca y el botón de Discord del upstream.

Este ADR diseña la solución técnica para completar la identidad Amaraza PDF en metadatos y textos minimizando al máximo los archivos del upstream modificados para blindar el repositorio frente a futuras sincronizaciones (ARS.n).

---

## 2. Metadatos desde `amaraza-identity`

Toda la identidad de metadatos se centraliza en `crates/amaraza-identity` (L0 standalone), la única fuente de verdad. Al ser capa L0, cualquier crate de capas superiores puede depender de ella sin violar las reglas de estratificación (`cargo xtask layers`).

### 2.1 Constantes en `amaraza-identity`

Se aprovechan las constantes existentes y se añaden las requeridas:

```rust
// crates/amaraza-identity/src/lib.rs
pub const PRODUCT_NAME: &str = "Amaraza PDF";
pub const FOLDER_NAME: &str = "AmarazaPDF";
pub const REPOSITORY: &str = "https://github.com/pony-svg/AmarazaPDF-rs";

// Nuevas constantes para metadatos (AR0.4):
pub const MCP_SERVER_NAME: &str = "amarazapdf";
pub const SHAPED_TEXT_FONT_FAMILY: &str = "Amaraza PDF Shaped Text";
pub const DEFAULT_IMAGE_NAME: &str = "AmarazaPDF";
```

### 2.2 Puntos de enganche de metadatos (7 puntos)

| Punto | Archivo | Línea upstream | Cambio propuesto | Capas y dependencias |
|---|---|---|---|---|
| 1. `Producer` en PDF creados | `crates/create/src/lib.rs` | l. 54 | `PdfString::text(amaraza_identity::PRODUCT_NAME)` | `create` (L4) añade `amaraza-identity` (L0) |
| 2. App en firmas digitales | `crates/sign/src/pdf.rs` | l. 1835 y 2008 | `Object::name(amaraza_identity::PRODUCT_NAME)` | `sign` (L4) añade `amaraza-identity` (L0) |
| 3. Autor por defecto | `crates/automation/src/comments.rs` | l. 15 | `pub(crate) const DEFAULT_AUTHOR: &str = amaraza_identity::PRODUCT_NAME;` | `automation` (L7) añade `amaraza-identity` (L0) |
| 4. Título de impresión web | `crates/ui-egui/src/web_print.rs` | l. 131 | `<title>{amaraza_identity::PRODUCT_NAME}</title>` | `ui-egui` (L7) ya depende de `amaraza-identity` |
| 5. Servidor MCP | `crates/automation/src/mcp.rs` | l. 134, l. 29 | `name: amaraza_identity::MCP_SERVER_NAME`, `title: amaraza_identity::PRODUCT_NAME`, `websiteUrl: amaraza_identity::REPOSITORY` | `automation` ya tiene la dependencia |
| 6. Fuente de texto moldeado | `crates/edit/src/added.rs` | l. 711 | `None => amaraza_identity::SHAPED_TEXT_FONT_FAMILY,` | `edit` (L4) añade `amaraza-identity` (L0) |
| 7. Imagen del portapapeles | `crates/ui-egui/src/image_pick.rs` | l. 77 | `if cleaned.is_empty() { amaraza_identity::DEFAULT_IMAGE_NAME.into() }` | `ui-egui` ya depende de `amaraza-identity` |

**Impacto:** 8 líneas modificadas en 7 archivos del upstream, todas referencias directas a constantes de L0.

---

## 3. Mensajes de la interfaz: mecanismo con pocos enganches

### 3.1 El problema de editar catálogos y fuentes del upstream

Existen 23 cadenas únicas en inglés que contienen "PdfCraft". Si se modificaran directamente:
- Habría que editar **457 líneas a través de los 22 archivos TSV** de `crates/ui-egui/src/i18n/`.
- Habría que editar ~15 archivos `.rs` del upstream (`home.rs`, `dialogs.rs`, `updates.rs`, `ocr_ui.rs`, etc.).
- **Riesgo crítico de conflicto:** en cada sincronización con el upstream (que a menudo reordena o añade claves en los catálogos), los 22 archivos `.tsv` colisionarían masivamente, exigiendo resolución manual propensa a errores.

### 3.2 Mecanismo propuesto: transformación en la capa del fork (Enganche E18)

En AR0.3 se introdujo el enganche E18 en `crates/ui-egui/src/i18n/mod.rs:224`:
```rust
let (mut catalog, errors) = Catalog::parse(&crate::amaraza::catalog_source(self.code, self.source), self.plural_forms());
```

Aprovechando esta misma función `catalog_source(code, source)` en `crates/ui-egui/src/amaraza/mod.rs` (archivo propio del fork), se resuelve la identidad en todos los idiomas sin tocar los archivos del upstream:

1. **Para los 22 idiomas con catálogo (`code != "en"`):**
   - El archivo TSV del upstream tiene el formato `context\tsource\ttranslation`.
   - `catalog_source` recorre las líneas: si la columna de traducción (`translation`) contiene `"PdfCraft"`, la sustituye en memoria por `"Amaraza PDF"`, **salvo si contiene la atribución autorizada** (`"Basado en PdfCraft"` / `"Based on PdfCraft"`), que se preserva intacta.
   - La columna de clave fuente (`source`, en inglés) **no se toca**.
   - Como `Catalog::parse` indexa `c.plain` por la clave `source` original, todas las llamadas `tl!("PdfCraft works offline...")` del código del upstream siguen encontrando la entrada en el catálogo sin fallar, pero la traducción devuelta dice `"Amaraza PDF funciona sin conexión..."`.
2. **Para inglés (`code == "en"`):**
   - En el upstream, el inglés tiene `source: ""` (catálogo vacío; el código asume que el texto fuente ya está en inglés).
   - `catalog_source("en", "")` inyecta las 23 filas que mapean cada clave fuente con "PdfCraft" a su versión en inglés con "Amaraza PDF":
     ```tsv
     \tFiles you open in PdfCraft appear here. Drop a PDF anywhere to open it.\tFiles you open in Amaraza PDF appear here. Drop a PDF anywhere to open it.
     \tPdfCraft works offline. No telemetry, no account, and no cloud processing unless you add a provider.\tAmaraza PDF works offline. No telemetry, no account, and no cloud processing unless you add a provider.
     \tPdfCraft {v} is available.\tAmaraza PDF {v} is available.
     \tPdfCraft {c} is up to date.\tAmaraza PDF {c} is up to date.
     ```
   - `Catalog::parse` carga estas 23 filas en el `c.plain` del catálogo inglés.
   - Cuando la UI corre en inglés, `tl!("PdfCraft works offline...")` busca en `c.plain` y devuelve `"Amaraza PDF works offline..."`. Cualquier otra cadena sin entrada en `c.plain` se devuelve idéntica (`unwrap_or(s)`).
3. **Casos especiales de comandos/enlaces:**
   - `c("help.upstream", "PdfCraft on GitHub", ...)`: el comando del menú Ayuda enlaza a `https://github.com/storytold/pdfcraft`. En la capa de idiomas, se mantiene como atribución honesta ("PdfCraft en GitHub" / "PdfCraft on GitHub") conforme a `00-amaraza.md` §1.

**Ventajas:**
- **0 líneas modificadas** en los 22 catálogos `.tsv` del upstream.
- **0 líneas modificadas** en los archivos `.rs` de la UI del upstream para los textos.
- Riesgo de conflicto en futuras sincronizaciones: **cero**.

---

## 4. Borrado de las 12 capturas de `docs/images/` y `ATTRIBUTION.toml`

Las 12 capturas de `docs/images/` muestran la interfaz original con la marca ArtCraft y botones de Discord:
- `pdfcraft-bookmarks.png`, `pdfcraft-dark.png`, `pdfcraft-find.png`, `pdfcraft-forms.png`, `pdfcraft-layers.png`, `pdfcraft-organize.png`, `pdfcraft-palette.png`, `pdfcraft-properties.png`, `pdfcraft-scripts.png`, `pdfcraft-split.png`, `pdfcraft-twoup.png`, `pdfcraft-viewer.png`.

**Acciones:**
1. Borrar los 12 archivos de `docs/images/`.
2. Borrar sus 12 bloques `[[asset]]` en `ATTRIBUTION.toml` (líneas 2780–2912).
3. Ejecutar `cargo xtask assets --write` para regenerar `ATTRIBUTION.md` (los activos del repositorio pasan de 251 a 239).
4. Actualizar `packaging/linux/io.github.pony_svg.AmarazaPDF.metainfo.xml.in:38` (eliminar el enlace remoto a `pdfcraft-viewer.png`).

---

## 5. Pruebas de guardas contra regresiones

Para garantizar que "PdfCraft" nunca vuelva a aparecer en los metadatos generados ni en los textos que ve el usuario, se implementarán pruebas sintéticas automatizadas:

1. **`producer_is_amaraza_pdf` (en `crates/create/src/lib.rs` o `crates/engine/src/amaraza/tests.rs`):**
   - Crea un documento con `new_document`, lo guarda y comprueba con `lopdf` que `/Producer` es `amaraza_identity::PRODUCT_NAME` y no contiene `"PdfCraft"`.
2. **`signature_app_name_is_amaraza_pdf` (en `crates/sign/src/pdf.rs`):**
   - Firma un documento en memoria y comprueba que en `/Prop_Build /App /Name` figura `"Amaraza PDF"`.
3. **`default_comment_author_is_amaraza_pdf` (en `crates/automation/tests/amaraza.rs`):**
   - Añade un comentario sin autor explícito mediante automatización y comprueba que `author` resultante es `"Amaraza PDF"`.
4. **`shaped_font_family_is_amaraza_pdf` (en `crates/edit/src/tests.rs`):**
   - Moldea texto bidireccional y comprueba que el `/FontFamily` generado es `"Amaraza PDF Shaped Text"`.
5. **`safe_name_fallback_is_amaraza_pdf` (en `crates/ui-egui/src/image_pick.rs`):**
   - Comprueba que `safe_name("  ..  ")` devuelve `"AmarazaPDF"`.
6. **`mcp_server_info_is_amaraza_pdf` (en `crates/automation/tests/amaraza.rs`):**
   - Inicializa el servidor MCP y verifica que `serverInfo.name == "amarazapdf"` y `serverInfo.title == "Amaraza PDF"`.
7. **`ui_catalog_scan_has_no_user_facing_pdfcraft` (en `crates/ui-egui/tests/amaraza.rs`):**
   - Itera por los 23 idiomas (`Lang::all()`) y por todas las entradas de `Catalog`.
   - Verifica que ninguna traducción contiene `"PdfCraft"`, excepto las cadenas de atribución permitidas (`amaraza_identity::BASED_ON` y el enlace al repositorio upstream).

---

## 6. Archivos del upstream modificados (resumen)

| Archivo | Cambio | Líneas |
|---|---|---|
| `crates/create/Cargo.toml` | Dependencia `amaraza-identity = { path = "../amaraza-identity" }` | +1 |
| `crates/create/src/lib.rs` | `Producer` usa `amaraza_identity::PRODUCT_NAME` | 1 |
| `crates/sign/Cargo.toml` | Dependencia `amaraza-identity = { path = "../amaraza-identity" }` | +1 |
| `crates/sign/src/pdf.rs` | `/Prop_Build /App /Name` usa `amaraza_identity::PRODUCT_NAME` | 2 |
| `crates/edit/Cargo.toml` | Dependencia `amaraza-identity = { path = "../amaraza-identity" }` | +1 |
| `crates/edit/src/added.rs` | Familia de fuente de texto moldeado | 1 |
| `crates/automation/src/comments.rs` | `DEFAULT_AUTHOR` usa `PRODUCT_NAME` | 1 |
| `crates/automation/src/mcp.rs` | `serverInfo` y descripción usan `amaraza-identity` | 2 |
| `crates/ui-egui/src/image_pick.rs` | Fallback de nombre seguro usa `FOLDER_NAME` | 1 |
| `crates/ui-egui/src/web_print.rs` | `<title>` de impresión web usa `PRODUCT_NAME` | 1 |
| `packaging/linux/io.github.pony_svg.AmarazaPDF.metainfo.xml.in` | Retirar enlace a captura borrada | 1 |
| `ATTRIBUTION.toml` y `ATTRIBUTION.md` | Borrado de los 12 bloques de capturas | −132 |

Total en código Rust del upstream: **12 líneas modificadas/insertadas en 8 archivos** (más las dependencias internas en 3 `Cargo.toml`). Cero líneas en catálogos `.tsv`.

---

## 7. Decisiones que se piden al titular

1. **Aprobación del mecanismo de traducción:** confirmar si se aprueba interceptar la columna de traducción en memoria vía E18 para evitar modificar 457 líneas en los 22 catálogos `.tsv` del upstream.
2. **Nombre del servidor MCP:** confirmar si se adopta `amarazapdf` como identificador de herramienta/servidor (`name`), con título `Amaraza PDF`.
3. **Nombre de la fuente moldeada:** confirmar `Amaraza PDF Shaped Text` como nombre de familia para texto Type3 moldeado.
4. **Mención upstream en Help:** confirmar que "PdfCraft on GitHub" se mantenga como enlace de atribución al repositorio original o si prefiere "Repositorio original (PdfCraft)".
