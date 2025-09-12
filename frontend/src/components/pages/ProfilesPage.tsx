import { AppWrapper } from "@/components/AppWrapper";
import ProfilesList from "@/components/profiles/ProfilesList";

export function ProfilesPage() {
  return (
    <AppWrapper>
      <section className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-10">
        <h1 className="mb-6 text-3xl font-bold tracking-tight text-gray-900">
          Profiles
        </h1>
        <p className="mb-8 max-w-2xl text-gray-600">
          Discover and certify fellow developers in our peer-to-peer network
        </p>
        <ProfilesList />
      </section>
    </AppWrapper>
  );
}

export default ProfilesPage;
