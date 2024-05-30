# Release notes

## Upcoming ignition-config 0.5.0 (unreleased)



## ignition-config 0.4.0 (2024-05-30)

- Require Rust ≥ 1.75.0
- Add packit initial support
- docs/release-notes: update for release 0.4.0


## ignition-config 0.3.0 (2023-02-20)

- Add Ignition 3.4.0 spec
- Require Rust ≥ 1.56.0
- Add release notes to repo


## ignition-config 0.2.0 (2021-12-03)

- Implement `Default` for `Config` structs
- Add `new()` associated functions for non-`Default` structs
- Add enum of version-specific `Config` structs, and associated functions to parse an Ignition config of unknown version


## ignition-config 0.1.0 (2021-11-23)

- Commit generated Rust code to the repo; switch `schemafy` to an optional build-time dependency
- Consolidate `Directory`/`File`/`Link` user/group structs into `NodeUser` and `NodeGroup`
- Rename `start_mi_b` → `start_mib`, `size_mi_b` → `size_mib`, `tpm_2` → `tpm2`


## ignition-config 0.0.1 (2021-07-30)

- Initial release
