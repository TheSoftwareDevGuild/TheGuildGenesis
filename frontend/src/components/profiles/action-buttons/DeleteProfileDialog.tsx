import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { useDeleteProfile } from "@/hooks/profiles/use-delete-profile";
import { useQueryClient } from "@tanstack/react-query";
import { useState } from "react";

interface DeleteProfileDialogProps {
  children: React.ReactNode;
}

export function DeleteProfileDialog({ children }: DeleteProfileDialogProps) {
  const [open, setOpen] = useState(false);
  const deleteProfile = useDeleteProfile();
  const queryClient = useQueryClient();

  const onConfirm = async () => {
    await deleteProfile.mutateAsync();
    await queryClient.invalidateQueries({ queryKey: ["profiles"] });
    setOpen(false);
  };

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>{children}</DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Delete Profile</DialogTitle>
          <DialogDescription>
            This action cannot be undone. This will permanently delete your
            profile.
          </DialogDescription>
        </DialogHeader>
        <div className="flex justify-end gap-2 pt-2">
          <DialogClose asChild>
            <Button type="button" variant="secondary">
              Cancel
            </Button>
          </DialogClose>
          <Button
            variant="destructive"
            onClick={onConfirm}
            disabled={deleteProfile.isPending}
          >
            {deleteProfile.isPending ? "Deleting..." : "Delete"}
          </Button>
        </div>
        {deleteProfile.isError ? (
          <p className="text-sm text-red-600">
            {(deleteProfile.error as Error).message}
          </p>
        ) : null}
      </DialogContent>
    </Dialog>
  );
}

export default DeleteProfileDialog;
