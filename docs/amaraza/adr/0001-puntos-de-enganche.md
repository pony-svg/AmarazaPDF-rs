# ADR-0001. Puntos de enganche para los crates `amaraza-*`

- **Estado:** aceptado el 2026-10-10, en la revisión D-13 de `aef6710`, con correcciones menores ya aplicadas. Decisiones del titular: AR-D3 a AR-D5 (§10).
- **Tarea:** AR0.3 (`docs/amaraza/PLAN.md`).
- **Fecha:** 2026-10-10.
- **Base:** `0f2f47f`. Su código es el de `e9b9ee6`: upstream v0.5.0 con AR0.1. Los números de línea se refieren a esa base.

## 1. Contexto

El fork tiene que añadir funciones propias (la primera, la firma por lote) sin que cada sincronización se llene de conflictos. El plan pide que lo nuevo vaya en crates propios y que el upstream solo se toque en el enganche. Tres registros del upstream deciden qué existe:

| Registro | Dónde | Forma | Quién lo usa |
|---|---|---|---|
| Comandos | `crates/engine/src/commands.rs:157`, `COMMANDS` | `const` con una lista de `CommandSpec` | menús (`registry_menu`), atajos, paleta, canal de control, `command_list` y `command_run`, las 13 pruebas `*_covers_commands_and_catalogue`, `every_registered_command_is_implemented`, `every_command_icon_exists` |
| Herramientas | `crates/automation/src/tools.rs:117`, `tools()` | `Vec<ToolDef>` creado en tiempo de ejecución | `Automation::call` (despacho por nombre, `lib.rs:148`), CLI `tools` y `run`, MCP, `tool_table_is_well_formed`, `hostile_args` |
| Pasos del asistente de acciones (`misc.action-wizard-run`) | `crates/engine/src/actions.rs:10`, `enum Step` | enum cerrado con `id`, `label`, `arg`, `arg_mut`, `all` y `from_id` | selector de pasos (`actions_ui.rs:313`), acciones guardadas (`encode` y `decode`, `actions_ui.rs:175–200`), `apply_step` (`actions.rs:149`), `run_on` (`actions.rs:203`), herramientas `action_list` y `action_run` (`a11y.rs:302` y `:313`) |

**Restricción de capas.** `CommandSpec` y `Step` viven en `pdfcraft-engine` (L6). Un crate `amaraza-*` que usa el motor queda por encima, y el motor no puede llamarlo, porque se formaría un ciclo. Por eso un crate no puede añadir entradas a estos registros por su cuenta: se declaran dentro del crate que define el tipo.

## 2. Decisión

Se sigue el reparto del upstream: el registro dice qué es cada comando y el frontend implementa qué hace (cabecera de `commands.rs`). Hay tres piezas.

1. **Declaración** en `crates/engine/src/amaraza.rs`, un archivo nuevo del fork dentro del motor. Solo datos:
   - los comandos del fork (`COMMANDS`);
   - los pasos del fork (`crate::amaraza::Step`), con su id, etiqueta y argumento;
   - la aplicación de los pasos que no necesitan contexto.
2. **Lógica** en el crate nuevo `crates/amaraza-ext`: una API sin interfaz sobre `Session` ("primero una API del motor", `AGENTS.md` §3). Aquí va todo lo que tiene ramas: el saludo de la demostración ahora y la firma por lote después.
3. **Superficies**: un módulo del fork en cada frontend.
   - `crates/automation/src/amaraza.rs`: los `ToolDef` y el despacho de las herramientas `amaraza_*` hacia `amaraza-ext`.
   - `crates/ui-egui/src/amaraza/`: la ejecución de los comandos `amaraza.*`, el menú "Amaraza" y la capa de traducciones del fork.

Los archivos del upstream solo reciben la línea que conecta cada pieza (§3). Cada pieza tiene su espacio de nombres:
- comandos `amaraza.*`;
- herramientas y pasos `amaraza_*`.

Así nunca coinciden con ids del upstream, y las pruebas de unicidad del upstream lo vigilan.

Los identificadores del código del fork van en inglés, como en `amaraza-identity`, y los textos que ve el usuario, en español (AR-D5).

### 2.1 Comandos (`COMMANDS`)

`COMMANDS` sigue siendo un `const` con todos los comandos, para que sus consumidores vean los del fork sin cambiarlos. La concatenación se hace al compilar:

```rust
// crates/engine/src/commands.rs
const UPSTREAM_COMMANDS: &[CommandSpec] = &[ /* lista del upstream, sin cambios */ ];

/// Every command: the upstream's, then the fork's (Amaraza, ADR-0001).
pub const COMMANDS: &[CommandSpec] =
    &crate::amaraza::commands::<{ UPSTREAM_COMMANDS.len() + crate::amaraza::COMMANDS.len() }>(UPSTREAM_COMMANDS);
```

