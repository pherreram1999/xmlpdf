#!/usr/bin/env bash
set -e

# Asegurar que Cargo esté en el PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Definir la ruta fija de salida deseada para el binario
OUTPUT_DIR="bin"
OUTPUT_BINARY="$OUTPUT_DIR/pdfparser"

echo "==> Compilando pdfparser en modo Release (estático)..."

# Compilar proyecto usando cargo
cargo build --release

# Detectar la ubicación exacta del binario generado por Cargo
TARGET_DIR="target"
COMPILED_BIN=""

if [ -f "$TARGET_DIR/x86_64-unknown-linux-musl/release/pdfparser" ]; then
    COMPILED_BIN="$TARGET_DIR/x86_64-unknown-linux-musl/release/pdfparser"
elif [ -f "$TARGET_DIR/release/pdfparser" ]; then
    COMPILED_BIN="$TARGET_DIR/release/pdfparser"
else
    # Buscar cualquier binario pdfparser en target/*/release/pdfparser
    COMPILED_BIN=$(find "$TARGET_DIR" -type f -name "pdfparser" | grep "/release/" | head -n 1)
fi

if [ -z "$COMPILED_BIN" ] || [ ! -f "$COMPILED_BIN" ]; then
    echo "[Error] No se encontró el binario compilado en la carpeta target."
    exit 1
fi

# Crear el directorio fijo de destino
mkdir -p "$OUTPUT_DIR"

# Copiar el binario a la ruta fija independiente de la arquitectura
cp "$COMPILED_BIN" "$OUTPUT_BINARY"
chmod +x "$OUTPUT_BINARY"

echo "==> ¡Compilación completada exitosamente!"
echo "==> Ruta fija del binario: $(pwd)/$OUTPUT_BINARY"
echo "==> Información del binario:"
file "$OUTPUT_BINARY"
