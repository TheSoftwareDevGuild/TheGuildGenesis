#!/bin/bash

# The Guild Genesis - Development Environment
# This script sets up the development environment using system tools

echo "🔧 The Guild Genesis Development Environment"
echo "📦 Using system tools instead of Nix"

# Check if required tools are installed
check_tool() {
    if ! command -v "$1" &> /dev/null; then
        echo "❌ $1 is not installed. Please install it with: brew install $1"
        exit 1
    fi
}

echo "🔍 Checking required tools..."
check_tool "node"
check_tool "npm"
check_tool "cargo"
check_tool "just"

# Set environment variables
export DATABASE_URL="postgres://guild_user:guild_password@localhost:5432/guild_genesis"
export RUST_LOG="debug"

echo "✅ All tools are available!"
echo ""
echo "📦 Available commands:"
echo "  just dev          - Start both frontend and backend"
echo "  just dev-frontend - Start frontend only"
echo "  just dev-backend  - Start backend only"
echo "  just db-start     - Start PostgreSQL database"
echo "  just db-stop      - Stop PostgreSQL database"
echo "  just db-setup     - Set up database with migrations"
echo "  just db-reset     - Reset database completely"
echo "  just install-all  - Install all dependencies"
echo ""
echo "🚀 Run 'just <command>' to execute scripts"
echo ""
echo "💡 This uses system tools (Node.js, Rust, PostgreSQL) instead of Nix"
echo "💡 Install them with: brew install node rust postgresql just"

# Start a new shell with the environment
exec "$SHELL" -i

