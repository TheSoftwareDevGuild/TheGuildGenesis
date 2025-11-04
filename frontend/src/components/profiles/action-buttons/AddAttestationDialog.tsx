import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
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
import { useCreateAttestation } from "@/hooks/attestations/use-create-attestation";
import { useGetBadges } from "@/hooks/badges/use-get-badges";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useGetAttestations } from "@/hooks/attestations/use-get-attestations";
import { useGetActivityTokenBalance } from "@/hooks/attestations/use-get-activity-token-balance";

const formSchema = z.object({
  badgeName: z.string().min(1, { message: "Badge name is required." }),
  justification: z.string().min(1, { message: "Justification is required." }),
});

type FormValues = z.infer<typeof formSchema>;

export function AddAttestationDialog({
  recipient,
  children,
}: {
  recipient: string;
  children: React.ReactElement;
}) {
  const [open, setOpen] = useState(false);
  const [errorDisplayText, setErrorDisplayText] = useState<string | null>(null);
  const {
    createAttestation,
    isPending,
    isConfirming,
    isConfirmed,
    error,
    reset,
  } = useCreateAttestation();
  const { data: badges, isLoading: badgesLoading } = useGetBadges();
  const { refetch: refetchAttestations } = useGetAttestations();
  const { refetch: refetchActivityTokenBalance } = useGetActivityTokenBalance();

  const form = useForm<FormValues>({
    resolver: zodResolver(formSchema),
    defaultValues: { badgeName: "", justification: "" },
  });

  const onSubmit = async (values: FormValues) => {
    try {
      setErrorDisplayText(null);
      await createAttestation(
        recipient as `0x${string}`,
        values.badgeName,
        values.justification
      );
    } catch (e) {
      const message = (e as Error)?.message || String(e);
      console.log("message", message);
      const isRpcRevert =
        message.includes("Internal JSON-RPC error") ||
        message.includes('Contract function "attest" reverted') ||
        message.toLowerCase().includes("revert");
      if (isRpcRevert) {
        setErrorDisplayText(
          "The network temporarily rejected this request. Please try again later."
        );
      } else {
        setErrorDisplayText(message);
      }
    }
  };

  useEffect(() => {
    if (isConfirmed) {
      refetchAttestations();
      refetchActivityTokenBalance();
      setOpen(false);
      form.reset();
      reset();
      setErrorDisplayText(null);
    }
  }, [
    isConfirmed,
    refetchAttestations,
    refetchActivityTokenBalance,
    form,
    reset,
  ]);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>{children}</DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Add Attestation</DialogTitle>
          <DialogDescription>
            Select a badge and provide a justification for this attestation.
          </DialogDescription>
        </DialogHeader>
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
            <FormField
              control={form.control}
              name="badgeName"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Badge</FormLabel>
                  <FormControl>
                    <Select
                      disabled={badgesLoading}
                      value={field.value}
                      onValueChange={field.onChange}
                    >
                      <SelectTrigger className="w-full">
                        <SelectValue
                          placeholder={
                            badgesLoading ? "Loading badges…" : "Select a badge"
                          }
                        />
                      </SelectTrigger>
                      <SelectContent>
                        {(badges || []).map((b) => (
                          <SelectItem key={b.name} value={b.name}>
                            {b.name}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="justification"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Justification</FormLabel>
                  <FormControl>
                    <Input
                      placeholder="Why are you issuing this badge?"
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
                    : "Add"}
              </Button>
            </div>
            {isConfirming ? (
              <p className="text-sm text-gray-600">
                Waiting for confirmations…
              </p>
            ) : null}
            {errorDisplayText ? (
              <p className="text-sm text-red-600">{errorDisplayText}</p>
            ) : error ? (
              <p className="text-sm text-red-600">{(error as Error).message}</p>
            ) : null}
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  );
}

export default AddAttestationDialog;
