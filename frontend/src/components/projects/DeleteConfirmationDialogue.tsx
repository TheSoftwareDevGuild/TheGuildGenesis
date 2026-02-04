import React, { useEffect } from 'react';
import { useDeleteProject } from '@/hooks/projects/useprojects';

interface DeleteConfirmationDialogProps {
  isOpen: boolean;
  projectId: string | null;
  projectName: string;
  onClose: () => void;
}

export default function DeleteConfirmationDialog({
  isOpen,
  projectId,
  projectName,
  onClose,
}: DeleteConfirmationDialogProps) {
  const deleteProjectMutation = useDeleteProject();

  useEffect(() => {
    if (deleteProjectMutation.isSuccess) {
      onClose();
    }
  }, [deleteProjectMutation.isSuccess, onClose]);

  const handleConfirm = () => {
    if (projectId) {
      deleteProjectMutation.mutate({ id: projectId });
    }
  };

  if (!isOpen || !projectId) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-white rounded-lg max-w-md w-full p-6">
        {/* Warning Icon */}
        <div className="flex justify-center mb-4">
          <div className="w-12 h-12 bg-red-100 rounded-full flex items-center justify-center">
            <svg className="w-6 h-6 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
            </svg>
          </div>
        </div>

        {/* Title */}
        <h2 className="text-xl font-bold text-gray-900 text-center mb-2">
          Delete Project
        </h2>

        {/* Message */}
        <p className="text-gray-600 text-center mb-6">
          Are you sure you want to delete{' '}
          <span className="font-semibold text-gray-900">"{projectName}"</span>?
          This action cannot be undone.
        </p>

        {/* Error Message */}
        {deleteProjectMutation.error && (
          <div className="bg-red-50 border border-red-200 rounded-lg p-3 mb-4">
            <p className="text-red-800 text-sm">
              {(deleteProjectMutation.error as Error)?.message || 'Failed to delete project'}
            </p>
          </div>
        )}

        {/* Action Buttons */}
        <div className="flex gap-3">
          <button
            type="button"
            onClick={onClose}
            className="flex-1 px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors"
            disabled={deleteProjectMutation.isPending}
          >
            Cancel
          </button>
          <button
            type="button"
            onClick={handleConfirm}
            className="flex-1 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors disabled:bg-red-400 disabled:cursor-not-allowed flex items-center justify-center"
            disabled={deleteProjectMutation.isPending}
          >
            {deleteProjectMutation.isPending ? (
              <>
                <svg className="animate-spin h-5 w-5 mr-2" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" fill="none" />
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
                </svg>
                Deleting...
              </>
            ) : (
              'Delete Project'
            )}
          </button>
        </div>
      </div>
    </div>
  );
}