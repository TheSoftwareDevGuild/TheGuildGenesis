interface ProjectDescriptionProps {
  description?: string;
}

export default function ProjectDescription({
  description,
}: ProjectDescriptionProps) {
  if (!description) {
    return null;
  }

  return (
    <div className="mt-8">
      <h2 className="text-xl font-semibold text-gray-900 mb-4">Description</h2>
      <div className="prose prose-gray max-w-none">
        <p className="text-gray-700 whitespace-pre-wrap">{description}</p>
      </div>
    </div>
  );
}