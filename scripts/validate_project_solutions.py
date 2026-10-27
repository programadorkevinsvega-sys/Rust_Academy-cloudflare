#!/usr/bin/env python3
"""Extract the runnable solutions from project Markdown files and run cargo test."""
from pathlib import Path
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
PROJECTS = [
    "rust-nivel-1-cierre-operativo",
    "rust-nivel-2-inventario-logistico",
    "rust-nivel-3-auditoria-eventos",
    "rust-nivel-4-conciliacion-facturas",
    "rust-nivel-5-ingesta-eventos",
]
FILE_HEADING = re.compile(r"^(Cargo\.toml|(?:src|data)/[A-Za-z0-9_./-]+\.(?:rs|log))$")

def extract_solution(markdown_path: Path) -> dict[str, str] | None:
    text = markdown_path.read_text(encoding="utf-8")
    marker = "# Solución de referencia completa"
    if marker not in text:
        raise RuntimeError(f"No se encontró la solución completa en {markdown_path}")

    lines = text.split(marker, 1)[1].splitlines()
    current_file = None
    inside_fence = False
    captured_file = None
    buffer: list[str] = []
    files: dict[str, str] = {}

    fence_char = None
    for line in lines:
        fence_match = re.match(r"^(`{3,}|~{3,})", line)
        if fence_match:
            this_fence = fence_match.group(1)[0]
            if not inside_fence:
                inside_fence = True
                fence_char = this_fence
                captured_file = current_file
                buffer = []
            elif this_fence == fence_char:
                inside_fence = False
                if captured_file:
                    files[captured_file] = "\n".join(buffer) + "\n"
                captured_file = None
                buffer = []
                current_file = None
                fence_char = None
            continue

        if inside_fence:
            if captured_file:
                buffer.append(line)
            continue

        if line.startswith("## "):
            heading = line[3:].strip()
            current_file = heading if FILE_HEADING.fullmatch(heading) else None

    if inside_fence:
        raise RuntimeError(f"Hay un bloque de código sin cerrar en {markdown_path}")
    if "Cargo.toml" not in files or not any(name.startswith("src/") and name.endswith(".rs") for name in files):
        # Some projects keep the full layered implementation in proyectos/<slug>
        # and document links to each source file instead of duplicating code blocks.
        physical_project = ROOT / "proyectos" / markdown_path.stem
        if (physical_project / "Cargo.toml").is_file() and (physical_project / "src" / "main.rs").is_file():
            print(
                f"Solución en archivos fuente enlazados; se validará desde {physical_project.relative_to(ROOT)}.",
                flush=True,
            )
            return None
        raise RuntimeError(
            f"No se pudieron extraer Cargo.toml y archivos Rust de {markdown_path}. "
            f"Archivos extraídos: {sorted(files)}"
        )
    return files

def main() -> None:
    with tempfile.TemporaryDirectory(prefix="rust-academy-solutions-") as tmp:
        temp_root = Path(tmp)
        for slug in PROJECTS:
            markdown_path = ROOT / "src" / "content" / "projects" / f"{slug}.md"
            files = extract_solution(markdown_path)
            if files is None:
                continue
            project_root = temp_root / slug
            for relative, content in files.items():
                output = project_root / relative
                output.parent.mkdir(parents=True, exist_ok=True)
                output.write_text(content, encoding="utf-8")

            print(f"\n=== Validando {slug} ===", flush=True)
            print(f"Archivos extraídos: {', '.join(sorted(files))}", flush=True)
            subprocess.run(
                ["cargo", "test", "--manifest-path", str(project_root / "Cargo.toml")],
                cwd=project_root,
                check=True,
            )

    print("\n=== Validando los proyectos Rust físicos en proyectos/ ===", flush=True)
    subprocess.run(
        ["cargo", "test", "--manifest-path", str(ROOT / "proyectos" / "Cargo.toml")],
        cwd=ROOT,
        check=True,
    )

    print("\nTodos los proyectos compilaron y sus pruebas terminaron correctamente.", flush=True)

if __name__ == "__main__":
    main()
