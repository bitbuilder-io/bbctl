import { z } from 'zod';
import { zodToOpenAPI } from '@asteasolutions/zod-to-openapi';
import { OpenAPIRegistry } from '@asteasolutions/zod-to-openapi';

// Initialize the OpenAPI registry
const registry = new OpenAPIRegistry();

// ==================================================================================
// Enum Schemas
// ==================================================================================

// Provider Types
export const ProviderTypeEnum = z.enum(['VyOS', 'Proxmox']);
export type ProviderType = z.infer<typeof ProviderTypeEnum>;

// Instance Status
export const InstanceStatusEnum = z.enum([
  'Running',
  'Stopped',
  'Failed',
  'Creating',
  'Restarting',
  'Deleting',
  'Unknown'
]);
export type InstanceStatus = z.infer<typeof InstanceStatusEnum>;

// Volume Status
export const VolumeStatusEnum = z.enum([
  'Available',
  'InUse',
  'Creating',
  'Deleting',
  'Error',
  'Unknown'
]);
export type VolumeStatus = z.infer<typeof VolumeStatusEnum>;

// Volume Type
export const VolumeTypeEnum = z.enum([
  'Standard',
  'SSD',
  'NVMe',
  'HDD',
  'Network'
]);
export type VolumeType = z.infer<typeof VolumeTypeEnum>;

// Network Status
export const NetworkStatusEnum = z.enum([
  'Available',
  'Creating',
  'Deleting',
  'Error',
  'Unknown'
]);
export type NetworkStatus = z.infer<typeof NetworkStatusEnum>;

// Network Type
export const NetworkTypeEnum = z.enum([
  'Bridged',
  'Routed',
  'Isolated',
  'VXLAN',
  'VPN'
]);
export type NetworkType = z.infer<typeof NetworkTypeEnum>;

// ==================================================================================
// Base Schemas
// ==================================================================================

// Resource Tags
export const TagsSchema = z.record(z.string());

// Resource Limits for a region
export const ResourceLimitsSchema = z.object({
  maxInstances: z.number().int().positive().optional(),
  maxVolumes: z.number().int().positive().optional(),
  maxNetworks: z.number().int().positive().optional(),
  maxCpuPerInstance: z.number().int().positive().optional(),
  maxMemoryPerInstance: z.number().int().positive().optional(),
  maxDiskPerInstance: z.number().int().positive().optional(),
});

// ==================================================================================
// Provider Schemas
// ==================================================================================

// Provider Configuration
export const ProviderConfigSchema = z.object({
  providerType: ProviderTypeEnum,
  name: z.string().min(1).max(64),
  host: z.string(),
  params: z.record(z.string()),
});
export type ProviderConfig = z.infer<typeof ProviderConfigSchema>;

// Region Schema
export const RegionSchema = z.object({
  id: z.string().min(1).max(16),
  name: z.string().min(1).max(64),
  provider: ProviderTypeEnum,
  location: z.string(),
  available: z.boolean(),
  limits: ResourceLimitsSchema,
});
export type Region = z.infer<typeof RegionSchema>;

// VyOS Credentials
export const VyOSCredentialsSchema = z.object({
  username: z.string(),
  password: z.string().optional(),
  keyPath: z.string().optional(),
  apiKey: z.string().optional(),
  sshPort: z.number().int().positive().optional(),
  apiPort: z.number().int().positive().optional(),
});
export type VyOSCredentials = z.infer<typeof VyOSCredentialsSchema>;

// Proxmox Token Auth
export const ProxmoxTokenAuthSchema = z.object({
  tokenId: z.string(),
  tokenSecret: z.string(),
});
export type ProxmoxTokenAuth = z.infer<typeof ProxmoxTokenAuthSchema>;

// Proxmox User/Pass Auth
export const ProxmoxUserPassAuthSchema = z.object({
  username: z.string(),
  password: z.string(),
  realm: z.string(),
});
export type ProxmoxUserPassAuth = z.infer<typeof ProxmoxUserPassAuthSchema>;

