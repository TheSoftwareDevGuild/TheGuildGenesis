import { Badge, Award, User, Edit, Trash, Plus } from "lucide-react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { EditProfileDialog } from "../action-buttons/EditProfileDialog";
import { useAccount } from "wagmi";
import DeleteProfileDialog from "../action-buttons/DeleteProfileDialog";
import { AddAttestationDialog } from "../action-buttons/AddAttestationDialog";
import AddressTokenBalance from "@/components/AddressTokenBalance";
import CopyAddressToClipboard from "@/components/CopyAddressToClipboard";

interface ProfileCardProps {
  address: string;
  name?: string;
  description?: string;
  avatar?: string;
  attestationCount: number;
  attestations: Array<{
    id: string;
    badgeName: string;
    justification: string;
    issuer: string;
  }>;
}

export function ProfileCard({
  address,
  name,
  description,
  avatar,
  attestationCount,
  attestations,
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
        <a
          href={`/profiles/${address}`}
          className="shrink-0"
          aria-label={`View ${displayName}`}
        >
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
        </a>
        <div className="min-w-0 flex-1">
          <CardTitle className="truncate text-lg flex items-center gap-2">
            <a href={`/profiles/${address}`} className="hover:underline">
              {displayName}
            </a>
            {isOwner && (
              <span className="text-xs bg-blue-100 text-blue-700 px-2 py-1 rounded-full font-medium">
                You
              </span>
            )}
          </CardTitle>
          <CardDescription>
            <a href={`/profiles/${address}`} className="hover:underline font-mono">
              {displayAddress}
            </a>
            <CopyAddressToClipboard 
              address={address}
              className="ml-2"
              iconSize="sm"
            />
          </CardDescription>
          <AddressTokenBalance address={address as `0x${string}`} />
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
        {!isOwner && (
          <AddAttestationDialog recipient={address}>
            <Button
              variant="ghost"
              size="sm"
              className="absolute top-2 right-2 h-8 w-8 p-0"
              aria-label="Add attestation"
            >
              <Plus className="h-4 w-4" />
            </Button>
          </AddAttestationDialog>
        )}
      </CardHeader>
      <CardContent>
        {description && (
          <p className="text-sm text-gray-600 line-clamp-2">{description}</p>
        )}

        <div className="mt-4 flex items-center space-x-4">
          <div className="flex items-center space-x-1 text-sm text-gray-500">
            <Award className="h-4 w-4" />
            <span>{attestationCount} attestations</span>
          </div>
        </div>

        {attestations.length > 0 && (
          <div className="mt-3 flex flex-wrap gap-2">
            {attestations.slice(0, 3).map((attestation) => (
              <span
                key={attestation.id}
                className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800"
              >
                <Badge className="h-3 w-3 mr-1" />
                {attestation.badgeName}
              </span>
            ))}
            {attestations.length > 3 && (
              <span className="text-xs text-gray-500">
                +{attestations.length - 3} more
              </span>
            )}
          </div>
        )}
        {isOwner && (
          <DeleteProfileDialog>
            <Button
              variant="destructive"
              size="sm"
              className="absolute bottom-2 right-2 h-8 w-8 p-0"
              aria-label="Delete profile"
            >
              <Trash className="h-4 w-4" />
            </Button>
          </DeleteProfileDialog>
        )}
      </CardContent>
    </Card>
  );
}

export default ProfileCard;
