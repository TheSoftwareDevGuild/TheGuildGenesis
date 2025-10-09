import { AppWrapper } from "@/components/AppWrapper";
import { ProfileMain } from "@/components/profiles/profile-page/ProfileMain";

type Props = { address?: string };

export default function ProfilePage({ address }: Props) {
  return (
    <AppWrapper>
      <ProfileMain address={address || ""} />
    </AppWrapper>
  );
}