- `crate::amaraza::commands::<N>` es una `const fn` que copia las dos listas.
- Solo se llama desde ese `const`, así que se evalúa al compilar: un `N` incorrecto o un índice fuera de rango detienen la compilación (`error[E0080]`) y nunca llegan a la ejecución.
- Lo comprobé en un archivo aparte con rustc 1.99: compila, clippy (`all` y `pedantic`) no da avisos y un `N` erróneo hace fallar la compilación.

Reglas para los comandos del fork:
- Sin atajo de teclado, para no competir con atajos futuros del upstream. Si hiciera falta alguno, `ids_and_shortcuts_are_unique` detecta el choque al sincronizar.
- Un icono que ya exista en el juego de Lucide incluido (`every_command_icon_exists`). Añadir un icono es un cambio de activos.
- `menu: Some("Amaraza")` o uno de los menús del upstream.

### 2.2 Herramientas (`tools()`)

```rust
// crates/automation/src/tools.rs
/// Every tool: the upstream's, then the fork's (Amaraza, ADR-0001).
pub fn tools() -> Vec<ToolDef> {
    let mut v = upstream_tools();
    v.extend(crate::amaraza::tools());
    v
}

/// Every tool, in a stable order.
fn upstream_tools() -> Vec<ToolDef> { /* cuerpo del upstream, sin cambios */ }
```

El despacho es un brazo con guarda al **principio** del `match` de `Automation::call`. El upstream añade sus brazos al final, antes de `other =>` (`lib.rs:615`):

```rust
let out = match name {
    n if n.starts_with(amaraza::PREFIX) => amaraza::call(self, n, &a)?, // Amaraza (ADR-0001)
    "doc_open" => self.doc_open(&a)?,
```

- `amaraza::call` devuelve `Result<Value>` y usa los mismos ayudantes (`Args`, `resolve`, `write_atomic`).
- Hereda así el confinamiento a `--root`, la escritura atómica y los errores accionables (`AGENTS.md` §3).
- `check_args` rechaza los argumentos desconocidos antes del despacho, como en todas las herramientas.

### 2.3 Menús y ejecución en la interfaz

- La paleta, la ventana de atajos y `command_list` muestran los comandos del fork sin enganche, porque recorren `COMMANDS`.
- **Menú propio "Amaraza"**, antes de "Help" en el menú principal (`chrome.rs:392`). Es una línea, `crate::amaraza::main_menu(app, ui);`, que dibuja `registry_menu(app, ui, "Amaraza")`.
  - Un comando del fork con `menu: Some("File")` (u otro menú del upstream) aparece al final de ese menú, sin enganche.
- **Ejecución:** un brazo con guarda al principio del `match id` de `execute_unguarded` (`commands.rs:126`): `fork if fork.starts_with("amaraza.") => return crate::amaraza::execute(self, fork),`.
  - Hereda la comprobación de `Needs` y la guarda contra pánicos `pdfcraft_engine::guard` de `execute`.

### 2.4 Traducciones

Las 13 pruebas `*_covers_commands_and_catalogue` exigen la etiqueta de cada comando en `ja`, `it`, `pt-br`, `es`, `te`, `zh-hans`, `fr`, `de`, `ru`, `bg`, `hu`, `uk` y `ar`. En `it` y `pt-br` exigen también cada menú.

AR0.1 metió sus textos en los 15 catálogos del upstream y en sus `sha256`, lo que crea 30 puntos de conflicto por cada texto nuevo. Este ADR usa en su lugar una **capa del fork**:
- **Archivo:** `crates/ui-egui/src/amaraza/i18n.tsv`, con las columnas `idioma`, `contexto`, `origen` y `traducción`. Un solo archivo para todos los idiomas, con una entrada en `ATTRIBUTION.toml` junto a las del logotipo Amaraza.
- **Enganche:** en `i18n/mod.rs:178`, `Catalog::parse(&crate::amaraza::catalog_source(self.code, self.source), …)`.
  - `catalog_source` devuelve el catálogo del upstream seguido de las filas de ese idioma, sin la primera columna.
  - Las filas pasan por el mismo analizador, con las mismas validaciones: marcadores, puntos suspensivos y duplicados.
- **Origen en español** (`00-amaraza.md` §8, como en AR0.1). Mientras no haya traducción, la fila repite el texto en español. La cabecera del archivo marca esas filas como "sin traducir", para que se lea como lo que es: el texto se muestra en español. Traducir es otra tarea.

### 2.5 Capas

`amaraza-ext` va en `Layer(6)`, como extensión de la fachada del motor, con la arista lateral `("amaraza-ext", "engine")` en `SIDEWAYS`.
- `automation` y `ui-egui` (L7) dependen de él hacia abajo, sin reglas nuevas.
- No usa ningún crate de interfaz (`UI_MIN_LAYER = 7`).
- La alternativa, L7 con dos aristas laterales, toca más líneas (§7).

### 2.6 Pasos del asistente de acciones

Se añade una variante de extensión **tipada** al enum cerrado:

```rust
pub enum Step {
    /// Amaraza PDF fork steps (ADR-0001).
    Amaraza(crate::amaraza::Step),
    RecognizeText,
    // …
}
```

