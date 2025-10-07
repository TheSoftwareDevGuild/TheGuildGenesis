import React from "react";

interface ProfileDescriptionProps {
  description?: string;
}

export const ProfileDescription: React.FC<ProfileDescriptionProps> = ({ description }) => {
  if (!description) return null;
  return (
    <div className="my-6">
      <p className="text-lg text-gray-700 whitespace-pre-line">{description}</p>
    </div>
  );
};

export default ProfileDescription;
