import { AppWrapper } from "@/components/AppWrapper";
import { ProjectMain } from "@/components/projects/project-page/ProjectMain";

type Props = { id?: string };

export default function ProjectPage({ id }: Props) {
  return (
    <AppWrapper>
      <ProjectMain id={id || ""} />
    </AppWrapper>
  );
}