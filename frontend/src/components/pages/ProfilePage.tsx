import { AppWrapper } from "@/components/AppWrapper";
import ProfileHeader from "@/components/profiles/profile-page/ProfileHeader";
import ProfileActions from "@/components/profiles/profile-page/ProfileActions";
import ProfileAttestations from "@/components/profiles/profile-page/ProfileAttestations";
import ProfileIssuedAttestations from "../profiles/profile-page/ProfileIssuedAttestations";

type Props = { address?: string };

export default function ProfilePage({ address }: Props) {
  return (
    <AppWrapper>
      <section className="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 py-10">
        <ProfileHeader address={address || ""} />

        <ProfileActions address={address || ""} />

        <ProfileAttestations address={address || ""} />

        <ProfileIssuedAttestations address={address || ""} />
      </section>
    </AppWrapper>
  );
}
