# ADR-0002. Identidad en metadatos y textos (AR0.4)

- **Estado:** aceptado (D-13, 2026-10-19); implementado en AR0.4.
- **Tarea:** AR0.4 (`docs/amaraza/PLAN.md`).
- **Fecha:** 2026-10-19.
- **Base:** `494f5c0` (`main`, con upstream v0.9.0 + 205 commits y AR0.3 cerrada).
- **Decisiones del titular:** AR-D6, AR-D7, AR-D8 (§8).

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
   - en v0.9.0 existen 23 cadenas fuente únicas en inglés que contienen "PdfCraft" (457 líneas traducidas en los 22 catálogos TSV del upstream);
   - 7 de esas 23 cadenas no han sido traducidas por el upstream en varios de los catálogos (por ejemplo, `"PdfCraft can't check this signature yet: {}."` solo figura en 5 de los 22 catálogos), por lo que sin un mecanismo de respaldo se mostrarían en inglés con "PdfCraft" al usuario de esos idiomas;
   - el comando `help.upstream` ("PdfCraft on GitHub") permanece registrado en el menú Ayuda y en la paleta ⌘K.
3. Permanecen como **activos visuales obsoletos**:
   - 12 capturas de pantalla de la interfaz de PdfCraft en `docs/images/`, registradas en `ATTRIBUTION.toml`, que muestran la marca y el botón de Discord del upstream.

Este ADR diseña la solución técnica para completar la identidad Amaraza PDF en metadatos y textos minimizando al máximo los archivos del upstream modificados para blindar el repositorio frente a futuras sincronizaciones (ARS.n).

---

## 2. Metadatos desde `amaraza-identity`

Toda la identidad de metadatos se centraliza en `crates/amaraza-identity` (L0 standalone), la única fuente de verdad. Al ser capa L0, cualquier crate de capas superiores puede depender de ella sin violar las reglas de estratificación (`cargo xtask layers`).

### 2.1 Constantes en `amaraza-identity`

Se aprovechan las constantes existentes y se añaden las requeridas (AR-D7):

```rust
// crates/amaraza-identity/src/lib.rs
pub const PRODUCT_NAME: &str = "Amaraza PDF";
pub const FOLDER_NAME: &str = "AmarazaPDF";
pub const REPOSITORY: &str = "https://github.com/pony-svg/AmarazaPDF-rs";
pub const UPSTREAM_REPOSITORY: &str = "https://github.com/storytold/pdfcraft";
pub const BASED_ON: &str = "Basado en PdfCraft, del equipo ArtCraft";

// Nuevas constantes para metadatos (AR0.4 / AR-D7):
pub const MCP_SERVER_NAME: &str = "amarazapdf";
pub const SHAPED_TEXT_FONT_FAMILY: &str = "Amaraza PDF Shaped Text";
pub const ICC_PROFILE_DESC: &str = "sRGB (Amaraza PDF)";
```

### 2.2 Puntos de enganche de metadatos (8 puntos)

| Punto | Archivo | Líneas upstream | Cambio propuesto | Capas y dependencias |
|---|---|---|---|---|
| 1. `Producer` en PDF creados | `crates/create/src/lib.rs` | l. 54 | `PdfString::text(amaraza_identity::PRODUCT_NAME)` | `create` (L4) añade `amaraza-identity` (L0) |
| 2. App en firmas digitales | `crates/sign/src/pdf.rs` | l. 1835 y 2008 | `Object::name(amaraza_identity::PRODUCT_NAME)` | `sign` (L4) añade `amaraza-identity` (L0) |
| 3. Autor por defecto | `crates/automation/src/comments.rs` | l. 15 | `pub(crate) const DEFAULT_AUTHOR: &str = amaraza_identity::PRODUCT_NAME;` | `automation` (L7) añade `amaraza-identity` (L0) en `Cargo.toml` y arista en `Cargo.lock` |
| 4. Título de impresión web | `crates/ui-egui/src/web_print.rs` | l. 131 | `<title>{amaraza_identity::PRODUCT_NAME}</title>` | `ui-egui` (L7) ya depende de `amaraza-identity` |
| 5. Servidor MCP y metaherramientas | `crates/automation/src/mcp.rs` | l. 29, 134, 206, 207, 220, 224, 225, 236 | `name: amaraza_identity::MCP_SERVER_NAME`, `title: amaraza_identity::PRODUCT_NAME`, instrucciones y metaherramientas ("Find Amaraza PDF tools", "Run any Amaraza PDF tool") | `automation` (L7) añade `amaraza-identity` (L0) |
| 6. Fuente de texto moldeado | `crates/edit/src/added.rs` | l. 711 | `None => amaraza_identity::SHAPED_TEXT_FONT_FAMILY,` | `edit` (L4) añade `amaraza-identity` (L0) |
| 7. Perfil ICC sRGB en PDF/A | `crates/preflight/src/icc.rs` | l. 51 | `(b"desc", desc(amaraza_identity::ICC_PROFILE_DESC))` | `preflight` (L4) añade `amaraza-identity` (L0) |
| 8. Imagen del portapapeles | `crates/ui-egui/src/image_pick.rs` | l. 77 | `if cleaned.is_empty() { amaraza_identity::FOLDER_NAME.into() }` | `ui-egui` ya depende de `amaraza-identity` |

