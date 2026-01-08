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
import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useCreateBadge } from "@/hooks/badges/use-create-badge";
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

const formSchema = z.object({
  name: z.string().min(1, { message: "Name is required." }).max(32),
  description: z.string().optional(),
});

type FormValues = z.infer<typeof formSchema>;

interface CreateBadgeButtonProps {
  onBadgeCreated?: () => void;
}

export function CreateBadgeButton({ onBadgeCreated }: CreateBadgeButtonProps) {
  const [open, setOpen] = useState(false);
  const { createBadge, isPending, error, reset, isConfirmed, isConfirming } =
    useCreateBadge();

  const form = useForm<FormValues>({
    resolver: zodResolver(formSchema),
    defaultValues: { name: "", description: "" },
  });

  const onSubmit = async (values: FormValues) => {
    await createBadge(values.name, values.description || "");
  };

  useEffect(() => {
    if (isConfirmed) {
      onBadgeCreated?.();
      setOpen(false);
      form.reset();
      reset();
    }
  }, [isConfirmed, onBadgeCreated, form, reset]);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button className="flex items-center space-x-2">
          <Plus className="h-4 w-4" />
          <span>Create Badge</span>
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create Badge</DialogTitle>
          <DialogDescription>
            Provide a name and an optional description for your badge.
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
                    <Input placeholder="Bug Hunter" {...field} />
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
                      placeholder="Reports and helps resolve significant issues"
                      {...field}
                    />
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
              <Button type="submit" disabled={isPending || isConfirming}>
                {isPending
                  ? "Submitting..."
                  : isConfirming
                    ? "Waiting for confirmations..."
                    : "Create"}
              </Button>
            </div>
            {isConfirming ? (
              <p className="text-sm text-gray-600">
                Waiting for 6 block confirmations…
              </p>
            ) : null}
            {error ? (
              <p className="text-sm text-red-600">{(error as Error).message}</p>
            ) : null}
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  );
}

export default CreateBadgeButton;
