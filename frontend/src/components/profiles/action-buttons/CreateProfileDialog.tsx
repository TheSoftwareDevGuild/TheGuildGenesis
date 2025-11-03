import { Plus } from "lucide-react";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { useCreateProfile } from "@/hooks/profiles/use-create-profile";
import { useQueryClient } from "@tanstack/react-query";
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
import { useGetNonce } from "@/hooks/profiles/use-get-nonce";
import { generateSiweMessage } from "@/lib/utils/siwe";
import { useAccount, useSignMessage } from "wagmi";

const formSchema = z.object({
  name: z.string().min(2, { message: "Name must be at least 2 characters." }),
  description: z.string().optional(),
  githubLogin: z.string().optional(),
});

type FormValues = z.infer<typeof formSchema>;

export function CreateProfileButton() {
  const [open, setOpen] = useState(false);
  const createProfile = useCreateProfile();
  const queryClient = useQueryClient();
  const { address } = useAccount();
  const { signMessageAsync } = useSignMessage();
  const { data: nonceData, isLoading: isLoadingNonce } = useGetNonce(address);

  const siweMessage = nonceData ? generateSiweMessage(nonceData) : "";

  const form = useForm<FormValues>({
    resolver: zodResolver(formSchema),
    defaultValues: { name: "", description: "" },
  });

  const onSubmit = async (values: FormValues) => {
    if (!siweMessage) {
      throw new Error("SIWE message not available");
    }

    // Sign the SIWE message
    const signature = await signMessageAsync({ message: siweMessage });

    await createProfile.mutateAsync({
      input: {
        name: values.name,
        description: values.description || "",
        github_login: values.githubLogin || "",
      },
      signature,
    });
    await queryClient.invalidateQueries({ queryKey: ["profiles"] });
    setOpen(false);
    form.reset();
  };

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button className="flex items-center space-x-2">
          <Plus className="h-4 w-4" />
          <span>Create Profile</span>
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create Profile</DialogTitle>
          <DialogDescription>
            Provide a name and an optional description for your profile.
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
                    <Textarea
                      placeholder="Write a short introduction..."
                      className="resize-none"
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
                  your profile creation.
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
                  createProfile.isPending || isLoadingNonce || !siweMessage
                }
              >
                {isLoadingNonce
                  ? "Loading..."
                  : createProfile.isPending
                    ? "Creating..."
                    : "Create"}
              </Button>
            </div>
            {createProfile.isError ? (
              <p className="text-sm text-red-600">
                {(createProfile.error as Error).message}
              </p>
            ) : null}
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  );
}

export default CreateProfileButton;