*Nota:* `websiteUrl` en `mcp.rs:134` ya utiliza `pdfcraft_engine::links::GITHUB`, que en el fork ya apunta a `amaraza_identity::REPOSITORY` (`crates/engine/src/links.rs:8`), por lo que no requiere modificación.

---

## 3. Mensajes de la interfaz: mecanismo dinámico en el fork (Enganche E18)

### 3.1 El problema de editar catálogos y fuentes del upstream

Si se modificaran directamente los catálogos TSV y los mensajes de origen:
- Habría que editar **457 líneas a través de los 22 archivos TSV** de `crates/ui-egui/src/i18n/`.
- Habría que editar múltiples archivos `.rs` del upstream (`home.rs`, `dialogs.rs`, `updates.rs`, `ocr_ui.rs`, etc.).
- **Riesgo crítico de conflicto:** en cada sincronización con el upstream (que a menudo reordena o añade claves en los catálogos), los 22 archivos `.tsv` colisionarían masivamente, exigiendo resolución manual propensa a errores.

### 3.2 Mecanismo propuesto: transformación en memoria y respaldo robusto (AR-D6)

En AR0.3 se introdujo el enganche E18 en `crates/ui-egui/src/i18n/mod.rs:224`:
```rust
let (mut catalog, errors) = Catalog::parse(&crate::amaraza::catalog_source(self.code, self.source), self.plural_forms());
```

Aprovechando esta misma función `catalog_source(code, source)` en `crates/ui-egui/src/amaraza/mod.rs` (archivo propio del fork), se resuelve la identidad en todos los idiomas de forma dinámica y robusta ante futuras sincronizaciones:

1. **Dinamismo en la detección de claves (sin listas fijas):**
   - El conjunto de claves que contienen `"PdfCraft"` no se fija a una constante estática de 23 cadenas.
   - Las claves se descubren a partir de los propios catálogos embebidos: cualquier clave `source` que contenga `"PdfCraft"` en cualquier catálogo del upstream se incluye en el procesamiento.
2. **Sustitución en memoria para idiomas con catálogo (`code != "en"`):**
   - En cada línea TSV (`context\tsource\ttranslation`), si la traducción `cols[2]` contiene `"PdfCraft"`, se sustituye en memoria por `"Amaraza PDF"`.
   - **Sin excepciones:** Al retirarse `help.upstream` de los comandos (AR-D8, §4), la clave `"PdfCraft on GitHub"` queda en los 22 catálogos TSV como línea huérfana, sin usar; no se tocan los catálogos para borrarla. Al no existir el comando en menús ni paleta, **desaparece la necesidad de excepciones por clave**. Toda aparición de `"PdfCraft"` en la columna traducida pasa a `"Amaraza PDF"`.
   - La clave fuente `cols[1]` permanece **intacta** (en el inglés original del upstream). Como `Catalog::parse` indexa `c.plain` por la clave original `source`, todas las llamadas `tl!(...)` del código del upstream siguen encontrando la entrada en el catálogo sin modificar una sola línea de código fuente de la UI.
   - **Limitación gramatical aceptada (AR-D6):** La sustitución mecánica en memoria puede generar discordancias de caso o sufijo en idiomas con declinación compleja (húngaro, esloveno, turco…); el titular evaluó y aceptó expresamente esta limitación técnica para la prueba de concepto, dado que en español e inglés la redacción resulta natural y gramaticalmente impecable.
