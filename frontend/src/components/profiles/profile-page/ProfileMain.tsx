import ProfileHeader from "@/components/profiles/profile-page/ProfileHeader";
import ProfileActions from "@/components/profiles/profile-page/ProfileActions";
import ProfileAttestations from "@/components/profiles/profile-page/ProfileAttestations";
import ProfileIssuedAttestations from "@/components/profiles/profile-page/ProfileIssuedAttestations";
import { useGetProfiles } from "@/hooks/profiles/use-get-profiles";
import { useMemo } from "react";

export function ProfileMain({ address }: { address: string }) {
  const profilesQuery = useGetProfiles();

  const profile = useMemo(() => {
    const list = profilesQuery.data ?? [];
    const p = list.find(
      (x) => x.address.toLowerCase() === address.toLowerCase()
    );
    return p;
  }, [profilesQuery.data, address]);

  return (
    <div className="max-w-4xl mx-auto p-6">
      <ProfileHeader
        address={address}
        name={profile?.name}
        description={profile?.description}
        githubLogin={profile?.github_login}
      />
      <div className="mt-6">
        <ProfileActions
          address={address}
          name={profile?.name}
          description={profile?.description}
          githubLogin={profile?.github_login}
        />
      </div>
      <ProfileAttestations address={address} />
      <ProfileIssuedAttestations address={address} />
    </div>
  );
}
