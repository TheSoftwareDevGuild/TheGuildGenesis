import { useMemo } from "react";
import { useProjects } from "@/hooks/projects/useprojects";
import ProjectHeader from "@/components/projects/project-page/ProjectHeader";
import ProjectActions from "@/components/projects/project-page/ProjectActions";
import ProjectDescription from "@/components/projects/project-page/ProjectDescription";

export function ProjectMain({ id }: { id: string }) {
  const projectsQuery = useProjects();

  const project = useMemo(() => {
    const list = projectsQuery.data ?? [];
    const p = list.find((x) => x.id === id);
    return p;
  }, [projectsQuery.data, id]);

  if (projectsQuery.isLoading) {
    return (
      <div className="max-w-4xl mx-auto p-6">
        <div className="flex justify-center items-center py-12">
          <svg className="animate-spin h-12 w-12 text-gray-400" viewBox="0 0 24 24">
            <circle
              className="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              strokeWidth="4"
              fill="none"
            />
            <path
              className="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            />
          </svg>
        </div>
      </div>
    );
  }

  if (!project) {
    return (
      <div className="max-w-4xl mx-auto p-6">
        <div className="text-center py-12">
          <h2 className="text-2xl font-bold text-gray-900 mb-2">Project Not Found</h2>
          <p className="text-gray-600">The project you're looking for doesn't exist.</p>
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto p-6">
      <ProjectHeader
        id={project.id}
        name={project.name}
        ownerAddress={project.ownerAddress}
        createdAt={project.createdAt}
      />
      <div className="mt-6">
        <ProjectActions
          id={project.id}
          name={project.name}
          description={project.description}
          ownerAddress={project.ownerAddress}
        />
      </div>
      <ProjectDescription description={project.description} />
    </div>
  );
}