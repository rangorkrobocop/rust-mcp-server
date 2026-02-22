USER_ID := "1"

release:
  cargo build --release

# Test with MCP inspector
mcp-test: release
  npx @modelcontextprotocol/inspector -e USER_ID={{ USER_ID }} ./target/release/todo_mcp

# Build the docker container
docker-build:
  docker build -t ghcr.io/rangorkrobocop/rust-mcp-server:latest .

# Push the docker container to ghcr
docker-push: docker-build
  docker push ghcr.io/rangorkrobocop/rust-mcp-server:latest

# Test the docker container with MCP inspector
docker-test: docker-build
  npx @modelcontextprotocol/inspector -e USER_ID={{ USER_ID }} docker run -i --rm -e USER_ID={{ USER_ID }} ghcr.io/rangorkrobocop/rust-mcp-server:latest
