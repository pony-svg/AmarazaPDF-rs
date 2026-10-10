# amaraza-identity

La identidad de Amaraza PDF en un solo sitio: nombre del producto, id de la app, carpetas de datos, marcadores del modo portátil, fabricante, direcciones web y logotipo. Es un crate propio del fork (`docs/amaraza/PLAN.md`, AR0.1).

- **Logotipo:** `docs/brand/amaraza-logo.{svg,png,ico}`. Es marca del titular, con licencia `LicenseRef-Amaraza-Trademark` (`docs/brand/LICENSE-brand.txt`), no MIT ni Apache, aunque el código de este crate sí lo sea.

- **Por qué un crate aparte:** los archivos que vienen de `storytold/pdfcraft` solo llaman a estas constantes. Así, al sincronizar con el upstream, los conflictos se reducen a unas pocas líneas.
- **Capa:** L0, sin dependencias (`xtask/src/layers.rs`). Cualquier crate lo puede usar.
- **Pruebas:** comprueban que ningún identificador reutiliza los de PdfCraft, PrintCraft, ArtCraft ni PDForte, que el id de la app tiene formato DNS inverso y que nombres y direcciones son coherentes entre sí.
- Los nombres internos (`pdfcraft-*`, `PDFCRAFT_*`) no cambian durante la prueba de concepto.
