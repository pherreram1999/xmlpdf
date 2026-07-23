# PDF Parser & Generator en Rust con Libharu

**pdfparser** es un binario ejecutable de alto rendimiento escrito en **Rust** que genera documentos PDF enriquecidos a partir de archivos declarativos **XML/CSS**. Utiliza la biblioteca nativa en C **Libharu** (`libhpdf`), compilada y vinculada de forma **100% estática** (`statically linked`), garantizando un binario autónomo sin ninguna dependencia dinámica de bibliotecas `.so` en la máquina destino.

---

## Tabla de Contenidos

- [Características Principales](#características-principales)
- [Compilación en Ruta Fija](#compilación-en-ruta-fija)
- [Uso Básico](#uso-básico)
- [Soporte de Caracteres en Español y Acentos](#soporte-de-caracteres-en-español-y-acentos)
- [Referencia Completa de Elementos XML](#referencia-completa-de-elementos-xml)
  - [1. `<pdf>` (Nodo Raíz)](#1-pdf-nodo-raíz)
  - [2. `<fonts>` y `<font>` (Fuentes TrueType)](#2-fonts-y-font-fuentes-truetype)
  - [3. `<style>` (Hoja de Estilos CSS)](#3-style-hoja-de-estilos-css)
  - [4. `<page>` (Definición de Hoja)](#4-page-definición-de-hoja)
  - [5. `<div>` / `<container>` / `<box>` (Caja Contenedora CSS)](#5-div--container--box-caja-contenedora-css)
  - [6. `<text>` / `<span>` (Texto de una Línea o Posicionado)](#6-text--span-texto-de-una-línea-o-posicionado)
  - [7. `<paragraph>` / `<p>` (Párrafo Multi-línea)](#7-paragraph--p-párrafo-multi-línea)
  - [8. `<grid>` / `<table>` (Tabla / Grilla)](#8-grid--table-tabla--grilla)
  - [9. `<row>` (Fila de Tabla)](#9-row-fila-de-tabla)
  - [10. `<cell>` (Celda de Tabla)](#10-cell-celda-de-tabla)
  - [11. `<rect>` / `<box>` (Rectángulo / Figura)](#11-rect--box-rectángulo--figura)
  - [12. `<line>` (Línea Recta)](#12-line-línea-recta)
  - [13. `<image>` (Imagen JPEG)](#13-image-imagen-jpeg)
  - [14. `<spacer>` (Espaciador Vertical)](#14-spacer-espaciador-vertical)
  - [15. `<page-break/>` (Salto de Página)](#15-page-break-salto-de-página)
- [Sintaxis de Estilos CSS Soportados](#sintaxis-de-estilos-css-soportados)
- [Ejemplo Completo de Factura/Reporte](#ejemplo-completo-de-facturareporte)

---

## Características Principales

- **Alto Rendimiento**: Procesamiento XML ultrarrápido con `roxmltree` y generación nativa de PDF con Libharu.
- **Portabilidad Total**: Vinculación 100% estática (`statically linked`) sin requerir `.so` dinámicas.
- **Comunicación Estándar (std)**: Integración nativa con canalizaciones UNIX (`stdin` -> `stdout`).
- **Soporte de Acentos**: Transcodificación de caracteres en español (`á, é, í, ó, ú, ñ, Á, É, Í, Ó, Ú, Ñ, ¿, ¡`).
- **Estilos CSS Integrados**: Soporte para etiquetas `<style>`, clases CSS, `style="..."` inline, `border-radius`, `padding`, `margin` y `opacity`.
- **Paginación Dinámica (Auto Page-Break)**: Soporte completo de saltos de página automáticos cuando los elementos (textos, párrafos, tablas, contenedores) desbordan el límite de la página.
- **Fondos Optimizados**: Soporte de imagen de fondo para todas las páginas o páginas específicas, con caché interno de memoria que asegura un archivo final ligero.

---

## Compilación en Ruta Fija

Para compilar el proyecto y obtener el ejecutable en una **ruta fija e independiente de la arquitectura** (`./bin/pdfparser`):

```bash
./build.sh
```

El binario compilado estará ubicado en:
```bash
./bin/pdfparser
```

Verificación del binario estático:
```bash
file ./bin/pdfparser
# Salida: ... static-pie linked, statically linked

ldd ./bin/pdfparser
# Salida: statically linked
```

---

## Uso Básico

### Generación mediante Pipes (stdin / stdout)

```bash
cat ejemplos/invoice.xml | ./bin/pdfparser > factura.pdf
```

### Generación mediante Archivos de Entrada y Salida

```bash
./bin/pdfparser -i ejemplos/invoice.xml -o factura.pdf
```

---

## Referencia Completa de Elementos XML

A continuación se detalla la sintaxis, atributos y comportamiento de **cada uno de los elementos XML** soportados por el motor.

---

### 1. `<pdf>` (Nodo Raíz)

Elemento obligatorio que envuelve todo el documento PDF. Configura los valores predeterminados globales.

```xml
<pdf page-size="A4" orientation="portrait" margin="35" font="Helvetica" size="10" color="#1e293b">
  <!-- Contenido -->
</pdf>
```

#### Atributos:
- `page-size`: Tamaño predeterminado de hoja (`A4`, `LETTER`, `LEGAL`, `A3`, `A5`, `B5`). Defecto: `A4`.
- `orientation`: Orientación predeterminada (`portrait`, `landscape`). Defecto: `portrait`.
- `margin`: Margen global de hoja en puntos (ej. `30`). Defecto: `30.0`. Se puede subdividir en `margin-top`, `margin-bottom`, `margin-left` y `margin-right`.
- `background-image`: Ruta a una imagen JPEG para utilizarla como fondo en todas las páginas generadas de forma automática o estática.
- `font`: Fuente global predeterminada (`Helvetica`, `Times-Roman`, `Courier` o alias registrado). Defecto: `Helvetica`.
- `size` / `font-size`: Tamaño de fuente predeterminado. Defecto: `10.0`.
- `color`: Color de texto global en hexadecimal (ej. `#1e293b`). Defecto: `#000000`.
- `class` / `style`: Estilos CSS aplicables al documento raíz.

---

### 2. `<fonts>` y `<font>` (Fuentes TrueType)

Permite registrar fuentes externas **TrueType (`.ttf`)** para su uso en el documento.

```xml
<fonts>
  <font name="RobotoReg" path="assets/fonts/Roboto-Regular.ttf" embed="true"/>
  <font name="RobotoBold" path="assets/fonts/Roboto-Bold.ttf" embed="true"/>
</fonts>
```

#### Atributos de `<font>`:
- `name` (**Requerido**): Nombre alias que se usará en atributos `font="..."` o CSS `font-family: ...`.
- `path` (**Requerido**): Ruta al archivo `.ttf` en el sistema de archivos.
- `embed`: Indica si la fuente se incrusta en el PDF (`true` o `false`). Defecto: `true`.

---

### 3. `<style>` (Hoja de Estilos CSS)

Contiene declaraciones de clases CSS reutilizables mediante sintaxis `.nombre-clase { propiedad: valor; }`.

```xml
<style>
  .tarjeta {
    background-color: #f8fafc;
    border-color: #cbd5e1;
    border-width: 1px;
    border-radius: 8px;
    padding: 12px;
    margin-bottom: 15px;
  }
  .titulo {
    font-family: Helvetica-Bold;
    font-size: 16px;
    color: #1e3a8a;
  }
</style>
```

---

### 4. `<page>` (Definición de Hoja)

Representa una página física dentro del documento PDF. Si se omite, los elementos se asignan automáticamente a una página predeterminada.

```xml
<page page-size="LETTER" orientation="landscape" margin="40">
  <!-- Elementos visuales de la página -->
</page>
```

#### Atributos:
- `page-size`: Sobrescribe el tamaño de hoja para esta página.
- `orientation`: Sobrescribe la orientación (`portrait` o `landscape`).
- `margin`: Sobrescribe el margen de esta página en puntos. También soporta `margin-top`, `margin-bottom`, `margin-left` y `margin-right`.
- `background-image`: Imagen JPEG específica de fondo para esta página, que ignora y reemplaza el fondo global si existe.

---

### 5. `<div>` / `<container>` / `<box>` (Caja Contenedora CSS)

Contenedor de caja de estilo web que agrupa otros elementos (textos, párrafos, grillas u otros `div` anidados) aplicando fondo, bordes, esquinas redondeadas y rellenos.

```xml
<div class="tarjeta" style="background: #f1f5f9; border-radius: 6px; padding: 10px; opacity: 0.95;">
  <text font="Helvetica-Bold" size="14" color="#0f172a">Título Interno</text>
  <paragraph color="#334155">Texto dentro del contenedor estilizado.</paragraph>
</div>
```

#### Atributos & Propiedades CSS:
- `class`: Clase o clases CSS definidas en `<style>`.
- `style`: Estilos CSS inline (ej. `style="padding: 10px; background: #eee;"`).
- `width` / `height`: Ancho y alto opcionales del contenedor.
- `background` / `fill`: Color de fondo hexadecimal.
- `border-color` / `stroke`: Color del borde.
- `border-width` / `line-width`: Grosor del borde en puntos.
- `border-radius` / `rx` / `ry`: Radio para esquinas redondeadas.
- `padding`: Relleno interior en puntos.
- `margin-bottom` / `margin-top`: Margen externo vertical.
- `opacity`: Nivel de transparencia (de `0.0` transparente a `1.0` opaco).

---

### 6. `<text>` / `<span>` (Texto de una Línea o Posicionado)

Renderiza un bloque de texto plano. Puede posicionarse libremente en coordenadas absolutas o fluir automáticamente siguiendo el cursor vertical.

```xml
<!-- Posicionamiento Absoluto -->
<text font="Helvetica-Bold" size="18" color="#ffffff" x="50" y="66">TITULO ABSOLUTO</text>

<!-- Flujo Dinámico -->
<text color="#334155" align="center" style="font-size: 14px;">Texto centrado dinámico</text>
```

#### Atributos:
- `x`, `y`: Coordenadas absolutas desde la esquina superior izquierda. Si se omiten, fluye secuencialmente.
- `font` / `font-family`: Nombre de la fuente.
- `size` / `font-size`: Tamaño en puntos.
- `color`: Color del texto en hexadecimal.
- `align` / `text-align`: Alineación horizontal (`left`, `center`, `right`, `justify`).
- `class` / `style`: Clases o estilos CSS inline.

---

### 7. `<paragraph>` / `<p>` (Párrafo Multi-línea)

Renderiza bloques de texto largo que se dividen y ajustan automáticamente en múltiples líneas según los márgenes laterales de la hoja. Si el párrafo es muy largo, genera automáticamente saltos de página continuando el flujo del texto de manera inteligente y manteniendo un espaciado consistente medido realísticamente.

```xml
<paragraph font="Helvetica" size="10" color="#475569" margin-bottom="15" line-height="14" align="justify">
  Este es un párrafo de texto explicativo que se dividirá en varias líneas automáticamente al llegar al margen derecho. Si el texto excede la altura de la página, continuará en una nueva hoja.
</paragraph>
```

#### Atributos:
- `font` / `font-family`: Fuente del párrafo.
- `size` / `font-size`: Tamaño del texto.
- `color`: Color hexadecimal del texto.
- `align` / `text-align`: Alineación (`left`, `center`, `right`, `justify`).
- `line-height`: Separación entre líneas en puntos.
- `margin-bottom`: Espacio vertical reservado al finalizar el párrafo.
- `class` / `style`: Clases o estilos CSS inline.

---

### 8. `<grid>` / `<table>` (Tabla / Grilla)

Estructura declarativa organizada en filas y celdas para presentar información tabulada.

```xml
<grid columns="80, 245, 60, 60, 80" border="1" border-color="#cbd5e1" border-width="1.0" cell-padding="6" margin-bottom="20">
  <!-- Filas <row> -->
</grid>
```

#### Atributos:
- `columns`: Especificación de anchos de columna separados por coma (ej. `columns="100, 200, 150"`). Si se omite, se dividen equitativamente.
- `border`: Indica si se dibujan bordes (`1`/`true` o `0`/`false`).
- `border-color`: Color hexadecimal del borde.
- `border-width`: Grosor del borde en puntos.
- `cell-padding` / `padding`: Relleno interno predeterminado de cada celda.
- `margin-bottom`: Margen inferior al terminar la tabla.
- `class` / `style`: Clases o estilos CSS inline.

---

### 9. `<row>` (Fila de Tabla)

Define una fila horizontal dentro de un nodo `<grid>` o `<table>`.

```xml
<row background="#1e293b" bold="true" align="center">
  <!-- Celdas <cell> -->
</row>
```

#### Atributos:
- `background`: Color de fondo hexadecimal para todas las celdas de la fila.
- `bold`: Si es `true`, aplica automáticamente la variante en negrita `Helvetica-Bold`.
- `font`, `size`, `align`: Estilos heredados por las celdas de la fila.
- `class` / `style`: Clases o estilos CSS inline.

---

### 10. `<cell>` (Celda de Tabla)

Define una celda individual dentro de una etiqueta `<row>`.

```xml
<cell align="right" background="#f8fafc" color="#0f172a" border-radius="4">
  <text>$ 1,500.00</text>
</cell>
```

#### Atributos:
- `width`: Sobrescribe el ancho específico de la celda.
- `align` / `text-align`: Alineación del contenido (`left`, `center`, `right`, `justify`).
- `background`: Color de fondo específico.
- `color`: Color del texto.
- `font`, `size`: Sobrescribe la fuente y el tamaño.
- `border`, `border-color`: Configuración de borde específico.
- `border-radius` / `rx`: Radio para esquinas redondeadas en la celda.
- `class` / `style`: Clases o estilos CSS inline.

---

### 11. `<rect>` / `<box>` (Rectángulo / Figura)

Dibuja una figura rectangular en coordenadas específicas con relleno, borde u opacidad.

```xml
<rect x="35" y="35" width="525" height="50" fill="#0f172a" stroke="#0f172a" border-radius="8" opacity="0.9"/>
```

#### Atributos:
- `x`, `y`: Posición de la esquina superior izquierda.
- `width`, `height`: Dimensiones en puntos.
- `border-radius` / `rx` / `ry`: Radio para esquinas redondeadas.
- `fill` / `background`: Color de relleno hexadecimal.
- `stroke` / `border-color`: Color del borde hexadecimal.
- `line-width` / `border-width`: Ancho de línea del borde.
- `opacity`: Transparencia (de `0.0` a `1.0`).
- `class` / `style`: Clases o estilos CSS inline.

---

### 12. `<line>` (Línea Recta)

Dibuja una línea recta entre dos coordenadas `(x1, y1)` y `(x2, y2)`.

```xml
<line x1="35" y1="170" x2="560" y2="170" color="#cbd5e1" width="1.0"/>
```

#### Atributos:
- `x1`, `y1`: Punto inicial.
- `x2`, `y2`: Punto final.
- `color`: Color hexadecimal de la línea.
- `width` / `border-width`: Grosor de la línea en puntos.
- `class` / `style`: Clases o estilos CSS inline.

---

### 13. `<image>` (Imagen JPEG)

Renderiza una imagen en formato JPEG en el documento.

```xml
<image src="assets/logo.jpg" x="35" y="35" width="120" height="60"/>
```

#### Atributos:
- `src` (**Requerido**): Ruta relativa o absoluta del archivo `.jpg` / `.jpeg`.
- `x`, `y`: Posición opcional. Si se omiten, la imagen fluye verticalmente.
- `width`, `height`: Dimensiones de renderizado en puntos.
- `class` / `style`: Clases o estilos CSS inline.

---

### 14. `<spacer>` (Espaciador Vertical)

Inserta un espacio en blanco de separación vertical.

```xml
<spacer height="20"/>
```

#### Atributos:
- `height`: Altura en puntos del espacio en blanco. Defecto: `10.0`.

---

### 15. `<page-break/>` (Salto de Página)

Fuerza un salto de página manual, creando una nueva hoja vacía en el PDF.

```xml
<page-break/>
```

---

## Sintaxis de Estilos CSS Soportados

Las propiedades CSS soportadas tanto en `<style>` como en `style="..."` incluyen:

- `color`: Hexadecimal (`#1e3a8a`, `#f00`).
- `background` / `background-color`: Hexadecimal (`#f8fafc`).
- `font-family` / `font`: Nombre de fuente (`Helvetica-Bold`).
- `font-size`: Tamaño (`14px`, `12pt`, `10`).
- `font-weight`: Peso (`bold`, `normal`).
- `border-color`: Color de borde hexadecimal.
- `border-width`: Grosor del borde (`1px`, `2pt`).
- `border-radius` / `rx`: Radio de curvatura (`8px`).
- `padding`: Relleno interior en puntos.
- `margin-bottom` / `margin-top`: Margen exterior vertical.
- `opacity`: Transparencia (`0.0` a `1.0`).
- `text-align` / `align`: Alineación (`left`, `center`, `right`, `justify`).
- `width` / `height`: Dimensiones del elemento.

---

## Ejemplo Completo de Factura/Reporte

```xml
<?xml version="1.0" encoding="UTF-8"?>
<pdf page-size="A4" orientation="portrait" margin="35" font="Helvetica" size="10" color="#1e293b">
  <style>
    .encabezado {
      background-color: #0f172a;
      border-radius: 8px;
      padding: 15px;
      margin-bottom: 20px;
    }
    .tarjeta {
      background-color: #f8fafc;
      border-color: #cbd5e1;
      border-width: 1px;
      border-radius: 8px;
      padding: 12px;
      margin-bottom: 15px;
    }
  </style>

  <page>
    <div class="encabezado">
      <text font="Helvetica-Bold" size="18" color="#ffffff">FACTURA COMERCIAL</text>
      <text font="Helvetica" size="10" color="#94a3b8" x="430" y="66">No: FAC-2026-0089</text>
    </div>

    <div class="tarjeta">
      <text font="Helvetica-Bold" size="12" color="#334155">DESCRIPCION GENERAL:</text>
      <spacer height="6"/>
      <paragraph color="#475569" align="justify">
        Comprobante emitido con acentuación en español (código, información, términos, navegación) y diseño estructurado mediante XML y Rust.
      </paragraph>
    </div>

    <grid columns="80, 245, 60, 60, 80" border="1" border-color="#cbd5e1" cell-padding="6" margin-bottom="20">
      <row background="#1e293b" bold="true">
        <cell align="center"><text color="#ffffff">Código</text></cell>
        <cell><text color="#ffffff">Descripción del Servicio</text></cell>
        <cell align="center"><text color="#ffffff">Cant.</text></cell>
        <cell align="right"><text color="#ffffff">Precio</text></cell>
        <cell align="right"><text color="#ffffff">Total</text></cell>
      </row>
      <row background="#f8fafc">
        <cell align="center"><text>PDF-01</text></cell>
        <cell><text>Motor XML a PDF en Rust</text></cell>
        <cell align="center"><text>1</text></cell>
        <cell align="right"><text>$ 1,500.00</text></cell>
        <cell align="right"><text>$ 1,500.00</text></cell>
      </row>
    </grid>

    <line x1="35" y1="760" x2="560" y2="760" color="#cbd5e1" width="1.0"/>
    <text font="Helvetica-Oblique" size="8" color="#94a3b8" align="center" x="150" y="780">
      Representación impresa de comprobante emitido vía XML/Rust.
    </text>
  </page>
</pdf>
```
