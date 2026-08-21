# Emenda Logo System Proposal

> **Classification:** External material — non-authoritative
>
> **Provenance:** Commit `05eadea4dc05e02b715618c458f7df4bbd9c0b10` from the divergent v2.0.2 sibling lineage
>
> **Historical source baseline:** Emenda Brand System v2.0.2
>
> **Active authority:** [`BRAND.md`](../BRAND.md), version 2.1.0
>
> **Purpose:** Preserve the proposed E-only successor concept for provenance and possible future evaluation.

This proposal creates no V0.1 product, asset, acceptance, or future-surface requirement. Normative-looking language below belongs to the preserved proposal and has no authority in the v2.1.0 freeze.

## 1. Design principle

Emenda uses a smallest-first identity.

The core mark must remain recognizable, balanced, and attractive at **16 × 16 px in one color**. Larger applications may add space, typography, or a restrained accent, but they must never depend on detail that disappears at small sizes.

The logo follows Swiss modernist principles:

- reduction
- functional clarity
- strong proportion
- disciplined grid
- typographic precision
- generous negative space
- optical rather than merely mathematical balance
- one clear visual idea

The logo is a mark, not an illustration of editing.

## 2. Core identity

The permanent identity consists of:

1. **Symbol:** a custom capital **E**
2. **Wordmark:** **Emenda**
3. **Optional accent:** one restrained oxblood intervention, used only where it remains visually clean
4. **Optional tagline:** **Preserve your Duktus**

Only the **E** is required for recognition.

The symbol must remain complete and recognizably Emenda without color, animation, a container, a tagline, or surrounding construction graphics.

## 3. The symbol

### 3.1 Form

The Emenda symbol is a single, clean capital **E**.

It should feel:

- precise
- quiet
- contemporary
- literate
- confident
- human without looking handmade
- geometric without looking sterile

The final E should be custom-drawn or optically modified rather than treated as an untouched font glyph.

Its silhouette must be distinctive enough to function alone in a browser tab, extension toolbar, application icon, or monochrome print application.

### 3.2 Construction

The E is built on a simple square grid.

The grid is a construction method only. It is never visible in the finished mark.

The design should use:

- a strong vertical stem
- three clearly separated horizontal arms
- generous internal openings
- sufficient stroke mass for 16 px rendering
- optical centering inside a square canvas
- simple terminals
- a controlled relationship between the upper, middle, and lower arms

The middle arm may contain a subtle custom proportion, cut, offset, or terminal treatment that makes the glyph proprietary. Any such distinction must survive reduction and must still look correct in pure monochrome.

### 3.3 Optical requirements

At every target size:

- the E must look centered rather than merely measure centered
- horizontal arms must remain visually distinct
- counters and gaps must remain open
- no essential feature may fall below one physical pixel
- the silhouette must remain recognizable under antialiasing
- the symbol must not depend on texture or distressing

## 4. What belongs outside the core logo

The previous Emenda identity used an incomplete circular construction arc, ruler gesture, crosshairs, alignment marks, and an oxblood registration line.

These elements may remain available as **secondary editorial graphics** in larger brand compositions, documentation, motion graphics, or campaign material. They are not part of the permanent core logo.

The core logo therefore remains visually stable when displayed at 16 px, printed in one color, rendered on dark interfaces, or reproduced by systems with limited detail.

## 5. Oxblood accent

Emenda retains **Oxblood `#5A1A1F`** as a brand accent.

The accent is optional in the logo.

Its role is to signal a precise editorial intervention, not to create the identity itself.

Rules:

- the monochrome logo is always the canonical fallback
- recognition never depends on oxblood
- the accent occupies a very small proportion of the overall mark
- the accent is omitted whenever it becomes visually noisy or pixel-ambiguous
- favicon and very small toolbar applications default to monochrome
- the accent may appear in larger symbol, lockup, app-icon, or motion applications after optical testing

A thick slash, decorative red bar, or large colored element is outside the logo system.

## 6. Wordmark

The primary wordmark is:

**Emenda**

The wordmark should use a neutral Swiss-style grotesk with clean proportions and strong screen rendering.

**Inter** is the practical baseline already present in the Emenda brand system. The final wordmark should be optically adjusted and stored as vector outlines so the logo does not depend on local font availability.

The wordmark should feel calmer and more contemporary than a distressed or typewriter treatment.

### Wordmark rules

- use title case: **Emenda**
- preserve consistent letterforms across all master assets
- use optical kerning
- keep spacing restrained but breathable
- preserve a clear lowercase rhythm after the capital E
- export the final master wordmark as paths
- treat the wordmark as a fixed logo asset rather than live UI text

Special Elite may remain part of broader editorial brand expression, but it is not required for the logo.

## 7. Logo configurations

### 7.1 Symbol

**E**

Use for:

- favicon
- browser tab
- extension toolbar
- compact navigation
- avatar
- app tile
- very small product surfaces

### 7.2 Horizontal lockup

**[E] Emenda**

Use for:

