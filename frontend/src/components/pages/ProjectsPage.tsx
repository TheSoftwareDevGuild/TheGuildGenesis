import { AppWrapper } from "@/components/AppWrapper";
import { ProjectsMain } from "@/components/projects/ProjectsMain";

export function ProjectsPage() {
  return (
    <AppWrapper>
      <section className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-10">
        <h1 className="mb-6 text-3xl font-bold tracking-tight text-gray-900">
          Projects
        </h1>
        <p className="mb-8 max-w-2xl text-gray-600">
          Showcase your work. Projects can be created by anyone and are managed by their owners.
        </p>
        <ProjectsMain />
      </section>
    </AppWrapper>
  );
}

export default ProjectsPage;