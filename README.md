# Kakoune bufname_minify

This small binary returns the smallest unique name for a buffer name based on the own buffer name and a comma separated list of buffers.
It allows to reduce the clutter when showing buffer names on the Kakoune modeline.

## Usage

bufname_minify <bufname> <comma_separated_buflist>

## Kakoune integration

It can be integrated in the Kakoune modeline as follows:

```
declare-option -hidden str bufname

hook global WinCreate .* %{
  set-option buffer bufname %sh{ bufname_minify $kak_bufname $(echo "$kak_buflist" | tr " " ",") }
}

set-option global modelinefmt '{StatusLine}%opt{bufname} (%val{cursor_line}:%val{cursor_char_column}){{context_info}} {{mode_info}} %val{client}@[%val{session}]'
```
