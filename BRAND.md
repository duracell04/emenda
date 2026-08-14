# Emenda Brand System

> **Frozen brand system, version 2.0.0**

> **Preserve your Duktus**

Emenda is a quiet editorial instrument. It measures, corrects, and refines with precision while leaving authorship visibly with the writer.

## 1. Brand idea

Emenda should feel:

```text
precise
restrained
mechanical
editorial
quietly intelligent
```

The visual language combines a typewriter, proofreader, measuring instrument, and mechanical drafting system.

## 2. Product principle

> **Emenda corrects the text while preserving the author's Duktus.**

## 3. Wordmark

Primary wordmark:

```text
Emenda
```

Preferred tagline:

```text
Preserve your Duktus
```

Preferred lockup:

```text
[ Emenda monogram ] | Emenda
                     Preserve your Duktus
```

## 4. Monogram

The core symbol is a capital **E** inside an incomplete circular construction arc.

Elements:

```text
capital E
incomplete circular arc
fine alignment marks
ruler or measurement ticks
one restrained oxblood correction marker
visible construction logic
```

The right side remains open. The E remains the visual anchor.

## 5. Logo geometry

### Circular arc

- approximately three quarters of a full circle;
- opening on the right;
- strong primary drafting stroke;
- lower-right termination transitioning toward a ruler gesture.

### E

- optically centered;
- approximately 45–50% of circle diameter in width;
- approximately 50–55% in height;
- typewriter-inspired with controlled human irregularity.

### Construction detail

Use very fine vertical and horizontal center lines, registration marks, and ruler ticks. One short oxblood line crosses the ruler near the lower-right region as an editorial correction mark.

### Stroke hierarchy

```text
E                       100%
main circular arc        75–85%
extension-icon border    70–80%
oxblood marker           30–40%
registration marks       20–30%
ruler baseline           20–25%
ruler ticks              15–20%
construction axes        10–15%
```

Visual reading order:

```text
E
→ arc
→ correction geometry
→ construction detail
```

## 6. Chromium extension assets

Create one canonical production vector:

```text
extension/assets/emenda-mark.svg
```

Derive locally bundled raster icon assets required by Manifest V3:

```text
extension/assets/icon-16.png
extension/assets/icon-32.png
extension/assets/icon-48.png
extension/assets/icon-128.png
```

Use the same mark for:

```text
toolbar action
extension-management page
options-page brand lockup
overlay brand detail
```

At 16 and 32 pixels, preserve only the E, incomplete arc, ruler gesture, and oxblood marker. Construction axes may be removed when they reduce legibility. Asset generation remains local and deterministic.

## 7. Extension action

The toolbar action is the main activation surface. It must remain recognizable at small browser-chrome sizes and in both light and dark browser themes.

Action state may use a restrained badge or icon treatment for:

```text
inactive origin
enabled origin
configuration required
```

State never relies on color alone. The action's accessible name communicates the current action.

## 8. Overlay

The overlay is a fixed, unanchored editorial card rather than a desktop window or caret-attached popover.

Use:

```text
Paper background
Ink Black text and primary control
Graphite secondary text
Steel Gray construction detail
Oxblood correction or failure cue
fine precise border
compact controls
generous internal whitespace
```

Suggested hierarchy:

```text
Spelling

liek → like

Corrects the transposed letters.

Apply   Dismiss
```

The card never autofocuses. Its shadow-root styles are self-contained and resilient to page CSS.

## 9. Options page

The options page is the calm configuration surface for:

- OpenRouter API key;
- concrete model;
- language profile;
- enabled origins;
- privacy disclosure.

Use a narrow readable column, explicit labels, visible saved state, and clear origin-revocation controls. The key is treated as write-only and is never displayed back in full.

## 10. Color palette

| Name | Hex | Role |
|---|---:|---|
| **Ink Black** | `#0E0F10` | primary typography, iconography, strong outlines |
| **Graphite** | `#2B2D2F` | secondary text and dark structure |
| **Steel Gray** | `#7A7F85` | non-text guides and qualifying large text |
| **Paper** | `#F4F2EE` | overlay and options background |
| **Oxblood** | `#5A1A1F` | rare correction, action, or failure accent |

The system is predominantly monochrome. Oxblood marks meaningful editorial action. Ink Black or Graphite carries normal-size text on Paper. Steel Gray is reserved for non-text detail or text whose size and weight independently meets WCAG 2.2 AA. Color never carries meaning alone.

## 11. Typography

### Special Elite Regular

Use sparingly for the wordmark, short brand-led headings, and editorial moments.

### Inter Regular

Use for overlay text, options, controls, statuses, and explanations.

Fonts are bundled locally. The interface remains legible with a system fallback while assets load.

## 12. Controls and focus

Primary action:

```text
Ink Black surface
Paper text
```

Secondary action:

```text
Paper surface
Ink Black border and text
```

Focus uses a visible outline with sufficient contrast and a non-color cue. Hover, focus, pressed, disabled, success, and error states remain distinguishable without motion.

## 13. Motion

Motion is measured, intentional, and restrained. Respect `prefers-reduced-motion`. Suggestion arrival may use a short opacity transition; the overlay does not travel from the caret or bounce.

A later brand animation may follow:

```text
align
→ draw arc
→ measure
→ reveal E
→ settle
→ reveal wordmark
```

It is not part of the V0.1 writing interaction.

## 14. Brand voice

Emenda speaks like a careful editor beside the author:

```text
precise
calm
editorial
literate
restrained
confident
non-performative
```

Core statements:

> We respect the writer.  
> We refine with precision.  
> We preserve your Duktus.

## 15. Design decision function

Every choice first meets legibility, keyboard-focus, reduced-motion, and WCAG 2.2 AA requirements. Among conforming options, choose what is:

```text
more precise
→ more restrained
→ more editorial
→ more mechanically coherent
→ quieter
```

## 16. Deferred brand surfaces

Native application icons, tray or menu assets, installers, signing art, store-listing creative, and release packaging are deferred. V0.1 brand production covers only the extension action, fixed overlay, options page, and locally bundled extension icons.

## 17. Brand essence

> **Emenda is a quiet editorial instrument. It measures, corrects, and refines with precision while leaving authorship visibly with the writer.**