- **`id` y `label`:** un brazo al principio de su `match self` (`Step::Amaraza(p) => p.id(),`). Su primer brazo (`Step::RecognizeText => …`) casi nunca cambia.
- **`arg` y `arg_mut`:** una guarda justo antes del `match`, porque su primer brazo es el patrón `Step::AddWatermark(s) | … => Some(s)`, que el upstream edita cada vez que añade un paso con argumento. Un brazo pegado a esa línea chocaría en cada sincronización:
  ```rust
  // Amaraza (ADR-0001)
  if let Step::Amaraza(p) = self {
      return p.arg_mut();
  }
  match self {
  ```
  Lo comprobé aparte con rustc 1.99, también el préstamo `&mut` de `arg_mut`.
- **`all()`:** pasa a llamarse `upstream_all()` y un envoltorio nuevo devuelve `upstream_all()` seguido de `crate::amaraza::Step::all()` envuelto en `Step::Amaraza`. `from_id` usa `all()`, así que no cambia.
- **`apply_step`:** un brazo al principio del `match step` (`actions.rs:152`): `Step::Amaraza(p) => return crate::amaraza::apply(s, id, p, log),`.

Con esto, el selector de pasos, las acciones guardadas, `action_list` y `action_run` ven los pasos del fork sin más cambios. Los pasos que necesitan contexto, como un certificado, no pueden ejecutarse dentro de `run_on`. Para ellos `apply` devuelve un error accionable, y los ejecuta el corredor del fork (§4).

## 3. Líneas del upstream que cambian

Cada enganche lleva el marcador `ADR-0001` en un comentario: al final de la línea si ocupa una sola, o en la línea anterior si es un bloque. `git grep -n "ADR-0001" -- ':!docs'` los lista, y tras cada sincronización deben aparecer todas las filas de esta tabla. Columnas de la tabla:
- **Ancla:** la línea en `0f2f47f` junto a la que va el cambio.
- **+:** líneas insertadas.
- **M:** líneas modificadas.

Historial del upstream (`git log -L` sobre cada ancla, hasta `0f2f47f`):
- las anclas de E4, E7, E8 (`arg` y `arg_mut`), E12 y E16 solo cambiaron en el commit que las creó;
- la de E18 cambió 2 veces;
- la de E17, el final del menú View, cambió 4 veces. Por eso es la única de riesgo medio.

### 3.1 Demostración (fase 1, AR0.3)

| # | Archivo | Ancla | Cambio | + | M | Conflicto al sincronizar y cómo se resuelve |
|---|---|---|---|---|---|---|
| E1 | `xtask/src/layers.rs` | tras l. 79 `("engine", Class::Layer(6)),` | comentario y `("amaraza-ext", Class::Layer(6)),` | 2 | | Solo si el upstream añade un crate L6 justo ahí: conservar ambos |
| E2 | `xtask/src/layers.rs` | tras l. 99 `pub const SIDEWAYS: … = &[` | `("amaraza-ext", "engine"),` | 1 | | El upstream añade aristas al final: no se tocan |
| E3 | `crates/engine/src/lib.rs` | tras l. 15 `pub mod actions;` | `pub mod amaraza;` | 1 | | Solo si el upstream crea un módulo entre `actions` y `catalog`: conservar ambos |
| E4 | `crates/engine/src/commands.rs` | l. 157 | `pub const COMMANDS: &[CommandSpec] = &[` → `const UPSTREAM_COMMANDS: &[CommandSpec] = &[` | | 1 | El upstream añade comandos dentro de la lista, no en su cabecera. Si choca: lado del upstream y volver a renombrar |
| E5 | `crates/engine/src/commands.rs` | tras l. 346 `];` | línea en blanco, comentario y el nuevo `pub const COMMANDS` | 4 | | Las inserciones del upstream antes de `];` no la tocan |
| E6 | `crates/engine/src/actions.rs` | tras l. 10 `pub enum Step {` | comentario y `Amaraza(crate::amaraza::Step),` | 2 | | El upstream añade variantes al final |
| E7 | `crates/engine/src/actions.rs` | tras l. 35 `impl Step {`; l. 37 | envoltorio `pub fn all()`; l. 37 → `fn upstream_all() -> Vec<Step> {` | 7 | 1 | Como E4 |
| E8 | `crates/engine/src/actions.rs` | tras l. 56 y 73 (`match self {` de `id` y `label`); antes de l. 91 y 98 (`match self {` de `arg` y `arg_mut`) | un brazo `Step::Amaraza(p) => …` en `id` y `label`; la guarda `if let` de §2.6 en `arg` y `arg_mut` | 10 | | El upstream añade brazos al final de `id` y `label`. En `arg` y `arg_mut` edita la l. 92 o la 99, y la guarda queda separada de ellas por la línea del `match` |
| E9 | `crates/engine/src/actions.rs` | tras l. 152 `let edit = match step {` | `Step::Amaraza(p) => return crate::amaraza::apply(s, id, p, log),` | 1 | | Ídem |
| E10 | `crates/automation/Cargo.toml` | tras l. 12 `pdfcraft-engine = …` | comentario y `amaraza-ext = { path = "../amaraza-ext" }` | 2 | | Bajo |
| E11 | `crates/automation/src/lib.rs` | tras l. 14 `mod a11y;` | `mod amaraza;` | 1 | | Como E3 |
| E12 | `crates/automation/src/lib.rs` | tras l. 148 `let out = match name {` | brazo con guarda `amaraza::PREFIX` | 1 | | El upstream añade brazos antes de `other =>` (l. 615) |
| E13 | `crates/automation/src/tools.rs` | antes de l. 116; l. 117 | envoltorio `pub fn tools()`; l. 117 → `fn upstream_tools() -> Vec<ToolDef> {` | 7 | 1 | Las herramientas nuevas del upstream entran dentro de `vec![…]`: no lo tocan |
| E14 | `crates/ui-egui/Cargo.toml` | tras l. 13 (dependencia de AR0.1) | `amaraza-ext = { path = "../amaraza-ext" }` | 1 | | Ya es un trozo del fork (AR0.1) |
| E15 | `crates/ui-egui/src/lib.rs` | tras l. 26 `mod actions_ui;` | `mod amaraza;` | 1 | | Como E3 |
| E16 | `crates/ui-egui/src/commands.rs` | tras l. 126 `match id {` | brazo con guarda `"amaraza."` | 1 | | El upstream añade brazos antes de `_ => return false` (l. 570) |
| E17 | `crates/ui-egui/src/chrome.rs` | antes de l. 392 (menú Help) | `crate::amaraza::main_menu(app, ui);` | 1 | | Medio: si el upstream añade un menú justo antes de Help, conservar ambas líneas, la del fork justo antes de Help |
| E18 | `crates/ui-egui/src/i18n/mod.rs` | l. 178 | `self.source` → `&crate::amaraza::catalog_source(self.code, self.source)` | | 1 | Si el upstream cambia la carga de catálogos, aplicar el envoltorio a su nueva fuente |
| E19 | `ATTRIBUTION.toml` (y `ATTRIBUTION.md`, generado) | junto a las entradas de `docs/brand/amaraza-logo.*` | entrada de `crates/ui-egui/src/amaraza/i18n.tsv` | 11 | | Bloque del fork |
| E20 | `Cargo.lock` | generado | paquete `amaraza-ext` y sus aristas internas, sin crates externos | | | Lado del upstream y dejar que cargo vuelva a añadir los paquetes internos. Comprobar que el diff solo trae paquetes del fork |

