# ADR-0002. Identidad en metadatos y textos (AR0.4)

- **Estado:** aceptado con correcciones en la revisión D-13 de `39d3457`; correcciones aplicadas.
- **Tarea:** AR0.4 (`docs/amaraza/PLAN.md`).
- **Fecha:** 2026-10-19.
- **Base:** `494f5c0` (`main`, con upstream v0.9.0 + 205 commits y AR0.3 cerrada).
- **Decisiones del titular:** §7.

---

## 1. Contexto

AR0.1 estableció la marca e identidad visual de Amaraza PDF (nombre del ejecutable, de la ventana, del instalador, carpetas de datos de la app y modo portátil, icono y logotipos del titular). AR0.3 estableció los puntos de enganche (ADR-0001) para que los crates `amaraza-*` extiendan el motor, la automatización y la interfaz sin conflictos al sincronizar con el upstream.

No obstante, en el motor, las herramientas y la interfaz persisten menciones a "PdfCraft" que:
1. Quedan escritas en los **metadatos de los documentos generados o modificados**:
   - el campo `/Producer` en el diccionario `/Info` de los PDF nuevos;
   - el nombre de la aplicación firmante en las firmas digitales (`/Prop_Build /App /Name`);
   - el autor por defecto de los comentarios y anotaciones en las herramientas de automatización;
   - el título en los trabajos de impresión en el navegador web;
   - el nombre, título e instrucciones del servidor MCP y sus metaherramientas;
   - el nombre de familia de las fuentes generadas para texto moldeado (`/FontFamily (PdfCraft Shaped Text)`);
   - la etiqueta descriptiva del perfil ICC sRGB incrustado en exportaciones PDF/A;
   - el nombre por defecto de los recortes de imágenes copiados al portapapeles.
2. Aparecen en **mensajes de la interfaz** para el usuario:
   - con v0.9.0, existen 23 cadenas fuente únicas en inglés que contienen "PdfCraft";
   - traducidas en los 22 catálogos TSV del upstream (`es`, `de`, `fr`, `ja`, etc.), suman **457 líneas** con "PdfCraft" en la columna de traducción;
   - 7 de esas 23 cadenas no han sido traducidas por el upstream en varios de los catálogos (por ejemplo, `"PdfCraft can't check this signature yet: {}."` solo figura en 5 de los 22 catálogos), por lo que sin un mecanismo de respaldo se mostrarían en inglés con "PdfCraft" al usuario de esos idiomas.
3. Permanecen como **activos visuales obsoletos**:
   - 12 capturas de pantalla de la interfaz de PdfCraft en `docs/images/`, registradas en `ATTRIBUTION.toml`, que muestran la marca y el botón de Discord del upstream.

Este ADR diseña la solución técnica para completar la identidad Amaraza PDF en metadatos y textos minimizando al máximo los archivos del upstream modificados para blindar el repositorio frente a futuras sincronizaciones (ARS.n).

---

## 2. Metadatos desde `amaraza-identity`

Toda la identidad de metadatos se centraliza en `crates/amaraza-identity` (L0 standalone), la única fuente de verdad. Al ser capa L0, cualquier crate de capas superiores puede depender de ella sin violar las reglas de estratificación (`cargo xtask layers`).

### 2.1 Constantes en `amaraza-identity`

Se aprovechan las constantes existentes y se añaden las requeridas (reutilizando `FOLDER_NAME` para imágenes):

```rust
// crates/amaraza-identity/src/lib.rs
pub const PRODUCT_NAME: &str = "Amaraza PDF";
pub const FOLDER_NAME: &str = "AmarazaPDF";
pub const REPOSITORY: &str = "https://github.com/pony-svg/AmarazaPDF-rs";

// Nuevas constantes para metadatos (AR0.4):
pub const MCP_SERVER_NAME: &str = "amarazapdf";
pub const SHAPED_TEXT_FONT_FAMILY: &str = "Amaraza PDF Shaped Text";
```

### 2.2 Puntos de enganche de metadatos (8 puntos)

