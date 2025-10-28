import { InstanceSchema, InstanceStatusEnum, ProviderTypeEnum } from '../schema.js';
import type { Instance } from '../schema.js';

console.log('BitBuilder Cloud CLI - Instance Validation Example');
console.log('------------------------------------------------');

// Sample valid instance data
const validInstance = {
  id: '550e8400-e29b-41d4-a716-446655440000',
  name: 'web-server-1',
  status: InstanceStatusEnum.enum.Running,
  provider: ProviderTypeEnum.enum.VyOS,
  providerId: 'vyos-1',
  region: 'nyc',
  size: {
    cpu: 2,
    memoryGb: 4,
    diskGb: 80
  },
  networks: [
    {
      networkId: '6ba7b810-9dad-11d1-80b4-00c04fd430c8',
      ip: '192.168.1.100',
      interface: 'eth0',
      mac: '00:0a:95:9d:68:16'
    }
  ],
  createdAt: new Date().toISOString(),
  updatedAt: new Date().toISOString(),
  tags: {
    environment: 'production',
    application: 'web-api',
    owner: 'devops'
  }
};

// Invalid instance data (missing required fields)
const invalidInstance = {
  id: '550e8400-e29b-41d4-a716-446655440000',
  name: 'web-server-1',
  // Missing status
  provider: 'Unknown', // Invalid provider
  providerId: 'vyos-1',
  region: 'nyc',
  // Missing size
  networks: [],
  createdAt: new Date().toISOString(),
  updatedAt: new Date().toISOString(),
  tags: {
    environment: 'production'
  }
};

// Function to validate instance data
function validateInstance(data: unknown): Instance | null {
  try {
    // Parse and validate the data
    const validatedInstance = InstanceSchema.parse(data);
    console.log('✅ Instance validation successful!');
    return validatedInstance;
  } catch (error) {
    console.error('❌ Instance validation failed:');
    if (error instanceof Error) {
      console.error(error.message);
    }
    return null;
  }
}

// Safe type checking function
function isInstance(data: unknown): data is Instance {
  try {
    InstanceSchema.parse(data);
    return true;
  } catch {
    return false;
  }
}

// Example usage
console.log('\n--- Testing valid instance ---');
const instance = validateInstance(validInstance);
if (instance) {
  console.log(`Instance name: ${instance.name}`);
  console.log(`Provider: ${instance.provider}`);
  console.log(`Status: ${instance.status}`);
  console.log(`CPUs: ${instance.size.cpu}`);
  console.log(`RAM: ${instance.size.memoryGb} GB`);
  console.log(`Disk: ${instance.size.diskGb} GB`);
  
  // Safe access to optional fields
  const primaryIp = instance.networks[0]?.ip || 'No IP assigned';
  console.log(`Primary IP: ${primaryIp}`);
}

console.log('\n--- Testing invalid instance ---');
const badInstance = validateInstance(invalidInstance);
// This will show validation errors

console.log('\n--- Type guard example ---');
const someData = { ...validInstance, extraField: 'should be ignored' };
if (isInstance(someData)) {
  console.log('Data is a valid instance');
  // TypeScript knows 'someData' is of type Instance here
  console.log(`Region: ${someData.region}`);
} else {
  console.log('Data is not a valid instance');
}

// Run this example with: bun run examples/validate-instance.ts