**Total de la fase 1:** 12 archivos del upstream (10 `.rs` y 2 `Cargo.toml`), con 43 líneas insertadas y 4 modificadas, más `ATTRIBUTION.*` y `Cargo.lock`. Ninguna cambia lógica del upstream. Archivos nuevos del fork, sin riesgo de conflicto:
- `crates/amaraza-ext/{Cargo.toml, clippy.toml, src/lib.rs}`;
- `crates/engine/src/amaraza.rs`;
- `crates/automation/src/amaraza.rs` y `crates/automation/tests/amaraza.rs`;
- `crates/ui-egui/src/amaraza/{mod.rs, i18n.tsv}` y `crates/ui-egui/tests/amaraza.rs`.

### 3.2 Firma por lote (fase 2, futura; no entra en AR0.3)

| # | Archivo | Ancla | Cambio | + | M |
|---|---|---|---|---|---|
| F1 | `crates/ui-egui/src/actions_ui.rs` | tras l. 41 `pub fn run_action_on(…) {` | `let Some((action, files)) = crate::amaraza::intercept_run(self, action, files) else { return };` | 1 | |
| F2 | `crates/automation/src/signing.rs` | l. 120 `fn open_id` | `pub(crate) fn open_id`, para reutilizar la resolución de identidades (almacén de Windows, Llavero, `.p12` dentro de `--root`) en vez de duplicarla | | 1 |

El paso `amaraza::Step::Sign` y la herramienta `amaraza_sign_batch` entran por los enganches de la fase 1, sin más líneas del upstream. `amaraza-ext` añadiría la dependencia interna `pdfcraft-sign`, que cambia `Cargo.lock`.

## 4. Paso "Firmar" por lote (diseño; no se implementa ahora)

### 4.1 Declaración

`crate::amaraza::Step::Sign { reason: String }`, con id `amaraza_sign`, etiqueta "Firmar con certificado" y el motivo como argumento.
- El certificado **nunca se guarda en la acción**: las acciones guardadas no llevan referencias a identidades, y la elección se hace en cada ejecución.
- Si se ejecuta por el camino del upstream (`run_on`, o `action_run`), `apply` devuelve `EditError::Invalid` con el texto: "Firmar necesita elegir un certificado: ejecuta la acción desde el asistente de Amaraza PDF o con la herramienta amaraza_sign_batch".

### 4.2 API en `amaraza_ext::batch`

