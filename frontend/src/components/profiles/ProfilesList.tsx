import { ProfileCard } from "./ProfileCard";
import { Search } from "lucide-react";
import { Input } from "@/components/ui/input";
import { CreateProfileButton } from "./CreateProfileButton";
import { useGetProfiles } from "@/hooks/use-get-profiles";

type ProfileBadge = {
  id: string;
  name: string;
  description: string;
  issuer: string;
};

type Profile = {
  address: string;
  name?: string;
  description?: string;
  badgeCount: number;
  badges: ProfileBadge[];
};

const PROFILES: Profile[] = [
  {
    address: "0x1234...5678",
    name: "Alice Developer",
    description: "Full-stack developer passionate about Web3 and Rust",
    badgeCount: 5,
    badges: [
      {
        id: "1",
        name: "Rust",
        description: "Rust programming",
        issuer: "0xabcd...1234",
      },
      {
        id: "2",
        name: "React",
        description: "React development",
        issuer: "0xefgh...5678",
      },
      {
        id: "3",
        name: "Web3",
        description: "Blockchain development",
        issuer: "0xijkl...9012",
      },
    ],
  },
  {
    address: "0x9876...5432",
    name: "Bob Builder",
    description: "Smart contract developer and DeFi enthusiast",
    badgeCount: 3,
    badges: [
      {
        id: "4",
        name: "Solidity",
        description: "Smart contract development",
        issuer: "0xmnop...3456",
      },
      {
        id: "5",
        name: "DeFi",
        description: "Decentralized Finance",
        issuer: "0xqrst...7890",
      },
    ],
  },
  {
    address: "0x5555...7777",
    badgeCount: 2,
    badges: [
      {
        id: "6",
        name: "TypeScript",
        description: "TypeScript development",
        issuer: "0xuvwx...1234",
      },
    ],
  },
];

export function ProfilesList() {
  const { data, isLoading, error } = useGetProfiles();

  const profiles: Profile[] =
    data && data.length > 0
      ? data.map((p) => ({
          address: p.address,
          name: p.name,
          description: p.description,
          badgeCount: 0,
          badges: [],
        }))
      : PROFILES;

  return (
    <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div className="flex gap-4 items-center pb-8">
        <div className="relative">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 h-4 w-4" />
          <input
            type="text"
            placeholder="Search profiles..."
            className="pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          />
        </div>

        <CreateProfileButton />
      </div>

      {isLoading ? (
        <p className="text-sm text-gray-600">Loading profiles...</p>
      ) : null}
      {error ? (
        <p className="text-sm text-red-600">{(error as Error).message}</p>
      ) : null}

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {profiles.map((profile) => (
          <ProfileCard
            key={profile.address}
            address={profile.address}
            name={profile.name}
            description={profile.description}
            badgeCount={profile.badgeCount}
            badges={profile.badges}
          />
        ))}
      </div>
    </main>
  );
}

export default ProfilesList;
