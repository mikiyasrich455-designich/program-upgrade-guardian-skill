#!/bin/bash
#
# Program Upgrade Guardian - Installer
# Simple, safe bash installer for the skill
# No hidden commands, no system modifications
#

set -e

SKILL_NAME="program-upgrade-guardian"
SKILL_VERSION="2026.06"
REPO_URL="https://github.com/mikiyasrich455-designich/program-upgrade-guardian-skill.git"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
RESET='\033[0m'

print_header() {
    echo ""
    echo "${BOLD}══════════════════════════════════════════════════${RESET}"
    echo "${BOLD}  Program Upgrade Guardian - Installer${RESET}"
    echo "${BOLD}  v${SKILL_VERSION}${RESET}"
    echo "${BOLD}══════════════════════════════════════════════════${RESET}"
    echo ""
}

print_ok() {
    echo "${GREEN}✓${RESET} $1"
}

print_warn() {
    echo "${YELLOW}⚠${RESET} $1"
}

print_error() {
    echo "${RED}✗${RESET} $1"
}

print_info() {
    echo "${BLUE}ℹ${RESET} $1"
}

# Detect OS
 detect_os() {
    case "$(uname -s)" in
        Linux*)     OS=Linux;;
        Darwin*)    OS=Mac;;
        CYGWIN*|MINGW*|MSYS*) OS=Windows;;
        *)          OS=Unknown;;
    esac
    echo "${OS}"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Main install function
install_skill() {
    print_header

    local install_location=""
    local os=$(detect_os)

    print_info "Detected OS: ${os}"
    echo ""

    # Check prerequisites
    print_info "Checking prerequisites..."

    local missing=()

    if ! command_exists "git"; then
        missing+=("git")
    fi

    if ! command_exists "solana"; then
        print_warn "Solana CLI not found. Install from https://docs.solana.com/cli/install"
    else
        print_ok "Solana CLI: $(solana --version 2>/dev/null | head -1)"
    fi

    if ! command_exists "anchor"; then
        print_warn "Anchor not found. Install: avm install latest"
    else
        print_ok "Anchor: $(anchor --version 2>/dev/null)"
    fi

    if [ ${#missing[@]} -ne 0 ]; then
        print_error "Missing required tools: ${missing[*]}"
        echo "Install them and re-run this script."
        exit 1
    fi

    echo ""

    # Ask install location
    echo "${BOLD}Choose install location:${RESET}"
    echo "  1) Personal: ~/.claude/skills/ (recommended)"
    echo "  2) Project: ./.claude/skills/ (current directory)"
    echo "  3) Custom: specify path"
    echo ""
    read -p "Enter choice [1-3] (default: 1): " choice

    case "${choice:-1}" in
        1)
            install_location="${HOME}/.claude/skills/${SKILL_NAME}"
            mkdir -p "${HOME}/.claude/skills"
            ;;
        2)
            install_location="./.claude/skills/${SKILL_NAME}"
            mkdir -p "./.claude/skills"
            ;;
        3)
            read -p "Enter custom path: " custom_path
            install_location="${custom_path}/${SKILL_NAME}"
            mkdir -p "${custom_path}"
            ;;
        *)
            install_location="${HOME}/.claude/skills/${SKILL_NAME}"
            mkdir -p "${HOME}/.claude/skills"
            ;;
    esac

    echo ""
    print_info "Installing to: ${install_location}"

    # Clone or copy
    if [ -d ".git" ]; then
        # Running from repo
        print_info "Copying from local repository..."
        mkdir -p "${install_location}"
        cp -r ./* "${install_location}/" 2>/dev/null || true
    else
        # Clone from GitHub
        print_info "Cloning from GitHub..."
        git clone --depth 1 "${REPO_URL}" "${install_location}"
    fi

    # Verify installation
    if [ -f "${install_location}/SKILL.md" ]; then
        echo ""
        print_ok "Skill installed successfully!"
        echo ""
        print_info "Location: ${install_location}"
        print_info "To use: Ask Claude 'Help me upgrade my Solana program'"
        echo ""
        print_info "Stack: Anchor 0.30, Solana 1.18, LiteSVM, Surfpool, Squads"
        echo ""

        # Show multi-AI support
        echo "${BOLD}Multi-AI Support:${RESET}"
        echo "  Claude:        ~/.claude/skills/${SKILL_NAME}/"
        echo "  Cursor:        .cursor/skills/${SKILL_NAME}/"
        echo "  Codex:         ~/.codex/skills/${SKILL_NAME}/"
        echo "  Gemini:        ~/.gemini/skills/${SKILL_NAME}/"
        echo "  GitHub Copilot: .github/skills/${SKILL_NAME}/"
        echo "  Windsurf:      .windsurf/skills/${SKILL_NAME}/"
        echo "  ChatGPT:       ~/.chatgpt/skills/${SKILL_NAME}/"
        echo "  DeepSeek:      ~/.deepseek/skills/${SKILL_NAME}/"
        echo ""

        print_ok "Guardian is ready. Safe upgrades only."
    else
        print_error "Installation failed. SKILL.md not found."
        exit 1
    fi
}

# Run dependency check
run_check() {
    print_header
    print_info "Running dependency check..."
    echo ""

    if [ -f "install.py" ]; then
        python3 install.py
    else
        print_warn "install.py not found. Run from repo root."
    fi
}

# Show help
show_help() {
    echo "${BOLD}Program Upgrade Guardian - Installer${RESET}"
    echo ""
    echo "Usage:"
    echo "  ./install.sh           Install the skill (interactive)"
    echo "  ./install.sh -y        Install with defaults (non-interactive)"
    echo "  ./install.sh check     Run dependency check only"
    echo "  ./install.sh help      Show this help"
    echo ""
    echo "Default install location: ~/.claude/skills/${SKILL_NAME}/"
    echo ""
    echo "Stack: Anchor 0.30, Solana 1.18, LiteSVM, Surfpool, Squads CLI"
}

# Main
case "${1:-}" in
    check)
        run_check
        ;;
    help|--help|-h)
        show_help
        ;;
    -y|--yes)
        # Non-interactive with defaults
        install_location="${HOME}/.claude/skills/${SKILL_NAME}"
        mkdir -p "${HOME}/.claude/skills"

        if [ -d ".git" ]; then
            mkdir -p "${install_location}"
            cp -r ./* "${install_location}/" 2>/dev/null || true
        else
            git clone --depth 1 "${REPO_URL}" "${install_location}"
        fi

        echo "${GREEN}✓${RESET} Installed to ${install_location}"
        ;;
    *)
        install_skill
        ;;
esac