3. **Respaldo único en inglés con "Amaraza PDF":**
   - Para las 7 claves identificadas que el upstream no tradujo en varios idiomas (por ejemplo, `"PdfCraft can't check this signature yet: {}."`, ausente en 17 de los 22 catálogos), `catalog_source` comprueba si alguna clave con `"PdfCraft"` falta en el catálogo del idioma `code`.
   - Si falta, inyecta automáticamente una fila de respaldo en inglés adaptado:
     `\t<clave_origen>\t<clave_origen_con_Amaraza_PDF>`
   - Se adopta de forma determinista el respaldo en **inglés con "Amaraza PDF"** (coincidiendo con el comportamiento de fallback del upstream, pero garantizando que jamás se muestra la marca original).
4. **Para inglés (`code == "en"`):**
   - En el upstream, el inglés tiene `source: ""` (catálogo vacío; el código asume que el texto fuente ya está en inglés).
   - `catalog_source("en", "")` inyecta las filas que mapean cada clave fuente con "PdfCraft" a su versión en inglés con "Amaraza PDF":
     ```tsv
     \tFiles you open in PdfCraft appear here. Drop a PDF anywhere to open it.\tFiles you open in Amaraza PDF appear here. Drop a PDF anywhere to open it.
     \tPdfCraft works offline. No telemetry, no account, and no cloud processing unless you add a provider.\tAmaraza PDF works offline. No telemetry, no account, and no cloud processing unless you add a provider.
     \tPdfCraft {v} is available.\tAmaraza PDF {v} is available.
     \tPdfCraft {c} is up to date.\tAmaraza PDF {c} is up to date.
     ```
   - `Catalog::parse` carga estas filas en `c.plain` del catálogo inglés.
   - Cuando la UI corre en inglés, `tl!("PdfCraft works offline...")` busca en `c.plain` y devuelve `"Amaraza PDF works offline..."`. Cualquier otra cadena sin entrada en `c.plain` se devuelve idéntica (`unwrap_or(s)`).

Este mecanismo requiere **0 líneas modificadas en los 22 catálogos TSV** y **0 líneas modificadas en el código de producción de la UI para traducción de mensajes**.

### 3.3 Delimitación y elementos fuera de alcance (con motivo)

1. **Marcador técnico `%PdfCraft` en flujos de contenido de edición (`crates/edit/src/lib.rs:330–334`):**
   - *Motivo:* Los literales binarios `b"q %PdfCraft\n"`, `b"\nQ %PdfCraft\n"` y `b"Q %PdfCraft\n"` son delimitadores estructurales internos que el motor de edición escribe y parsea para reconocer y envolver modificaciones previas. Alterar este marcador rompería el reconocimiento de ediciones anteriores e introduciría incompatibilidades técnicas con documentos ya editados, sin ningún beneficio para el usuario.
2. **`<title>PdfCraft</title>` en `apps/pdfcraft-web/index.html`:**
   - *Motivo:* El objetivo de compilación y empaquetado web no forma parte del alcance de la prueba de concepto (limitada estrictamente al entorno de escritorio en Windows).
3. **Mensajes de registro técnico (`log::info!`, `log::debug!`, etc.):**
   - *Motivo:* Son diagnósticos técnicos para desarrolladores emitidos a la consola o terminal; no persisten en los documentos PDF ni son visibles por el usuario en la interfaz gráfica.
4. **Errores internos en inglés del motor y la CLI:**
   - *Motivo:* Los mensajes de depuración en enums de error de bajo nivel en más de 20 crates no pasan por la capa de internacionalización de la UI. Modificarlos ampliaría enormemente la superficie de colisión al sincronizar sin aportar valor funcional al usuario de la prueba.
5. **Texto exacto de `OcrError::NoModels` (`crates/ocr/src/lib.rs:33`):**
   - *Motivo:* Es la clave exacta que indexan los 22 catálogos TSV de la UI; se preserva intacta en `crates/ocr` y la interfaz (`ocr_ui.rs`) se encarga de presentar al usuario "Amaraza PDF".

---

## 4. Retirada de "PdfCraft en GitHub" de la interfaz y enlace en "Acerca de" (AR-D8)

### 4.1 Problema y restricción legal

