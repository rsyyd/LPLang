# Versioning Policy

## Scheme
`MAJOR.MINOR.PATCH[-TIER.BUILD]`

## Tiers (computed, not chosen)
- `alpha` — CI green, compiles
- `beta` — 80% coverage, 2 weeks soak
- `rc` — 90% coverage, 4 weeks soak, benchmarks ±5%
- `stable` — 95% coverage, 8 weeks soak, zero known bugs

No human overrides data gates.