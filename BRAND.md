# Emenda Brand System

> **Preserve your Duktus**

Emenda is a minimal local writing assistant that corrects and refines text while preserving the author’s Duktus.

---

## 1. Brand Idea

Emenda should feel like a precise editorial instrument rather than a generic AI product.

The visual language combines the character of a typewriter, proofreader, measuring instrument, and mechanical drafting system. The brand is quiet, exact, tactile, and restrained.

### Brand keywords

- precise
- restrained
- mechanical
- editorial
- quiet intelligence

### Product principle

> **Emenda corrects the text while preserving the author’s Duktus.**

---

## 2. Wordmark

The primary wordmark is:

**Emenda**

It uses a typewriter-inspired serif/monospaced character with visible human irregularity.

The wordmark can appear:

1. on its own;
2. with the tagline **Preserve your Duktus**;
3. as a ruled lockup with fine measurement lines;
4. beside the Emenda monogram.

### Preferred lockup

```text
[ Emenda monogram ] | Emenda
                     Preserve your Duktus
```

The lockup should feel balanced, editorial, and lightly engineered.

---

## 3. Monogram

The core Emenda symbol is a capital **E** held inside an incomplete circular construction arc.

### Elements

- capital `E`
- incomplete circular arc
- horizontal and vertical alignment marks
- ruler / measurement ticks
- one restrained oxblood accent
- visible construction logic

The mark should feel as though it has been measured and assembled rather than decorated.

The circle remains open. Its incompleteness gives the mark movement and keeps it from feeling like a conventional badge.

The `E` remains the visual anchor.

### Character

The monogram should communicate:

- correction
- measurement
- editorial precision
- mechanical alignment
- authorship
- restraint

---

## 4. Logo Geometry & Construction

Use the monogram as a measured construction, not as freely arranged decorative elements.

### Coordinate system

Define the monogram on a square construction field with the `E` placed on the central vertical axis.

```text
              12 o'clock
                  │
                  │
          ╭───────┼───
       ╭──╯       │
      │           │
9 ────┼────── E ──┼────
      │           │
       ╰──╮       │
          ╰───────┼──── ruler →
                  │        │
              6 o'clock    oxblood
```

### Circular arc

- The primary construction is an **incomplete circle** around the `E`.
- The circle is centred approximately on the optical centre of the `E`.
- The arc covers roughly **three quarters of a full circle**.
- The opening sits on the **right-hand side**, approximately between the 1–2 o'clock and 4 o'clock regions.
- The arc crosses the principal construction axes at approximately:
  - `12 o'clock`
  - `9 o'clock`
  - `6 o'clock`
- The right side remains deliberately open.
- The lower-right end transitions visually toward the ruler/measuring element.
- The arc should read as a drafting stroke rather than a perfect digital ring.

### `E`

- The `E` sits centrally inside the circular construction.
- It is vertically dominant and slightly narrower than the circle's inner diameter.
- Its visual centre aligns with the vertical construction axis.
- The typeform retains the irregular, inked/typewriter character visible in the reference.
- The `E` should occupy roughly **45–50% of the circle diameter in width** and **50–55% in height**.
- Optical centring takes precedence over mathematical centring.

### Construction axes

Use very fine secondary lines:

- one vertical centre line through the `E`;
- one horizontal centre line;
- short perpendicular registration marks where the circle crosses the main axes.

The construction lines are substantially lighter and thinner than the primary arc and `E`.

### Registration points

The visible registration points occur primarily at:

```text
top    → circle / vertical-axis intersection
left   → circle / horizontal-axis intersection
bottom → circle / vertical-axis intersection
right  → ruler / correction construction
```

Each should resemble a technical drafting registration mark rather than a decorative cross.

### Ruler element

The lower-right ruler is a distinct geometric component.

- It sits approximately on the horizontal level of the lower-right arc termination.
- Its baseline extends horizontally to the right.
- Fine perpendicular ticks run across it.
- Tick lengths vary subtly in a ruler-like cadence.
- It should remain materially thinner than the main arc.
- The ruler visually completes the geometry without closing the circle.

### Oxblood correction mark

The oxblood line is the only strong colour accent.

- It is a **short vertical stroke**.
- It intersects the ruler near its right-hand portion.
- It extends slightly above and below the ruler baseline.
- It behaves like an editorial correction/alignment marker.
- Its placement should feel exact rather than centred for symmetry.

### Stroke hierarchy

Relative visual weight:

```text
E                       100%
main circular arc        75–85%
outer app-icon border    70–80%
registration marks       20–30%
ruler baseline           20–25%
ruler ticks              15–20%
construction axes        10–15%
oxblood marker           30–40%
```

