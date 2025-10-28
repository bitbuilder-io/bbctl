import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { openApiSchema } from '../schema';

// Get current file directory with ESM compatibility
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const OUTPUT_DIR = join(__dirname, '../api-docs');
const OUTPUT_FILE = join(OUTPUT_DIR, 'openapi.json');

// Create directory if it doesn't exist
if (!existsSync(OUTPUT_DIR)) {
  console.log(`Creating directory: ${OUTPUT_DIR}`);
  mkdirSync(OUTPUT_DIR, { recursive: true });
}

// Write OpenAPI schema to file
try {
  writeFileSync(OUTPUT_FILE, JSON.stringify(openApiSchema, null, 2), 'utf8');
  console.log(`Successfully generated OpenAPI schema: ${OUTPUT_FILE}`);
} catch (error) {
  console.error('Error generating OpenAPI schema:', error);
  process.exit(1);
}

// Generate a simple HTML to view the schema with Swagger UI
const HTML_FILE = join(OUTPUT_DIR, 'index.html');
const htmlContent = `
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>BitBuilder Cloud CLI API</title>
  <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui.css">
  <style>
    body {
      margin: 0;
      padding: 0;
    }
    #swagger-ui {
      max-width: 1200px;
      margin: 0 auto;
    }
    .docs-links {
      background: #f8f9fa;
      padding: 15px;
      border-radius: 4px;
      margin: 20px auto;
      max-width: 1200px;
    }
    .docs-links h3 {
      margin-top: 0;
    }
    .docs-links a {
      display: inline-block;
      margin-right: 15px;
      color: #0D5AA7;
      text-decoration: none;
      font-weight: 500;
    }
    .docs-links a:hover {
      text-decoration: underline;
    }
  </style>
</head>
<body>
  <div class="docs-links">
    <h3>BitBuilder Cloud CLI Documentation</h3>
    <a href="../docs/api-readme.md" target="_blank">API Documentation</a>
    <a href="../docs/rust-integration.md" target="_blank">Rust Integration Guide</a>
    <a href="../docs/ARCHITECTURE_DESIGN.md" target="_blank">Architecture Design</a>
    <a href="../README.md" target="_blank">Main README</a>
  </div>
  <div id="swagger-ui"></div>
  <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-bundle.js"></script>
  <script>
    window.onload = function() {
      const ui = SwaggerUIBundle({
        url: "./openapi.json",
        dom_id: '#swagger-ui',
        deepLinking: true,
        presets: [
          SwaggerUIBundle.presets.apis,
          SwaggerUIBundle.SwaggerUIStandalonePreset
        ],
        layout: "BaseLayout",
        validatorUrl: null
      });
      window.ui = ui;
    };
  </script>
</body>
</html>
`;

try {
  writeFileSync(HTML_FILE, htmlContent, 'utf8');
  console.log(`Successfully generated Swagger UI HTML: ${HTML_FILE}`);
  console.log(`Open ${HTML_FILE} in your browser to view the API documentation`);
  console.log(`Documentation links have been added to the UI:`);
  console.log(`- API Documentation: docs/api-readme.md`);
  console.log(`- Rust Integration Guide: docs/rust-integration.md`);
  console.log(`- Architecture Design: docs/ARCHITECTURE_DESIGN.md`);
} catch (error) {
  console.error('Error generating Swagger UI HTML:', error);
}
