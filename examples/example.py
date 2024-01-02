import vye_svgbob


def save_svg(output, file):
    # Save SVG content to a file
    with open(file, 'w') as svg_file:
        svg_file.write(output)


def to_svg(ascii_art):
    # Convert ASCII art to SVG string
    svg_output = vye_svgbob.to_svg(ascii_art)
    save_svg(svg_output, 'to_svg.svg')


def to_svg_string_compressed(ascii_art):
    # Convert ASCII art to compressed SVG string
    svg_output = vye_svgbob.to_svg_string_compressed(ascii_art)
    save_svg(svg_output, 'to_svg_string_compressed.svg')


def to_svg_string_pretty(ascii_art):
    # Convert ASCII art to pretty-formatted SVG
    svg_output = vye_svgbob.to_svg_string_pretty(ascii_art)
    save_svg(svg_output, 'to_svg_string_pretty.svg')


def to_svg_with_settings(ascii_art):
    # All params in settings are optional.
    settings = vye_svgbob.Settings()

    # Convert ASCII art to SVG with custom settings
    svg_output = vye_svgbob.to_svg_with_settings(ascii_art, settings)
    save_svg(svg_output, 'to_svg_with_settings.svg')


def to_svg_with_override_size(ascii_art):
    # All params in settings are optional
    settings = vye_svgbob.Settings(
        font_size=14,
        font_family="Iosevka Fixed, monospace",
        fill_color="black",
        stroke_color="white",
        background="black",
        stroke_width=2.0,
        scale=8.0,
        include_backdrop=True,
        include_styles=True,
        include_defs=True,
    )

    # Convert ASCII art to SVG with custom settings and override size
    svg_output = vye_svgbob.to_svg_with_override_size(ascii_art, settings, 200, 200)
    save_svg(svg_output, 'to_svg_with_override_size.svg')


if __name__ == '__main__':
    # Example ASCII art
    ascii_art = """
        +---------------+
        | Hello,        |
        |   vye_svgbob! |
        +---------------+
    """

    # Perform various conversions and save the results
    to_svg(ascii_art)
    to_svg_string_compressed(ascii_art)
    to_svg_string_pretty(ascii_art)
    to_svg_with_override_size(ascii_art)
    to_svg_with_settings(ascii_art)
