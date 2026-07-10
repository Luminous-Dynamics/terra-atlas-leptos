#!/usr/bin/env bash
# Sync Sol Atlas (Leptos web globe) to its standalone public GitHub repo.
#
# Standalone: github.com/Luminous-Dynamics/terra-atlas-leptos
#   (repo keeps its pre-rename name; it serves atlas.luminousdynamics.io via
#    GitHub Pages — DNS CNAMEs to luminous-dynamics.github.io, NOT the
#    Cloudflare tunnel. Pushing to main triggers .github/workflows/deploy.yml.)
#
# Layout mapping:
#   monorepo sol-atlas-leptos/  -> standalone /            (repo root)
#   monorepo sol-atlas-core/    -> standalone /sol-atlas-core/  (vendored)
#
# Fixups applied to the synced Cargo.toml:
#   - path dep "../sol-atlas-core" -> "sol-atlas-core"
#   - the optional mycelix-leptos-core dep + `holochain` feature are stripped
#     (monorepo-only; the cfg(feature="holochain") code is inert without it)
#
# Usage:
#   bash sol-atlas-leptos/scripts/sync-to-standalone.sh [--dry-run] [--force]
#
# ONLY run this once the tree builds green — pushing to main deploys the
# public site.

set -euo pipefail

STANDALONE_REMOTE="git@github.com:Luminous-Dynamics/terra-atlas-leptos.git"

MONOREPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
LEPTOS_DIR="${MONOREPO_ROOT}/sol-atlas-leptos"
CORE_DIR="${MONOREPO_ROOT}/sol-atlas-core"
STANDALONE_REPO="/tmp/terra-atlas-leptos-standalone-sync"

DRY_RUN=false
FORCE=false
for arg in "$@"; do
    case "$arg" in
        --dry-run) DRY_RUN=true ;;
        --force)   FORCE=true ;;
    esac
done

GREEN="\033[32m"; YELLOW="\033[33m"; RED="\033[31m"; CYAN="\033[36m"; RESET="\033[0m"
info()  { printf "${CYAN}[info]${RESET}  %s\n" "$*"; }
warn()  { printf "${YELLOW}[warn]${RESET}  %s\n" "$*"; }
ok()    { printf "${GREEN}[ok]${RESET}    %s\n" "$*"; }
error() { printf "${RED}[error]${RESET} %s\n" "$*"; exit 1; }

[ -f "${LEPTOS_DIR}/Cargo.toml" ] || error "Cannot find ${LEPTOS_DIR}/Cargo.toml"
[ -f "${CORE_DIR}/Cargo.toml" ]   || error "Cannot find ${CORE_DIR}/Cargo.toml"

info "Monorepo root: ${MONOREPO_ROOT}"
$DRY_RUN && warn "DRY RUN — no commits or pushes"

# --- Clone or update standalone -----------------------------------------------

if [ -d "${STANDALONE_REPO}/.git" ]; then
    info "Updating existing standalone clone..."
    git -C "${STANDALONE_REPO}" fetch origin
    git -C "${STANDALONE_REPO}" reset --hard origin/main
else
    info "Cloning standalone repo..."
    git clone "${STANDALONE_REMOTE}" "${STANDALONE_REPO}"
fi

# --- Export committed HEAD (NOT the working tree) ------------------------------
# 12+ concurrent sessions leave uncommitted WIP in the shared tree; rsyncing
# the working tree would publish everyone's WIP wholesale (see
# MASTER_ROADMAP.md "Clean-checkpoint sync cadence"). git archive exports
# exactly what is committed.

STAGING="$(mktemp -d /tmp/sol-atlas-sync-staging.XXXXXX)"
trap 'rm -rf "${STAGING}"' EXIT

info "Exporting committed HEAD to staging..."
git -C "${MONOREPO_ROOT}" archive HEAD -- sol-atlas-leptos sol-atlas-core \
    | tar -x -C "${STAGING}"

info "Syncing sol-atlas-leptos (HEAD) -> standalone root..."
rsync -a --delete \
    --exclude='.git' \
    --exclude='sol-atlas-core/' \
    "${STAGING}/sol-atlas-leptos/" "${STANDALONE_REPO}/"

info "Syncing sol-atlas-core (HEAD) -> standalone/sol-atlas-core..."
rsync -a --delete \
    --exclude='.git' \
    "${STAGING}/sol-atlas-core/" "${STANDALONE_REPO}/sol-atlas-core/"

# --- Fixups -------------------------------------------------------------------

info "Applying Cargo.toml fixups..."
sed -i \
    -e 's|sol-atlas-core = { path = "../sol-atlas-core" }|sol-atlas-core = { path = "sol-atlas-core" }|' \
    -e '/mycelix-leptos-core = { path/d' \
    -e '/^# Holochain live data bridge/d' \
    -e '/^holochain = \["dep:mycelix-leptos-core"\]/d' \
    "${STANDALONE_REPO}/Cargo.toml"

grep -q 'path = "sol-atlas-core"' "${STANDALONE_REPO}/Cargo.toml" \
    || error "Cargo.toml fixup failed — path dep not rewritten"
grep -q 'mycelix' "${STANDALONE_REPO}/Cargo.toml" \
    && error "Cargo.toml fixup failed — mycelix dep still present"

# --- Commit and push ----------------------------------------------------------

cd "${STANDALONE_REPO}"
git add -A
if git diff --cached --quiet; then
    ok "No changes to sync"
    exit 0
fi

CHANGED=$(git diff --cached --stat | tail -1)
info "Changes: ${CHANGED}"

if $DRY_RUN; then
    warn "DRY RUN — skipping commit and push"
    git diff --cached --stat
    exit 0
fi

MONO_SHA=$(git -C "${MONOREPO_ROOT}" rev-parse --short HEAD)
COMMIT_MSG="sync: update from monorepo @ ${MONO_SHA} ($(date -u +%Y-%m-%dT%H:%M:%SZ))"

if ! $FORCE; then
    echo ""
    echo "About to commit and push (this DEPLOYS atlas.luminousdynamics.io):"
    git diff --cached --stat
    echo ""
    read -rp "Proceed? [y/N] " confirm
    [ "$confirm" = "y" ] || [ "$confirm" = "Y" ] || { warn "Aborted"; exit 1; }
fi

git commit -m "${COMMIT_MSG}"
git push origin HEAD
ok "Synced to terra-atlas-leptos — GitHub Pages deploy will run from main"
