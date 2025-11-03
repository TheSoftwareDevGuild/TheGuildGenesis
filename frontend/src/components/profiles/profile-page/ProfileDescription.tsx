interface ProfileDescriptionProps {
  description?: string;
}

export function ProfileDescription({
  description,
}: ProfileDescriptionProps) {
  if (!description) {
    return null;
  }

  return (
    <section className="mt-6 py-4 border-t border-b border-gray-200">
      <p className="text-gray-700 whitespace-pre-wrap">{description}</p>
    </section>
  );
}

export default ProfileDescription;