```rust
/// A validated action: the steps before signing, and the signing reason if it signs.
pub struct Plan { /* before: Action, sign: Option<String> */ }

/// The identity chosen for the whole batch. Its private key is only used while signing (A6).
pub struct Signer<'a> {
    pub id: &'a pdfcraft_sign::DigitalId,
    pub location: Option<String>,
}

pub struct FileOutcome {
    pub name: String,
    pub bytes: Arc<Vec<u8>>,
    pub log: Vec<String>,
    pub prior_signatures: usize,
}

pub enum BatchError {
    SignNotLast,
    AlreadySigned { count: usize },
    ReadOnly(String),
    NoSigner,
    Engine(String),
    Verification(String),
}

/// Checks the action before touching any file: `Sign` at most once, and only as the last step.
pub fn plan(action: &Action) -> Result<Plan, BatchError>;

/// Runs the plan on one file.
pub fn run(
    plan: &Plan,
    name: &str,
    bytes: Arc<Vec<u8>>,
    signer: Option<&Signer<'_>>,
    progress: impl FnMut(usize, usize),
) -> Result<FileOutcome, BatchError>;
```

`run`, archivo por archivo:
1. **Abrir** el archivo en una `Session`. Si es de solo lectura, error. Se cuentan las firmas existentes (`n`).
2. **A5 (D-03):** si `n > 0` y hay pasos antes de Firmar, se rechaza ese archivo con `AlreadySigned`. El mensaje dice: "El documento ya tiene {n} firmas; los pasos previos a Firmar las invalidarían. Quita esos pasos o firma este archivo aparte." Es más estricto que `run_on` del upstream, como pide el criterio de `01-amaraza-heredadas.md`.
3. **Pasos previos:** se ejecutan con `run_on` del upstream, sobre una acción con esos pasos. No se copia su lógica.
4. **Firmar:** `Session::sign(doc, signer.id, SignOptions { reason, location, .. })`.
   - La firma es incremental (PAdES B-B), en un campo invisible.
   - Si el documento ya estaba firmado, se firma encima y el registro lo avisa: "Firmado sobre {n} firmas existentes" (A5).
   - Si el documento está certificado sin cambios permitidos, decide el firmador del upstream, y su error se devuelve para ese archivo.
5. **A1:** se abre el resultado en otra `Session`. Debe tener `n + 1` firmas y la nueva con el resumen intacto (estado distinto de `invalid`; `unknown` se admite si el firmante no es de confianza). Si no, `Verification` y no se devuelve nada.
6. **Registro A8/A9:** `log::info!("event=amaraza.batch_sign.file result=ok prior_signatures={n}")`, sin nombre, ruta ni titular del certificado. El `log` que ve el usuario sí puede decir "Firmado".

### 4.3 Interfaz (F1)

`intercept_run` deja pasar sin cambios las acciones que no firman. Si la acción firma:
1. **Selector de identidades:** `pdfcraft_sign::windows::identities()`, que no pide PIN pero abre la clave en modo silencioso (§4.5).
2. **Confirmación (A3):** "Se firmarán {n} archivos con «{nombre del certificado}». La firma no se puede deshacer en los archivos resultantes; los originales no cambian."
3. **Carpeta de resultados.** Un archivo que ya existe **nunca se sobrescribe**: ese archivo da error. Así ningún original se sustituye y no hace falta la copia de A2.
4. **Hilo de trabajo** como el del upstream, con progreso y un resumen de correctos y fallidos (A10).
5. **Diálogo:** se puede usar a 800×560 (A13), con colores de `theme::Tokens` (A12) y en español (§8). En la web, Firmar se rechaza ("solo en escritorio").

### 4.4 Automatización

Herramienta del fork `amaraza_sign_batch`, con los argumentos `{ paths, folder, id, password?, reason?, location?, action? | steps?, confirm }`:
- Va marcada como `destructive`.
- Exige `confirm: true`: es el equivalente de A3 para los agentes.
- Resuelve la identidad igual que `sign_document` (F2).
- Escribe con `write_atomic` en `folder`, dentro de `--root`, y rechaza los archivos que ya existen.
- `action_run` del upstream con `amaraza_sign` devuelve, para cada archivo, el error de §4.1.

### 4.5 Claves (A6 y RF-FIRMA-01)

- **A6 se cumple solo en parte al listar.** `identities()` del upstream no pide PIN, pero abre la clave en modo silencioso para comprobar su algoritmo (`crates/sign/src/windows.rs:80`), y A6 pide no abrirla.
  - Cumplirla del todo exige listar sin abrir la clave, solo con los datos del certificado.
  - Se decide en AR1.2, junto con lo que haga el token del titular con la apertura silenciosa (si lo excluye de la lista o lo afecta de otro modo).
  - Leer el almacén desde un crate del fork necesitaría `rustls-cng`, una decisión de la fase 2 (§10).
