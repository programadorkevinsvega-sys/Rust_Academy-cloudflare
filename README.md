# Rust Academy 🦀

Plataforma educativa estática y extensible para aprender lenguajes de programación. Construida con Astro, TypeScript y Tailwind CSS, preparada para Cloudflare Pages.

## Principios
- 100% estático: Astro genera HTML, CSS y JavaScript en tiempo de compilación.
- Sin backend ni base de datos: no hay API propia, autenticación ni servicios de servidor.
- Progreso local: los cursos completados se guardan en localStorage del navegador.
- Multilenguaje: el catálogo vive en src/data/languages.ts; cada curso es Markdown con metadatos.
- Los cinco niveles de Rust están incorporados como Markdown en `src/content/courses/`.
- Cada nivel tiene un proyecto empresarial independiente en `src/content/projects/`, con una página estática propia y un enlace desde su curso.

## Ejecutar localmente
Requisitos: Node.js 22 recomendado, npm y Git.

```bash
npm install
npm run dev
```

## Validar y compilar
```bash
npm run check
npm run build
npm run preview
```
El sitio compilado queda en dist/.

## Desplegar en Cloudflare Pages
1. Abre Workers & Pages en Cloudflare.
2. Selecciona Create application → Pages → Connect to Git.
3. Conecta programadorkevinsvega-sys/Rust_Academy-cloudflare.
4. Configura Production branch: main; Build command: npm run build; Build output directory: dist; Root directory: vacío.
5. Guarda y despliega.

Cada push a main dispara una nueva compilación. No configures un adaptador de Cloudflare Workers: el proyecto genera recursos estáticos y no necesita runtime de servidor.

## Cursos de Rust incluidos

- Nivel 1: Fundamentos del lenguaje.
- Nivel 2: Tipos compuestos, colecciones y abstracciones iniciales.
- Nivel 3: Ownership, borrowing y lifetimes.
- Nivel 4: Manejo de errores y código seguro.
- Nivel 5: Concurrencia y programación asíncrona.

## Añadir otro lenguaje
1. Agrega el lenguaje en src/data/languages.ts, con un id corto en minúsculas.
2. Crea un Markdown dentro de src/content/courses/, por ejemplo python-nivel-1.md.
3. Incluye este frontmatter al principio:

```yaml
---
title: "Python — Nivel 1: Fundamentos"
description: "Una introducción práctica a Python."
language: python
level: 1
levelLabel: "Nivel 01"
category: "Fundamentos"
duration: "Principiante"
order: 10
slug: python-nivel-1
tags: ["variables", "funciones"]
---
```

4. Añade el contenido Markdown después de los metadatos, ejecuta npm run build y sube los cambios.
La portada, las tarjetas, la búsqueda y las rutas se generan automáticamente a partir de los archivos Markdown.

## Estructura
```
src/
├── content/courses/       # Cursos Markdown por lenguaje y nivel
├── content/projects/      # Enunciados independientes de proyectos empresariales
├── data/languages.ts      # Catálogo de lenguajes
├── layouts/BaseLayout.astro
├── pages/index.astro
├── pages/404.astro
├── pages/cursos/[slug].astro
├── pages/proyectos/[slug].astro
└── styles/global.css
public/
├── favicon.svg
└── _headers
```

## Límites de esta versión
- El progreso se queda en el navegador actual; no se sincroniza entre dispositivos.
- No hay cuentas, pagos, certificados verificables ni guardado centralizado.
- El sitio no compila Rust en el navegador; se pueden añadir enlaces a Rust Playground más adelante.
- Los cursos se generan durante el build; al cambiar Markdown hay que hacer commit y esperar el despliegue.