This hierarchy is important. The eye should see:

```text
E
→ arc
→ correction geometry
→ construction detail
```

rather than seeing all lines with equal weight.

### Optical asymmetry

The mark is deliberately **not geometrically closed or perfectly symmetrical**.

The right-side opening, lower-right ruler and red marker create controlled imbalance. This gives the mark its editorial/mechanical character.

Preserve this controlled asymmetry. In particular, preserve:

- the incomplete circle;
- the deliberate right-side opening;
- the asymmetrical lower-right ruler gesture;
- the off-centre oxblood correction mark;
- the strong stroke hierarchy between primary and construction elements.

A complete circle, perfectly symmetric `C`, centred red line, equally weighted construction lines, or generic target/crosshair treatment changes the identity of the mark.

### App-icon geometry

For the app icon:

- centre the monogram inside a rounded square;
- maintain generous internal clear space;
- the rounded square should frame the monogram rather than tightly contain it;
- the monogram should occupy roughly **60–65% of the icon width**;
- retain the circle opening and oxblood marker at all normal icon sizes.

At very small sizes, simplify in this order:

```text
construction-axis detail
→ minor ruler ticks
→ minor registration lines
```

Preserve longest:

```text
E
→ incomplete arc
→ ruler gesture
→ oxblood marker
```

### Clear space

Use the width of the capital `E` stem as a practical minimum clear-space unit around the standalone monogram and wordmark lockup.

### Canonical vector source

> **The supplied brand boards define the visual geometry, but they are raster concept artwork rather than a production vector master. Exact Bézier coordinates, stroke widths, corner radii and spacing should therefore be reconstructed once as a canonical SVG and treated as the geometric source of truth thereafter.**

Create and maintain one canonical production master as `emenda-mark.svg`. Derive favicons, tray icons, installer icons, animation assets and UI marks from that vector source rather than redrawing the symbol independently.

---

## 5. Logo System

### Wordmark

```text
Emenda
Preserve your Duktus
```

### Ruled lockup

The wordmark sits between or alongside fine horizontal measurement rules and tick marks.

### Monogram

The `E` appears inside the incomplete circular construction mark.

### Horizontal lockup

The monogram appears to the left of the wordmark, separated by generous whitespace or a subtle vertical rule.

### Usage principle

Typography leads. Mechanical details support the identity rather than competing with it.

---

## 6. App Icon and Favicon

The application icon uses the monogram inside a softly rounded square.

### Icon composition

- paper/off-white background
- black rounded-square outline
- central Emenda `E`
- incomplete circular arc
- fine construction/alignment marks
- ruler ticks
- small oxblood vertical accent

### Scale targets

Design and test the symbol at:

- `32 × 32`
- `16 × 16`
- `8 × 8`

At smaller sizes, preserve the recognisable `E` and circular construction gesture first. Secondary measurement details may simplify as required for clarity.

### Icon character

> **Precise · Mechanical · Editorial**

---

## 7. Colour Palette

| Name | Hex | Role |
|---|---:|---|
| **Ink Black** | `#0E0F10` | Primary typography, iconography, strong outlines |
| **Graphite** | `#2B2D2F` | Secondary dark surfaces and UI details |
| **Steel Gray** | `#7A7F85` | Secondary text, guides, construction lines |
| **Paper** | `#F4F2EE` | Primary light background |
| **Oxblood** | `#5A1A1F` | Rare functional accent |

### Colour principle

The system is predominantly monochrome.

**Oxblood is a precise functional cue.** Use it for meaningful correction, alignment, selection, or action details rather than general decoration.

Paper backgrounds should feel warm rather than pure white.

---

## 8. Typography

### Primary typeface

**Special Elite Regular**

Use for:

- Emenda wordmark
- editorial display text
- short labels where the mechanical/typewriter voice is desirable
- brand-led headings
- selected interface details

Character:

- monospaced / typewriter-inspired
- tactile
- editorial
- mechanically expressive
- deliberately imperfect

### Secondary typeface

**Inter Regular**

Use for:

- application UI
- body copy
- settings
- descriptions
- long-form interface text
- dense information

Character:

- neutral
- highly legible
- contemporary
- quiet
- functional

### Typography principle

Use Special Elite to provide identity and Inter to provide clarity.

The interface should remain easy to read even when the brand typography is expressive.

---

## 9. Motifs and Iconography

The brand draws from editorial, mechanical, and typographic tools.

Core motifs:

