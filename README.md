# Chrona (working title)

Watch-style health and sensor assistant running on desktop/laptop hardware with tiered constraints (8 GB / 16 GB / 32 GB). All tiers share the same core ML/LLM tasks but scale capability, storage, and context.

## Layout
- **docs/**: design notes and architecture decisions
- **rust/**: core runtime (Rust workspace with iced UI)
- **python/**: data prep, training, export pipelines
- **sql/**: schema snippets, migrations, and queries
- **data/**: tier-specific local storage (mini/, regular/, pro/); max few hundred MB each

## UI & Interaction
- **iced-based** multi-display interface: view 1, 2, or all 3 tiers simultaneously
- **Tap** (mouse): clicks for selection/acknowledge
- **Gestures** (keyboard arrows): navigate between screens
- **Voice** (push-to-talk): speech-to-text for LLM queries (whisper.cpp or vosk)
- **Zoom** (spacebar/+/-): zoom charts and detail views

See [docs/ui-design.md](docs/ui-design.md) for details.

## Next steps
- Flesh out tier configs and manifests (models, prompts, storage quotas).
- Add initial synthetic sensor generators and TinyML model loaders.
- Bring in llamafile-based local LLM loaders per tier.
- Integrate whisper.cpp or vosk for local speech-to-text.
- Add golden traces for pipeline tests.
