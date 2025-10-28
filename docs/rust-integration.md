# Integrating Rust and TypeScript in the bbctl Project

This guide explains how to maintain consistency between the Rust backend and TypeScript API schema in the BitBuilder Cloud CLI project.

## Overview

The bbctl project consists of:

1. A Rust CLI application (`/src/*.rs`) that implements the core functionality
2. TypeScript schemas (`schema.ts`) using ES modules for API validation and documentation

Both codebases need to share consistent data models. This document outlines the approach for maintaining this consistency.

## Data Model Mapping

### Rust to TypeScript Type Mapping

| Rust Type | TypeScript/Zod Type |
|-----------|---------------------|
| `String` | `z.string()` |
| `i32`, `u32`, etc. | `z.number().int()` |
| `f32`, `f64` | `z.number()` |
| `bool` | `z.boolean()` |
| `Option<T>` | `z.optional()` |
| `Vec<T>` | `z.array(...)` |
| `HashMap<K, V>` | `z.record(...)` |
| `enum` | `z.enum()` or `z.discriminatedUnion()` |
| `struct` | `z.object()` |
| `Uuid` | `z.string().uuid()` |
| `DateTime<Utc>` | `z.string().datetime()` |

## Development Workflow

### When Modifying Rust Models

1. Update the Rust model in `src/models/*.rs`
2. Update the corresponding TypeScript schema in `schema.ts`
3. Run `npm run generate-openapi` to update API documentation
4. If applicable, update any related API implementation code

### When Adding New API Endpoints

1. Design your API endpoint and data models in both languages
2. Implement the Rust functionality first
3. Add the TypeScript schema definitions
4. Add the endpoint to the OpenAPI paths in `schema.ts`
5. Generate updated documentation

## Code Generation Options

### Generating TypeScript from Rust

For automated type generation, you can use:

```bash
# Install typescript-from-rust
cargo install typeshare-cli

# Generate TypeScript interfaces
typeshare --lang=typescript ./src/models/ --output-file=./generated-types.ts
```

Then manually adapt the generated types to Zod schemas.

### Generating Rust from OpenAPI

You can also generate Rust code from the OpenAPI spec:

```bash
# Install openapi-generator with Bun
bun install @openapitools/openapi-generator-cli -g

# Generate Rust client
bunx openapi-generator-cli generate -i ./api-docs/openapi.json -g rust -o ./generated-rust-client
```

## Maintaining API Version Compatibility

1. Use semantic versioning for both the Rust crate and TypeScript package
2. Document breaking changes in CHANGELOG.md
3. Include version compatibility information in documentation
4. When possible, maintain backward compatibility

## Testing Cross-Language Integration

1. Generate TypeScript API clients from the OpenAPI schema
2. Create integration tests that verify the TypeScript client works with the Rust implementation
3. Test all CRUD operations for each resource type

## Example: Keeping Models in Sync

### Rust Model:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub id: Uuid,
    pub name: String,
    pub status: InstanceStatus,
    pub provider: ProviderType,
    pub provider_id: String,
    pub region: String,
    pub size: InstanceSize,
    pub networks: Vec<InstanceNetwork>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}
```

### TypeScript/Zod Schema:

```typescript
// ES module import syntax
import { z } from 'zod';
import { InstanceStatusEnum, ProviderTypeEnum, InstanceSizeSchema, InstanceNetworkSchema, TagsSchema } from './schemas.js';

export const InstanceSchema = z.object({
  id: z.string().uuid(),
  name: z.string().min(1).max(64),
  status: InstanceStatusEnum,
  provider: ProviderTypeEnum,
  providerId: z.string(),
  region: z.string(),
  size: InstanceSizeSchema,
  networks: z.array(InstanceNetworkSchema),
  createdAt: z.string().datetime(),
  updatedAt: z.string().datetime(),
  tags: TagsSchema,
});
```

## Future Improvements

1. Implement automated code generation tools to keep models in sync
2. Create a CI check that verifies model consistency
3. Develop a shared schema format that can be consumed by both languages
4. Leverage Bun's performance for faster API development and testing

## Common Issues and Troubleshooting

1. **Inconsistent naming conventions**: Rust uses snake_case, TypeScript uses camelCase
2. **Different validation rules**: Ensure limits (min/max values, string lengths) are consistent
3. **Enum handling differences**: Map string values consistently 
4. **Date/time format issues**: Use ISO 8601 format (RFC 3339) consistently
5. **UUID representation**: Use the same format (hyphenated, lowercase)
6. **ES module issues**: Remember to add `.js` extensions to imports in TypeScript
7. **Bun compatibility**: Ensure all dependencies are compatible with Bun's runtime