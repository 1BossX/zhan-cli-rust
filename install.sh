#!/bin/sh
# zhan-cli One-line Installer
# Supports macOS (Intel/Apple Silicon) and Linux (x86_64/ARM64)

set -e

REPO="1BossX/zhan-cli-rust"
BINARY_NAME="zhan"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Detect target platform
get_target() {
    local os=$(uname -s)
    local arch=$(uname -m)

    case "$os" in
        Darwin)
            case "$arch" in
                arm64|aarch64)
                    echo "aarch64-apple-darwin"
                    ;;
                x86_64|amd64)
                    echo "x86_64-apple-darwin"
                    ;;
                *)
                    echo "${RED}Error: Unsupported macOS architecture: $arch${NC}" >&2
                    exit 1
                    ;;
            esac
            ;;
        Linux)
            case "$arch" in
                aarch64|arm64)
                    echo "aarch64-unknown-linux-gnu"
                    ;;
                x86_64|amd64)
                    echo "x86_64-unknown-linux-gnu"
                    ;;
                *)
                    echo "${RED}Error: Unsupported Linux architecture: $arch${NC}" >&2
                    exit 1
                    ;;
            esac
            ;;
        MINGW*|MSYS*|CYGWIN*)
            echo "x86_64-pc-windows-gnu"
            ;;
        *)
            echo "${RED}Error: Unsupported operating system: $os${NC}" >&2
            exit 1
            ;;
    esac
}

# Get latest version tag
get_latest_tag() {
    local api_url="https://api.github.com/repos/${REPO}/releases/latest"
    local tag=$(curl -sSL "$api_url" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

    if [ -z "$tag" ]; then
        echo "${RED}Error: Unable to get latest version information${NC}" >&2
        exit 1
    fi

    echo "$tag"
}

# Main installation logic
main() {
    echo "${GREEN}=== zhan-cli Installer ===${NC}"
    echo ""

    # Detect platform
    local target=$(get_target)
    echo "Detected platform: ${YELLOW}$target${NC}"

    # Get latest version
    echo "Fetching latest version information..."
    local tag=$(get_latest_tag)
    echo "Latest version: ${YELLOW}$tag${NC}"

    # Determine archive extension
    case "$target" in
        *-windows-gnu)
            local archive_name="${BINARY_NAME}-${tag}-${target}.zip"
            ;;
        *)
            local archive_name="${BINARY_NAME}-${tag}-${target}.tar.gz"
            ;;
    esac

    # Construct download URL
    local download_url="https://github.com/${REPO}/releases/download/${tag}/${archive_name}"

    echo "Downloading from: $download_url"

    # Create temp directory
    local tmp_dir=$(mktemp -d)
    cd "$tmp_dir"

    # Download and extract
    curl -sSL "$download_url" -o "$archive_name"

    # Extract
    case "$archive_name" in
        *.tar.gz)
            tar -xzf "$archive_name"
            ;;
        *.zip)
            unzip -q "$archive_name"
            ;;
    esac

    # Install binary
    echo "Installing to ${INSTALL_DIR}..."
    case "$target" in
        *-windows-gnu)
            cp "${BINARY_NAME}.exe" "${INSTALL_DIR}/${BINARY_NAME}.exe"
            ;;
        *)
            cp "$BINARY_NAME" "${INSTALL_DIR}/${BINARY_NAME}"
            chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
            ;;
    esac

    # Cleanup
    cd /
    rm -rf "$tmp_dir"

    echo ""
    echo "${GREEN}✅ Installation complete!${NC}"
    echo ""
    echo "Run '${BINARY_NAME}' to get started:"
    echo "  ${YELLOW}${BINARY_NAME} login${NC}        # 登录"
    echo "  ${YELLOW}${BINARY_NAME} whoami${NC}       # 查看当前用户"
    echo "  ${YELLOW}${BINARY_NAME} feed${NC}         # 浏览社区"
    echo ""
}

main
