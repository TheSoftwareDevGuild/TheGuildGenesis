import { AppWrapper } from "@/components/AppWrapper";
import ProfileHeader from "@/components/profiles/profile-page/ProfileHeader";
import ProfileActions from "@/components/profiles/profile-page/ProfileActions";
import ProfileAttestations from "@/components/profiles/profile-page/ProfileAttestations";
import ProfileDescription from "@/components/profiles/profile-page/ProfileDescription";
import { useMemo } from "react";
import { useGetProfiles } from "@/hooks/profiles/use-get-profiles";
import ProfileIssuedAttestations from "../profiles/profile-page/ProfileIssuedAttestations";

type Props = { address?: string };

export default function ProfilePage({ address }: Props) {
  const profilesQuery = useGetProfiles();
  const profile = useMemo(() => {
    const list = profilesQuery.data ?? [];
    return list.find(
      (x) => x.address.toLowerCase() === (address || "").toLowerCase()
    );
  }, [profilesQuery.data, address]);

  return (
    <AppWrapper>
      <section className="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 py-10">
        <ProfileHeader address={address || ""} />

        <ProfileActions address={address || ""} name={profile?.name} description={profile?.description} />

        <ProfileDescription description={profile?.description} />

        <ProfileAttestations address={address || ""} />

        <ProfileIssuedAttestations address={address || ""} />
      </section>
    </AppWrapper>
  );
}
