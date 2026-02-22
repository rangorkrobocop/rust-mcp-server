# CRUD MCP Server - sample code

> **NOTE:** This code is presented AS-IS for educational and demonstration purposes. It is not intended for production use without proper review, testing, and security considerations.

A Rust implementation of a Model Context Protocol (MCP) server that provides CRUD operations for a TODO API.

## Features

- **Get TODOs**: Retrieve all TODO entries from the API
- **Create TODOs**: Add new TODO entries with title and body
- Built with Rust using the `rmcp` crate
- Async/await throughout for optimal performance
- Proper error handling and MCP protocol compliance

## Prerequisites

- Rust (latest stable)
- A running TODO API server on `localhost:3000`
- Node.js (for MCP inspector testing)

## Quick Start

1. **Build the project:**
   ```bash
   just release
   ```

2. **Test with MCP Inspector:**
   ```bash
   just mcp-test
   ```

## Docker

You can run the server in a Docker container (perfect for adding it as a tool to MCP clients like Claude Desktop):

1. **Build the image:**
   ```bash
   just docker-build
   ```

2. **Test the image with MCP Inspector:**
   ```bash
   just docker-test
   ```

## Environment Variables

- `USER_ID`: Required integer for TODO operations (set automatically in justfile)

## API Integration

The server uses the public [JSONPlaceholder](https://jsonplaceholder.typicode.com/todos) API for testing:

- `GET /todos` - Returns array of TODO objects
- `POST /todos` - Creates new TODO from JSON body

## Usage

Once running, the MCP server provides two tools:

- `get_all_todos`: Fetches and returns all TODO entries
- `create_new_todo`: Creates a new TODO with specified title and body

Perfect for demonstrating Rust MCP server development patterns and async API integration.