- **La clave solo se usa para firmar al firmar.** `WindowsKey::sign` la abre sin el modo silencioso (`windows.rs:26`), y el proveedor muestra **su propio diálogo de PIN**. Amaraza PDF nunca muestra un campo de PIN ni recibe el PIN.
- **¿PIN por archivo?** `WindowsKey::sign` vuelve a abrir la clave en cada firma, así que algunos tokens podrían pedir el PIN una vez por archivo. AR1.2 lo comprueba. Si ocurre, hay dos opciones, que se deciden al empezar la fase 2:
  - un `ExternalKey` del fork que mantenga un solo identificador abierto durante el lote y lo cierre al final (necesita `rustls-cng` en un crate del fork);
  - proponer el cambio al upstream.
- **Nunca se exporta.** El código del fork no llama a `pkcs12::write` con identidades del almacén, ni a `NCryptExportKey`, `CryptExportKey`, `PFXExportCertStore*` o `NCRYPT_ALLOW_EXPORT_FLAG`. Una prueba del fork busca esos nombres en sus fuentes.
- **El `DigitalId` solo vive en memoria** durante el lote y se suelta al terminar. Nunca se serializa ni aparece en los registros.
- **Pruebas:** una identidad efímera generada en memoria (`PrivateKey::generate_p256` y `Certificate::self_signed`), sin el almacén real y sin `--ignored` (`00-amaraza.md` §5, A4, A11).

## 5. Demostración "hola Amaraza": plan de verificación

### 5.1 Qué se construye

- **API** (`amaraza-ext`): `hello::greeting(&Session, Option<DocId>) -> Result<Greeting, EditError>`.
  - `Greeting { product, version, pages: Option<usize> }`; `text()` da "Hola, Amaraza PDF 0.5.0", y añade " · 3 páginas" si hay documento.
  - Solo lectura.
- **Comando** `amaraza.hello`: etiqueta "Hola, Amaraza", menú "Amaraza", `Needs::Nothing` e icono `hand`, que ya está en el juego incluido. Muestra el texto en un aviso.
- **Herramienta** `amaraza_hello` `{ doc? }`: de solo lectura, con `cmd("amaraza.hello")`. Devuelve `{ product, version, pages?, text }`.
- **Paso** `amaraza_hello` ("Saludo de Amaraza", sin argumento): añade "Hola, Amaraza" al registro del paso sin cambiar el documento. Sirve para probar el enganche de los pasos.

### 5.2 Pruebas

1. **Unitarias de `amaraza-ext`:** sin documento, con un PDF sintético y con un id que no existe (error).
2. **De extremo a extremo en `crates/automation/tests/amaraza.rs`** (`AGENTS.md` §3):
   - `amaraza_hello` sin documento y con un PDF sintético abierto con `doc_open` en una raíz temporal;
   - `command_list` enlaza `amaraza.hello` con `amaraza_hello`, y `command_run {id: "amaraza.hello"}` funciona;
   - `action_list` incluye el paso `amaraza_hello`, y `action_run` con `steps: [{step: "amaraza_hello"}]` escribe el resultado con el saludo en su registro;
   - un argumento desconocido se rechaza con un mensaje accionable.
3. **De interfaz con kittest en `crates/ui-egui/tests/amaraza.rs`:**
   - ejecutar el comando muestra el aviso;
   - Menú ▸ Amaraza ▸ "Hola, Amaraza" se ve y funciona;
   - la paleta encuentra "Hola";
   - el selector de pasos muestra el paso del fork, y `encode` y `decode` lo conservan;
   - en cada idioma, el catálogo con la capa del fork no da errores y contiene todos los textos del fork.
4. **Barridos del upstream** que pasan a cubrir el fork sin editarlos:
   - `ids_and_shortcuts_are_unique`, `every_command_icon_exists` y `every_registered_command_is_implemented`;
   - las 13 `*_covers_commands_and_catalogue`;
   - `tool_table_is_well_formed`;
   - `hostile_args`: argumentos hostiles y confinamiento a la raíz.
5. **Comprobaciones de `xtask` y clippy:** `cargo xtask layers`, `cargo xtask assets` y `cargo clippy -p amaraza-ext --all-targets -- -D warnings` (A7).
6. **CLI:** `pdfcraft-cli tools` lista `amaraza_hello`, y `pdfcraft-cli run --script` funciona con `[{"tool": "amaraza_hello", "args": {}}]`.
7. **Captura** de Menú ▸ Amaraza con `pdfcraft --control` y `pdfcraft-cli ui screenshot`, marcada como "evidencia sintética".
8. **Puertas completas** con `LANG=en_US.UTF-8`, `CARGO_BUILD_JOBS=6` y el portátil a la corriente.

**Detección de enganches perdidos.** Cada prueba de los puntos 2 y 3 recorre un enganche (comando, herramienta, despacho, menú, capa de traducciones, paso). Si una sincronización pierde uno, falla su prueba.

### 5.3 Commits de la implementación (después de aprobar)

Una tarea AR0.3 en varios commits, todos en verde:
1. crate `amaraza-ext` y capas;
2. declaración en el motor, con los enganches de `COMMANDS` y de `Step`;
3. automatización y sus pruebas;
4. interfaz, capa de traducciones y `ATTRIBUTION`;
5. registro: tabla de archivos del upstream modificados.