- En AR0.1 se registró el comando `help.upstream` ("PdfCraft on GitHub") en `crates/engine/src/commands.rs:375` (dentro del menú Ayuda) y en `links::LINKS` (`crates/engine/src/links.rs:27`), lo que hacía que apareciera en el menú Ayuda y en la paleta de comandos ⌘K.
- El titular decidió expresamente (**AR-D8**) retirar "PdfCraft en GitHub" del menú Ayuda y de la paleta.
- **Restricción legal (`00-amaraza.md` §1):** La ventana "Acerca de" debe seguir mostrando la atribución `"Basado en PdfCraft, del equipo ArtCraft"` con enlace interactivo a `https://github.com/storytold/pdfcraft`.

### 4.2 Solución de diseño

1. **Retirada de comandos y menús:**
   - Se elimina la entrada `c("help.upstream", "PdfCraft on GitHub", HELP, ...)` de `COMMANDS` en `crates/engine/src/commands.rs:375`. Al no ser un comando registrado, deja de figurar en el menú Ayuda y en la paleta ⌘K.
   - Se retira `help.upstream` del array `links::LINKS` en `crates/engine/src/links.rs:27`. `LINKS` pasa a contener únicamente `help.github` ("Amaraza PDF en GitHub").
   - Las constantes públicas `UPSTREAM` (`amaraza_identity::UPSTREAM_REPOSITORY`) y `BASED_ON` (`amaraza_identity::BASED_ON`) permanecen en `crates/engine/src/links.rs`.
2. **Enlace interactivo en "Acerca de":**
   - En `crates/ui-egui/src/dialogs.rs`, la línea de atribución `BASED_ON` se presenta como un enlace interactivo directo utilizando el patrón diferido propio de `dialogs.rs` (una bandera dentro del bucle de dibujo y la apertura fuera de él):
     ```rust
     // Dentro del bucle de dibujo del diálogo About (l. 1175):
     if ui.add(egui::Link::new(egui::RichText::new(pdfcraft_engine::links::BASED_ON).color(t.text_muted)))
         .on_hover_text(pdfcraft_engine::links::UPSTREAM)
         .clicked()
     {
         open_upstream = true;
     }

     // Fuera del bucle de dibujo, junto a link_command (l. ~1391):
     if open_upstream {
         app.open_url(pdfcraft_engine::links::UPSTREAM);
     }
     ```
   - A continuación, `widgets::project_links(ui)` renderiza únicamente el botón destacado del repositorio propio (`help.github`, "Amaraza PDF en GitHub").
   - De este modo, "Acerca de" cumple la obligación legal de atribución y enlace sin contaminar el menú de Ayuda ni la paleta de comandos.
   - `README.md` y `NOTICE` mantienen sus menciones y enlaces intactos.
   - Como consecuencia directa, **desaparece la excepción por clave `"PdfCraft on GitHub"` en `catalog_source`**: toda mención a "PdfCraft" se transforma en "Amaraza PDF".

---

## 5. Borrado de las 12 capturas de `docs/images/` y `ATTRIBUTION.toml`

Las 12 capturas de `docs/images/` muestran la interfaz original con la marca ArtCraft y botones de Discord:
- `pdfcraft-bookmarks.png`, `pdfcraft-dark.png`, `pdfcraft-find.png`, `pdfcraft-forms.png`, `pdfcraft-layers.png`, `pdfcraft-organize.png`, `pdfcraft-palette.png`, `pdfcraft-properties.png`, `pdfcraft-scripts.png`, `pdfcraft-split.png`, `pdfcraft-twoup.png`, `pdfcraft-viewer.png`.

**Acciones:**
1. Borrar los 12 archivos de `docs/images/`.
2. Borrar sus 12 bloques `[[asset]]` en `ATTRIBUTION.toml` (líneas 2780–2912).
3. Ejecutar `cargo xtask assets --write` para regenerar `ATTRIBUTION.md` (los activos del repositorio pasan de 251 a 239).
4. Actualizar `packaging/linux/io.github.pony_svg.AmarazaPDF.metainfo.xml.in:38` (eliminar el enlace remoto a `pdfcraft-viewer.png`).

---

## 6. Pruebas de guardas contra regresiones y adaptación de pruebas existentes

### 6.1 Adaptación de pruebas existentes del upstream (7 pruebas)

El cambio de textos, metadatos y enlaces requiere adaptar 7 pruebas del upstream que asertan explícitamente cadenas antiguas:

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
   - Adaptar para comprobar que el catálogo inglés contiene las filas de anulación de identidad en lugar de estar vacío.