- **Carriage return**
- **Correction mark**
- **Paper**
- **Typebar**
- **Ruler**
- **Keycap**
- **Margin**
- **Mechanism / gear**
- **Alignment crosshair**

### Icon style

Icons should use:

- thin or restrained strokes
- simple geometry
- mechanical construction
- editorial references
- minimal ornament
- clear functional meaning

---

## 10. UI Style

The interface should feel quiet, exact, and native to the act of writing.

### General principles

- generous whitespace
- subtle borders
- mechanical alignment details
- restrained use of colour
- typography-led hierarchy
- compact controls
- low visual noise
- clear state transitions

### Buttons

#### Primary

Dark Ink Black surface with light text.

#### Secondary

Paper/light surface with a fine dark border.

#### Tertiary

Minimal text treatment with little or no container.

### Suggestion card

A suggestion can follow this structure:

```text
Suggestion

Refined wording for clarity while
preserving your style.

                         Apply   Dismiss
```

`Apply` may use the oxblood accent.

### Settings card

Example structure:

```text
⚙ Settings

Model        [ selected model / provider ▼ ]
```

### Modes

The visual system can distinguish compact writing modes such as:

```text
[ Correct ]   [ Refine ]
```

### Sensitivity

Sensitivity may be represented through a quiet stepped indicator using small circular marks, with the active point using oxblood.

### Status

A compact status element can communicate readiness:

```text
● Ready
```

Status language should remain concise and functional.

---

## 11. Brand Voice

The brand voice follows the product philosophy:

> We respect the writer.  
> We refine with precision.  
> We never overwrite voice.  
> We preserve your Duktus.

The voice should feel:

- precise
- calm
- editorial
- literate
- restrained
- confident
- non-performative

Emenda speaks like a careful editor working beside the author.

---

## 12. Usage Notes

- Use generous whitespace.
- Prefer subtle borders and mechanical details.
- Let typography lead.
- Keep accents rare and meaningful.
- Keep construction marks precise.
- Preserve visual quiet.
- Use the warm Paper background as the default brand canvas where appropriate.

---

## 13. Animated Logo Concept

### Concept

A precise, mechanical build assembles the Emenda mark from technical elements.

The motion should feel:

- measured
- intentional
- engineered
- restrained

### Sequence

#### 01 — Align

**Time:** `00:00`

Alignment marks fade in.

A baseline tick lands with precision.

#### 02 — Draw

**Time:** `00:06`

The circular arc draws in with mechanical precision.

#### 03 — Measure

**Time:** `00:12`

Ruler ticks slide into place.

The mark locks to centre.

#### 04 — Emerge

**Time:** `00:18`

The `E` resolves at the centre, anchored and exact.

#### 05 — Finalize

**Time:** `00:24`

Details settle.

The mark holds in stillness.

#### 06 — Reveal

**Time:** `00:30`

The **Emenda** wordmark types on subtly.

The tagline **Preserve your Duktus** fades in.

---

## 14. Motion Principles

### Precise

Every movement is intentional and measured.

### Mechanical

Elements move with clean, engineered logic.

### Restrained

Motion remains minimal and controlled.

### Subtle accent

Oxblood appears only as a precise functional cue.

### Timing

- approximately `30 frames`
- approximately `1.0 s`
- smooth, consistent pacing

### Loop behaviour

The animation can:

- run as a complete intro;
- hold on the finalized monogram;
- loop from the construction sequence where appropriate.

### Background

Best suited to light, Paper-like backgrounds.

---

## 15. Visual Hierarchy

The identity should generally follow this order:

```text
Typography
→ structure
→ measurement
→ mechanical detail
→ oxblood accent
```

The oxblood accent is the final cue, not the dominant visual element.

---

## 16. Design Decision Test

When a visual or UI decision is ambiguous, prefer the option that feels:

```text
more precise
→ more restrained
→ more editorial
→ more mechanically coherent
→ quieter
```

The interface should feel like a well-made editorial instrument.

---

## 17. Canonical Brand Elements

```text
Product:   Emenda
Tagline:   Preserve your Duktus
Keywords:  Precise · Restrained · Mechanical · Editorial · Quiet intelligence
```

### Canonical colours

```text
Ink Black   #0E0F10
Graphite     #2B2D2F
Steel Gray   #7A7F85
Paper        #F4F2EE
Oxblood      #5A1A1F
```

### Canonical typography

```text
Primary:    Special Elite Regular
Secondary:  Inter Regular
```

---

## 18. Brand Essence

> **Emenda is a quiet editorial instrument. It measures, corrects and refines with precision while leaving authorship visibly with the writer.**