## 6. Reglas heredadas (`01-amaraza-heredadas.md`) en este diseño

| Regla | Cómo se cumple |
|---|---|
| A1 | La firma por lote verifica volviendo a abrir el resultado antes de entregarlo. Cada frontend escribe por un único camino atómico (`write_atomically`, `write_atomic`). La demostración no escribe PDF propios |
| A2 | Nunca se sustituye un archivo existente: se rechaza. Sin sustitución no hacen falta copias, y el ayudante compartido de A2 queda para la tarea que sustituya archivos |
| A3 | Confirmación en la interfaz y `confirm` en la automatización. El lote no toca el historial de los documentos abiertos |
| A4 | Sin `cfg!(test)`. Las pruebas pasan la identidad y las carpetas por parámetros |
| A5 | §4.2, pasos 2 y 4 |
| A6, RF-FIRMA-01 | **En parte.** Al firmar se cumple: el PIN lo pide el diálogo de Windows, la clave nunca se exporta y el `DigitalId` solo vive en memoria. Al listar no: `identities()` del upstream abre la clave en modo silencioso (`crates/sign/src/windows.rs:80`). No pide PIN, pero A6 pide no abrirla. Cumplirla del todo exige listar sin abrir la clave, y se decide en AR1.2 (§4.5) |
| A7 | `amaraza-ext` lleva su `clippy.toml` (las 4 claves de la raíz y `cognitive-complexity-threshold = 15`) y `#![warn(clippy::cognitive_complexity)]`, y se comprueba con `cargo clippy -p amaraza-ext --all-targets`. **Excepción explícita:** los módulos de pegamento dentro de crates del upstream (`engine/src/amaraza.rs`, `automation/src/amaraza.rs`, `ui-egui/src/amaraza/`) quedan con el umbral 25 de la raíz, porque clippy no admite un umbral por módulo. Se mitiga sin lógica con ramas en ellos: solo despachan y declaran, y llevan también `#![warn(clippy::cognitive_complexity)]`. La lógica con ramas va en `amaraza-ext`. El archivo del upstream que más crece es `actions.rs` (20 líneas sobre 220). Los grandes (`tools.rs`, `automation/src/lib.rs`, `ui-egui/src/commands.rs`, `chrome.rs`, `i18n/mod.rs`) crecen entre 0 y 7 |
| A8, A9 | Registros `event=amaraza.*` con `error_code=…`, sin rutas, nombres, titular del certificado ni PIN |
| A10 | `?` en todo el código. Cada archivo del lote da su resultado o su error |
| A11 | Las pruebas solo usan carpetas temporales |
| A12, A13 | Diálogos del fork con `theme::Tokens`, que se pueden usar a 800×560 |
| A14 | Ninguna copia extra en disco: solo los resultados, donde los pide el usuario |

Del upstream:
- **Nunca cerrarse:** sin `unwrap` ni `panic!`. El `assert!` de `commands::<N>` solo se evalúa al compilar, porque solo se llama desde un `const`, y así lo dice su comentario.
- Sin `unsafe`, capas comprobadas y solo Rust.
- **Activos:** la capa de traducciones lleva su entrada en `ATTRIBUTION.toml`.
- No se tocan `parity/` ni `ROADMAP.md` (`00-amaraza.md` §2): la demostración no es una función de Acrobat.
- Ninguna dependencia externa nueva.

## 7. Alternativas descartadas

- **Editar los registros del upstream en su sitio** (entradas del fork dentro de `COMMANDS`, de `tools()`, de cada `match` y de los catálogos). Crea menos archivos, pero mezcla las líneas del fork con las zonas que el upstream edita más, así que habría conflictos en cada sincronización. Contradice la estrategia del plan y A7.
- **Registro en tiempo de ejecución en el upstream** (complementos que las apps registran al arrancar). Es lo que más desacopla, pero:
  - `COMMANDS` se usa como `const` en unos 20 sitios y en 13 pruebas, y habría que reescribir esos consumidores;
  - añade estado global;
  - las pruebas que crean `PdfCraftApp` directamente no verían los registros.

  Se podría proponer al upstream más adelante (decisión del titular, §6 de las reglas).
- **App aparte `apps/amaraza`**, con su CLI y su interfaz sobre los crates del upstream. Evita enganches en la automatización, pero no en la interfaz, porque los menús y los comandos están en `ui-egui`, y duplica el armazón de la app.
- **Paso genérico `Step::Extension { id, arg }`** con un registro de manejadores. Pierde la comprobación exhaustiva de los `match` y necesita estado global. Se prefiere `amaraza::Step` tipado.
- **Declaraciones en un crate de capa baja** (por ejemplo, L5) del que dependa el motor. Evita un archivo del fork dentro del motor, pero añade una arista del motor hacia el fork y necesita los mismos enganches. Cambiar a esta opción más tarde sería fácil.
- **Traducciones del fork en los 15 catálogos del upstream** (como AR0.1): 30 puntos de conflicto por texto. Variante con una tabla en Rust: evita `ATTRIBUTION`, pero deja las traducciones fuera del formato validado.
- **`amaraza-ext` en L7** con dos aristas laterales: una línea más, y lo coloca como frontend cuando no tiene interfaz.
- **Firma por lote con argumentos nuevos en `action_run`:** cambia el esquema y el manejador del upstream. Una herramienta del fork no toca el upstream.

