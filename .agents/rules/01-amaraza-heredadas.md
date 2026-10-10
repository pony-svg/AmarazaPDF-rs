---
trigger: always_on
---

# Amaraza PDF (Rust): reglas heredadas de PDForte (AR0.2)

Complementan `.agents/rules/00-amaraza.md`, que manda en todo lo demás.

La clasificación completa está en `docs/amaraza/EQUIVALENCIAS.md`. **Criterio:** en el código del original mandan sus reglas; en el código del fork (crates `amaraza-*`, `apps/amaraza` si existe y los enganches de AR0.3) se aplica la más estricta de las dos. Mientras el titular no decida otra cosa (EQUIVALENCIAS §4), estas reglas no obligan a cambiar el código del original.

- **A1. Un solo camino de escritura, con verificación previa.** El código del fork no escribe el PDF del usuario por su cuenta: muta a través del motor y guarda por un único camino. El candidato va a un temporal de la misma carpeta, se verifica volviéndolo a abrir con el motor y solo después sustituye al original.
- **A2. Barrera y copia.** Antes de sustituir: comprobar que el archivo en disco no cambió desde que se abrió (tamaño, fecha y huella) y conservar una copia del original hasta verificar la nueva versión. Un único ayudante compartido, nunca reimplementado en cada función.
- **A3. Operaciones terminales** (censura, firma en el sitio, compresión en el sitio y similares): confirmación previa que avisa de que no se pueden deshacer, y vacían el historial de deshacer. Un fallo crítico no se revierte en silencio: se avisa y se conserva la copia de A2.
- **A4. Sin código consciente de las pruebas** en producción. Los módulos `#[cfg(test)]` están permitidos. Lo prohibido es que el código de producción cambie de comportamiento con `cfg!(test)`, detecte dobles de prueba o use identidades inventadas. Las pruebas inyectan lo que necesitan por parámetros.
- **A5. Documentos firmados** (D-03): toda mutación de un documento con firmas activas se rechaza por defecto con un error accionable. Solo con el consentimiento explícito del usuario, si se implementa esa preferencia, diciendo cuántas firmas se verán afectadas. Firmar sobre un firmado siempre se permite (de forma incremental, con aviso).
- **A6. Claves** (RF-FIRMA-01): al listar certificados no se abre la clave privada ni se pide PIN; solo se abre al firmar. Nunca exportarla (`00-amaraza.md` §5).
- **A7. Complejidad** ≤ 15 por función, también en las pruebas (`clippy::cognitive_complexity`).
  - Cada crate `amaraza-*` tiene su `clippy.toml`. Clippy no fusiona configuraciones, así que repite las 4 claves `allow-*-in-tests` de la raíz y añade `cognitive-complexity-threshold = 15`.
  - El lint es de nursery: se activa con `#![warn(clippy::cognitive_complexity)]` en el crate, porque `[lints] workspace = true` impide `[lints.clippy]` en su `Cargo.toml`.
  - Se comprueba con `cargo clippy -p <crate> --all-targets -- -D warnings`.
  - No hacer crecer los archivos grandes del upstream: lo nuevo entra por los enganches de AR0.3.
- **A8. Registros estructurados:** `event=…` y, si hay error, `error_code=…` (y `operation_id=…` cuando exista).
- **A9. Registros sin datos personales:** nada de rutas completas (como mucho, el nombre del archivo o una ruta relativa a una carpeta conocida), nombres de usuario, contenido de documentos ni secretos.
- **A10. Ningún `Result` se descarta en silencio:** se propaga, se registra o lleva un comentario que diga por qué se puede ignorar.
- **A11. Las pruebas no escriben en las carpetas reales** de la app (`%APPDATA%\AmarazaPDF`, `%LOCALAPPDATA%\AmarazaPDF`): usan carpetas temporales con limpieza garantizada.
- **A12. Colores solo de los tokens del tema** (`theme::Tokens`), sin `Color32` literales.
- **A13. 800×560** (D-08): toda pantalla y diálogo nuevos se pueden usar en una ventana de ese tamaño.
- **A14. Nada en claro en disco** (D-10): ninguna copia de un documento se escribe sin cifrar; los snapshots se cifran (AES-GCM, clave solo en memoria) y los restos se purgan a los 30 días. Una dependencia nueva se pide al titular (`00-amaraza.md` §6).
- **A15. Instancia única** (D-12): la cubre el original en Windows desde v0.5.0 (`apps/pdfcraft/src/single_instance.rs`). El código del fork no la reimplementa, y su socket sigue en la carpeta de Amaraza PDF, para no entregar nunca archivos a PdfCraft ni a PDForte.
