import { Badge, Award, User, Edit } from "lucide-react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { EditProfileDialog } from "./EditProfileDialog";
import { useAccount } from "wagmi";

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
  badges,
}: ProfileCardProps) {
  const { address: connectedAddress } = useAccount();
  const isOwner = connectedAddress?.toLowerCase() === address.toLowerCase();

  const displayName = name || `${address.slice(0, 6)}...${address.slice(-4)}`;
  const displayAddress = `${address.slice(0, 6)}...${address.slice(-4)}`;

  return (
    <Card
      className={`hover:shadow-md transition-shadow relative ${
        isOwner ? "border-blue-200 bg-blue-50/30 ring-1 ring-blue-100" : ""
      }`}
    >
      <CardHeader className="flex flex-row items-start gap-4">
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
        <div className="min-w-0 flex-1">
          <CardTitle className="truncate text-lg flex items-center gap-2">
            {displayName}
            {isOwner && (
              <span className="text-xs bg-blue-100 text-blue-700 px-2 py-1 rounded-full font-medium">
                You
              </span>
            )}
          </CardTitle>
          <CardDescription className="font-mono">
            {displayAddress}
          </CardDescription>
        </div>
        {isOwner && (
          <EditProfileDialog
            address={address}
            name={name}
            description={description}
          >
            <Button
              variant="ghost"
              size="sm"
              className="absolute top-2 right-2 h-8 w-8 p-0"
            >
              <Edit className="h-4 w-4" />
            </Button>
          </EditProfileDialog>
        )}
      </CardHeader>
      <CardContent>
        {description && (
          <p className="text-sm text-gray-600 line-clamp-2">{description}</p>
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
      </CardContent>
    </Card>
  );
}

export default ProfileCard;
