import { AppWrapper } from "@/components/AppWrapper";
import BadgesList from "@/components/badges/BadgesList";

export function BadgesPage() {
  return (
    <AppWrapper>
      <section className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-10">
        <h1 className="mb-6 text-3xl font-bold tracking-tight text-gray-900">
          Badges
        </h1>
        <p className="mb-8 max-w-2xl text-gray-600">
          Explore community badges. These are separate from profile badges and
          represent different achievements within The Guild Genesis.
        </p>
        <BadgesList />
      </section>
    </AppWrapper>
  );
}

export default BadgesPage;
