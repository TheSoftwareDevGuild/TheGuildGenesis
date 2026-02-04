import type { Project } from "../types/projects";

export const PROJECTS: Project[] = [
  {
    id: "proj_001",
    name: "DeFi Yield Aggregator",
    description: "A comprehensive DeFi platform that aggregates yield farming opportunities across multiple protocols. Users can compare APYs, manage their positions, and automatically compound their rewards.",
    ownerAddress: "0x1234...5678",
    createdAt: "2026-01-15T10:30:00Z",
    updatedAt: "2026-01-20T14:45:00Z",
  },
  {
    id: "proj_002",
    name: "NFT Marketplace",
    description: "Decentralized NFT marketplace with low fees and cross-chain support. Features include lazy minting, batch transfers, and royalty management.",
    ownerAddress: "0x9876...5432",
    createdAt: "2026-01-18T09:15:00Z",
    updatedAt: "2026-01-25T16:20:00Z",
  },
  {
    id: "proj_003",
    name: "Web3 Social Network",
    description: "Privacy-focused social platform built on blockchain. Users own their data, earn tokens for content creation, and have encrypted messaging.",
    ownerAddress: "0x1234...5678",
    createdAt: "2026-01-20T11:00:00Z",
    updatedAt: "2026-01-28T10:30:00Z",
  },
  {
    id: "proj_004",
    name: "DAO Governance Tool",
    description: "Streamlined governance platform for DAOs with proposal creation, voting mechanisms, treasury management, and delegation features.",
    ownerAddress: "0x5555...7777",
    createdAt: "2026-01-22T14:20:00Z",
    updatedAt: "2026-01-29T09:15:00Z",
  },
  {
    id: "proj_005",
    name: "Cross-Chain Bridge",
    description: "Secure bridge for transferring assets between different blockchains. Supports EVM chains and includes liquidity pools for fast transfers.",
    ownerAddress: "0x9876...5432",
    createdAt: "2026-01-25T08:45:00Z",
    updatedAt: "2026-02-01T11:00:00Z",
  },
  {
    id: "proj_006",
    name: "Blockchain Analytics Dashboard",
    description: "Real-time analytics platform for tracking on-chain metrics, wallet activities, and DeFi protocol statistics with custom alerts.",
    ownerAddress: "0x1234...5678",
    createdAt: "2026-01-28T13:30:00Z",
    updatedAt: "2026-02-02T15:45:00Z",
  },
  {
    id: "proj_007",
    name: "Smart Contract Auditing Tool",
    description: "Automated security analysis tool for smart contracts. Detects common vulnerabilities and provides detailed security reports.",
    ownerAddress: "0xabcd...ef01",
    createdAt: "2026-01-30T10:00:00Z",
    updatedAt: "2026-02-03T12:20:00Z",
  },
  {
    id: "proj_008",
    name: "Decentralized Identity System",
    description: "Self-sovereign identity solution allowing users to control their personal data and credentials on the blockchain.",
    ownerAddress: "0x5555...7777",
    createdAt: "2026-02-01T09:30:00Z",
    updatedAt: "2026-02-03T14:15:00Z",
  },
  {
    id: "proj_009",
    name: "GameFi Platform",
    description: "Play-to-earn gaming platform with tokenized in-game assets, staking rewards, and competitive tournaments.",
    ownerAddress: "0x9876...5432",
    createdAt: "2026-02-02T11:45:00Z",
    updatedAt: "2026-02-04T10:30:00Z",
  },
];

// Get projects by owner's address //
export function getProjectsByOwner(ownerAddress: string): Project[] {
  return PROJECTS.filter(
    (project) =>
      project.ownerAddress.toLowerCase() === ownerAddress.toLowerCase()
  );
}

// get project by ID //
export function getProjectById(id: string): Project | undefined {
  return PROJECTS.find((project) => project.id === id);
}

// get project by name and description //
export function searchProjects(query: string): Project[] {
  const lowerQuery = query.toLowerCase();
  return PROJECTS.filter(
    (project) =>
      project.name.toLowerCase().includes(lowerQuery) ||
      project.description.toLowerCase().includes(lowerQuery)
  );
}