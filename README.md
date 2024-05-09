[![Following released Bevy versions](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://bevyengine.org/learn/quick-start/plugin-development/#main-branch-tracking)

# Upstream Compatability

| bevy  | bevybio_sequence_widget |
|-------|-------------------------|
| 0.13  | 0.1                     |

# Sequence Alignment in 2D space for the Bevy Engine

*WIP - Draft - Ideas*

- Show monospaced font
- One or more lines
- Load entire sequence(s) and let widget handle scrolling
- Alternatively: Load up more than visible width (peek left, peek right) and generate events for scrolling / moving / zooming
- Each character is an entity (maybe?) so we can highlight certain substrings easily
- Convert substring coordinates to screen location (for drawing of other aligned data, such as gene widgets, exons, etc...)
- Overlay annotations (? - Maybe just leave it as part of coordinates above, but make is easy to generate box coordinates)
- Also world location
- Support WASM and native
- Tooltips (again, substring, or whole sequence)
- Efficient data handling (load directly from sfasta)
- Mouse clicks, dragging, highlighting
- Hooks for keyboard shortcuts (left/right arrows, PgUp/Down for zoom) but easy to switch off
- Color coding
- Customize: Font, Size, Scrolling Speed, Interactions
- Reset view / Zoom to fit
- Calculate how many characters shown on screen - What happens when zoomed too far out?
- Performance (shader? each line as an entity?)
- Hide/Show additional lines

# What's bevybio?
Just me trying to not pollute the namespace with things that aren't game related.