7. `crates/ui-egui/tests/ui_group/links.rs:35–62`:
   - En `help_commands_open_each_link`: comprobar que `help.upstream` ya no es un comando registrado (`assert!(commands::command("help.upstream").is_none());`).
   - En `about_dialog_credits_pdfcraft_and_opens_the_project_links`: comprobar que hacer clic en el texto `links::BASED_ON` abre `links::UPSTREAM`.

### 6.2 Pruebas nuevas de guardas contra regresiones

Para garantizar que "PdfCraft" nunca vuelva a aparecer en los metadatos generados ni en los textos que ve el usuario:

1. **`ui_code_scan_has_no_unhandled_pdfcraft` (en `crates/ui-egui/tests/amaraza.rs`):**
   - Escanea todos los archivos fuente Rust de la interfaz (`crates/ui-egui/src/` y `apps/pdfcraft/src/`) y extrae los textos literales suministrados a `tl!`, `tl_ctx!`, `notify_tr` y `notify_fmt`.
   - Si detecta cualquier literal que contenga `"PdfCraft"`, evalúa mediante `tr(lang, &key)` en todos los 23 idiomas (`Lang::all()`) que la traducción devuelta NO contenga `"PdfCraft"`.
   - Si en una sincronización futura el upstream añade un mensaje nuevo con "PdfCraft", esta prueba fallará de inmediato en `cargo test`, alertando antes de integrar.
2. **`producer_is_amaraza_pdf` (en `crates/create/src/lib.rs` o `crates/engine/src/amaraza/tests.rs`):**
   - Crea un documento con `new_document`, lo guarda y comprueba con `pdfcraft_cos` que `/Producer` es `amaraza_identity::PRODUCT_NAME` y no contiene `"PdfCraft"`.
3. **`signature_app_name_is_amaraza_pdf` (en `crates/sign/src/pdf.rs`):**
   - Firma un documento en memoria y comprueba que en `/Prop_Build /App /Name` figura `"Amaraza PDF"`.
4. **`default_comment_author_is_amaraza_pdf` (en `crates/automation/tests/amaraza.rs`):**
   - Añade un comentario sin autor explícito mediante automatización y comprueba que `author` resultante es `"Amaraza PDF"`.
5. **`shaped_font_family_is_amaraza_pdf` (en `crates/edit/src/tests.rs`):**
   - Moldea texto bidireccional y comprueba que el `/FontFamily` generado es `"Amaraza PDF Shaped Text"`.
6. **`safe_name_fallback_is_amaraza_pdf` (en `crates/ui-egui/src/image_pick.rs`):**
   - Comprueba que `safe_name("  ..  ")` devuelve `"AmarazaPDF"`.
7. **`mcp_server_info_is_amaraza_pdf` (en `crates/automation/tests/amaraza.rs`):**
   - Inicializa el servidor MCP y verifica que `serverInfo.name == "amarazapdf"` y `serverInfo.title == "Amaraza PDF"`.
8. **`packaging_macos_has_no_pdfcraft_icon` (en `crates/amaraza-identity/src/lib.rs`):**
   - Verifica en `the_other_platforms_packages_use_the_app_id` que `packaging/macos/Info.plist.in` no contenga `CFBundleTypeIconFile` con `"PdfCraft"` (corregido en ARS.4).

---

## 7. Archivos del upstream modificados (resumen)