// Proxmox Credentials
export const ProxmoxCredentialsSchema = z.object({
  port: z.number().int().positive().optional(),
  useTokenAuth: z.boolean(),
  tokenAuth: ProxmoxTokenAuthSchema.optional(),
  userPassAuth: ProxmoxUserPassAuthSchema.optional(),
  verifySsl: z.boolean(),
});
export type ProxmoxCredentials = z.infer<typeof ProxmoxCredentialsSchema>;

// Provider Credentials
export const ProviderCredentialsSchema = z.discriminatedUnion('type', [
  z.object({ type: z.literal('VyOS'), credentials: VyOSCredentialsSchema }),
  z.object({ type: z.literal('Proxmox'), credentials: ProxmoxCredentialsSchema }),
]);
export type ProviderCredentials = z.infer<typeof ProviderCredentialsSchema>;

// ==================================================================================
// Instance Schemas
// ==================================================================================

// Instance Size
export const InstanceSizeSchema = z.object({
  cpu: z.number().int().positive().min(1),
  memoryGb: z.number().int().positive().min(1),
  diskGb: z.number().int().positive().min(1),
});
export type InstanceSize = z.infer<typeof InstanceSizeSchema>;

// Instance Network
export const InstanceNetworkSchema = z.object({
  networkId: z.string().uuid(),
  ip: z.string().ip().optional(),
  interface: z.string().optional(),
  mac: z.string().optional(),
});
export type InstanceNetwork = z.infer<typeof InstanceNetworkSchema>;

// Instance
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
export type Instance = z.infer<typeof InstanceSchema>;

// Instance Creation Request
export const CreateInstanceRequestSchema = z.object({
  name: z.string().min(1).max(64),
  provider: ProviderTypeEnum,
  region: z.string(),
  size: InstanceSizeSchema,
  networkId: z.string().uuid().optional(),
  tags: TagsSchema.optional(),
});
export type CreateInstanceRequest = z.infer<typeof CreateInstanceRequestSchema>;

// ==================================================================================
// Volume Schemas
// ==================================================================================

// Volume
export const VolumeSchema = z.object({
  id: z.string().uuid(),
  name: z.string().min(1).max(64),
  status: VolumeStatusEnum,
  provider: ProviderTypeEnum,
  providerId: z.string(),
  region: z.string(),
  sizeGb: z.number().int().positive(),
  volumeType: VolumeTypeEnum,
  attachedTo: z.string().uuid().optional(),
  device: z.string().optional(),
  createdAt: z.string().datetime(),
  updatedAt: z.string().datetime(),
  tags: TagsSchema,
});
export type Volume = z.infer<typeof VolumeSchema>;

// Volume Creation Request
export const CreateVolumeRequestSchema = z.object({
  name: z.string().min(1).max(64),
  provider: ProviderTypeEnum,
  region: z.string(),
  sizeGb: z.number().int().positive(),
  volumeType: VolumeTypeEnum,
  tags: TagsSchema.optional(),
});
export type CreateVolumeRequest = z.infer<typeof CreateVolumeRequestSchema>;

// Volume Attachment Request
export const AttachVolumeRequestSchema = z.object({
  volumeId: z.string().uuid(),
  instanceId: z.string().uuid(),
  device: z.string().optional(),
});
export type AttachVolumeRequest = z.infer<typeof AttachVolumeRequestSchema>;

// ==================================================================================
// Network Schemas
// ==================================================================================

// IP Allocation
export const IpAllocationSchema = z.object({
  ip: z.string().ip(),
  instanceId: z.string().uuid().optional(),
  assignedAt: z.string().datetime().optional(),
});
export type IpAllocation = z.infer<typeof IpAllocationSchema>;

// Network
export const NetworkSchema = z.object({
  id: z.string().uuid(),
  name: z.string().min(1).max(64),
  status: NetworkStatusEnum,
  provider: ProviderTypeEnum,
  providerId: z.string(),
  region: z.string(),
  cidr: z.string().regex(/^([0-9]{1,3}\.){3}[0-9]{1,3}\/[0-9]{1,2}$/),
  networkType: NetworkTypeEnum,
  gateway: z.string().ip().optional(),
  dnsServers: z.array(z.string().ip()),
  instances: z.array(z.string().uuid()),
  ipAllocations: z.array(IpAllocationSchema),
  createdAt: z.string().datetime(),
  updatedAt: z.string().datetime(),
  tags: TagsSchema,
  config: z.record(z.string()),
});
export type Network = z.infer<typeof NetworkSchema>;

