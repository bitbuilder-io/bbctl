# BitBuilder Cloud CLI API Documentation

This directory contains the API schema definitions for the BitBuilder Cloud CLI (bbctl) using Zod and OpenAPI 3.1. These schemas provide type validation, documentation, and a foundation for building TypeScript/JavaScript clients to interact with bbctl.

## Overview

The API schema is defined using [Zod], a TypeScript-first schema validation library, and converted to OpenAPI 3.1 format using [zod-to-openapi]. This provides:

[Zod]: https://github.com/colinhacks/zod
[zod-to-openapi]: https://github.com/asteasolutions/zod-to-openapi

- Runtime type validation
- Static TypeScript types
- OpenAPI documentation
- API client generation capabilities

## Getting Started

### Prerequisites

- Bun 1.0 or higher

### Installation

<<<<<<< Updated upstream
```bash
=======
<<<<<<< HEAD
``` bash
=======
```bash
>>>>>>> chore/bisect
>>>>>>> Stashed changes
cd bitbuilder.io/bbctl
bun install
```

### Generating OpenAPI Documentation

To generate the OpenAPI schema and documentation:

<<<<<<< Updated upstream
```bash
=======
<<<<<<< HEAD
``` bash
=======
```bash
>>>>>>> chore/bisect
>>>>>>> Stashed changes
bun run generate-openapi
```

This creates: - `api-docs/openapi.json` - The complete OpenAPI 3.1 specification - `api-docs/index.html` - A Swagger UI page for exploring the API

Open `api-docs/index.html` in your browser to view the interactive API documentation.

## Schema Usage

### Validating Data

You can use the Zod schemas to validate data at runtime:

<<<<<<< Updated upstream
```typescript
=======
<<<<<<< HEAD
``` typescript
=======
```typescript
>>>>>>> chore/bisect
>>>>>>> Stashed changes
import { InstanceSchema } from './schema.js';

// Data from API or user input
const instanceData = {
  id: '550e8400-e29b-41d4-a716-446655440000',
  name: 'web-server-1',
  status: 'Running',
  provider: 'VyOS',
  // ...
};

// Validate the data
try {
  const validatedInstance = InstanceSchema.parse(instanceData);
  console.log('Valid instance:', validatedInstance);
} catch (error) {
  console.error('Invalid instance data:', error);
}
```

### Type Safety

The schemas also provide TypeScript types:

<<<<<<< Updated upstream
```typescript
=======
<<<<<<< HEAD
``` typescript
=======
```typescript
>>>>>>> chore/bisect
>>>>>>> Stashed changes
import { Instance, InstanceStatus } from './schema.js';

// Type-safe instance object
const instance: Instance = {
  id: '550e8400-e29b-41d4-a716-446655440000',
  name: 'web-server-1',
  status: InstanceStatus.Running,
  // ...
};
```

## Integration with Rust CLI

The Zod/OpenAPI schema and the Rust CLI share the same data models. When updating models:

1. Modify both the Rust structs (`src/models/*.rs`) and the TypeScript schemas (`schema.ts`)
2. Regenerate the OpenAPI documentation
3. Update any dependent code in both languages

## API Endpoints

The OpenAPI documentation details all available endpoints:

- `/providers` - Manage infrastructure providers
- `/instances` - Create and manage virtual machines
- `/volumes` - Manage storage volumes
- `/networks` - Configure virtual networks

For detailed parameters and response formats, refer to the Swagger UI documentation.

## Extending the API

To extend the API schema:

1. Add new Zod schemas in `schema.ts`
2. Register your schemas with the OpenAPI registry
3. Define new paths and operations in the OpenAPI schema
4. Regenerate the documentation

## Testing with the API

The OpenAPI documentation can be used to generate clients in various languages using tools like:

- [OpenAPI Generator]
- [Swagger Codegen]

[OpenAPI Generator]: https://github.com/OpenAPITools/openapi-generator
[Swagger Codegen]: https://github.com/swagger-api/swagger-codegen

For example, to generate a TypeScript client:

<<<<<<< Updated upstream
```bash
=======
<<<<<<< HEAD
``` bash
=======
```bash
>>>>>>> chore/bisect
>>>>>>> Stashed changes
bunx --bun @openapitools/openapi-generator-cli generate \
  -i api-docs/openapi.json \
  -g typescript-axios \
  -o ./generated-client
```

## License

MIT License - See project LICENSE file for details.
