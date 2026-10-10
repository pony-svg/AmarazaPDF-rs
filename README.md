<p align="center">
  <img src="docs/brand/amaraza-logo.svg" alt="Amaraza PDF" width="96">
</p>

<h1 align="center">Amaraza PDF (Rust)</h1>

<p align="center">
  Aplicación de PDF de escritorio, de código abierto y escrita en Rust.<br>
  Basado en PdfCraft, del equipo ArtCraft (<a href="https://github.com/storytold/pdfcraft">github.com/storytold/pdfcraft</a>).
</p>

## Qué es

Amaraza PDF sirve para leer, comentar, rellenar y firmar PDF; organizar, combinar y dividir páginas, y editar, censurar, proteger y exportar documentos. Trabaja sin conexión: sin telemetría, sin cuenta y sin procesamiento en la nube.

Este repositorio es un fork de [PdfCraft](https://github.com/storytold/pdfcraft) y está en **fase de prueba de concepto**. La prueba sirve para decidir con evidencia si Amaraza PDF pasa de su versión actual en Python a Rust. El plan y los criterios de decisión están en [`docs/amaraza/PLAN.md`](docs/amaraza/PLAN.md), y el registro de lo hecho, en [`docs/amaraza/REGISTRO.md`](docs/amaraza/REGISTRO.md).

## Estado

- **Prueba de concepto:** no hay versiones publicadas ni binarios.
- **Plataforma de la prueba:** Windows. El empaquetado de Linux, Flatpak, macOS y FreeBSD está adaptado a la identidad de Amaraza PDF, pero no se ha probado.
- **Idioma:** los textos nuevos están en español. La interfaz heredada sigue en inglés (con las traducciones del upstream), y algunos de sus mensajes todavía nombran PdfCraft. Traducirla es una tarea aparte.

## Compilar y ejecutar

Hace falta Rust 1.90 o posterior. En Windows, con el objetivo MSVC y Visual Studio (o sus Build Tools) con la carga de trabajo "Desarrollo para el escritorio con C++".

```bash
cargo run -p pdfcraft -- documento.pdf
```

- `cargo run -p pdfcraft-cli -- --help` muestra la herramienta de línea de órdenes, que maneja el motor sin interfaz.
- `cargo test --workspace` ejecuta las pruebas. Con Windows en otro idioma que el inglés, 9 pruebas de interfaz dependen del idioma: ejecútalas con `LANG=en_US.UTF-8`.
- Los nombres internos (crates `pdfcraft-*`, variables `PDFCRAFT_*`, el binario `pdfcraft`) no cambian durante la prueba, para que sincronizar con el upstream sea sencillo. El instalador y el zip portátil instalan el ejecutable como `AmarazaPDF.exe`.

**Dónde guarda sus datos:** los ajustes van en `%APPDATA%\AmarazaPDF` y la recuperación en `%LOCALAPPDATA%\AmarazaPDF\Recovery`. Con el archivo `AmarazaPDF.portable` junto al ejecutable, todo se guarda en la carpeta `AmarazaPDFData`, a su lado. Amaraza PDF nunca lee ni mueve las carpetas de PdfCraft.

## Licencia

- **Código:** `MIT OR Apache-2.0`, igual que el upstream ([`LICENSE-MIT`](LICENSE-MIT), [`LICENSE-APACHE`](LICENSE-APACHE)). [`NOTICE`](NOTICE) dice que los archivos del upstream fueron modificados, y [`docs/amaraza/REGISTRO.md`](docs/amaraza/REGISTRO.md) los lista.
- **Marca Amaraza** (nombre y logotipo, [`docs/brand/`](docs/brand/)): pertenece a su titular y **no** es software libre. Sus condiciones están en [`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt).
- **Activos de terceros:** iconos Lucide (ISC), fuentes OFL y demás, con su atribución en [`ATTRIBUTION.md`](ATTRIBUTION.md).
- Las marcas del equipo ArtCraft no forman parte de este repositorio, y el equipo ArtCraft no patrocina ni avala Amaraza PDF.

## Para colaboradores y agentes

Las reglas del fork están en [`.agents/rules/00-amaraza.md`](.agents/rules/00-amaraza.md) y prevalecen donde lo indican sobre [`AGENTS.md`](AGENTS.md) y [`CLAUDE.md`](CLAUDE.md), que vienen del upstream y siguen vigentes en lo demás. Entre otras cosas: sala limpia (nada de Adobe y nada de código GPL/AGPL), la aplicación nunca debe cerrarse por errores, y cada commit pasa `cargo fmt --check`, `cargo clippy --workspace -- -D warnings`, `cargo test --workspace` y `cargo xtask assets`.
