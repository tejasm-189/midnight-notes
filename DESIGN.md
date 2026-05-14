---
name: Midnight Developer Notes
colors:
  surface: '#131313'
  surface-dim: '#131313'
  surface-bright: '#393939'
  surface-container-lowest: '#0e0e0e'
  surface-container-low: '#1c1b1b'
  surface-container: '#201f1f'
  surface-container-high: '#2a2a2a'
  surface-container-highest: '#353534'
  on-surface: '#e5e2e1'
  on-surface-variant: '#b9cacb'
  inverse-surface: '#e5e2e1'
  inverse-on-surface: '#313030'
  outline: '#849495'
  outline-variant: '#3b494b'
  surface-tint: '#00dbe9'
  primary: '#dbfcff'
  on-primary: '#00363a'
  primary-container: '#00f0ff'
  on-primary-container: '#006970'
  inverse-primary: '#006970'
  secondary: '#7dffa2'
  on-secondary: '#003918'
  secondary-container: '#05e777'
  on-secondary-container: '#00622e'
  tertiary: '#fff5de'
  on-tertiary: '#3b2f00'
  tertiary-container: '#fed639'
  on-tertiary-container: '#715d00'
  error: '#ffb4ab'
  on-error: '#690005'
  error-container: '#93000a'
  on-error-container: '#ffdad6'
  primary-fixed: '#7df4ff'
  primary-fixed-dim: '#00dbe9'
  on-primary-fixed: '#002022'
  on-primary-fixed-variant: '#004f54'
  secondary-fixed: '#62ff96'
  secondary-fixed-dim: '#00e475'
  on-secondary-fixed: '#00210b'
  on-secondary-fixed-variant: '#005226'
  tertiary-fixed: '#ffe179'
  tertiary-fixed-dim: '#eac324'
  on-tertiary-fixed: '#231b00'
  on-tertiary-fixed-variant: '#554500'
  background: '#131313'
  on-background: '#e5e2e1'
  surface-variant: '#353534'
typography:
  display:
    fontFamily: Inter
    fontSize: 32px
    fontWeight: '700'
    lineHeight: 40px
    letterSpacing: -0.02em
  headline-lg:
    fontFamily: Inter
    fontSize: 24px
    fontWeight: '600'
    lineHeight: 32px
    letterSpacing: -0.01em
  headline-md:
    fontFamily: Inter
    fontSize: 20px
    fontWeight: '600'
    lineHeight: 28px
  body-lg:
    fontFamily: Inter
    fontSize: 16px
    fontWeight: '400'
    lineHeight: 24px
  body-md:
    fontFamily: Inter
    fontSize: 14px
    fontWeight: '400'
    lineHeight: 20px
  code-block:
    fontFamily: JetBrains Mono
    fontSize: 14px
    fontWeight: '400'
    lineHeight: 22px
  label-mono:
    fontFamily: JetBrains Mono
    fontSize: 12px
    fontWeight: '500'
    lineHeight: 16px
    letterSpacing: 0.05em
  headline-lg-mobile:
    fontFamily: Inter
    fontSize: 20px
    fontWeight: '600'
    lineHeight: 28px
rounded:
  sm: 0.125rem
  DEFAULT: 0.25rem
  md: 0.375rem
  lg: 0.5rem
  xl: 0.75rem
  full: 9999px
spacing:
  unit: 4px
  xs: 4px
  sm: 8px
  md: 16px
  lg: 24px
  xl: 40px
  gutter: 16px
  margin-mobile: 16px
  margin-desktop: 32px
---

## Brand & Style

This design system is built for a developer-centric, privacy-focused environment. The aesthetic, titled "Midnight," prioritizes visual comfort during long coding or writing sessions by utilizing an OLED-friendly dark mode. 

The brand personality is **precise, technical, and unobtrusive**. It draws heavily from **Minimalism** and **Modern Corporate** styles, stripping away unnecessary ornamentation to focus on content hierarchy and functional clarity. The emotional response should be one of deep focus and reliability, evoking the feeling of a high-end integrated development environment (IDE).

## Colors

The palette is anchored in absolute black (`#000000`) to maximize contrast and energy efficiency on OLED displays. 

- **Primary (Electric Blue):** Used for primary actions, active focus states, and key interactive markers.
- **Secondary (Emerald Green):** Used for success states, saved indicators, and secondary highlights.
- **Surface Tiers:** Charcoal grays (`#121212` and `#1E1E1E`) differentiate sidebars and modals from the main editing canvas.
- **Borders:** A consistent charcoal (`#2C2C2C`) is used for subtle structural definition instead of shadows.

## Typography

The typography system uses a dual-font approach to distinguish between the interface and the content. 

- **Inter** handles all UI elements, headings, and standard body text. It is chosen for its exceptional legibility and neutral, modern tone.
- **JetBrains Mono** is utilized for the editor, code snippets, and metadata labels (like file paths or timestamps). This reinforces the developer-focused nature of the tool.

Hierarchy is established through weight and color (using dimmed grays for secondary info) rather than dramatic size shifts.

## Layout & Spacing

The design system employs a **Fixed Grid** for the main editor to ensure line-length readability, while utilizing a **Fluid Sidebar** for navigation.

- **Grid:** A 12-column system is used on desktop, but the central "Note Editor" is capped at a maximum width of 840px to maintain optimal character counts per line.
- **Rhythm:** An 8px linear scale (with a 4px step for tight components) governs all padding and margins. 
- **Adaptation:** On mobile, sidebars collapse into a drawer, and horizontal margins shrink to 16px. The editor transitions to a full-width fluid layout.

## Elevation & Depth

To maintain the "Midnight" aesthetic, this system avoids traditional drop shadows. Depth is communicated through **Tonal Layering** and **Subtle Outlines**:

1.  **Level 0 (Base):** Absolute Black (`#000000`) for the main editor background.
2.  **Level 1 (Surface):** Charcoal (`#121212`) for sidebars, navigation, and persistent panels.
3.  **Level 2 (Overlay):** Dark Gray (`#1E1E1E`) for modals, tooltips, and floating menus.
4.  **Borders:** Every interactive element or distinct container is defined by a 1px solid border (`#2C2C2C`). When an element is focused, the border transitions to the Primary Electric Blue.

## Shapes

The shape language is **Soft** and highly disciplined. Elements use a small 4px (`0.25rem`) radius to take the edge off the "brutalist" feel without appearing "bubbly" or overly consumer-grade.

- **Standard Elements:** 4px radius (Buttons, Input fields, Chips).
- **Large Containers:** 8px radius (Cards, Modals).
- **Selection Indicators:** 2px radius (Used for active line markers in the editor).

## Components

- **Buttons:** Primary buttons are solid Electric Blue with black text. Secondary buttons are ghost-style with a charcoal border and white text. Focus states trigger a 1px primary glow/outline.
- **Editor:** The core component. Features a "gutter" for line numbers using JetBrains Mono. Active lines are highlighted with a subtle `#121212` background tint.
- **Chips/Tags:** Used for note categorization. These use a monochromatic style (Charcoal background, light gray text) to avoid distracting from the main content.
- **Input Fields:** Minimalist design with only a bottom border or a very subtle 1px frame. The label should use `label-mono` for a technical feel.
- **Cards (Note Preview):** Defined by a 1px border. No shadows. On hover, the border color brightens slightly.
- **Lists:** Clean, tight vertical spacing. Hover states on list items use a simple `#1E1E1E` background fill with no border change.
- **Privacy Indicators:** A persistent, small "Encrypted" status icon (using the Emerald Green accent) should be visible in the editor header.