// Network Creation Request
export const CreateNetworkRequestSchema = z.object({
  name: z.string().min(1).max(64),
  provider: ProviderTypeEnum,
  region: z.string(),
  cidr: z.string().regex(/^([0-9]{1,3}\.){3}[0-9]{1,3}\/[0-9]{1,2}$/),
  networkType: NetworkTypeEnum,
  gateway: z.string().ip().optional(),
  dnsServers: z.array(z.string().ip()).optional(),
  tags: TagsSchema.optional(),
  config: z.record(z.string()).optional(),
});
export type CreateNetworkRequest = z.infer<typeof CreateNetworkRequestSchema>;

// Network Connection Request
export const ConnectNetworkRequestSchema = z.object({
  networkId: z.string().uuid(),
  instanceId: z.string().uuid(),
  ip: z.string().ip().optional(),
});
export type ConnectNetworkRequest = z.infer<typeof ConnectNetworkRequestSchema>;

// ==================================================================================
// WireGuard Schemas
// ==================================================================================

// WireGuard Peer
export const WireGuardPeerSchema = z.object({
  publicKey: z.string(),
  endpoint: z.string(),
  allowedIps: z.array(z.string()),
  persistentKeepalive: z.number().int(),
});
export type WireGuardPeer = z.infer<typeof WireGuardPeerSchema>;

// WireGuard Config
export const WireGuardConfigSchema = z.object({
  privateKey: z.string(),
  address: z.string(),
  port: z.number().int().positive(),
  peers: z.array(WireGuardPeerSchema),
});
export type WireGuardConfig = z.infer<typeof WireGuardConfigSchema>;

// ==================================================================================
// Register schemas with OpenAPI registry
// ==================================================================================

registry.register('ProviderType', ProviderTypeEnum);
registry.register('InstanceStatus', InstanceStatusEnum);
registry.register('VolumeStatus', VolumeStatusEnum);
registry.register('VolumeType', VolumeTypeEnum);
registry.register('NetworkStatus', NetworkStatusEnum);
registry.register('NetworkType', NetworkTypeEnum);

registry.register('ResourceLimits', ResourceLimitsSchema);
registry.register('ProviderConfig', ProviderConfigSchema);
registry.register('Region', RegionSchema);
registry.register('VyOSCredentials', VyOSCredentialsSchema);
registry.register('ProxmoxTokenAuth', ProxmoxTokenAuthSchema);
registry.register('ProxmoxUserPassAuth', ProxmoxUserPassAuthSchema);
registry.register('ProxmoxCredentials', ProxmoxCredentialsSchema);
registry.register('ProviderCredentials', ProviderCredentialsSchema);

registry.register('InstanceSize', InstanceSizeSchema);
registry.register('InstanceNetwork', InstanceNetworkSchema);
registry.register('Instance', InstanceSchema);
registry.register('CreateInstanceRequest', CreateInstanceRequestSchema);

registry.register('Volume', VolumeSchema);
registry.register('CreateVolumeRequest', CreateVolumeRequestSchema);
registry.register('AttachVolumeRequest', AttachVolumeRequestSchema);

registry.register('IpAllocation', IpAllocationSchema);
registry.register('Network', NetworkSchema);
registry.register('CreateNetworkRequest', CreateNetworkRequestSchema);
registry.register('ConnectNetworkRequest', ConnectNetworkRequestSchema);

registry.register('WireGuardPeer', WireGuardPeerSchema);
registry.register('WireGuardConfig', WireGuardConfigSchema);

// ==================================================================================
// OpenAPI schema generation
// ==================================================================================

