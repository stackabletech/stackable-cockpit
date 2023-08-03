# Cockpit Localization

We use [Fluent](https://projectfluent.org/) to power our translations.

Each translation is stored as a file in the `locale/` folder, following the naming convention `locale/{langtag}.ftl`, where
`{langtag}` IETF language tag (for example: `en` for "generic english", or `en-US` for "US english").

Translation keys are named according using `kebab-case`, with `--` used as the hierarchy separator (for example:
`category--long-key`).