| Punto | Archivo | Línea upstream | Cambio propuesto | Capas y dependencias |
|---|---|---|---|---|
| 1. `Producer` en PDF creados | `crates/create/src/lib.rs` | l. 54 | `PdfString::text(amaraza_identity::PRODUCT_NAME)` | `create` (L4) añade `amaraza-identity` (L0) |
| 2. App en firmas digitales | `crates/sign/src/pdf.rs` | l. 1835 y 2008 | `Object::name(amaraza_identity::PRODUCT_NAME)` | `sign` (L4) añade `amaraza-identity` (L0) |
| 3. Autor por defecto | `crates/automation/src/comments.rs` | l. 15 | `pub(crate) const DEFAULT_AUTHOR: &str = amaraza_identity::PRODUCT_NAME;` | `automation` (L7) añade `amaraza-identity` (L0) en `Cargo.toml` y arista en `Cargo.lock` |
| 4. Título de impresión web | `crates/ui-egui/src/web_print.rs` | l. 131 | `<title>{amaraza_identity::PRODUCT_NAME}</title>` | `ui-egui` (L7) ya depende de `amaraza-identity` |
| 5. Servidor MCP y metaherramientas | `crates/automation/src/mcp.rs` | l. 29, 134, 206, 207, 220, 224, 225, 236 | `name: amaraza_identity::MCP_SERVER_NAME`, `title: amaraza_identity::PRODUCT_NAME`, instrucciones y metaherramientas ("Find Amaraza PDF tools", "Run any Amaraza PDF tool") | `automation` (L7) añade `amaraza-identity` (L0) |
| 6. Fuente de texto moldeado | `crates/edit/src/added.rs` | l. 711 | `None => amaraza_identity::SHAPED_TEXT_FONT_FAMILY,` | `edit` (L4) añade `amaraza-identity` (L0) |
| 7. Perfil ICC sRGB en PDF/A | `crates/preflight/src/icc.rs` | l. 51 | `(b"desc", desc(&format!("sRGB ({})", amaraza_identity::PRODUCT_NAME)))` | `preflight` (L4) añade `amaraza-identity` (L0) |
| 8. Imagen del portapapeles | `crates/ui-egui/src/image_pick.rs` | l. 77 | `if cleaned.is_empty() { amaraza_identity::FOLDER_NAME.into() }` | `ui-egui` ya depende de `amaraza-identity` |

*Nota:* `websiteUrl` en `mcp.rs:134` ya utiliza `pdfcraft_engine::links::GITHUB`, que en el fork ya apunta a `amaraza_identity::REPOSITORY` (`crates/engine/src/links.rs:8`), por lo que no requiere modificación.

---

## 3. Mensajes de la interfaz: mecanismo con pocos enganches

### 3.1 El problema de editar catálogos y fuentes del upstream

Existen 23 cadenas únicas en inglés que contienen "PdfCraft". Si se modificaran directamente:
- Habría que editar **457 líneas a través de los 22 archivos TSV** de `crates/ui-egui/src/i18n/`.
- Habría que editar múltiples archivos `.rs` del upstream (`home.rs`, `dialogs.rs`, `updates.rs`, `ocr_ui.rs`, etc.).
- **Riesgo crítico de conflicto:** en cada sincronización con el upstream (que a menudo reordena o añade claves en los catálogos), los 22 archivos `.tsv` colisionarían masivamente, exigiendo resolución manual propensa a errores.

### 3.2 Mecanismo propuesto: transformación en la capa del fork (Enganche E18)

En AR0.3 se introdujo el enganche E18 en `crates/ui-egui/src/i18n/mod.rs:224`:
```rust
let (mut catalog, errors) = Catalog::parse(&crate::amaraza::catalog_source(self.code, self.source), self.plural_forms());
```

Aprovechando esta misma función `catalog_source(code, source)` en `crates/ui-egui/src/amaraza/mod.rs` (archivo propio del fork), se resuelve la identidad en todos los idiomas sin tocar los catálogos del upstream:

1. **Regla de sustitución y excepción precisa:**
   - La sustitución en cada línea TSV (`context\tsource\ttranslation`) examina la clave de origen `cols[1]` (`source`).
   - **Excepción:** si `cols[1] == "PdfCraft on GitHub"`, la línea se preserva sin cambios, manteniendo la atribución honesta y el enlace al repositorio original (`00-amaraza.md` §1).
   - En cualquier otra línea donde la traducción `cols[2]` contenga `"PdfCraft"`, se sustituye por `"Amaraza PDF"`.
   - La clave fuente `cols[1]` permanece **completamente intacta** (en el inglés original del upstream). Como `Catalog::parse` indexa `c.plain` por la clave original `source`, todas las llamadas `tl!(...)` del código de producción del upstream siguen encontrando la entrada en el catálogo sin modificar una sola línea de código fuente de la UI.
