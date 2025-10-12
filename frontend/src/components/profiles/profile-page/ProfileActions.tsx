import { Edit, Plus, Trash } from "lucide-react";
import { Button } from "@/components/ui/button";
import { EditProfileDialog } from "@/components/profiles/action-buttons/EditProfileDialog";
import DeleteProfileDialog from "@/components/profiles/action-buttons/DeleteProfileDialog";
import { AddAttestationDialog } from "@/components/profiles/action-buttons/AddAttestationDialog";
import { useAccount } from "wagmi";

export function ProfileActions({
  address,
  name,
  description,
  githubLogin,
}: {
  address?: string;
  name?: string;
  description?: string;
  githubLogin?: string;
}) {
  const { address: connectedAddress } = useAccount();
  const isOwner =
    !!connectedAddress &&
    !!address &&
    connectedAddress.toLowerCase() === address.toLowerCase();

  return (
    <section className="mt-6 flex flex-wrap items-center gap-2">
      {isOwner ? (
        <EditProfileDialog
          address={address || ""}
          name={name}
          description={description}
          githubLogin={githubLogin}
        >
          <Button variant="outline" className="flex items-center gap-2">
            <Edit className="h-4 w-4" /> Edit Profile
          </Button>
        </EditProfileDialog>
      ) : null}

      {!isOwner && address ? (
        <AddAttestationDialog recipient={address}>
          <Button className="flex items-center gap-2">
            <Plus className="h-4 w-4" /> Add Attestation
          </Button>
        </AddAttestationDialog>
      ) : null}

      {isOwner ? (
        <DeleteProfileDialog>
          <Button variant="destructive" className="flex items-center gap-2">
            <Trash className="h-4 w-4" /> Delete Profile
          </Button>
        </DeleteProfileDialog>
      ) : null}
    </section>
  );
}

export default ProfileActions;
