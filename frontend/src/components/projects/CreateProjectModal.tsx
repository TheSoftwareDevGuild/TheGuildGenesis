import React, { useState, useEffect } from 'react';
import { useCreateProject } from '@/hooks/projects/useprojects';
import { Button } from '@/components/ui/button';
import { Loader2, X } from 'lucide-react';

interface CreateProjectModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export default function CreateProjectModal({ isOpen, onClose }: CreateProjectModalProps) {
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [validationError, setValidationError] = useState('');

  const createProjectMutation = useCreateProject();

  useEffect(() => {
    if (isOpen) {
      setName('');
      setDescription('');
      setValidationError('');
    }
  }, [isOpen]);

  useEffect(() => {
    if (createProjectMutation.isSuccess) {
      onClose();
    }
  }, [createProjectMutation.isSuccess, onClose]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setValidationError('');

    // Validation
    if (!name.trim()) {
      setValidationError('Project name is required');
      return;
    }

    if (name.length > 100) {
      setValidationError('Project name must be 100 characters or less');
      return;
    }

    if (!description.trim()) {
      setValidationError('Project description is required');
      return;
    }

    if (description.length > 500) {
      setValidationError('Description must be 500 characters or less');
      return;
    }

    createProjectMutation.mutate({ input: { name: name.trim(), description: description.trim(),  status: "proposal" } });
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-white rounded-lg max-w-md w-full p-6">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-2xl font-bold text-gray-900">Create Project</h2>
          <Button
            variant="ghost"
            size="icon"
            onClick={onClose}
            disabled={createProjectMutation.isPending}
            className="text-gray-400 hover:text-gray-600"
          >
            <X />
          </Button>
        </div>

        <form onSubmit={handleSubmit} className="space-y-4">
          {/* Project Name */}
          <div>
            <label htmlFor="name" className="block text-sm font-medium text-gray-700 mb-1">
              Project Name <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              id="name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              placeholder="My Awesome Project"
              maxLength={100}
              disabled={createProjectMutation.isPending}
            />
            <p className="text-xs text-gray-500 mt-1">{name.length}/100 characters</p>
          </div>

          {/* Description */}
          <div>
            <label htmlFor="description" className="block text-sm font-medium text-gray-700 mb-1">
              Description <span className="text-red-500">*</span>
            </label>
            <textarea
              id="description"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none"
              placeholder="Describe your project..."
              rows={4}
              maxLength={500}
              disabled={createProjectMutation.isPending}
            />
            <p className="text-xs text-gray-500 mt-1">{description.length}/500 characters</p>
          </div>

          {/* Error Messages */}
          {(validationError || createProjectMutation.error) && (
            <div className="bg-red-50 border border-red-200 rounded-lg p-3">
              <p className="text-red-800 text-sm">
                {validationError || (createProjectMutation.error as Error)?.message}
              </p>
            </div>
          )}

          {/* Action Buttons */}
          <div className="flex gap-3 pt-2">
            <Button
              type="button"
              variant="outline"
              onClick={onClose}
              disabled={createProjectMutation.isPending}
              className="flex-1"
            >
              Cancel
            </Button>
            <Button
              type="submit"
              variant="default"
              disabled={createProjectMutation.isPending}
              className="flex-1 bg-black hover:bg-gray-800"
            >
              {createProjectMutation.isPending ? (
                <>
                  <Loader2 className="animate-spin" />
                  Creating...
                </>
              ) : (
                'Create Project'
              )}
            </Button>
          </div>
        </form>
      </div>
    </div>
  );
}