Rust reqwest/api.mustache is derived from OpenAPITools/openapi-generator v7.26.0:
https://github.com/OpenAPITools/openapi-generator/blob/v7.26.0/modules/openapi-generator/src/main/resources/rust/reqwest/api.mustache
Licensed under Apache-2.0; upstream license is included as LICENSE.openapitools.

MailChannels modifications (2026-10-06): success only for HTTP 2xx; choose success/error variant by numeric HTTP status; do not derive body-only Deserialize for ambiguous response enums; surface malformed nonempty known success JSON instead of silently discarding it. Preserve raw error content/status for unknown or malformed error schemas. Generation config requires supportMultipleResponses=true. Native tests compile all API modules but exercise sending endpoints only. No claim of complete response coverage for all 42 operations.

configuration.mustache and api_mod.mustache are derived from the same upstream rust/reqwest directory/tag and license. Configuration changes: HTTPS only, redirect Policy::none, retry::never, 10-second connect/30-second request timeout; exposes transport_builder for explicit trusted overrides. Error changes: redacted Display/Debug and no automatic source chain; raw typed error fields remain accessible. This customization targets the configured async reqwest client, not unsupported middleware/AWS/blocking generator combinations.

Path encoding correction: the generic urlencode helper is used by generated path parameters. Form-encoded spaces are converted from + to %20; literal plus remains %2B. Verified through captured native sub-account requests containing slash, space, plus, question mark and fragment characters.

Optional JSON request body correction: required bodies always serialize; optional bodies serialize only for Some. None sends no JSON body instead of literal null, matching the optional/non-nullable published create-sub-account schema.

model.mustache derives from upstream v7.26.0 rust/model.mustache under the included Apache-2.0 license. Structured models print only type/redaction markers, preserving serialization and direct access. Response enum Debug and Configuration Debug are also redacted; unknown JSON variants cannot bypass this. Static enum labels retain Debug. Configuration ApiKey redacts its optional prefix as well as the key. Four regression tests cover credentials, message/DKIM data, caller-provided config values and unknown JSON.

Cargo.mustache and README.mustache are based on the same upstream v7.26.0 rust templates. Customized package metadata uses company MIT licensing/support precedent and publish=false for the unreleased candidate, with src/docs/README/LICENSE inclusions. Template lint corrections remove needless returns, iteration over references via into_iter, empty enum doc comments and a manual Option map.