// Generate OpenAPI schema from registry
export const openApiSchema = {
  openapi: '3.1.0',
  info: {
    title: 'BitBuilder Cloud CLI API',
    version: '1.0.0',
    description: 'API for BitBuilder Cloud CLI (bbctl)',
    contact: {
      name: 'BitBuilder.io',
      url: 'https://bitbuilder.io',
    },
    license: {
      name: 'MIT',
      url: 'https://opensource.org/licenses/MIT',
    },
  },
  servers: [
    {
      url: 'https://api.bitbuilder.io/v1',
      description: 'Production API server',
    },
    {
      url: 'http://localhost:8080/v1',
      description: 'Local development server',
    },
  ],
  paths: {
    '/providers': {
      get: {
        summary: 'List all providers',
        operationId: 'listProviders',
        responses: {
          '200': {
            description: 'List of providers',
            content: {
              'application/json': {
                schema: {
                  type: 'array',
                  items: {
                    $ref: '#/components/schemas/ProviderConfig',
                  },
                },
              },
            },
          },
        },
      },
      post: {
        summary: 'Add a new provider',
        operationId: 'createProvider',
        requestBody: {
          content: {
            'application/json': {
              schema: {
                $ref: '#/components/schemas/ProviderConfig',
              },
            },
          },
          required: true,
        },
        responses: {
          '201': {
            description: 'Provider created',
            content: {
              'application/json': {
                schema: {
                  $ref: '#/components/schemas/ProviderConfig',
                },
              },
            },
          },
        },
      },
    },
    '/instances': {
      get: {
        summary: 'List all instances',
        operationId: 'listInstances',
        responses: {
          '200': {
            description: 'List of instances',
            content: {
              'application/json': {
                schema: {
                  type: 'array',
                  items: {
                    $ref: '#/components/schemas/Instance',
                  },
                },
              },
            },
          },
        },
      },
      post: {
        summary: 'Create a new instance',
        operationId: 'createInstance',
        requestBody: {
          content: {
            'application/json': {
              schema: {
                $ref: '#/components/schemas/CreateInstanceRequest',
              },
            },
          },
          required: true,
        },
        responses: {
          '201': {
            description: 'Instance created',
            content: {
              'application/json': {
                schema: {
                  $ref: '#/components/schemas/Instance',
                },
              },
            },
          },
        },
      },
    },
    '/instances/{instanceId}': {
      get: {
        summary: 'Get an instance by ID',
        operationId: 'getInstance',
        parameters: [
          {
            name: 'instanceId',
            in: 'path',
            required: true,
            schema: {
              type: 'string',
              format: 'uuid',
            },
          },
        ],
        responses: {
          '200': {
            description: 'Instance details',
            content: {
              'application/json': {
                schema: {
                  $ref: '#/components/schemas/Instance',
                },
              },
            },
          },
          '404': {
            description: 'Instance not found',
          },
        },
      },
      delete: {
        summary: 'Delete an instance',
        operationId: 'deleteInstance',
        parameters: [
          {
            name: 'instanceId',
            in: 'path',
            required: true,
            schema: {
              type: 'string',
              format: 'uuid',
            },
          },
        ],
        responses: {
          '204': {
            description: 'Instance deleted',
          },
          '404': {
            description: 'Instance not found',
          },
        },
      },
    },
    '/instances/{instanceId}/start': {
      post: {
        summary: 'Start an instance',
        operationId: 'startInstance',
        parameters: [
          {
            name: 'instanceId',
            in: 'path',
            required: true,
            schema: {
              type: 'string',
              format: 'uuid',
            },
          },
        ],
        responses: {
          '200': {
            description: 'Instance started',
            content: {
              'application/json': {
                schema: {
                  $ref: '#/components/schemas/Instance',
                },
              },
            },
          },
          '404': {
            description: 'Instance not found',
          },
        },
      },
    },
    '/instances/{instanceId}/stop': {
      post: {
        summary: 'Stop an instance',
        operationId: 'stopInstance',
        parameters: [
          {
            name: 'instanceId',
            in: 'path',
            required: true,
            schema: {
              type: 'string',
              format: 'uuid',
            },
          },
        ],
        responses: {
          '200': {
            description: 'Instance stopped',
            content: {
              'application/json': {
                schema: {
                  $ref: '#/components/schemas/Instance',
                },
              },
            },
          },
          '404': {
            description: 'Instance not found',
          },
        },
      },
    },
    '/volumes': {
      get: {
        summary: 'List all volumes',
        operationId: 'listVolumes',
        responses: {
          '200': {
            description: 'List of volumes',
            content: {
              'application/json': {
                schema: {
                  type: 'array',
                  items: {
                    $ref: '#/components/schemas/Volume',
                  },
                },
              },
            },
          },
        },
      },
      post: {
        summary: 'Create a new volume',
        operationId: 'createVolume',
        requestBody: {
          content: {
            'application/json': {
              schema: {
                $ref: '#/components/schemas/CreateVolumeRequest',
              },
            },
          },
          required: true,
        },
        responses: {
          '201': {
            description: 'Volume created',
            content: {
              'application/json': {
                schema: {
                  $ref: '#/components/schemas/Volume',
                },
              },
            },
          },
        },
      },
    },
    '/volumes/{volumeId}': {
      get: {
        summary: 'Get a volume by ID',
        operationId: 'getVolume',
        parameters: [
          {
            name: 'volumeId',
            in: 'path',
            required: true,
            schema: {
              type: 'string',
              format: 'uuid',
            },
          },
        ],
        responses: {
          '200': {
            description: 'Volume details',
            content: {
              'application/json': {
                schema: {
                  $ref: '#/components/schemas/Volume',
                },
              },
            },
          },
          '404': {
            description: 'Volume not found',
          },
        },
      },
      delete: {
        summary: 'Delete a volume',
        operationId: 'deleteVolume',
        parameters: [
          {
            name: 'volumeId',
            in: 'path',
            required: true,
            schema: {
              type: 'string',
              format: 'uuid',
            },
          },
        ],
        responses: {
          '204': {
            description: 'Volume deleted',
          },
          '404': {
            description: 'Volume not found',
          },
        },
      },
    },
    '/volumes/attach': {
      post: {
        summary: 'Attach a volume to an instance',
        operationId: 'attachVolume',
        requestBody: {
          content: {
            'application/json': {
              schema: {
                $ref: '#/components/schemas/AttachVolumeRequest',
              },
            },
          },
          required: true,
        },
        responses: {
          '200': {
            description: 'Volume attached',
            content: {
              'application/json': {
                schema: {
                  $ref: '#/components/schemas/Volume',
                },
              },
            },
          },
          '404': {
            description: 'Volume or instance not found',
          },
        },
      },
    },
    '/volumes/{volumeId}/detach': {
      post: {
        summary: 'Detach a volume from an instance',
        operationId: 'detachVolume',
        parameters: [
          {
            name: 'volumeId',
            in: 'path',
            required: true,
            schema: {
              type: 'string',
              format: 'uuid',
            },
          },
        ],
        responses: {
          '200': {
            description: 'Volume detached',
            content: {
              'application/json': {
                schema: {
                  $ref: '#/components/schemas/Volume',
                },
              },
            },
          },
          '404': {
            description: 'Volume not found',
          },
        },
      },
    },
    '/networks': {
      get: {
        summary: 'List all networks',
        operationId: 'listNetworks',
        responses: {
          '200': {
            description: 'List of networks',
            content: {
              'application/json': {
                schema: {
                  type: 'array',
                  items: {
                    $ref: '#/components/schemas/Network',
                  },
                },
              },
            },
          },
        },
      },
      post: {
        summary: 'Create a new network',
        operationId: 'createNetwork',
        requestBody: {
          content: {
            'application/json': {
              schema: {
                $ref: '#/components/schemas/CreateNetworkRequest',
              },
            },
          },
          required: true,
        },
        responses: {
          '201': {
            description: 'Network created',
            content: {
              'application/json': {
                schema: {
                  $ref: '#/components/schemas/Network',
                },
              },
            },
          },
        },
      },
    },
    '/networks/{networkId}': {
      get: {
        summary: 'Get a network by ID',
        operationId: 'getNetwork',
        parameters: [
          {
            name: 'networkId',
            in: 'path',
            required: true,
            schema: {
              type: 'string',
              format: 'uuid',
            },
          },
        ],
        responses: {
          '200': {
            description: 'Network details',
            content: {
              'application/json': {
                schema: {
                  $ref: '#/components/schemas/Network',
                },
              },
            },
          },
          '404': {
            description: 'Network not found',
          },
        },
      },
      delete: {
        summary: 'Delete a network',
        operationId: 'deleteNetwork',
        parameters: [
          {
            name: 'networkId',
            in: 'path',
            required: true,
            schema: {
              type: 'string',
              format: 'uuid',
            },
          },
        ],
        responses: {
          '204': {
            description: 'Network deleted',
          },
          '404': {
            description: 'Network not found',
          },
        },
      },
    },
    '/networks/connect': {
      post: {
        summary: 'Connect an instance to a network',
        operationId: 'connectNetwork',
        requestBody: {
          content: {
            'application/json': {
              schema: {
                $ref: '#/components/schemas/ConnectNetworkRequest',
              },
            },
          },
          required: true,
        },
        responses: {
          '200': {
            description: 'Instance connected to network',
            content: {
              'application/json': {
                schema: {
                  $ref: '#/components/schemas/Network',
                },
              },
            },
          },
          '404': {
            description: 'Network or instance not found',
          },
        },
      },
    },
    '/networks/{networkId}/disconnect/{instanceId}': {
      post: {
        summary: 'Disconnect an instance from a network',
        operationId: 'disconnectNetwork',
        parameters: [
          {
            name: 'networkId',
            in: 'path',
            required: true,
            schema: {
              type: 'string',
              format: 'uuid',
            },
          },
          {
            name: 'instanceId',
            in: 'path',
            required: true,
            schema: {
              type: 'string',
              format: 'uuid',
            },
          },
        ],
        responses: {
          '200': {
            description: 'Instance disconnected from network',
            content: {
              'application/json': {
                schema: {
                  $ref: '#/components/schemas/Network',
                },
              },
            },
          },
          '404': {
            description: 'Network or instance not found',
          },
        },
      },
    },
  },
  components: {
    schemas: zodToOpenAPI(registry.definitions),
  },
};