## 8. Riesgos

| # | Riesgo | Mitigación |
|---|---|---|
| R1 | Conflictos en las líneas de enganche | Posiciones que el upstream casi no edita (principio de los `match`, cabeceras, envoltorios), el marcador `ADR-0001`, la receta de §3 y las pruebas de §5.2 |
| R2 | El upstream reorganiza un registro (`COMMANDS` deja de ser `const`, `tools()` se reparte, cambian `Step`, la carga de catálogos o la tabla de capas) | Saldría como error de compilación o como prueba del fork fallida al sincronizar. Se rediseña ese enganche en la `ARS.n` correspondiente y se actualiza este ADR |
| R3 | Un catálogo nuevo del upstream con prueba de cobertura | La prueba falla tras sincronizar. Se añaden sus filas a la capa del fork (en español hasta que se traduzcan) |
| R4 | `decode` descarta en silencio los pasos desconocidos (`actions_ui.rs:194`): si un paso del fork desaparece (por revertir o renombrar), una acción "Firmar…" se ejecutaría sin firmar. Lo mismo puede pasar con la importación y exportación de acciones que el upstream tiene planeada (`misc.action-wizard-manage`): si reutiliza `decode` o su formato, perdería los pasos del fork al importar, y un archivo exportado con pasos `amaraza_` no se podría leer en PdfCraft | Los ids de paso nunca se renombran. Antes de revertir, se listan las acciones guardadas con pasos `amaraza_` (§9). Cuando el upstream publique la importación y exportación, se revisa en la `ARS.n` correspondiente y se añade una prueba del fork que exporte e importe una acción con un paso del fork. Opcional: proponer al upstream que rechace las acciones con pasos desconocidos en vez de descartarlos (decisión del titular) |
| R5 | Con claves de software, Windows puede no pedir nada, y un agente podría firmar sin intervención humana | MCP solo bajo demanda (`AGENTS.md` §3), `destructive` con `confirm`, solo archivos nuevos y confirmación en la interfaz. El riesgo residual se anota en AR1.x |
| R6 | PIN repetido por archivo, o la lista silenciosa con tokens | AR1.2 (§4.5) |
| R7 | Cambios en `Cargo.lock` | Solo paquetes internos. Se comprueba el diff en cada commit y en cada sincronización |
| R8 | La concatenación `const` es poco habitual para quien lee el upstream | Va comentada y se comprobó con rustc 1.99 y clippy |
| R9 | Los pasos que piden datos al usuario o muestran instrucciones, que el upstream tiene planeados (`misc.action-prompt-steps`, P2), pueden tocar `run_action_on`, donde va F1, o pedir datos antes de ejecutar, igual que el diálogo de firma del fork | F1 es de la fase 2: al empezarla se revisa si el upstream ya lo publicó, y el diálogo de firma se coloca después de sus preguntas. Si llega después, se resuelve en la `ARS.n` correspondiente, con la prueba de interfaz de la firma por lote como red |

## 9. Cómo se revierte

- **Todo:** la implementación va en commits AR0.3 separados (§5.3). Se revierte con `git revert` en orden inverso, que crea commits nuevos sin reescribir la historia. Después, puertas completas y una entrada en el registro.
- **Una superficie:** basta con quitar su línea de enganche (por ejemplo, E17, el menú). Lo demás sigue funcionando: la paleta y las herramientas siguen llegando al comando.
- **Datos:** solo las acciones guardadas, en la carpeta de datos de Amaraza PDF, pueden contener pasos del fork. Antes de revertir se listan, porque después `decode` los descartaría (R4).
- **Comprobación:**
  - `git grep -n "ADR-0001" -- ':!docs'` no devuelve nada;
  - `git diff e9b9ee6 -- <archivos de la tabla de §3>` solo muestra los cambios de AR0.1.

## 10. Decisiones del titular (2026-10-10)

Anotadas en `docs/amaraza/PLAN.md`, "Decisiones del titular".

- **AR-D3:** se crea `crates/amaraza-ext`, con dependencias internas (`path`) desde `automation` y `ui-egui`. `Cargo.lock` cambia solo con el paquete interno: ni crates externos ni versiones nuevas. Se comprueba en cada commit.
- **AR-D4:** los textos del fork se muestran en español en los 14 idiomas restantes, con filas "sin traducir" en la capa del fork. Traducirlos es otra tarea.
- **AR-D5:** los identificadores del código del fork van en inglés (`Step`, `Signer`, `plan`, `run`…, como en `amaraza-identity`); los textos que ve el usuario, en español.
- **Fase 2:** `pdfcraft-sign`, `rustls-cng` y cualquier PR al upstream (el cambio de `decode`, o mantener abierto el identificador de la clave durante el lote) se deciden al empezarla, no ahora.
