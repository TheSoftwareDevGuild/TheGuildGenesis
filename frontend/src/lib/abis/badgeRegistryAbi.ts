// ABI fragments for V1 contract (bytes32 description)
export const badgeRegistryAbiV1 = [
  {
    type: "function",
    name: "totalBadges",
    stateMutability: "view",
    inputs: [],
    outputs: [{ name: "", type: "uint256" }],
  },
  {
    type: "function",
    name: "getBadgeAt",
    stateMutability: "view",
    inputs: [{ name: "index", type: "uint256" }],
    outputs: [
      { name: "", type: "bytes32" },
      { name: "", type: "bytes32" },
      { name: "", type: "address" },
    ],
  },
  {
    type: "function",
    name: "createBadge",
    stateMutability: "nonpayable",
    inputs: [
      { name: "name", type: "bytes32" },
      { name: "description", type: "bytes32" },
    ],
    outputs: [],
  },
] as const;

// ABI fragments for V2 contract (bytes description)
export const badgeRegistryAbiV2 = [
  {
    type: "function",
    name: "totalBadges",
    stateMutability: "view",
    inputs: [],
    outputs: [{ name: "", type: "uint256" }],
  },
  {
    type: "function",
    name: "getBadgeAt",
    stateMutability: "view",
    inputs: [{ name: "index", type: "uint256" }],
    outputs: [
      { name: "", type: "bytes32" },
      { name: "", type: "bytes" },
      { name: "", type: "address" },
    ],
  },
  {
    type: "function",
    name: "createBadge",
    stateMutability: "nonpayable",
    inputs: [
      { name: "name", type: "bytes32" },
      { name: "description", type: "bytes" },
    ],
    outputs: [],
  },
] as const;