// Named exports for individual schemas
export {
  ProviderTypeEnum,
  InstanceStatusEnum,
  VolumeStatusEnum,
  VolumeTypeEnum,
  NetworkStatusEnum,
  NetworkTypeEnum,
  ResourceLimitsSchema,
  ProviderConfigSchema,
  RegionSchema,
  VyOSCredentialsSchema,
  ProxmoxTokenAuthSchema,
  ProxmoxUserPassAuthSchema,
  ProxmoxCredentialsSchema,
  ProviderCredentialsSchema,
  InstanceSizeSchema,
  InstanceNetworkSchema,
  InstanceSchema,
  CreateInstanceRequestSchema,
  VolumeSchema,
  CreateVolumeRequestSchema,
  AttachVolumeRequestSchema,
  IpAllocationSchema,
  NetworkSchema,
  CreateNetworkRequestSchema,
  ConnectNetworkRequestSchema,
  WireGuardPeerSchema,
  WireGuardConfigSchema,
  openApiSchema,
};

// For backward compatibility
export default {
  schemas: {
    ProviderTypeEnum,
    InstanceStatusEnum,
    VolumeStatusEnum,
    VolumeTypeEnum,
    NetworkStatusEnum,
    NetworkTypeEnum,
    ResourceLimitsSchema,
    ProviderConfigSchema,
    RegionSchema,
    VyOSCredentialsSchema,
    ProxmoxTokenAuthSchema,
    ProxmoxUserPassAuthSchema,
    ProxmoxCredentialsSchema,
    ProviderCredentialsSchema,
    InstanceSizeSchema,
    InstanceNetworkSchema,
    InstanceSchema,
    CreateInstanceRequestSchema,
    VolumeSchema,
    CreateVolumeRequestSchema,
    AttachVolumeRequestSchema,
    IpAllocationSchema,
    NetworkSchema,
    CreateNetworkRequestSchema,
    ConnectNetworkRequestSchema,
    WireGuardPeerSchema,
    WireGuardConfigSchema,
  },
  openApiSchema,
};