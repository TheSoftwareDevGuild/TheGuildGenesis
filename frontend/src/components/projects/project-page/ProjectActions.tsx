import { useState } from "react";
import { useAccount } from "wagmi";
import EditProjectModal from "@/components/projects/EditProjectModal";
import DeleteConfirmationDialog from "@/components/projects/DeleteConfirmationDialogue";
import type { Project } from "@/lib/types/projects";

interface ProjectActionsProps {
  id: string;
  name: string;
  description: string;
  ownerAddress: string;
}

export default function ProjectActions({
  id,
  name,
  description,
  ownerAddress,
}: ProjectActionsProps) {
  const { address } = useAccount();
  const [showEditModal, setShowEditModal] = useState(false);
  const [showDeleteDialog, setShowDeleteDialog] = useState(false);

  const isOwner = address && ownerAddress.toLowerCase() === address.toLowerCase();

  if (!isOwner) {
    return null;
  }

  const project: Project = {
    id,
    name,
    description,
    ownerAddress,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  };

  return (
    <>
      <div className="flex gap-3">
        <button
          onClick={() => setShowEditModal(true)}
          className="flex items-center gap-2 px-4 py-2 text-sm font-medium text-blue-600 bg-blue-50 rounded-lg hover:bg-blue-100 transition-colors"
        >
          <svg 
            className="w-4 h-4" 
            fill="none" 
            stroke="currentColor" 
            viewBox="0 0 24 24"
          >
            <path 
              strokeLinecap="round" 
              strokeLinejoin="round" 
              strokeWidth={2} 
              d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" 
            />
          </svg>
          Edit Project
        </button>
        <button
          onClick={() => setShowDeleteDialog(true)}
          className="flex items-center gap-2 px-4 py-2 text-sm font-medium text-red-600 bg-red-50 rounded-lg hover:bg-red-100 transition-colors"
        >
          <svg 
            className="w-4 h-4" 
            fill="none" 
            stroke="currentColor" 
            viewBox="0 0 24 24"
          >
            <path 
              strokeLinecap="round" 
              strokeLinejoin="round" 
              strokeWidth={2} 
              d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" 
            />
          </svg>
          Delete Project
        </button>
      </div>

      <EditProjectModal
        isOpen={showEditModal}
        project={project}
        onClose={() => setShowEditModal(false)}
      />

      <DeleteConfirmationDialog
        isOpen={showDeleteDialog}
        projectId={id}
        projectName={name}
        onClose={() => setShowDeleteDialog(false)}
      />
    </>
  );
}