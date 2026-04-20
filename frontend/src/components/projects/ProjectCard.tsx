import React from 'react';
import type { Project } from '@/lib/types/projects';
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardFooter,
  CardAction,
} from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Pencil, Trash2, User, Calendar, Check } from 'lucide-react';

interface ProjectCardProps {
  project: Project;
  isOwner: boolean;
  onEdit: () => void;
  onDelete: () => void;
}

const formatAddress = (address: string): string => {
  if (!address) return '';
  return `${address.slice(0, 6)}...${address.slice(-4)}`;
};

const formatDate = (dateString: string): string => {
  const date = new Date(dateString);
  return date.toLocaleDateString('en-US', { 
    year: 'numeric', 
    month: 'short', 
    day: 'numeric' 
  });
};

export default function ProjectCard({ 
  project, 
  isOwner, 
  onEdit, 
  onDelete 
}: ProjectCardProps) {
  return (
    <Card 
      with3D={true}
      foregroundIcon={
        <svg 
          className="w-24 h-24 text-blue-600/10" 
          fill="currentColor"
          viewBox="0 0 24 24"
        >
          <path d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
        </svg>
      }
      className="group hover:shadow-xl transition-shadow duration-300"
    >
      <CardHeader>
        {/* Project Icon */}
        <div className="flex justify-center mb-4">
          <div className="w-16 h-16 bg-blue-100 rounded-full flex items-center justify-center">
            <svg 
              className="w-8 h-8 text-blue-600" 
              fill="none" 
              stroke="currentColor" 
              viewBox="0 0 24 24"
            >
              <path 
                strokeLinecap="round" 
                strokeLinejoin="round" 
                strokeWidth={2} 
                d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" 
              />
            </svg>
          </div>
        </div>

        <CardTitle className="text-xl text-center">
          {project.name}
        </CardTitle>
        
        <CardDescription className="text-center line-clamp-3 min-h-[60px]">
          {project.description}
        </CardDescription>

        {/* Action Buttons (only for owner) */}
        {isOwner && (
          <CardAction>
            <div className="flex gap-1">
              <Button
                variant="ghost"
                size="icon"
                onClick={onEdit}
                className="text-blue-600 hover:bg-blue-50 hover:text-blue-700"
                title="Edit project"
              >
                <Pencil />
              </Button>
              <Button
                variant="ghost"
                size="icon"
                onClick={onDelete}
                className="text-red-600 hover:bg-red-50 hover:text-red-700"
                title="Delete project"
              >
                <Trash2 />
              </Button>
            </div>
          </CardAction>
        )}
      </CardHeader>

      <CardContent>
        {/* Owner Info */}
        <div className="flex items-center justify-center gap-2 text-xs text-muted-foreground mb-2">
          <User className="w-4 h-4" />
          <span className="truncate">{formatAddress(project.ownerAddress)}</span>
        </div>

        {/* Created Date */}
        <div className="flex items-center justify-center gap-2 text-xs text-muted-foreground">
          <Calendar className="w-4 h-4" />
          <span>{formatDate(project.createdAt)}</span>
        </div>
      </CardContent>

      {/* Owner Badge */}
      {isOwner && (
        <CardFooter className="border-t">
          <div className="flex items-center gap-2 text-xs text-blue-600 font-medium">
            <Check className="w-4 h-4" />
            Your Project
          </div>
        </CardFooter>
      )}
    </Card>
  );
}