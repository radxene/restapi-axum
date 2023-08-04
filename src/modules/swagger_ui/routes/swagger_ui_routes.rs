use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::modules::author::routes::{self as author};
use crate::shared::models::dto::response::RejectJsonResponse;

#[derive(OpenApi)]
#[openapi(
    paths(
        author::get_author,
        author::create_author,
    ),
    components(
        schemas(RejectJsonResponse),
    ),
    modifiers(),
    info(
        title = "RestApi-Axum - OpenAPI 3.0",
        description = "This is a sample RestApi Server based on the OpenAPI 3.0 specification.",
        contact(
            email = "message.raduene@gmail.com",
        ),
        license(name = "Apache 2.0", url = "http://www.apache.org/licenses/LICENSE-2.0.html"),
        version = "1.0.0",
    ),
    external_docs(url = "http://swagger.io", description = "Find out more about Swagger"),
    tags(
        (name = "author", description = "Operation about author"),
    ),
)]
struct ApiDoc;

pub fn swagger_ui_routes() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi())
}
