#!/bin/bash
# Exit immediately if any command exits with a non-zero status
set -e

# Constant for the production branch name.
PRODUCTION_BRANCH="main"

# Verify that the project title and Git repository URL are provided.
if [ "$#" -lt 2 ]; then
    echo "Usage: $0 <project_title> <git_repo_url>"
    exit 1
fi

PROJECT_TITLE=$1
REPO_URL=$2

# Check if git-flow is installed.
if ! command -v git-flow >/dev/null 2>&1; then
    echo "Error: git-flow is not installed. Please install git-flow and try again."
    exit 1
fi

# Check if release-plz is installed.
if ! command -v release-plz >/dev/null 2>&1; then
    echo "Error: release-plz is not installed. Please install release-plz and try again."
    exit 1
fi

# Initialize a git repository if one does not already exist.
if [ ! -d .git ]; then
    echo "Initializing git repository..."
    git init
fi

# Add remote 'origin' if not already present.
if ! git remote get-url origin >/dev/null 2>&1; then
    echo "Adding remote repository: ${REPO_URL}"
    git remote add origin "${REPO_URL}"
fi

# Initialize git flow with default settings.
echo "Initializing git flow with default settings..."
git flow init -d

# Update git flow configuration to use the production branch constant.
echo "Setting production branch to '${PRODUCTION_BRANCH}' in git flow configuration..."
git config gitflow.branch.master "$PRODUCTION_BRANCH"

# If a branch named "master" exists, rename it to our production branch.
if git show-ref --verify --quiet refs/heads/master; then
    echo "Renaming 'master' branch to '${PRODUCTION_BRANCH}'..."
    git branch -m master "$PRODUCTION_BRANCH"
fi

# Create an initial commit if the repository has no commits yet.
if [ -z "$(git rev-parse --verify HEAD 2>/dev/null)" ]; then
    echo "Creating initial commit..."
    if [ ! -f README.md ]; then
        echo "# ${PROJECT_TITLE}" > README.md
    fi
    git add README.md
    git commit -m "Initial commit: ${PROJECT_TITLE}"
fi

# Create a feature branch named "${PROJECT_TITLE}-initial_commit" using git flow.
FEATURE_BRANCH="${PROJECT_TITLE}-initial_commit"
echo "Starting feature branch ${FEATURE_BRANCH}..."
git flow feature start "${FEATURE_BRANCH}"

# (Optional) Do some work on the feature branch. For demonstration, we create an empty commit.
git commit --allow-empty -m "Work on feature ${FEATURE_BRANCH}"

echo "Finishing feature branch ${FEATURE_BRANCH}..."
git flow feature finish "${FEATURE_BRANCH}"

# Check for Cargo.toml and create one if it doesn't exist.
if [ ! -f Cargo.toml ]; then
    echo "Cargo.toml not found. Creating a minimal Cargo.toml file..."
    cat <<EOF > Cargo.toml
[package]
name = "${PROJECT_TITLE}"
version = "0.1.0"
edition = "2021"
license = "MIT"
description = "Initial package for ${PROJECT_TITLE}"
EOF
    git add Cargo.toml
    git commit -m "Add minimal Cargo.toml for release-plz"
fi

# If there are any uncommitted changes, commit them to get a clean working directory.
if [ -n "$(git status --porcelain)" ]; then
    echo "Auto-committing uncommitted changes to clean working directory for release-plz..."
    git add -A
    git commit -m "Auto-commit uncommitted changes before running release-plz update"
fi

# Initialize release-plz configuration.
echo "Initializing release-plz configuration..."
release-plz init

# Update release-plz configuration (this typically updates release notes/changelog).
echo "Updating release-plz configuration..."
release-plz update

echo "Setup complete:
  - Git repository and remote set up
  - Git flow initialized with production branch '${PRODUCTION_BRANCH}'
  - Initial commit created with README
  - Feature branch '${FEATURE_BRANCH}' created and finished
  - Minimal Cargo.toml created with license and description
  - All uncommitted changes auto-committed
  - release-plz initialized and updated"

