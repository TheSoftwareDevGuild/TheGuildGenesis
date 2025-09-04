import { Badge, Award, User } from 'lucide-react';

interface ProfileCardProps {
  address: string;
  name?: string;
  description?: string;
  avatar?: string;
  badgeCount: number;
  badges: Array<{
    id: string;
    name: string;
    description: string;
    issuer: string;
  }>;
}

export function ProfileCard({ 
  address, 
  name, 
  description, 
  avatar, 
  badgeCount, 
  badges 
}: ProfileCardProps) {
  const displayName = name || `${address.slice(0, 6)}...${address.slice(-4)}`;
  const displayAddress = `${address.slice(0, 6)}...${address.slice(-4)}`;
  
  return (
    <div className="bg-white rounded-lg shadow-sm border p-6 hover:shadow-md transition-shadow">
      <div className="flex items-start space-x-4">
        <div className="flex-shrink-0">
          {avatar ? (
            <img
              src={avatar}
              alt={displayName}
              className="h-12 w-12 rounded-full object-cover"
            />
          ) : (
            <div className="h-12 w-12 rounded-full bg-gray-200 flex items-center justify-center">
              <User className="h-6 w-6 text-gray-400" />
            </div>
          )}
        </div>
        
        <div className="flex-1 min-w-0">
          <h3 className="text-lg font-semibold text-gray-900 truncate">
            {displayName}
          </h3>
          <p className="text-sm text-gray-500 font-mono">
            {displayAddress}
          </p>
          {description && (
            <p className="mt-2 text-sm text-gray-600 line-clamp-2">
              {description}
            </p>
          )}
          
          <div className="mt-4 flex items-center space-x-4">
            <div className="flex items-center space-x-1 text-sm text-gray-500">
              <Award className="h-4 w-4" />
              <span>{badgeCount} badges</span>
            </div>
          </div>
          
          {badges.length > 0 && (
            <div className="mt-3 flex flex-wrap gap-2">
              {badges.slice(0, 3).map((badge) => (
                <span
                  key={badge.id}
                  className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800"
                >
                  <Badge className="h-3 w-3 mr-1" />
                  {badge.name}
                </span>
              ))}
              {badges.length > 3 && (
                <span className="text-xs text-gray-500">
                  +{badges.length - 3} more
                </span>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
