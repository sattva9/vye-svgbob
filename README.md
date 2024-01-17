# vye-svgbob

`vye-svgbob` is a Python package that provides bindings for the Rust crate [svgbob](https://crates.io/crates/svgbob), allowing you to render ASCII art diagrams into SVG format in Python.

## Installation

### Requirements

Make sure you have the following prerequisites installed on your system:

- [Rust](https://www.rust-lang.org/tools/install)
- [Maturin](https://pypi.org/project/maturin/)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Maturin
pip install maturin
```

### Install package

```bash
pip install vye-svgbob
```

## Example

```python
# Import the vye_svgbob module
import vye_svgbob

# Your ASCII art diagram as a string
ascii_art = """
+---------------+
| Hello,        |
|   vye_svgbob! |
+---------------+
"""

# Render ASCII art to SVG
svg_output = vye_svgbob.to_svg(ascii_art)

# Save SVG to a file
with open('output.svg', 'w') as svg_file:
    svg_file.write(svg_output)
```
