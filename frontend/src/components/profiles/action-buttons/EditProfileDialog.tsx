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
import { useUpdateProfile } from "@/hooks/profiles/use-update-profile";
import { useQueryClient } from "@tanstack/react-query";
import { useState, useEffect } from "react";
import { useGetNonce } from "@/hooks/profiles/use-get-nonce";
import { generateSiweMessage } from "@/lib/utils/siwe";
import { useAccount, useSignMessage } from "wagmi";

interface EditProfileDialogProps {
  address: string;
  name?: string;
  description?: string;
  githubLogin?: string;
  children: React.ReactNode;
}

const formSchema = z.object({
  name: z.string().min(2, { message: "Name must be at least 2 characters." }),
  description: z.string().optional(),
  githubLogin: z.string().optional(),
});

type FormValues = z.infer<typeof formSchema>;

export function EditProfileDialog({
  address,
  name,
  description,
  githubLogin,
  children,
}: EditProfileDialogProps) {
  const [open, setOpen] = useState(false);
  const updateProfile = useUpdateProfile();
  const queryClient = useQueryClient();
  const { address: signerAddress } = useAccount();
  const { signMessageAsync } = useSignMessage();
  const { data: nonceData, isLoading: isLoadingNonce } =
    useGetNonce(signerAddress);

  const siweMessage = nonceData ? generateSiweMessage(nonceData) : "";

  const form = useForm<FormValues>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: name || "",
      description: description || "",
      githubLogin: githubLogin || "",
    },
  });

  useEffect(() => {
    if (open) {
      form.reset({
        name: name || "",
        description: description || "",
        githubLogin: githubLogin || "",
      });
    }
  }, [open, name, description, githubLogin, form]);

  const onSubmit = async (values: FormValues) => {
    if (!siweMessage) {
      throw new Error("SIWE message not available");
    }

    try {
      // Sign the SIWE message
      const signature = await signMessageAsync({ message: siweMessage });

      await updateProfile.mutateAsync({
        input: {
          name: values.name,
          description: values.description || "",
          github_login: values.githubLogin || "",
        },
        signature,
      });
      await queryClient.invalidateQueries({ queryKey: ["profiles"] });
      setOpen(false);
    } catch (error) {
      console.error("Failed to update profile:", error);
    }
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
                    <Input
                      placeholder="Write a short introduction..."
                      {...field}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="githubLogin"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>GitHub Handle</FormLabel>
                  <FormControl>
                    <div className="flex items-center gap-2">
                      <span className="text-sm text-gray-500">@</span>
                      <Input placeholder="username" {...field} />
                    </div>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            {siweMessage && (
              <div className="space-y-2">
                <FormLabel>Message to Sign</FormLabel>
                <div className="p-3 bg-gray-50 rounded-md text-sm font-mono break-all">
                  {siweMessage}
                </div>
                <p className="text-xs text-gray-600">
                  This message will be signed with your wallet to authenticate
                  your profile update.
                </p>
              </div>
            )}
            <div className="flex justify-end gap-2 pt-2">
              <DialogClose asChild>
                <Button type="button" variant="secondary">
                  Cancel
                </Button>
              </DialogClose>
              <Button
                type="submit"
                disabled={
                  updateProfile.isPending || isLoadingNonce || !siweMessage
                }
              >
                {isLoadingNonce
                  ? "Loading..."
                  : updateProfile.isPending
                    ? "Updating..."
                    : "Update"}
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
