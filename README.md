# Chrona (working title)

Watch-style health and sensor assistant running on desktop/laptop hardware with tiered constraints (8 GB / 16 GB / 32 GB). All tiers share the same core ML/LLM tasks but scale capability, storage, and context.

## Layout
- docs/: design notes
- rust/: core runtime (Rust workspace)
- python/: data prep, training, export pipelines
- sql/: schema snippets, migrations, and queries
- data/: local datasets or fixtures (kept small by default)

## Next steps
- Flesh out tier configs and manifests (models, prompts, storage quotas).
- Add initial synthetic sensor generators and TinyML model loaders.
- Bring in llamafile-based local LLM loaders per tier.
- Add golden traces for pipeline tests.