- website header
- extension management page
- options page
- documentation
- product splash
- repository or project presentation

The symbol and wordmark should align optically around the wordmark cap height and visual center.

### 7.3 Wordmark only

**Emenda**

Use where the brand name already has sufficient space and the symbol would add unnecessary density.

### 7.4 Tagline lockup

**[E] Emenda**

**Preserve your Duktus**

Use only where the tagline can be read comfortably.

The tagline is not part of the compact logo and never appears in a favicon, toolbar icon, or small app icon.

## 8. Responsive logo system

The logo uses optical-size variants rather than one vector mechanically scaled to every size.

| Rendered size | Required treatment |
|---|---|
| 12–20 px | E only, monochrome, pixel-tested |
| 21–31 px | E only; optional accent only if it renders cleanly |
| 32–63 px | Full E; restrained accent permitted |
| 64–127 px | Full symbol or symbol + wordmark |
| 128 px+ | Full symbol, wordmark, or tagline lockup |
| Large-format | Same core geometry; surrounding brand graphics may be added separately |

The silhouette remains consistent across variants. Optical adjustments may increase spacing, stroke mass, or simplify a small terminal where necessary for raster clarity.

## 9. Favicon

The favicon is the most restrictive expression of the identity and therefore acts as a design test for the entire system.

### Required favicon behavior

At **16 × 16 px**:

- use the E only
- use one color
- preserve clear internal gaps
- use a transparent background where supported
- remain legible in both light and dark browser chrome
- avoid fine decoration
- avoid an outer frame unless a specific platform requires one

At **32 × 32 px** and above, the same E remains the default. An accent may be introduced only if direct browser-tab testing shows that it improves rather than fragments the mark.

### Favicon deliverables

- `favicon.svg`
- `favicon.ico` containing at least 16 × 16 and 32 × 32
- PNG fallback where required

## 10. Browser extension icon

The extension icon uses the same E as the favicon.

The toolbar is a functional environment, so the icon prioritizes contrast and recognition over decorative branding.

### Toolbar sizes

Prepare and test at:

- 16 × 16
- 19 × 19 where required by legacy or platform surfaces
- 20 × 20
- 24 × 24
- 32 × 32

The smallest toolbar asset should be optically tuned rather than generated by blind downscaling.

### Extension-management sizes

Prepare:

- 32 × 32
- 48 × 48
- 128 × 128

These larger assets may use the restrained accent if it remains coherent with the monochrome base.

Product state indicators are treated as UI state, not as changes to the permanent brand geometry. The Emenda logo itself stays recognizable and stable.

## 11. App icon

An app icon is a platform container around the Emenda symbol, not a different logo.

### App-icon principles

- center the E optically
- preserve generous internal padding
- let the operating system define corner treatment where applicable
- use a simple Ink/Paper relationship
- use oxblood only as a restrained optional accent
- keep the symbol visually dominant
- preserve the same E silhouette used in the favicon and extension

A rounded square, circle, badge, shadow, or border is never required for brand recognition.

### Future app-icon exports

Where needed:

- 180 × 180
- 192 × 192
- 256 × 256
- 512 × 512
- 1024 × 1024

## 12. Color variants

### Primary light-background mark

- Symbol / wordmark: **Ink Black `#0E0F10`**
- Background: transparent or **Paper `#F4F2EE`**

### Primary dark-background mark

- Symbol / wordmark: **Paper `#F4F2EE`**
- Background: transparent or **Ink Black `#0E0F10`**

### Accent

- Oxblood: **`#5A1A1F`**

### Monochrome

A pure black and pure white version must always exist.

The monochrome version is a first-class master, not an emergency fallback.

## 13. Backgrounds

Preferred logo files use transparent backgrounds.

When a background is necessary:

- use a calm solid field
- preserve strong contrast
- keep the logo visually separated from surrounding content
- keep platform containers separate from the permanent mark

The logo should work on light, dark, grayscale, and photographic backgrounds when sufficient contrast is provided.

## 14. Clear space

The symbol requires clear space on every side.

Define **x** as the thickness of the E's main vertical stem.

Minimum clear space:

- symbol: **2x** on every side
- symbol + wordmark: **2x** around the full lockup
- tagline lockup: **2x** around the complete composition

More space is preferred when the surrounding layout permits it.

## 15. Minimum sizes

Recommended digital minimums:

- symbol: **16 px**
- horizontal lockup: **96 px** total width
- tagline lockup: only where the tagline renders at a comfortably readable text size

Recommended print minimums should be established after physical proofing of the final vector, with the one-color symbol as the reference.

## 16. Animation

Logo animation must reveal the existing identity rather than add decorative machinery.

### Primary animation concept

A single part of the E begins fractionally out of alignment and resolves into its final position.

The result is a restrained visual metaphor for correction:

**misalignment → precise correction → stable Emenda mark**

### Motion rules

