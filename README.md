# PDF Parser & Generator en Rust con Libharu

**pdfparser** es un binario ejecutable de alto rendimiento escrito en **Rust** que genera documentos PDF enriquecidos a partir de archivos declarativos **XML/CSS**. Utiliza la biblioteca nativa en C **Libharu** (`libhpdf`), compilada y vinculada de forma **100% estática** (`statically linked`), garantizando un binario autónomo sin ninguna dependencia dinámica de bibliotecas `.so` en la máquina destino.

---

## Tabla de Contenidos

- [Características Principales](#características-principales)
- [Funcionalidades de Diseño Web & CSS](#funcionalidades-de-diseño-web--css)
- [Compilación en Ruta Fija](#compilación-en-ruta-fija)
- [Uso Básico](#uso-básico)
- [Soporte de Caracteres en Español y Acentos](#soporte-de-caracteres-en-español-y-acentos)
- [Documentación Detallada del Esquema XML & CSS](#documentación-detallada-del-esquema-xml--css)
  - [1. Etiqueta de Estilos `<style>`](#1-etiqueta-de-estilos-style)
  - [2. Atributos Inline `style="..."` y `class="..."`](#2-atributos-inline-style-y-class)
  - [3. Contenedores `<div>` / `<box>`](#3-contenedores-div--box)
  - [4. Esquinas Redondeadas (`border-radius`)](#4-esquinas-redondeadas-border-radius)
  - [5. Transparencia y Opacidad (`opacity`)](#5-transparencia-y-opacidad-opacity)
  - [6. Elementos Estándar](#6-elementos-estándar)
- [Ejemplo Completo con Estilos CSS](#ejemplo-completo-con-estilos-css)

---

## Funcionalidades de Diseño Web & CSS

Pensado para diseñadores y desarrolladores web familiarizados con **HTML y CSS**:

- **Bloques `<style>`**: Soporte para hojas de estilo con clases CSS (ej. `.card { background-color: #f8fafc; border-radius: 8px; padding: 12px; }`).
- **Atributo inline `style="..."`**: Permite definir propiedades CSS directamente en los elementos (ej. `style="color: #1e3a8a; font-size: 14px; margin-bottom: 10px;"`).
- **Contenedores `<div>` / `<box>`**: Elementos de caja con modelo de caja CSS (padding, margin, width, height, background, border, radius).
- **Esquinas Redondeadas (`border-radius` / `rx`)**: Renderizado de cajas, tarjetas y celdas con esquinas suavizadas mediante curvas Bézier.
- **Opacidad (`opacity`)**: Control de transparencia alpha (de `0.0` a `1.0`) en rellenos y trazos.

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
cat ejemplos/web_design.xml | ./bin/pdfparser > factura_estilizada.pdf
```

### Generación mediante Archivos de Entrada y Salida

```bash
./bin/pdfparser -i ejemplos/web_design.xml -o factura_estilizada.pdf
```

---

## Documentación Detallada del Esquema XML & CSS

### 1. Etiqueta de Estilos `<style>`

Permite definir reglas CSS reutilizables mediante clases `.nombre-clase`.

```xml
<style>
  .header-box {
    background-color: #0f172a;
    border-radius: 8px;
    padding: 15px;
    margin-bottom: 20px;
  }
  .card {
    background-color: #f8fafc;
    border-color: #cbd5e1;
    border-width: 1px;
    border-radius: 8px;
    padding: 12px;
  }
</style>
```

### 2. Atributos Inline `style="..."` y `class="..."`

Todos los elementos soportan las propiedades CSS comunes:

| Propiedad CSS | Descripción | Ejemplo |
| :--- | :--- | :--- |
| `color` | Color del texto (Hexadecimal). | `color: #1e3a8a;` |
| `background-color` / `background` | Color de fondo del contenedor o celda. | `background: #f8fafc;` |
| `font-family` / `font` | Fuente a utilizar (`Helvetica`, `.ttf`, etc.). | `font-family: Helvetica-Bold;` |
| `font-size` | Tamaño de letra en puntos/píxeles. | `font-size: 14px;` |
| `border-color` | Color del borde. | `border-color: #cbd5e1;` |
| `border-width` | Grosor del borde en puntos. | `border-width: 1.5px;` |
| `border-radius` / `rx` | Radio de esquinas redondeadas. | `border-radius: 8px;` |
| `padding` | Espaciado interno del contenedor. | `padding: 12px;` |
| `margin-bottom` | Margen inferior externo. | `margin-bottom: 15px;` |
| `opacity` | Transparencia (0.0 a 1.0). | `opacity: 0.8;` |
| `text-align` | Alineación (`left`, `center`, `right`, `justify`). | `text-align: center;` |

---

### 3. Contenedores `<div>` / `<box>`

Cajas contenedoras de estilo web que agrupan y organizan otros elementos (textos, párrafos, grillas u otros `<div>` anidados):

```xml
<div class="card" style="opacity: 0.95;">
  <text font="Helvetica-Bold" size="14" color="#1e3a8a">Título dentro de tarjeta</text>
  <paragraph color="#475569">Contenido envuelto en un contenedor div estilizado.</paragraph>
</div>
```

---

## Ejemplo Completo con Estilos CSS

```xml
<?xml version="1.0" encoding="UTF-8"?>
<pdf page-size="A4" orientation="portrait" margin="35" font="Helvetica" size="10" color="#1e293b">
  <style>
    .header-box {
      background-color: #0f172a;
      border-radius: 8px;
      padding: 15px;
      margin-bottom: 20px;
    }
    .card {
      background-color: #f8fafc;
      border-color: #cbd5e1;
      border-width: 1px;
      border-radius: 8px;
      padding: 12px;
      margin-bottom: 15px;
    }
  </style>

  <page>
    <div class="header-box">
      <text font="Helvetica-Bold" size="18" color="#ffffff">PANEL DE CONTROL WEB &amp; PDF</text>
      <spacer height="6"/>
      <text color="#94a3b8" size="10">Diseño estilizado usando sintaxis de CSS inline y hojas de estilo</text>
    </div>

    <div class="card">
      <text font="Helvetica-Bold" size="14" color="#1e3a8a">Resumen de Métricas</text>
      <spacer height="8"/>
      <paragraph color="#475569" size="10" align="justify">
        Este documento demuestra la capacidad del motor en Rust para procesar etiquetas &lt;div&gt; contenedoras, esquinas redondeadas (border-radius), rellenos (padding) y hojas de estilo CSS.
      </paragraph>
    </div>
  </page>
</pdf>
```