| Archivo | Cambio | Líneas |
|---|---|---|
| `crates/create/Cargo.toml` | Dependencia `amaraza-identity = { path = "../amaraza-identity" }` | +1 |
| `crates/create/src/lib.rs` | `Producer` usa `amaraza_identity::PRODUCT_NAME` | 1 |
| `crates/sign/Cargo.toml` | Dependencia `amaraza-identity = { path = "../amaraza-identity" }` | +1 |
| `crates/sign/src/pdf.rs` | `/Prop_Build /App /Name` usa `amaraza_identity::PRODUCT_NAME` | 2 |
| `crates/edit/Cargo.toml` | Dependencia `amaraza-identity = { path = "../amaraza-identity" }` | +1 |
| `crates/edit/src/added.rs` | Familia de fuente de texto moldeado (l. 711) y declaración de `amaraza_tests` (l. 1152, marcador ADR-0001) | 2 |
| `crates/preflight/Cargo.toml` | Dependencia `amaraza-identity = { path = "../amaraza-identity" }` | +1 |
| `crates/preflight/src/icc.rs` | Etiqueta `desc` del perfil ICC sRGB usa `ICC_PROFILE_DESC` | 1 |
| `crates/automation/Cargo.toml` | Dependencia `amaraza-identity = { path = "../amaraza-identity" }` | +1 |
| `crates/automation/src/comments.rs` | `DEFAULT_AUTHOR` usa `PRODUCT_NAME` | 1 |
| `crates/automation/src/mcp.rs` | `serverInfo` (nombre y título), instrucciones y metaherramientas (título, descripción y anotaciones) usan `amaraza-identity` / `"Amaraza PDF"` | 8 |
| `crates/engine/src/commands.rs` | Retirada del comando `help.upstream` del menú Ayuda | 1 |
| `crates/engine/src/links.rs` | Retirada de `help.upstream` del array `LINKS` | 1 |
| `crates/ui-egui/src/dialogs.rs` | `BASED_ON` se presenta como enlace interactivo directo a `UPSTREAM` | 2 |
| `crates/ui-egui/src/image_pick.rs` | Fallback de nombre seguro usa `FOLDER_NAME` y aserción | 2 |
| `crates/ui-egui/src/web_print.rs` | `<title>` de impresión web usa `PRODUCT_NAME` | 1 |
| `crates/ui-egui/tests/ui.rs` | Aserción de texto reciente l. 114 | 1 |
| `crates/ui-egui/tests/ui_group/updates.rs` | Aserciones de actualización l. 58 y l. 102 | 2 |
| `crates/ui-egui/tests/signing.rs` | Aserción de texto de almacén l. 536 | 1 |
| `crates/automation/tests/automation.rs` | Aserción de nombre MCP l. 1300 | 1 |
| `crates/ui-egui/tests/ui_group/links.rs` | Adaptación de pruebas de enlace directo y ausencia de comando | 2 |
| `Cargo.lock` | Aristas de dependencias locales añadidas (`create`, `sign`, `edit`, `preflight`, `automation` hacia `amaraza-identity`) | ~ |
| `packaging/linux/io.github.pony_svg.AmarazaPDF.metainfo.xml.in` | Retirar enlace a captura borrada | 1 |
| `ATTRIBUTION.toml` y `ATTRIBUTION.md` | Borrado de los 12 bloques de capturas | −132 |

Total en código Rust del upstream: **33 líneas modificadas/insertadas/retiradas en 21 archivos** (incluyendo 5 archivos de dependencias `Cargo.toml` y 6 archivos de pruebas existentes; `Cargo.lock` actualiza las aristas locales). Cero líneas en los 22 catálogos `.tsv` y cero líneas en código de producción de la UI para textos traducidos.

---

## 8. Decisiones del titular (2026-10-19)

1. **AR-D6 (Mecanismo en memoria vía E18):** Aprobada la transformación en memoria mediante `catalog_source` para sustituir `"PdfCraft"` por `"Amaraza PDF"`, evitando modificar 457 líneas en los 22 catálogos TSV del upstream. Se acepta expresamente la limitación gramatical en idiomas con declinación compleja (húngaro, esloveno, turco…), constatando que en español e inglés la lectura es perfectamente correcta.
2. **AR-D7 (Identificadores de herramientas y metadatos):**
   - Servidor MCP: identificador `amarazapdf`, título `Amaraza PDF`.
   - Fuente de texto moldeado: `Amaraza PDF Shaped Text`.
   - Perfil ICC sRGB en PDF/A: `sRGB (Amaraza PDF)`.
3. **AR-D8 (Retirada de "PdfCraft en GitHub" de menús y enlace en "Acerca de"):**
   - Se retira el comando `help.upstream` ("PdfCraft en GitHub") del menú Ayuda y de la paleta de comandos ⌘K.
   - En la ventana "Acerca de", se conserva la atribución legal `"Basado en PdfCraft, del equipo ArtCraft"` como un enlace interactivo directo a `https://github.com/storytold/pdfcraft` (`links::UPSTREAM`) sin necesidad de registrar un comando de menú.
   - `README.md` y `NOTICE` mantienen su atribución intacta.
   - Al retirarse el comando, desaparece por completo la necesidad de excepciones por clave en `catalog_source`.