- animate only existing logo geometry
- end on the exact static master logo
- use one clear movement
- keep motion short and controlled
- avoid continuous looping in resting UI
- avoid construction lines, rulers, crosshairs, sparks, particles, or decorative drawing sequences
- support `prefers-reduced-motion` by showing the final static mark immediately

Recommended duration for a standard reveal:

**300–450 ms**

A longer brand-film treatment may use the same principle at a slower pace, but the motion language remains minimal.

## 17. Light and dark mode

Light and dark variants are designed intentionally.

The system must be checked in:

- light browser chrome
- dark browser chrome
- light extension UI
- dark extension UI
- high-density and standard-density displays

The symbol may receive small optical weight adjustments between light-on-dark and dark-on-light exports if required to produce equal perceived weight.

## 18. Accessibility and robustness

The logo must remain identifiable:

- without color
- in grayscale
- at 16 px
- on high-DPI and standard-DPI displays
- under browser antialiasing
- under SVG and raster rendering
- on light and dark surfaces

Color is never the sole carrier of identity or state.

Where the logo is interactive, the accessible name describes the action or destination rather than relying on the visual mark.

## 19. Canonical vector rules

The canonical logo master is SVG.

Master SVG requirements:

- explicit `viewBox`
- vector paths only for final logo geometry
- transparent background
- no embedded raster image
- no filters required for recognition
- no runtime font dependency
- no clipping or masking required for the basic silhouette
- deterministic geometry
- clean path structure
- visually identical output across Chromium, Firefox, Safari, and standard SVG viewers

Final wordmark text is converted to vector outlines in production master assets.

## 20. Raster generation

Raster assets are generated from approved optical masters.

Small assets are inspected individually after rasterization.

A 16 px icon is not accepted merely because it was exported successfully from the master SVG.

Each required size is checked for:

- stroke consistency
- open counters
- pixel balance
- centering
- aliasing
- edge clarity
- dark-mode appearance
- light-mode appearance

## 21. Asset structure

Recommended canonical structure:

```text
assets/
  logo/
    emenda-symbol.svg
    emenda-symbol-dark.svg
    emenda-symbol-light.svg
    emenda-wordmark.svg
    emenda-lockup-horizontal.svg
    emenda-lockup-tagline.svg
  favicon/
    favicon.svg
    favicon.ico
  extension/
    icon-16.png
    icon-20.png
    icon-24.png
    icon-32.png
    icon-48.png
    icon-128.png
  app/
    icon-180.png
    icon-192.png
    icon-256.png
    icon-512.png
    icon-1024.png
```

Only assets actually required by the current product need to ship. The structure describes the logo system, not an obligation to implement every future surface immediately.

## 22. Logo acceptance tests

A final Emenda logo is approved only after the following checks pass.

### Recognition

- [ ] The E is recognizable at 16 × 16 px.
- [ ] The symbol is distinctive without the wordmark.
- [ ] The mark remains coherent in pure black.
- [ ] The mark remains coherent in pure white.
- [ ] Removing oxblood does not remove brand recognition.

### Rendering

- [ ] 16 px favicon tested in a real browser tab.
- [ ] 32 px favicon tested in a real browser tab.
- [ ] Extension toolbar icon tested in light browser chrome.
- [ ] Extension toolbar icon tested in dark browser chrome.
- [ ] 48 px and 128 px extension assets tested in extension-management UI.
- [ ] SVG tested in Chromium.
- [ ] SVG tested in Firefox.
- [ ] SVG tested in Safari or WebKit rendering.
- [ ] Raster exports inspected at 100% scale rather than only zoomed.

### Composition

- [ ] The symbol is optically centered.
- [ ] Internal gaps remain open.
- [ ] No visual feature becomes accidental noise at small size.
- [ ] Wordmark spacing is optically balanced.
- [ ] Clear space remains intact in standard lockups.

### Color

- [ ] Ink-on-Paper version approved.
- [ ] Paper-on-Ink version approved.
- [ ] Monochrome black version approved.
- [ ] Monochrome white version approved.
- [ ] Oxblood accent, where used, remains subordinate to the mark.

### Motion

- [ ] Animation ends on the exact static logo.
- [ ] Animation reads clearly without decorative elements.
- [ ] Reduced-motion mode shows the final mark directly.
- [ ] No resting product UI depends on a looping logo animation.

### Reproduction

- [ ] Grayscale rendering remains recognizable.
- [ ] One-color print rendering remains recognizable.
- [ ] High-DPI display rendering remains balanced.
- [ ] Standard-DPI display rendering remains balanced.

## 23. Final design constraint

The identity should be explainable in one sentence:

> **Emenda is a precise capital E, reduced to the simplest form that remains unmistakably itself.**

Everything else is application, typography, color, motion, or surrounding brand language.

The logo succeeds when nothing additional is required to make it feel complete.

## 24. Repository governance

This document is a proposed logo-system successor to the logo-specific provisions of the historical Emenda Brand System v2.0.2.

Adopting the proposal as repository authority requires an explicit future versioned brand decision. [`BRAND.md`](../BRAND.md) v2.1.0 remains the frozen repository authority.
