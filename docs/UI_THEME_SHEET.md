# UI Theme Sheet

This sheet defines palette, mood, and color semantics. For layout, component anatomy, copy rules, and screen hierarchy, use `docs/UI_STYLE_GUIDE.md`.

## Direction

The UI should move away from cool blue-gray framing and into a darker, warmer identity:

- burgundy
- candlelight
- lacquered wood
- dusk
- expensive danger

The target style is **Monsterhall Guild Hall**.

This is not neon nightclub, bright anime chrome, or muddy brown fantasy UI. The game should feel sensual, readable, and deliberate.

## Core Rule

Use color for meaning, not decoration.

- Purple = selected, focus, active destination
- Wine red = pressure, residue, danger, debt
- Gold = premium, reward, upgrades, notable value
- Green = valid, ready, successful
- Amber = caution, partial fit, limited resource
- Gray-violet = passive chrome, inactive, disabled support

## Palette

### Base surfaces

- `bg_0`: `#120A12`
- `bg_1`: `#18111D`
- `panel_0`: `#211827`
- `panel_1`: `#2A1D31`
- `panel_2`: `#35233E`

### Borders

- `border_0`: `#4A3755`
- `border_1`: `#6A4D79`
- `border_hi`: `#C79A3B`
- `border_danger`: `#9B3A4D`

### Text

- `text_0`: `#F1E8F4`
- `text_1`: `#C9B8CE`
- `text_2`: `#8F7D97`
- `text_dim`: `#645866`

### Accents

- `accent_primary`: `#6A3A7E`
- `accent_secondary`: `#9A6CC2`
- `accent_wine`: `#6E1F3A`
- `accent_gold`: `#C79A3B`
- `accent_rose`: `#B56A7A`

### Semantic

- `success`: `#5FA36E`
- `warning`: `#D39B45`
- `danger`: `#B84A5A`
- `neutral`: `#7A63B8`

## Usage Ratio

- 70% dark neutral plum-charcoal
- 20% muted purple
- 8% wine and rose accents
- 2% gold highlights

Do not saturate the whole interface in red and purple. Most surface area should stay dark and restrained.

## Surface System

### Backdrop

- Full-screen backdrops should sit on `bg_0` and `bg_1`.
- Background gradients can push into deep burgundy, but should stay darker than primary panels.
- Decorative glows should be subtle and warm, not electric blue.

### Panels

- Default panel fill: `panel_0`
- Raised or dominant panel fill: `panel_1`
- Hovered or selected panel fill: `panel_2`
- Inset or low-emphasis tray fill: `bg_1`

### Borders

- Default panel border: `border_0`
- Hover border: `border_1`
- Selected border: `border_hi`
- Danger border: `border_danger`

The current thin blue borders should be fully removed from active UI chrome.

## Button System

### Primary button

- Fill: `accent_primary`
- Hover: lighten toward `accent_secondary`
- Pressed: darken from `accent_primary`
- Text: `text_0`
- Use for the dominant action on a screen

### Secondary button

- Fill: `panel_1`
- Hover: `panel_2`
- Border: `border_1`
- Text: `text_0`
- Use for valid but non-dominant actions

### Utility button

- Fill: `bg_1`
- Hover: `panel_0`
- Border: `border_0`
- Text: `text_1`
- Use for navigation, back, settings, close

### Danger button

- Fill: `accent_wine`
- Hover: slightly brighter wine
- Text: `text_0`
- Use for debt, destructive actions, forced tradeoffs

### Premium or confirm-build button

- Fill: dark gold-brown derived from `accent_gold`
- Hover: brighter gold
- Text: `text_0`
- Use sparingly for high-value confirms, upgrades, or major unlocks

## Text Hierarchy

Keep the existing structural rule of limited text sizes. Color should reinforce hierarchy, not replace it.

- Headings: `text_0`
- Body copy: `text_1`
- Secondary labels: `text_2`
- Disabled or inactive text: `text_dim`

Avoid pale blue-gray body copy. It weakens the new theme and makes the interface feel like a generic dev tool.

## Selected, Hovered, Disabled

### Selected

- Fill shifts toward `panel_2`
- Border uses `border_hi`
- Key label or chip can use `accent_secondary`

### Hovered

- Slight fill lift
- Border shifts from `border_0` to `border_1`
- Never rely on color alone; keep the shape and spacing stable

### Disabled

- Fill stays dark and flat
- Border stays muted
- Text uses `text_dim`
- Never use bright accent colors for disabled controls

## Semantic States

### Success

- Use `success` for ready, valid, confirmed, profitable

### Warning

- Use `warning` for risky, partial, low-confidence, limited stock

### Danger

- Use `danger` or wine-border treatment for debt, invalid actions, failure pressure, injury risk

### Neutral selected info

- Use `neutral` for active tabs, selected routing, and non-danger decision focus

## Chips, Badges, and Status Rows

- Positive badges: dark base with `success` edge or text
- Warning badges: dark base with `warning` edge or text
- Danger badges: dark base with `danger` edge or text
- Selection badges: dark base with purple edge or text
- Premium badges: dark base with gold edge or text

Badges should stay readable and compact. Avoid fully saturated fills unless the badge marks a critical state.

## Screen Chrome Mapping

### Top utility bar

- Background: `bg_1`
- Divider or edge: `border_0`
- Active utility action: `accent_primary`
- Passive utility action: `text_1`

### Footer action bar

- Background: `panel_0`
- Dominant action slot: primary purple
- Shortcut links: secondary or utility styling
- Danger or blocked states: wine or danger text treatment

### Lists and cards

- Default card: `panel_0`
- Hover card: `panel_1`
- Selected card: `panel_2` with gold border
- Invalid or blocked card: `bg_1` with muted text and danger marker if relevant

## Art Integration

The art should not feel pasted onto unrelated interface panels.

- Image frames should use `border_0` by default
- Important framed art can use a restrained gold edge
- Illustration glows should echo the screen accent color
- Character panels should share the panel palette, not introduce a separate blue lighting system

## Contrast and Clarity Rules

- No overlapping text and UI elements
- No low-contrast purple text on dark purple panels
- No blue accents should remain in active UI chrome
- If a panel contains both status and action, the action must stay visually stronger
- If a screen has one dominant action, only one button should read as primary

## Code Mapping

This sheet should drive implementation in the shared UI layer first.

### Primary files

- `src/ui/theme.rs`
- `src/ui/core.rs`
- `src/ui/chrome.rs`
- `src/ui/components.rs`
- `src/ui/art.rs`

### Required implementation pass

1. Replace current blue-led semantic constants in `theme.rs` with this palette.
2. Update shared button fills, borders, hover states, and text colors in `core.rs`.
3. Update panel tiers and chrome frames in `chrome.rs`.
4. Update badges, cards, and inline status components in `components.rs`.
5. Retune backdrop overlays and frame accents in `art.rs` so the screen mood matches the UI.

## Main Menu Mapping

For the title page specifically:

- Remove blue framing entirely
- Use `panel_0` and `panel_1` for the central menu block
- Make the dominant action purple
- Use muted plum borders
- Use gold sparingly for separators or selected emphasis
- Keep body copy warm off-white, not cold gray

## Done Criteria

The theme pass is complete when:

- no major interactive surface still reads blue-gray
- selected, warning, danger, and premium states are distinct at a glance
- panels, buttons, tabs, cards, and footers all share the same palette logic
- text remains readable on every dark surface
- no text or UI elements overlap after the recolor and spacing pass
