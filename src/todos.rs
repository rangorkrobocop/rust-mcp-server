use std::sync::Arc;

use rmcp::{handler::server::{tool::ToolRouter, wrapper::Parameters}, model::{CallToolResult, Content, Implementation, InitializeResult, ProtocolVersion, ServerCapabilities, ServerInfo}, schemars, tool, tool_handler, tool_router, ServerHandler};
use serde::{Serialize, Deserialize};
use rmcp::ErrorData as McpError;

// Struct for our Tools and Tool Router
#[derive(Clone)]
pub struct TodoMcpServer {
    tool_router: ToolRouter<Self> 
}

// Here be Todos
#[derive(Debug, Clone,Deserialize, Serialize)]
pub struct Todos {
    todos: Arc<Vec<Todo>>,
}

// Individual todo
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    id: u32,
    #[serde(rename = "userId")]
    user_id: u32,
    title: String,
    #[serde(default)] // JSONPlaceholder GET /todos doesn't return a body, but POST might
    body: Option<String>,
    #[serde(rename = "completed", default)]
    complete: bool,
}

// Helper Struct that we pass on to the CRUD API to create a new TODO
#[derive(Debug, Serialize, Deserialize, Clone)]
struct NewTodo {
    #[serde(rename = "userId")]
    user_id: u32,
    title: String,
    body: String
}

// This is a Parmeter struct for creating a new TODO. This tells the LLM what it needs to pass
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[schemars(description = "Input for creating a new TODO entry")]
pub struct NewTodoParameters {
    #[schemars(description = "Title of the TODO item")]
    title: String,
    #[schemars(description = "Body of the TODO item")]
    body: String
}

// Impl block for our Todos. These are not Tools (yet)
impl Todos {
    /// Get all TODOs from the API using reqwest
    async fn get_todos() -> Result<Self, anyhow::Error> {
        let response: Vec<Todo> = reqwest::get("https://jsonplaceholder.typicode.com/todos")
            .await?
            .json()
            .await?;
        
        Ok(Self {todos: Arc::new(response)}) // Return all todos as an Arc
    }

    /// Creates a new TODO from a tool call
    async fn create_todo(todo: NewTodo) -> Result<Todo, anyhow::Error> {
        let response: Todo = reqwest::Client::new()
            .post("https://jsonplaceholder.typicode.com/todos")
            .json(&todo) // Sending a serialized NewTodo
            .send()
            .await? // Send the request and wait
            .json() // Deserialize the JSON back
            .await?; // Wait for it to happen
        
        // Return the newly created TODO entry as Todo
        Ok(response)
    }
}

#[tool_router] // Macro that generates the tool router
impl TodoMcpServer {
    pub fn new() -> Self {
        Self { 
            tool_router: Self::tool_router()
        }
    }

    // Tools
    /// Get all TODO entries from via the get_todos() method
    // This description is being passed onto the MCP client. The name of the tool (by default) is
    // the function name.
    #[tool(description = "Get all available TODOs in JSON format")]
    async fn get_all_todos() -> Result<CallToolResult, McpError> {
        // Call the get_todos() method to get all the todos as a Todos struct
        let todos = Todos::get_todos()
            .await
            .map_err(|e| McpError::internal_error(
                format!("Error getting TODOs: {}",e), None))?;

        // Convert it to json Content
        let content = Content::json(todos)
            .map_err(|e| McpError::internal_error(
                format!("Error converting TODOs into JSON values: {}", e), None))?;

        Ok(CallToolResult::success(
            vec![content]
        ))
    }
    /// Creates a new TODO by getting the NewTodoParameters from the client
    #[tool(description = "Create a new TODO entry by passing on the title and body of the TODO item.")]
    async fn create_new_todo(
        Parameters(NewTodoParameters {
        title,
        body,
    }): Parameters<NewTodoParameters>,
    )-> Result<CallToolResult, McpError> {
        // Get the user_id from the environment variable
        let user_id_str = std::env::var("USER_ID")
            .map_err(|_| McpError::internal_error(
                "USER_ID environment variable MUST be set".to_string(), None))?;
        let user_id: u32 = user_id_str.parse()
            .map_err(|_| McpError::internal_error(
                "USER_ID MUST be an integer".to_string(), None))?;
        // Create a NewTodo from the parameters supplied by the MCP client
        let new_todo_request = NewTodo {
            user_id,
            title,
            body,
        };
        let new_todo = Todos::create_todo(new_todo_request) // Invoke the create_todo 
            .await
            .map_err(|e| McpError::internal_error(          // Let the MCP client know if we fail
                    format!("Error creating new TODO entry: {}", e), None))?;

        // Return a structure message back instead of pure JSON
        let return_message = format!(
            "A new TODO entry has been created, here are its details:
            - id: {}
            - title: {}
            - body: {}
            Use the ID of the TODO for any todo specific instructions.
            ", new_todo.id, new_todo.title, new_todo.body.unwrap_or_default()
        );

        // Return CallToolResult as `text`
        Ok(CallToolResult::success(
            vec![Content::text(return_message)]
        ))
    }
}

#[tool_handler] // Macro that will generate a tool handler
impl ServerHandler for TodoMcpServer {
    fn get_info(&self) -> rmcp::model::ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_06_18,
            capabilities: ServerCapabilities::builder()
                .enable_tools() // Only enable tools
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
        "I manage a list of TODOs. That are stored behind an API server.

        The available actions are:
        - get_all_todos: Get a list of all the todos.
        - create_new_todo: Create a new todo entry
        ".to_string()),
        }
    }
    async fn initialize(
            &self,
            _request: rmcp::model::InitializeRequestParam,
            _context: rmcp::service::RequestContext<rmcp::RoleServer>,
        ) -> Result<InitializeResult, McpError> {
            Ok(self.get_info())
    }
}
    



