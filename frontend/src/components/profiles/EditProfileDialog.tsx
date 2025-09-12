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
import { Input } from "@/components/ui/input";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { useUpdateProfile } from "@/hooks/use-update-profile";
import { useQueryClient } from "@tanstack/react-query";
import { useState } from "react";

interface EditProfileDialogProps {
  address: string;
  name?: string;
  description?: string;
  children: React.ReactNode;
}

export function EditProfileDialog({
  address,
  name,
  description,
  children,
}: EditProfileDialogProps) {
  const [open, setOpen] = useState(false);
  const updateProfile = useUpdateProfile();
  const queryClient = useQueryClient();

  const form = useForm<{
    name: string;
    description?: string;
  }>({
    resolver: zodResolver(
      z.object({
        name: z
          .string()
          .min(2, { message: "Name must be at least 2 characters." }),
        description: z.string().optional(),
      })
    ),
    defaultValues: { name: name || "", description: description || "" },
  });

  const onSubmit = async (values: { name: string; description?: string }) => {
    await updateProfile.mutateAsync({
      input: {
        name: values.name,
        description: values.description || "",
        siweMessage: "LOGIN_NONCE",
      },
    });
    await queryClient.invalidateQueries({ queryKey: ["profiles"] });
    setOpen(false);
  };

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>{children}</DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Edit Profile</DialogTitle>
          <DialogDescription>
            Update your profile information.
          </DialogDescription>
        </DialogHeader>
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
            <FormField
              control={form.control}
              name="name"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Name</FormLabel>
                  <FormControl>
                    <Input placeholder="My profile" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="description"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Description</FormLabel>
                  <FormControl>
                    <Input placeholder="Hello world" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <div className="flex justify-end gap-2 pt-2">
              <DialogClose asChild>
                <Button type="button" variant="secondary">
                  Cancel
                </Button>
              </DialogClose>
              <Button type="submit" disabled={updateProfile.isPending}>
                {updateProfile.isPending ? "Updating..." : "Update"}
              </Button>
            </div>
            {updateProfile.isError ? (
              <p className="text-sm text-red-600">
                {(updateProfile.error as Error).message}
              </p>
            ) : null}
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  );
}

export default EditProfileDialog;
