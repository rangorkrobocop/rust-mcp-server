USER_ID := "1"
IMAGE_URL := "ghcr.io/rangorkrobocop/rust-mcp-server:latest"

release:
  cargo build --release

# Test with MCP inspector
mcp-test: release
  npx @modelcontextprotocol/inspector -e USER_ID={{ USER_ID }} ./target/release/todo_mcp

# Build the docker container
docker-build:
  docker build --platform linux/amd64,linux/arm64 -t {{ IMAGE_URL }} .

# Push the docker container to ghcr
docker-push: docker-build
  docker push {{ IMAGE_URL }}

# Test the docker container with standard MCP inspector
docker-test: docker-build
  npx @modelcontextprotocol/inspector -e USER_ID={{ USER_ID }} docker run -i --rm -e USER_ID={{ USER_ID }} {{ IMAGE_URL }}

# --- Docker MCP Catalog ---

CATALOG_NAME := "my-private-catalog"
SERVER_NAME := "todo_mcp"

# Initialize a private MCP catalog (run once)
mcp-init-catalog:
  -docker mcp catalog create {{ CATALOG_NAME }}
  docker mcp catalog add {{ CATALOG_NAME }} {{ SERVER_NAME }} todo-mcp.yaml --force

# Run the docker container via MCP Gateway
mcp-gateway-run-catalog: mcp-refresh-server
  docker mcp gateway run --catalog {{ CATALOG_NAME }}

# Enable the custom server
mcp-enable-server:
  -docker mcp server enable {{ SERVER_NAME }}

# Force refresh server metadata
mcp-refresh-server:
  -docker mcp server disable {{ SERVER_NAME }}
  docker mcp server enable {{ SERVER_NAME }}

# Inspect the server configuration
mcp-inspect:
  docker mcp catalog show {{ CATALOG_NAME }} --format yaml | grep -A 4 "{{ SERVER_NAME }}:"
