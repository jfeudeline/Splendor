# 0013. Distribution, licensing and card data

- Status: Accepted
- Date: 2026-11-03

## Context

The code is MIT. Splendor, 6 nimmt!, Gang of Four and Agricola — their rules, card
data and artwork — belong to their publishers. Numeric card data (costs, bonuses,
points) are facts and can be stored; artwork, logos and the publishers' visual
identity cannot be redistributed. The roadmap plans a public web app and server, and a
public PWA named "Splendor" with official art would be a trademark and copyright risk.
The project is a personal learning project, not a commercial product.

## Decision

1. **Code.** MIT (see `LICENSE`). The README keeps the disclaimer: the project is not
   affiliated with the publishers; rules and card data belong to their owners.
2. **No official assets.** No artwork, logos, fonts or card images in the repository
   or in a deployed app. The UI uses a generic theme (abstract gems, neutral colours),
   not the publisher's visual identity.
3. **Card data.** Costs, bonuses, points and counts live in a data file (for example
   `crates/splendor/data/cards.ron` or `.json`) loaded with `include_str!`. Card names
   and flavour text are not required for play; if included, keep them in a separate
   optional file that can be removed. The engine reads `CardId` and `NobleId` indices;
   it does not hard-code names or art.
4. **Branding.** A public deployment uses a neutral name (the project's own name, or a
   generic title). "Splendor" appears in the README and docs as a description ("a
   clone of Splendor"), not as the product name or the URL. Nominative use in the docs
   is acceptable; the app name is the risk.
5. **Distribution.** Default to a **private deployment** (authenticated) or a
   code-only release. The data may ship with the code for a personal learning project;
   a public commercial release requires a license from the publishers. The README
   states this.
6. **Tests.** Verify 90 development cards (40 level 1, 30 level 2, 20 level 3), 10
   nobles, token counts for 2, 3 and 4 players, unique IDs and the cost/bonus ranges.
   The data file is the single source.
7. **Removal path.** If a publisher requests removal, the data file can be replaced or
   removed without touching the engine or the rules. The `CardId` catalog is separate
   from the rules.

## Alternatives considered

- **Ship official artwork.** Not licensed; rejected.
- **Hard-code card data in Rust.** Harder to replace; a data file is better. Rejected.
- **Name the app "Splendor".** Trademark risk; rejected for public deployment.
- **No card data in the repository (the user supplies it).** Safest, but it hurts
  out-of-the-box play. Keep the data, isolate it and document the risk; accepted for a
  personal project.
- **Obtain a license from the publisher.** Not feasible now; revisit if the project
  becomes commercial.

## Consequences

- The repository can be public for learning without shipping assets.
- The engine and the data are decoupled; a data swap is easy.
- A public app needs a neutral brand; the README can still describe the project as a
  Splendor clone.
- The legal risk is documented and bounded, not hidden.
- The card-data test is a phase 1/2 exit criterion (90/10 and token counts).