2. **Respaldo para claves ausentes (7 claves incompletas en el upstream):**
   - 7 de las 23 claves con "PdfCraft" no están traducidas en varios catálogos del upstream (por ejemplo, `"PdfCraft can't check this signature yet: {}."` solo está en 5 catálogos). Si un idioma no tiene esa entrada en su TSV, `tr(lang, clave)` caería al texto en inglés que dice "PdfCraft".
   - `catalog_source` comprueba si alguna de las 23 claves no apareció en el TSV del idioma `code`. Si falta, inyecta automáticamente una fila de respaldo (con la traducción al español conforme a AR-D4, o con la versión en inglés de Amaraza PDF) evitando duplicados.
   - De este modo, se garantiza que `tr(lang, clave)` **nunca** devuelve un texto con "PdfCraft" en ninguno de los 22 idiomas.
3. **Para inglés (`code == "en"`):**
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

### 3.3 Alcance y delimitación clara

- **Texto de `OcrError::NoModels` (`crates/ocr/src/lib.rs:33`):**
  - El texto exacto `#[error("the text recognition models are not installed (reinstall PdfCraft, or set PDFCRAFT_MODELS to the folder that holds them)")]` es la clave exacta que indexan los 22 catálogos del upstream. Por tanto, **no se modifica en `crates/ocr`**, ya que rompería la coincidencia de clave en los catálogos. La capa de internacionalización de la UI se encarga de traducirlo y presentar al usuario "Amaraza PDF".
- **Errores internos en inglés del motor y la CLI:**
  - Los mensajes de error internos en inglés para desarrolladores, volcados de depuración y errores de la CLI que no pasan por la capa de internacionalización de la UI quedan **fuera del alcance de esta prueba de concepto** (AR0.4 se enfoca estrictamente en los metadatos persistentes y en lo que ve el usuario final). Modificar enums de error internos en más de 20 crates aumentaría enormemente la superficie de colisión sin aportar valor funcional al usuario.

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

## 5. Pruebas de guardas contra regresiones y adaptación de pruebas existentes

### 5.1 Adaptación de pruebas existentes del upstream

El cambio de textos y metadatos hace que 6 aserciones de pruebas existentes del upstream fallen porque buscan explícitamente "PdfCraft". Se adaptan en la implementación:

1. `crates/ui-egui/tests/ui.rs:114`:
   - De: `h.get_by_label_contains("Files you open in PdfCraft appear here");`
   - A: `h.get_by_label_contains("Files you open in Amaraza PDF appear here");`
2. `crates/ui-egui/tests/ui_group/updates.rs:58` y `:102`:
   - De: `h.get_by_label_contains("PdfCraft 99.0.0 is available");`
   - A: `h.get_by_label_contains("Amaraza PDF 99.0.0 is available");`
3. `crates/ui-egui/tests/signing.rs:536`:
   - De: `h.get_by_label("The key is in the Windows certificate store, which may ask to allow PdfCraft to use it.");`
   - A: `h.get_by_label("The key is in the Windows certificate store, which may ask to allow Amaraza PDF to use it.");`
4. `crates/automation/tests/automation.rs:1300`:
   - De: `assert_eq!(replies[0]["result"]["serverInfo"]["name"], "pdfcraft");`
   - A: `assert_eq!(replies[0]["result"]["serverInfo"]["name"], "amarazapdf");`
5. `crates/ui-egui/src/image_pick.rs:251`:
   - De: `assert_eq!(safe_name("  ..  "), "PdfCraft");`
   - A: `assert_eq!(safe_name("  ..  "), "AmarazaPDF");`
6. `crates/ui-egui/src/amaraza/mod.rs:91`:
   - Adaptar para comprobar que el catálogo inglés contiene las 23 filas de anulación de identidad en lugar de estar vacío.

### 5.2 Pruebas nuevas de guardas contra regresiones

