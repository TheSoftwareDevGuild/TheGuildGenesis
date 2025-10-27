// ABI fragments for the required contract read methods
export const badgeRegistryAbi = [
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
      { name: "", type: "int256" },
    ],
  },
  {
    type: "function",
    name: "getBadge",
    stateMutability: "view",
    inputs: [{ name: "name", type: "bytes32" }],
    outputs: [
      { name: "", type: "bytes32" },
      { name: "", type: "bytes32" },
      { name: "", type: "address" },
      { name: "", type: "int256" },
    ],
  },
  {
    type: "function",
    name: "getPointers",
    stateMutability: "view",
    inputs: [{ name: "name", type: "bytes32" }],
    outputs: [{ name: "", type: "bytes32[]" }],
  },
  {
    type: "function",
    name: "getVoteScore",
    stateMutability: "view",
    inputs: [{ name: "name", type: "bytes32" }],
    outputs: [{ name: "", type: "int256" }],
  },
  {
    type: "function",
    name: "hasVoted",
    stateMutability: "view",
    inputs: [
      { name: "badgeName", type: "bytes32" },
      { name: "voter", type: "address" },
    ],
    outputs: [{ name: "", type: "bool" }],
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
  {
    type: "function",
    name: "addPointers",
    stateMutability: "nonpayable",
    inputs: [
      { name: "fromBadge", type: "bytes32" },
      { name: "toBadges", type: "bytes32[]" },
    ],
    outputs: [],
  },
  {
    type: "function",
    name: "vote",
    stateMutability: "nonpayable",
    inputs: [
      { name: "badgeName", type: "bytes32" },
      { name: "isUpvote", type: "bool" },
    ],
    outputs: [],
  },
] as const;
