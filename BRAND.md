# Emenda Brand System

> **Frozen brand system, version 2.0.1**

> **Preserve your Duktus**

Emenda is a quiet editorial instrument: precise, restrained, mechanical, literate, and visibly respectful of authorship.

## Identity

The primary wordmark is **Emenda**. The preferred tagline is **Preserve your Duktus**.

The mark is a capital **E** inside an incomplete circular construction arc, with a ruler gesture and one restrained oxblood correction marker. The E and open arc must remain recognizable at extension-toolbar sizes; fine construction axes may be omitted when they reduce legibility.

Future implementation derives locally bundled 16, 32, 48, and 128 pixel extension icons from one canonical vector. The same identity serves the toolbar action, extension-management page, options page, and a restrained overlay detail. Asset organization is an ordinary implementation choice.

## Product surfaces

### Toolbar action

The action communicates three states without relying on color alone:

```text
inactive origin
enabled origin
configuration required
```

Its accessible name states the available action. Disabled, unavailable, and configuration-required conditions remain distinguishable by text, shape, or icon treatment.

### Overlay

The overlay is a fixed, unanchored editorial card. It shows one category, exact before and after text, a concise explanation, Apply, and Dismiss. It never autofocuses, tracks the caret, or obscures state through decorative animation.

### Options

The options page is a calm, narrow configuration surface for the API key, concrete model, profile, enabled origins, revocation, save state, errors, and the exact privacy disclosure owned by [`UX.md`](UX.md). The API key is write-only and never displayed back in full.

## Visual system

| Token | Value | Use |
|---|---:|---|
| Ink Black | `#0E0F10` | primary text, controls, strong outlines |
| Graphite | `#2B2D2F` | secondary structure and text |
| Steel Gray | `#7A7F85` | qualifying large text and non-text guides |
| Paper | `#F4F2EE` | overlay and options background |
| Oxblood | `#5A1A1F` | rare correction, action, or failure accent |

Use Inter for functional interface text and Special Elite sparingly for the wordmark or short editorial moments. Fonts and icons are bundled locally; system fallbacks remain legible.

The system is predominantly monochrome. Oxblood marks meaningful editorial action, never ordinary decoration. Color is never the only carrier of meaning.

## Controls, focus, and motion

- Primary controls use Ink Black with Paper text.
- Secondary controls use Paper with an Ink Black border and text.
- Focus is visible, sufficiently contrasted, and identifiable without color alone.
- Hover, pressed, disabled, saved, and error states are distinguishable without motion.
- Motion is short and restrained and honors `prefers-reduced-motion`.
- Emenda-owned UI targets WCAG 2.2 AA.

## Voice

Emenda speaks like a careful editor beside the author: precise, calm, concise, confident, and non-performative. It describes the local text issue and never presents itself as rewriting or improving the writer as a person.

Preferred statements:

> We respect the writer.  
> We refine with precision.  
> We preserve your Duktus.

## Scope

V0.1 brand work covers only the Chromium action, fixed overlay, options page, and locally bundled extension icons. Native application art, installers, store listings, commercial surfaces, and release packaging are deferred.
