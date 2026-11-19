# Private release dependency access

The release caller pins the authenticated `private-release.yml` workflow in
`tomblanchard312/craft-common-actions`. It retains the upstream release recipe
from revision `3e2ac193fdf93009ef6e88ae34fe1c32340ae039`.
`ERIDU_STYLE_DEPLOY_KEY` is a repository secret with read-only access to
`tomblanchard312/tavru-style`. Source jobs use the same pinned checkout as CI.
The key is removed after checkout, and private source and compiled Cargo outputs
are excluded from public Actions caches. FreeBSD receives the checkout through
rsync and resolves it locally in the VM.

To check release dependency access, dispatch the **Release** workflow with
`validate-only` enabled. This resolves the committed lockfile on Linux, Windows,
macOS and FreeBSD. Packaging, signing, registry pushes and release creation are
skipped in this mode. A successful access check does not validate the installers
or signing credentials. Regular release runs retain the existing signing gates
and package formats.