Para garantizar que "PdfCraft" nunca vuelva a aparecer en los metadatos generados ni en los textos que ve el usuario:

1. **`producer_is_amaraza_pdf` (en `crates/create/src/lib.rs` o `crates/engine/src/amaraza/tests.rs`):**
   - Crea un documento con `new_document`, lo guarda y comprueba con `pdfcraft_cos` que `/Producer` es `amaraza_identity::PRODUCT_NAME` y no contiene `"PdfCraft"`.
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
   - Itera por los 23 idiomas (`Lang::all()`) y por las 23 claves con "PdfCraft".
   - Verifica mediante `tr(lang, clave)` que ninguna traducción visible contiene `"PdfCraft"`, evaluando tanto las entradas cargadas como las inyectadas por respaldo, salvo la excepción autorizada (`cols[1] == "PdfCraft on GitHub"`).
8. **`packaging_macos_has_no_pdfcraft_icon` (en `crates/amaraza-identity/tests/identity.rs`):**
   - Verifica que `packaging/macos/Info.plist.in` no contenga `CFBundleTypeIconFile` con `"PdfCraft"` (corregido en ARS.4).

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
| `crates/preflight/Cargo.toml` | Dependencia `amaraza-identity = { path = "../amaraza-identity" }` | +1 |
| `crates/preflight/src/icc.rs` | Etiqueta `desc` del perfil ICC sRGB | 1 |
| `crates/automation/Cargo.toml` | Dependencia `amaraza-identity = { path = "../amaraza-identity" }` | +1 |
| `crates/automation/src/comments.rs` | `DEFAULT_AUTHOR` usa `PRODUCT_NAME` | 1 |
| `crates/automation/src/mcp.rs` | `serverInfo` (nombre y título), instrucciones y metaherramientas (título, descripción y anotaciones) usan `amaraza-identity` / `"Amaraza PDF"` | 8 |
| `crates/ui-egui/src/image_pick.rs` | Fallback de nombre seguro usa `FOLDER_NAME` y aserción | 2 |
| `crates/ui-egui/src/web_print.rs` | `<title>` de impresión web usa `PRODUCT_NAME` | 1 |
| `crates/ui-egui/tests/ui.rs` | Aserción de texto reciente l. 114 | 1 |
| `crates/ui-egui/tests/ui_group/updates.rs` | Aserciones de actualización l. 58 y l. 102 | 2 |
| `crates/ui-egui/tests/signing.rs` | Aserción de texto de almacén l. 536 | 1 |
| `crates/automation/tests/automation.rs` | Aserción de nombre MCP l. 1300 | 1 |
| `Cargo.lock` | Aristas de dependencias locales añadidas (`create`, `sign`, `edit`, `preflight`, `automation` hacia `amaraza-identity`) | ~ |
| `packaging/linux/io.github.pony_svg.AmarazaPDF.metainfo.xml.in` | Retirar enlace a captura borrada | 1 |
| `ATTRIBUTION.toml` y `ATTRIBUTION.md` | Borrado de los 12 bloques de capturas | −132 |

Total en código Rust del upstream: **26 líneas modificadas/insertadas en 17 archivos** (incluyendo 5 archivos de dependencias `Cargo.toml` y 5 archivos de pruebas existentes; `Cargo.lock` actualiza las aristas locales). Cero líneas en los 22 catálogos `.tsv` y cero líneas en código de producción de la UI para textos traducidos.

---

## 7. Decisiones del titular

1. **Aprobación del mecanismo de traducción:** confirmada la interceptación en memoria vía E18 con excepción por clave de origen (`cols[1] == "PdfCraft on GitHub"`) y respaldo automático para las 7 claves incompletas, evitando tocar 457 líneas en los 22 catálogos `.tsv`.
2. **Nombre del servidor MCP:** `amarazapdf` como identificador de herramienta/servidor (`name`), con título `Amaraza PDF`.
3. **Nombre de la fuente moldeada:** `Amaraza PDF Shaped Text` como nombre de familia para texto Type3 moldeado.
4. **Perfil ICC sRGB de PDF/A:** `sRGB (Amaraza PDF)` como descriptor.
5. **Mención upstream en Help:** `PdfCraft on GitHub` se mantiene como enlace de atribución al repositorio original (`00-amaraza.md` §1).
