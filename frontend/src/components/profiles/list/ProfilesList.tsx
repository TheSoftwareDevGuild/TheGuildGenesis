import { ProfileCard } from "./ProfileCard";
import { Search } from "lucide-react";
import { CreateProfileButton } from "../action-buttons/CreateProfileDialog";
import { useGetProfiles } from "@/hooks/profiles/use-get-profiles";
import { PROFILES } from "@/lib/constants/profileConstants";
import type { Profile } from "@/lib/types/profiles";
import { useMemo, useState } from "react";
import { useGetAttestations } from "@/hooks/attestations/use-get-attestations";
import { Input } from "../../ui/input";

export function ProfilesList() {
  const { data, isLoading, error } = useGetProfiles();
  const [searchQuery, setSearchQuery] = useState("");
  const attestations = useGetAttestations();

  const baseProfiles: Profile[] =
    data && data.length > 0
      ? data.map((p) => ({
        address: p.address,
        name: p.name || "",
        description: p.description || "",
        githubLogin: p.github_login,
        attestationCount: 0,
        attestations: [],
      }))
      : PROFILES;

  const attestationsByRecipient = useMemo(() => {
    const map = new Map<
      string,
      { id: string; badgeName: string; justification: string; issuer: string }[]
    >();
    const list = attestations.data ?? [];
    for (let i = 0; i < list.length; i++) {
      const a = list[i];
      const arr = map.get(a.recipient.toLowerCase()) ?? [];
      arr.push({
        id: String(i),
        badgeName: a.badgeName,
        justification: a.attestationJustification,
        issuer: a.issuer,
      });
      map.set(a.recipient.toLowerCase(), arr);
    }
    return map;
  }, [attestations.data]);

  const profiles: Profile[] = useMemo(() => {
    return baseProfiles.map((p) => {
      const list =
        attestationsByRecipient.get((p.address || "").toLowerCase()) ?? [];
      return {
        ...p,
        attestationCount: list.length,
        attestations: list,
      };
    });
  }, [baseProfiles, attestationsByRecipient]);

  const filtered = useMemo(() => {
    const q = searchQuery.trim().toLowerCase();
    if (!q) return profiles;
    return profiles.filter((p) => (p.name || "").toLowerCase().includes(q));
  }, [profiles, searchQuery]);

  return (
    <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div className="flex gap-4 items-center pb-8">
        <div className="relative">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 h-4 w-4" />
          <Input
            type="text"
            placeholder="Search profiles..."
            className="pl-10"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
        </div>
        <CreateProfileButton />
      </div>
      {isLoading || attestations.isLoading ? (
        <p className="text-sm text-gray-600">Loading profiles...</p>
      ) : null}
      {error || attestations.error ? (
        <p className="text-sm text-red-600">
          {(error as any)?.message ??
            (attestations.error as any)?.message ??
            "An error occurred"}
        </p>
      ) : null}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {filtered.map((profile) => (
          <ProfileCard
            key={profile.address}
            address={profile.address}
            name={profile.name}
            description={profile.description}
            githubLogin={profile.githubLogin}
            attestationCount={profile.attestationCount}
            attestations={profile.attestations}
          />
        ))}
      </div>
    </main>
  );
}

export default ProfilesList;